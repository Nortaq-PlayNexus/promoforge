use promoforge_core::*;
use promoforge_ingestion::{FilesystemWalker, ProjectClassifier, ProjectAnalyzer};

pub struct PkbBuilder;

impl PkbBuilder {
    pub fn build_from_path(project_path: &str) -> Result<(Project, ProductKnowledgeBase)> {
        tracing::info!("Starting PKB build for: {}", project_path);

        // Phase 1: Filesystem scan
        tracing::info!("Phase 1: Scanning filesystem...");
        let manifest = FilesystemWalker::scan(project_path)?;
        tracing::info!(
            "Found {} files, {} total size",
            manifest.total_files,
            manifest.total_size
        );

        // Phase 2: Project classification
        tracing::info!("Phase 2: Classifying project...");
        let (project_type, confidence) = ProjectClassifier::classify(&manifest);
        tracing::info!(
            "Classified as {:?} with confidence {:.2}",
            project_type,
            confidence
        );

        // Phase 3: Analysis
        tracing::info!("Phase 3: Analyzing project...");
        let mut pkb = ProjectAnalyzer::analyze(&manifest, &project_type);
        tracing::info!(
            "Analysis complete: {} features, {} personas, {} tech stack items",
            pkb.features.len(),
            pkb.personas.len(),
            pkb.technical.tech_stack.len()
        );

        // Create project record
        let project = Project {
            id: new_id(),
            name: ProjectClassifier::infer_name(&manifest, &project_type),
            path: project_path.to_string(),
            project_type,
            status: ProjectStatus::Ready,
            created_at: now(),
            updated_at: now(),
            pkb_version: Some(1),
            settings: ProjectSettings::default(),
        };

        pkb.project_id = project.id.clone();
        pkb.version = 1;

        tracing::info!("PKB build complete for project: {}", project.name);
        Ok((project, pkb))
    }

    pub fn validate(pkb: &ProductKnowledgeBase) -> Vec<String> {
        let mut issues = Vec::new();

        if pkb.identity.inferred_name.is_empty() {
            issues.push("Missing project name".into());
        }
        if pkb.features.is_empty() {
            issues.push("No features detected - manual review recommended".into());
        }
        if pkb.personas.is_empty() {
            issues.push("No personas inferred - target audience unclear".into());
        }
        if pkb.technical.languages.is_empty() {
            issues.push("No programming languages detected".into());
        }
        for feature in &pkb.features {
            if feature.differentiation_score < 0.3 {
                issues.push(format!("Feature '{}' has low differentiation score", feature.name));
            }
        }

        issues
    }
}
