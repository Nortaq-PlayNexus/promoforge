use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Project ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub project_type: ProjectType,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pkb_version: Option<i32>,
    pub settings: ProjectSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Analyzing,
    Ready,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectType {
    SteamGame,
    MobileApp,
    RobloxExperience,
    WebApplication,
    DesktopApplication,
    CliTool,
    BrowserExtension,
    SaasPlatform,
    ApiService,
    OpenSourceLibrary,
    DigitalProduct,
    Hybrid,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub autonomy: AutonomyConfig,
    pub ai_provider: String,
    pub model_override: Option<String>,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            autonomy: AutonomyConfig::default(),
            ai_provider: "anthropic".into(),
            model_override: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomyConfig {
    pub analysis: AutonomyLevel,
    pub strategy: AutonomyLevel,
    pub content_generation: AutonomyLevel,
    pub social_posting: AutonomyLevel,
    pub outreach: AutonomyLevel,
    pub ad_spending: AutonomyLevel,
}

impl Default for AutonomyConfig {
    fn default() -> Self {
        Self {
            analysis: AutonomyLevel::Auto,
            strategy: AutonomyLevel::Supervised,
            content_generation: AutonomyLevel::BatchReview,
            social_posting: AutonomyLevel::Manual,
            outreach: AutonomyLevel::Manual,
            ad_spending: AutonomyLevel::Manual,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AutonomyLevel {
    Auto,
    Supervised,
    BatchReview,
    Manual,
}

// ─── Product Knowledge Base ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductKnowledgeBase {
    pub project_id: String,
    pub version: i32,
    pub identity: PkbIdentity,
    pub features: Vec<PkbFeature>,
    pub personas: Vec<PkbPersona>,
    pub technical: PkbTechnical,
    pub brand: PkbBrand,
    pub competitive: PkbCompetitive,
    pub constraints: PkbConstraints,
    pub assets: PkbAssets,
    pub knowledge_graph: KnowledgeGraph,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PkbIdentity {
    pub inferred_name: String,
    pub tagline_candidates: Vec<String>,
    pub elevator_pitches: Vec<String>,
    pub category: String,
    pub maturity_signals: MaturitySignals,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaturitySignals {
    pub has_tests: bool,
    pub has_docs: bool,
    pub has_ci: bool,
    pub commit_frequency: String,
    pub contributor_count: i32,
    pub test_coverage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PkbFeature {
    pub id: String,
    pub name: String,
    pub description: String,
    pub code_references: Vec<String>,
    pub user_journey_steps: Vec<String>,
    pub differentiation_score: f64,
    pub marketing_ready: bool,
    pub sub_features: Vec<PkbFeature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PkbPersona {
    pub id: String,
    pub name: String,
    pub description: String,
    pub pain_points: Vec<String>,
    pub current_solutions: Vec<String>,
    pub value_propositions: Vec<String>,
    pub preferred_channels: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PkbTechnical {
    pub languages: Vec<LanguageInfo>,
    pub framework: Option<String>,
    pub architecture_summary: String,
    pub tech_stack: Vec<String>,
    pub quality_assessment: QualityAssessment,
    pub dependencies: Vec<DependencyInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageInfo {
    pub name: String,
    pub file_count: usize,
    pub line_count: usize,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAssessment {
    pub test_coverage: Option<f64>,
    pub documentation_score: f64,
    pub code_quality_score: f64,
    pub type_safety_score: f64,
    pub has_ci: bool,
    pub overall_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub name: String,
    pub version: String,
    pub category: String,
    pub popularity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PkbBrand {
    pub inferred_voice: BrandVoice,
    pub visual_signals: VisualSignals,
    pub vocabulary: VocabularyRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandVoice {
    pub tone: Vec<String>,
    pub complexity: String,
    pub personality_traits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualSignals {
    pub dominant_colors: Vec<String>,
    pub style_preferences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularyRules {
    pub preferred: Vec<String>,
    pub avoid: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PkbCompetitive {
    pub competitors: Vec<Competitor>,
    pub positioning_opportunities: Vec<String>,
    pub market_gaps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competitor {
    pub name: String,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub positioning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PkbConstraints {
    pub budget: Option<BudgetConstraint>,
    pub timeline: Option<String>,
    pub platform_rules: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConstraint {
    pub total: f64,
    pub currency: String,
    pub monthly_limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PkbAssets {
    pub existing: Vec<AssetEntry>,
    pub gaps: Vec<AssetGap>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetEntry {
    pub name: String,
    pub asset_type: String,
    pub quality_score: f64,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetGap {
    pub asset_type: String,
    pub platform: String,
    pub priority: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub node_type: String,
    pub label: String,
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub edge_type: String,
    pub weight: f64,
}

// ─── Campaign / Strategy ────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub status: CampaignStatus,
    pub strategy_version: i32,
    pub phases: Vec<CampaignPhase>,
    pub total_budget: f64,
    pub spent_budget: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CampaignStatus {
    Draft,
    Planning,
    Active,
    Paused,
    Completed,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignPhase {
    pub id: String,
    pub name: String,
    pub status: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub objectives: Vec<String>,
    pub channels: Vec<ChannelPlan>,
    pub budget_allocation: f64,
    pub kpi_targets: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPlan {
    pub channel: String,
    pub platforms: Vec<String>,
    pub tactics: Vec<Tactic>,
    pub content_types: Vec<String>,
    pub frequency: String,
    pub budget: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tactic {
    pub name: String,
    pub description: String,
    pub effort: String,
    pub expected_impact: String,
    pub kpis: Vec<String>,
}

// ─── Content Assets ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAsset {
    pub id: String,
    pub project_id: String,
    pub campaign_id: Option<String>,
    pub asset_type: AssetType,
    pub subtype: String,
    pub platform: String,
    pub status: AssetStatus,
    pub content: AssetContent,
    pub variants: Vec<AssetVariant>,
    pub performance: AssetPerformance,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    Text,
    Image,
    Video,
    Audio,
    Document,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AssetStatus {
    Draft,
    InReview,
    Approved,
    Rejected,
    Published,
    Scheduled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetContent {
    pub text: Option<String>,
    pub file_path: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetVariant {
    pub variant_id: String,
    pub content: AssetContent,
    pub diversity_score: f64,
    pub brand_consistency_score: f64,
    pub platform_compliance: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetPerformance {
    pub impressions: u64,
    pub engagement: u64,
    pub clicks: u64,
    pub conversions: u64,
    pub rate: f64,
}

impl Default for AssetPerformance {
    fn default() -> Self {
        Self {
            impressions: 0,
            engagement: 0,
            clicks: 0,
            conversions: 0,
            rate: 0.0,
        }
    }
}

// ─── Settings ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub ai_providers: HashMap<String, AiProviderConfig>,
    pub default_autonomy: AutonomyConfig,
    pub storage_path: String,
    pub theme: String,
    pub notifications: NotificationSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProviderConfig {
    pub provider: String,
    pub api_key: Option<String>,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub desktop: bool,
    pub email_digest: String,
    pub critical_alerts: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        let mut providers = HashMap::new();
        providers.insert(
            "anthropic".into(),
            AiProviderConfig {
                provider: "anthropic".into(),
                api_key: None,
                model: "claude-sonnet-4-20250514".into(),
                max_tokens: 4096,
                temperature: 0.7,
                enabled: true,
            },
        );
        providers.insert(
            "openai".into(),
            AiProviderConfig {
                provider: "openai".into(),
                api_key: None,
                model: "gpt-4o".into(),
                max_tokens: 4096,
                temperature: 0.7,
                enabled: false,
            },
        );
        Self {
            ai_providers: providers,
            default_autonomy: AutonomyConfig::default(),
            storage_path: "~/.promoforge".into(),
            theme: "dark".into(),
            notifications: NotificationSettings {
                desktop: true,
                email_digest: "daily".into(),
                critical_alerts: true,
            },
        }
    }
}

// ─── File Analysis ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileManifest {
    pub root: String,
    pub files: Vec<FileEntry>,
    pub total_files: usize,
    pub total_size: u64,
    pub languages_detected: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub relative_path: String,
    pub category: FileCategory,
    pub language: Option<String>,
    pub size: u64,
    pub line_count: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FileCategory {
    SourceCode,
    Configuration,
    Documentation,
    Asset,
    Build,
    Test,
    Dependency,
    CiCd,
    License,
    Other,
}

// ─── Strategy ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    pub id: String,
    pub project_id: String,
    pub campaign_id: String,
    pub executive_summary: String,
    pub objectives: Vec<StrategyObjective>,
    pub phases: Vec<StrategyPhase>,
    pub asset_requirements: AssetRequirements,
    pub budget_breakdown: BudgetBreakdown,
    pub risks: Vec<Risk>,
    pub assumptions: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyObjective {
    pub description: String,
    pub target: String,
    pub timeline: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyPhase {
    pub name: String,
    pub duration: String,
    pub channels: Vec<ChannelPlan>,
    pub content_calendar: Vec<CalendarEntry>,
    pub milestones: Vec<String>,
    pub kpi_targets: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEntry {
    pub date: String,
    pub channel: String,
    pub content_type: String,
    pub description: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRequirements {
    pub total_count: usize,
    pub by_type: HashMap<String, usize>,
    pub by_priority: HashMap<String, usize>,
    pub production_schedule: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetBreakdown {
    pub total: f64,
    pub generation_api: f64,
    pub ad_spend: f64,
    pub tools: f64,
    pub human_review: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risk {
    pub description: String,
    pub probability: String,
    pub impact: String,
    pub mitigation: String,
}

// ─── LLM ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub content: String,
    pub model: String,
    pub tokens_used: u32,
    pub cost: f64,
}

// ─── Generation ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub asset_type: String,
    pub platform: String,
    pub purpose: String,
    pub brief: String,
    pub brand_rules: Option<PkbBrand>,
    pub quantity: usize,
    pub quality_tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    pub assets: Vec<ContentAsset>,
    pub total_cost: f64,
    pub quality_scores: Vec<f64>,
}

// ─── Analytics ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub channel: String,
    pub platform: String,
    pub event_type: String,
    pub metric_name: String,
    pub metric_value: f64,
    pub dimensions: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardMetrics {
    pub health_score: f64,
    pub active_channels: usize,
    pub total_reach: u64,
    pub conversion_rate: f64,
    pub budget_status: BudgetStatus,
    pub recent_activity: Vec<ActivityEntry>,
    pub channel_performance: Vec<ChannelPerformance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetStatus {
    pub total: f64,
    pub spent: f64,
    pub remaining: f64,
    pub daily_rate: f64,
    pub days_remaining: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEntry {
    pub id: String,
    pub agent: String,
    pub action: String,
    pub timestamp: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPerformance {
    pub channel: String,
    pub impressions: u64,
    pub engagement: u64,
    pub trend: Vec<f64>,
}

// ─── Playbook ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playbook {
    pub id: String,
    pub name: String,
    pub description: String,
    pub project_type: ProjectType,
    pub phases: Vec<PlaybookPhase>,
    pub tactics: Vec<PlaybookTactic>,
    pub asset_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybookPhase {
    pub name: String,
    pub duration: String,
    pub objectives: Vec<String>,
    pub channels: Vec<String>,
    pub key_tactics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybookTactic {
    pub name: String,
    pub description: String,
    pub phase: String,
    pub effort: String,
    pub expected_impact: String,
    pub prerequisites: Vec<String>,
    pub kpis: Vec<String>,
}
