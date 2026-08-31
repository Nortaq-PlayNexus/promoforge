use promoforge_core::*;
use promoforge_ingestion::{FilesystemWalker, ProjectClassifier, ProjectAnalyzer};
use promoforge_pkb::PkbBuilder;
use promoforge_storage::Storage;
use promoforge_strategy::StrategyEngine;
use std::sync::OnceLock;
use tauri::AppHandle;
use tauri::Manager;

static STORAGE: OnceLock<Storage> = OnceLock::new();

fn get_storage() -> &'static Storage {
    STORAGE.get().expect("Storage not initialized")
}

pub async fn init_storage(app: &AppHandle) -> Result<()> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| Error::Config(format!("Cannot get data dir: {}", e)))?;

    std::fs::create_dir_all(&data_dir)?;

    let db_path = data_dir.join("promoforge.db");
    let db_str = db_path.to_string_lossy().to_string();

    let storage = Storage::new(&db_str).await?;
    STORAGE.set(storage).map_err(|_| Error::Config("Storage already initialized".into()))?;

    tracing::info!("Storage initialized at: {}", db_str);
    Ok(())
}

// ─── Project Commands ───────────────────────────────────────────────

#[tauri::command]
pub async fn create_project(path: String, name: Option<String>) -> Result<Project> {
    let storage = get_storage();

    let manifest = FilesystemWalker::scan(&path)?;
    let (project_type, _confidence) = ProjectClassifier::classify(&manifest);

    let project_name = name.unwrap_or_else(|| ProjectClassifier::infer_name(&manifest, &project_type));

    let project = Project {
        id: new_id(),
        name: project_name,
        path: path.clone(),
        project_type,
        status: ProjectStatus::Analyzing,
        created_at: now(),
        updated_at: now(),
        pkb_version: None,
        settings: ProjectSettings::default(),
    };

    storage.create_project(&project).await?;
    Ok(project)
}

#[tauri::command]
pub async fn get_projects() -> Result<Vec<Project>> {
    let storage = get_storage();
    storage.list_projects().await
}

#[tauri::command]
pub async fn get_project(id: String) -> Result<Project> {
    let storage = get_storage();
    storage
        .get_project(&id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Project not found: {}", id)))
}

// ─── Analysis Commands ──────────────────────────────────────────────

#[tauri::command]
pub async fn analyze_project(project_id: String) -> Result<ProductKnowledgeBase> {
    let storage = get_storage();

    let project = storage
        .get_project(&project_id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Project not found: {}", project_id)))?;

    let (mut updated_project, mut pkb) = PkbBuilder::build_from_path(&project.path)?;
    pkb.project_id = project.id.clone();
    updated_project.id = project.id.clone();
    updated_project.name = project.name.clone();

    let validation_issues = PkbBuilder::validate(&pkb);
    if !validation_issues.is_empty() {
        tracing::warn!("PKB validation issues: {:?}", validation_issues);
    }

    storage.save_pkb(&pkb).await?;

    updated_project.status = ProjectStatus::Ready;
    updated_project.pkb_version = Some(1);
    updated_project.updated_at = now();
    storage.update_project(&updated_project).await?;

    Ok(pkb)
}

#[tauri::command]
pub async fn get_pkb(project_id: String) -> Result<ProductKnowledgeBase> {
    let storage = get_storage();
    storage
        .get_pkb(&project_id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("PKB not found for project: {}", project_id)))
}

// ─── Strategy Commands ──────────────────────────────────────────────

#[tauri::command]
pub async fn generate_strategy(project_id: String) -> Result<Strategy> {
    let storage = get_storage();

    let project = storage
        .get_project(&project_id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Project not found: {}", project_id)))?;

    let pkb = storage
        .get_pkb(&project_id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("PKB not found. Run analysis first.")))?;

    let (campaign, strategy) = StrategyEngine::generate_strategy(&pkb, &project)?;

    storage.create_campaign(&campaign).await?;
    storage.save_strategy(&strategy).await?;

    Ok(strategy)
}

// ─── Generation Commands ────────────────────────────────────────────

#[tauri::command]
pub async fn generate_text_assets(
    project_id: String,
    asset_type: String,
    platform: String,
    count: usize,
) -> Result<Vec<ContentAsset>> {
    let storage = get_storage();

    let pkb = storage
        .get_pkb(&project_id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("PKB not found. Run analysis first.")))?;

    // Check if LLM is configured
    let settings_json = storage.get_setting("settings").await?;
    let settings: AppSettings = settings_json
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    let provider_config = settings
        .ai_providers
        .values()
        .find(|p| p.enabled && p.api_key.is_some())
        .ok_or_else(|| Error::Config("No AI provider configured with API key. Go to Settings to configure.".into()))?;

    let llm = promoforge_api::LlmClient::new(
        &provider_config.provider,
        provider_config.api_key.as_ref().unwrap(),
        &provider_config.model,
    );

    let generator = promoforge_api::TextGenerator::new(&llm);

    let mut assets = Vec::new();

    match asset_type.as_str() {
        "social" => {
            let posts = generator.generate_social_posts(&pkb, &platform, count).await?;
            for post in posts {
                let asset = ContentAsset {
                    id: new_id(),
                    project_id: project_id.clone(),
                    campaign_id: None,
                    asset_type: AssetType::Text,
                    subtype: "social_post".into(),
                    platform: platform.clone(),
                    status: AssetStatus::Draft,
                    content: AssetContent {
                        text: Some(post),
                        file_path: None,
                        metadata: HashMap::new(),
                    },
                    variants: Vec::new(),
                    performance: AssetPerformance::default(),
                    created_at: now(),
                    updated_at: now(),
                };
                storage.save_asset(&asset).await?;
                assets.push(asset);
            }
        }
        "store" => {
            let metadata = generator.generate_store_metadata(&pkb, &platform).await?;
            let asset = ContentAsset {
                id: new_id(),
                project_id: project_id.clone(),
                campaign_id: None,
                asset_type: AssetType::Text,
                subtype: "store_metadata".into(),
                platform: platform.clone(),
                status: AssetStatus::Draft,
                content: AssetContent {
                    text: Some(serde_json::to_string_pretty(&metadata)?),
                    file_path: None,
                    metadata: serde_json::from_value(metadata).unwrap_or_default(),
                },
                variants: Vec::new(),
                performance: AssetPerformance::default(),
                created_at: now(),
                updated_at: now(),
            };
            storage.save_asset(&asset).await?;
            assets.push(asset);
        }
        "press_kit" => {
            let press_kit = generator.generate_press_kit(&pkb).await?;
            let asset = ContentAsset {
                id: new_id(),
                project_id: project_id.clone(),
                campaign_id: None,
                asset_type: AssetType::Document,
                subtype: "press_kit".into(),
                platform: "all".into(),
                status: AssetStatus::Draft,
                content: AssetContent {
                    text: Some(press_kit),
                    file_path: None,
                    metadata: HashMap::new(),
                },
                variants: Vec::new(),
                performance: AssetPerformance::default(),
                created_at: now(),
                updated_at: now(),
            };
            storage.save_asset(&asset).await?;
            assets.push(asset);
        }
        "aeo" => {
            let aeo = generator.generate_aeo_content(&pkb).await?;
            let asset = ContentAsset {
                id: new_id(),
                project_id: project_id.clone(),
                campaign_id: None,
                asset_type: AssetType::Document,
                subtype: "aeo_content".into(),
                platform: "all".into(),
                status: AssetStatus::Draft,
                content: AssetContent {
                    text: Some(serde_json::to_string_pretty(&aeo)?),
                    file_path: None,
                    metadata: HashMap::new(),
                },
                variants: Vec::new(),
                performance: AssetPerformance::default(),
                created_at: now(),
                updated_at: now(),
            };
            storage.save_asset(&asset).await?;
            assets.push(asset);
        }
        _ => {
            return Err(Error::InvalidInput(format!(
                "Unknown asset type: {}",
                asset_type
            )));
        }
    }

    Ok(assets)
}

// ─── Settings Commands ──────────────────────────────────────────────

#[tauri::command]
pub async fn get_settings() -> Result<AppSettings> {
    let storage = get_storage();
    let settings_json = storage.get_setting("settings").await?;
    match settings_json {
        Some(s) => serde_json::from_str(&s).map_err(|e| Error::Config(e.to_string())),
        None => Ok(AppSettings::default()),
    }
}

#[tauri::command]
pub async fn update_settings(settings: AppSettings) -> Result<()> {
    let storage = get_storage();
    let json = serde_json::to_string(&settings)?;
    storage.set_setting("settings", &json).await
}

// ─── Campaign Commands ──────────────────────────────────────────────

#[tauri::command]
pub async fn get_campaign(id: String) -> Result<Campaign> {
    let storage = get_storage();
    storage
        .get_campaign(&id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Campaign not found: {}", id)))
}

#[tauri::command]
pub async fn list_campaigns(project_id: String) -> Result<Vec<Campaign>> {
    let storage = get_storage();
    storage.list_campaigns(&project_id).await
}

// ─── Content Commands ───────────────────────────────────────────────

#[tauri::command]
pub async fn approve_content(asset_id: String, approved: bool) -> Result<ContentAsset> {
    let storage = get_storage();
    let mut assets = storage.list_assets("").await?;

    let asset = assets
        .iter_mut()
        .find(|a| a.id == asset_id)
        .ok_or_else(|| Error::NotFound(format!("Asset not found: {}", asset_id)))?;

    asset.status = if approved {
        AssetStatus::Approved
    } else {
        AssetStatus::Rejected
    };
    asset.updated_at = now();

    storage.save_asset(asset).await?;
    Ok(asset.clone())
}

// ─── Export Commands ────────────────────────────────────────────────

#[tauri::command]
pub async fn export_package(project_id: String) -> Result<String> {
    let storage = get_storage();

    let project = storage
        .get_project(&project_id)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Project not found: {}", project_id)))?;

    let pkb = storage.get_pkb(&project_id).await;
    let assets = storage.list_assets(&project_id).await.unwrap_or_default();
    let campaigns = storage.list_campaigns(&project_id).await.unwrap_or_default();

    let mut export = serde_json::json!({
        "project": {
            "name": project.name,
            "type": format!("{:?}", project.project_type),
            "path": project.path,
            "created_at": project.created_at.to_rfc3339(),
        },
        "generated_at": now().to_rfc3339(),
    });

    if let Ok(pkb) = pkb {
        export["product_knowledge_base"] = serde_json::to_value(&pkb)?;
    }

    let asset_summaries: Vec<serde_json::Value> = assets
        .iter()
        .map(|a| {
            serde_json::json!({
                "id": a.id,
                "type": format!("{:?}", a.asset_type),
                "subtype": a.subtype,
                "platform": a.platform,
                "status": format!("{:?}", a.status),
                "text_preview": a.content.text.as_ref().map(|t| {
                    if t.len() > 200 { format!("{}...", &t[..200]) } else { t.clone() }
                }),
            })
        })
        .collect();

    export["content_assets"] = serde_json::to_value(&asset_summaries)?;

    let campaign_summaries: Vec<serde_json::Value> = campaigns
        .iter()
        .map(|c| {
            serde_json::json!({
                "id": c.id,
                "name": c.name,
                "status": format!("{:?}", c.status),
                "phases": c.phases.len(),
            })
        })
        .collect();

    export["campaigns"] = serde_json::to_value(&campaign_summaries)?;

    let export_json = serde_json::to_string_pretty(&export)?;

    // Save to project directory
    let export_path = std::path::Path::new(&project.path).join("promoforge-export.json");
    std::fs::write(&export_path, &export_json)?;

    Ok(export_path.to_string_lossy().to_string())
}

// ─── Health ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn health_check() -> Result<serde_json::Value> {
    Ok(serde_json::json!({
        "status": "ok",
        "version": "0.1.0",
        "timestamp": now().to_rfc3339(),
    }))
}

use std::collections::HashMap;
