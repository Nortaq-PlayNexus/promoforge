use promoforge_core::*;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Row;
use std::path::Path;

pub struct Storage {
    pool: SqlitePool,
}

impl Storage {
    pub async fn new(db_path: &str) -> Result<Self> {
        let path = Path::new(db_path);
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let url = format!("sqlite:{}?mode=rwc", db_path);
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        let storage = Self { pool };
        storage.migrate().await?;
        Ok(storage)
    }

    async fn migrate(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL,
                project_type TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                pkb_version INTEGER,
                settings TEXT NOT NULL DEFAULT '{}'
            );

            CREATE TABLE IF NOT EXISTS pkbs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id TEXT NOT NULL,
                version INTEGER NOT NULL,
                data TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            );

            CREATE TABLE IF NOT EXISTS campaigns (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                name TEXT NOT NULL,
                status TEXT NOT NULL,
                strategy_version INTEGER NOT NULL DEFAULT 1,
                phases TEXT NOT NULL DEFAULT '[]',
                total_budget REAL NOT NULL DEFAULT 0,
                spent_budget REAL NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            );

            CREATE TABLE IF NOT EXISTS strategies (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                campaign_id TEXT NOT NULL,
                data TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id),
                FOREIGN KEY (campaign_id) REFERENCES campaigns(id)
            );

            CREATE TABLE IF NOT EXISTS content_assets (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                campaign_id TEXT,
                asset_type TEXT NOT NULL,
                subtype TEXT NOT NULL,
                platform TEXT NOT NULL,
                status TEXT NOT NULL,
                content TEXT NOT NULL,
                variants TEXT NOT NULL DEFAULT '[]',
                performance TEXT NOT NULL DEFAULT '{}',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS metric_events (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                channel TEXT NOT NULL,
                platform TEXT NOT NULL,
                event_type TEXT NOT NULL,
                metric_name TEXT NOT NULL,
                metric_value REAL NOT NULL,
                dimensions TEXT NOT NULL DEFAULT '{}'
            );
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }

    // ─── Projects ───────────────────────────────────────────────

    pub async fn create_project(&self, project: &Project) -> Result<()> {
        let settings = serde_json::to_string(&project.settings)?;
        sqlx::query(
            "INSERT INTO projects (id, name, path, project_type, status, created_at, updated_at, pkb_version, settings)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&project.id)
        .bind(&project.name)
        .bind(&project.path)
        .bind(serde_json::to_string(&project.project_type)?)
        .bind(serde_json::to_string(&project.status)?)
        .bind(project.created_at.to_rfc3339())
        .bind(project.updated_at.to_rfc3339())
        .bind(project.pkb_version)
        .bind(&settings)
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn get_project(&self, id: &str) -> Result<Option<Project>> {
        let row = sqlx::query("SELECT * FROM projects WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(row.map(|r| Project {
            id: r.get("id"),
            name: r.get("name"),
            path: r.get("path"),
            project_type: serde_json::from_str(&r.get::<String, _>("project_type")).unwrap_or(ProjectType::Unknown),
            status: serde_json::from_str(&r.get::<String, _>("status")).unwrap_or(ProjectStatus::Ready),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at"))
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at"))
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            pkb_version: r.get("pkb_version"),
            settings: serde_json::from_str(&r.get::<String, _>("settings")).unwrap_or_default(),
        }))
    }

    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        let rows = sqlx::query("SELECT * FROM projects ORDER BY updated_at DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|r| Project {
                id: r.get("id"),
                name: r.get("name"),
                path: r.get("path"),
                project_type: serde_json::from_str(&r.get::<String, _>("project_type")).unwrap_or(ProjectType::Unknown),
                status: serde_json::from_str(&r.get::<String, _>("status")).unwrap_or(ProjectStatus::Ready),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at"))
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at"))
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                pkb_version: r.get("pkb_version"),
                settings: serde_json::from_str(&r.get::<String, _>("settings")).unwrap_or_default(),
            })
            .collect())
    }

    pub async fn update_project(&self, project: &Project) -> Result<()> {
        let settings = serde_json::to_string(&project.settings)?;
        sqlx::query(
            "UPDATE projects SET name=?, path=?, project_type=?, status=?, updated_at=?, pkb_version=?, settings=? WHERE id=?",
        )
        .bind(&project.name)
        .bind(&project.path)
        .bind(serde_json::to_string(&project.project_type)?)
        .bind(serde_json::to_string(&project.status)?)
        .bind(project.updated_at.to_rfc3339())
        .bind(project.pkb_version)
        .bind(&settings)
        .bind(&project.id)
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }

    // ─── PKBs ───────────────────────────────────────────────────

    pub async fn save_pkb(&self, pkb: &ProductKnowledgeBase) -> Result<()> {
        let data = serde_json::to_string(pkb)?;
        sqlx::query(
            "INSERT INTO pkbs (project_id, version, data, created_at) VALUES (?, ?, ?, ?)",
        )
        .bind(&pkb.project_id)
        .bind(pkb.version)
        .bind(&data)
        .bind(pkb.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn get_pkb(&self, project_id: &str) -> Result<Option<ProductKnowledgeBase>> {
        let row = sqlx::query(
            "SELECT data FROM pkbs WHERE project_id = ? ORDER BY version DESC LIMIT 1",
        )
        .bind(project_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        match row {
            Some(r) => {
                let data: String = r.get("data");
                let pkb: ProductKnowledgeBase =
                    serde_json::from_str(&data).map_err(|e| Error::Database(e.to_string()))?;
                Ok(Some(pkb))
            }
            None => Ok(None),
        }
    }

    // ─── Campaigns ──────────────────────────────────────────────

    pub async fn create_campaign(&self, campaign: &Campaign) -> Result<()> {
        let phases = serde_json::to_string(&campaign.phases)?;
        sqlx::query(
            "INSERT INTO campaigns (id, project_id, name, status, strategy_version, phases, total_budget, spent_budget, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&campaign.id)
        .bind(&campaign.project_id)
        .bind(&campaign.name)
        .bind(serde_json::to_string(&campaign.status)?)
        .bind(campaign.strategy_version)
        .bind(&phases)
        .bind(campaign.total_budget)
        .bind(campaign.spent_budget)
        .bind(campaign.created_at.to_rfc3339())
        .bind(campaign.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn get_campaign(&self, id: &str) -> Result<Option<Campaign>> {
        let row = sqlx::query("SELECT * FROM campaigns WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(row.map(|r| Campaign {
            id: r.get("id"),
            project_id: r.get("project_id"),
            name: r.get("name"),
            status: serde_json::from_str(&r.get::<String, _>("status")).unwrap_or(CampaignStatus::Draft),
            strategy_version: r.get("strategy_version"),
            phases: serde_json::from_str(&r.get::<String, _>("phases")).unwrap_or_default(),
            total_budget: r.get("total_budget"),
            spent_budget: r.get("spent_budget"),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at"))
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at"))
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }))
    }

    pub async fn list_campaigns(&self, project_id: &str) -> Result<Vec<Campaign>> {
        let rows = sqlx::query("SELECT * FROM campaigns WHERE project_id = ? ORDER BY created_at DESC")
            .bind(project_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|r| Campaign {
                id: r.get("id"),
                project_id: r.get("project_id"),
                name: r.get("name"),
                status: serde_json::from_str(&r.get::<String, _>("status")).unwrap_or(CampaignStatus::Draft),
                strategy_version: r.get("strategy_version"),
                phases: serde_json::from_str(&r.get::<String, _>("phases")).unwrap_or_default(),
                total_budget: r.get("total_budget"),
                spent_budget: r.get("spent_budget"),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at"))
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at"))
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
            })
            .collect())
    }

    // ─── Strategies ─────────────────────────────────────────────

    pub async fn save_strategy(&self, strategy: &Strategy) -> Result<()> {
        let data = serde_json::to_string(strategy)?;
        sqlx::query(
            "INSERT INTO strategies (id, project_id, campaign_id, data, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&strategy.id)
        .bind(&strategy.project_id)
        .bind(&strategy.campaign_id)
        .bind(&data)
        .bind(strategy.generated_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn get_strategy(&self, campaign_id: &str) -> Result<Option<Strategy>> {
        let row = sqlx::query("SELECT data FROM strategies WHERE campaign_id = ? ORDER BY created_at DESC LIMIT 1")
            .bind(campaign_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        match row {
            Some(r) => {
                let data: String = r.get("data");
                let strategy: Strategy =
                    serde_json::from_str(&data).map_err(|e| Error::Database(e.to_string()))?;
                Ok(Some(strategy))
            }
            None => Ok(None),
        }
    }

    // ─── Content Assets ─────────────────────────────────────────

    pub async fn save_asset(&self, asset: &ContentAsset) -> Result<()> {
        let content = serde_json::to_string(&asset.content)?;
        let variants = serde_json::to_string(&asset.variants)?;
        let performance = serde_json::to_string(&asset.performance)?;

        sqlx::query(
            "INSERT OR REPLACE INTO content_assets (id, project_id, campaign_id, asset_type, subtype, platform, status, content, variants, performance, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&asset.id)
        .bind(&asset.project_id)
        .bind(&asset.campaign_id)
        .bind(serde_json::to_string(&asset.asset_type)?)
        .bind(&asset.subtype)
        .bind(&asset.platform)
        .bind(serde_json::to_string(&asset.status)?)
        .bind(&content)
        .bind(&variants)
        .bind(&performance)
        .bind(asset.created_at.to_rfc3339())
        .bind(asset.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn list_assets(&self, project_id: &str) -> Result<Vec<ContentAsset>> {
        let rows = sqlx::query(
            "SELECT * FROM content_assets WHERE project_id = ? ORDER BY created_at DESC",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|r| ContentAsset {
                id: r.get("id"),
                project_id: r.get("project_id"),
                campaign_id: r.get("campaign_id"),
                asset_type: serde_json::from_str(&r.get::<String, _>("asset_type")).unwrap_or(AssetType::Text),
                subtype: r.get("subtype"),
                platform: r.get("platform"),
                status: serde_json::from_str(&r.get::<String, _>("status")).unwrap_or(AssetStatus::Draft),
                content: serde_json::from_str(&r.get::<String, _>("content")).unwrap_or(AssetContent {
                    text: None,
                    file_path: None,
                    metadata: Default::default(),
                }),
                variants: serde_json::from_str(&r.get::<String, _>("variants")).unwrap_or_default(),
                performance: serde_json::from_str(&r.get::<String, _>("performance")).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at"))
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at"))
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
            })
            .collect())
    }

    // ─── Settings ───────────────────────────────────────────────

    pub async fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let row = sqlx::query("SELECT value FROM settings WHERE key = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(row.map(|r| r.get("value")))
    }

    pub async fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)")
            .bind(key)
            .bind(value)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn close(&self) {
        self.pool.close().await;
    }
}
