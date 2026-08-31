use promoforge_core::*;
use std::collections::HashMap;
use std::path::Path;

pub struct ProjectAnalyzer;

impl ProjectAnalyzer {
    pub fn analyze(manifest: &FileManifest, project_type: &ProjectType) -> ProductKnowledgeBase {
        let name = crate::classifier::ProjectClassifier::infer_name(manifest, project_type);
        let languages = Self::analyze_languages(manifest);
        let features = Self::extract_features(manifest);
        let quality = Self::assess_quality(manifest);
        let tech_stack = Self::detect_tech_stack(manifest);
        let dependencies = Self::extract_dependencies(manifest);
        let personas = Self::infer_personas(project_type, &features);
        let brand = Self::infer_brand(manifest, project_type);
        let graph = Self::build_knowledge_graph(&features, &personas, project_type);

        let category = match project_type {
            ProjectType::SteamGame => "Indie Game",
            ProjectType::MobileApp => "Mobile Application",
            ProjectType::RobloxExperience => "Roblox Experience",
            ProjectType::WebApplication => "Web Application",
            ProjectType::DesktopApplication => "Desktop Application",
            ProjectType::CliTool => "CLI Tool / Library",
            ProjectType::BrowserExtension => "Browser Extension",
            ProjectType::SaasPlatform => "SaaS Platform",
            ProjectType::ApiService => "API Service",
            ProjectType::OpenSourceLibrary => "Open Source Library",
            ProjectType::DigitalProduct => "Digital Product",
            ProjectType::Hybrid => "Hybrid Product",
            ProjectType::Unknown => "Software Project",
        };

        ProductKnowledgeBase {
            project_id: String::new(),
            version: 1,
            identity: PkbIdentity {
                inferred_name: name.clone(),
                tagline_candidates: Self::generate_tagline_candidates(&name, &features, project_type),
                elevator_pitches: Self::generate_elevator_pitches(&name, &features, project_type),
                category: category.to_string(),
                maturity_signals: MaturitySignals {
                    has_tests: manifest.files.iter().any(|f| f.category == FileCategory::Test),
                    has_docs: manifest.files.iter().any(|f| f.category == FileCategory::Documentation),
                    has_ci: manifest.files.iter().any(|f| f.category == FileCategory::CiCd),
                    commit_frequency: "unknown".into(),
                    contributor_count: 1,
                    test_coverage: None,
                },
            },
            features,
            personas,
            technical: PkbTechnical {
                languages,
                framework: Self::detect_framework(manifest),
                architecture_summary: Self::summarize_architecture(manifest, project_type),
                tech_stack,
                quality_assessment: quality,
                dependencies,
            },
            brand,
            competitive: PkbCompetitive {
                competitors: Vec::new(),
                positioning_opportunities: Vec::new(),
                market_gaps: Vec::new(),
            },
            constraints: PkbConstraints {
                budget: None,
                timeline: None,
                platform_rules: HashMap::new(),
            },
            assets: PkbAssets {
                existing: Vec::new(),
                gaps: Vec::new(),
            },
            knowledge_graph: graph,
            created_at: now(),
            updated_at: now(),
        }
    }

    fn analyze_languages(manifest: &FileManifest) -> Vec<LanguageInfo> {
        let mut lang_map: HashMap<String, (usize, usize)> = HashMap::new();

        for file in &manifest.files {
            if file.category == FileCategory::SourceCode {
                if let Some(ref lang) = file.language {
                    let entry = lang_map.entry(lang.clone()).or_insert((0, 0));
                    entry.0 += 1;
                    entry.1 += file.line_count.unwrap_or(0);
                }
            }
        }

        let total_files: usize = lang_map.values().map(|(f, _)| f).sum();

        lang_map
            .into_iter()
            .map(|(name, (file_count, line_count))| LanguageInfo {
                name,
                file_count,
                line_count,
                confidence: file_count as f64 / total_files as f64,
            })
            .collect()
    }

    fn extract_features(manifest: &FileManifest) -> Vec<PkbFeature> {
        let mut features = Vec::new();

        let source_files: Vec<&FileEntry> = manifest
            .files
            .iter()
            .filter(|f| f.category == FileCategory::SourceCode)
            .collect();

        // Analyze file paths to infer features
        let mut feature_map: HashMap<String, Vec<String>> = HashMap::new();

        for file in &source_files {
            let parts: Vec<&str> = file.relative_path.split('/').collect();
            if parts.len() >= 2 {
                let module = parts[parts.len() - 2];
                if !module.starts_with('.') && module != "src" && module != "lib" && module != "app" {
                    feature_map
                        .entry(module.to_string())
                        .or_default()
                        .push(file.relative_path.clone());
                }
            }
        }

        for (module_name, code_refs) in feature_map {
            let name = module_name
                .replace('-', " ")
                .replace('_', " ")
                .split(' ')
                .map(|w| {
                    let mut c = w.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");

            features.push(PkbFeature {
                id: new_id(),
                name,
                description: format!("Feature detected from module: {}", module_name),
                code_references: code_refs,
                user_journey_steps: Vec::new(),
                differentiation_score: 0.5,
                marketing_ready: false,
                sub_features: Vec::new(),
            });
        }

        if features.is_empty() {
            features.push(PkbFeature {
                id: new_id(),
                name: "Core Application".into(),
                description: "Main application functionality".into(),
                code_references: source_files.iter().take(5).map(|f| f.relative_path.clone()).collect(),
                user_journey_steps: Vec::new(),
                differentiation_score: 0.5,
                marketing_ready: false,
                sub_features: Vec::new(),
            });
        }

        features
    }

    fn assess_quality(manifest: &FileManifest) -> QualityAssessment {
        let has_tests = manifest.files.iter().any(|f| f.category == FileCategory::Test);
        let has_docs = manifest
            .files
            .iter()
            .any(|f| f.category == FileCategory::Documentation);
        let has_ci = manifest.files.iter().any(|f| f.category == FileCategory::CiCd);
        let has_config = manifest
            .files
            .iter()
            .any(|f| f.category == FileCategory::Configuration);

        let test_score = if has_tests { 0.8 } else { 0.2 };
        let doc_score = if has_docs { 0.7 } else { 0.3 };
        let ci_score = if has_ci { 0.8 } else { 0.2 };
        let config_score = if has_config { 0.7 } else { 0.4 };

        let overall = (test_score + doc_score + ci_score + config_score) / 4.0;

        QualityAssessment {
            test_coverage: if has_tests { Some(0.6) } else { None },
            documentation_score: doc_score,
            code_quality_score: config_score,
            type_safety_score: 0.5,
            has_ci,
            overall_score: overall,
        }
    }

    fn detect_tech_stack(manifest: &FileManifest) -> Vec<String> {
        let mut stack = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for file in &manifest.files {
            let name = Path::new(&file.path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_lowercase();

            let checks: &[(&str, &str)] = &[
                ("package.json", "Node.js"),
                ("tsconfig.json", "TypeScript"),
                ("tailwind.config", "Tailwind CSS"),
                ("next.config", "Next.js"),
                ("nuxt.config", "Nuxt.js"),
                ("vite.config", "Vite"),
                ("vue.config", "Vue.js"),
                ("angular.json", "Angular"),
                ("svelte.config", "Svelte"),
                ("Cargo.toml", "Rust"),
                ("go.mod", "Go"),
                ("requirements.txt", "Python"),
                ("pyproject.toml", "Python"),
                ("Gemfile", "Ruby"),
                ("pom.xml", "Java"),
                ("build.gradle", "Gradle"),
                ("dockerfile", "Docker"),
                ("docker-compose", "Docker Compose"),
                ("kubernetes", "Kubernetes"),
                (".github/workflows", "GitHub Actions"),
                ("tauri.conf", "Tauri"),
                ("electron-builder", "Electron"),
                ("pubspec.yaml", "Flutter/Dart"),
                ("manifest.json", "Browser Extension"),
                ("steam_appid", "Steam"),
            ];

            for (check, tech) in checks {
                if name.contains(check) && !seen.contains(*tech) {
                    stack.push(tech.to_string());
                    seen.insert(*tech.to_string());
                }
            }

            if let Some(ref lang) = file.language {
                if !seen.contains(lang.as_str()) && file.category == FileCategory::SourceCode {
                    stack.push(lang.clone());
                    seen.insert(lang.clone());
                }
            }
        }

        stack
    }

    fn detect_framework(manifest: &FileManifest) -> Option<String> {
        for file in &manifest.files {
            let name = Path::new(&file.path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_lowercase();

            if name.contains("next.config") {
                return Some("Next.js".into());
            }
            if name.contains("nuxt.config") {
                return Some("Nuxt.js".into());
            }
            if name.contains("tauri.conf") {
                return Some("Tauri".into());
            }
            if name.contains("angular.json") {
                return Some("Angular".into());
            }
            if name.contains("svelte.config") {
                return Some("Svelte".into());
            }
            if name.contains("vue.config") || name.contains("vite.config") {
                if let Ok(content) = std::fs::read_to_string(&file.path).ok() {
                    if content.contains("vue") {
                        return Some("Vue.js".into());
                    }
                }
                return Some("Vite".into());
            }
            if name == "flutter" || name == "pubspec.yaml" {
                return Some("Flutter".into());
            }
        }
        None
    }

    fn summarize_architecture(manifest: &FileManifest, project_type: &ProjectType) -> String {
        let file_count = manifest.total_files;
        let source_count = manifest
            .files
            .iter()
            .filter(|f| f.category == FileCategory::SourceCode)
            .count();
        let languages: Vec<&str> = manifest
            .languages_detected
            .iter()
            .take(3)
            .map(|s| s.as_str())
            .collect();

        format!(
            "{} project with {} source files ({} total files). Primary languages: {}.",
            match project_type {
                ProjectType::SteamGame => "Game",
                ProjectType::MobileApp => "Mobile",
                ProjectType::WebApplication => "Web",
                ProjectType::DesktopApplication => "Desktop",
                ProjectType::CliTool => "CLI",
                ProjectType::SaasPlatform => "SaaS",
                ProjectType::ApiService => "API",
                _ => "Software",
            },
            source_count,
            file_count,
            if languages.is_empty() {
                "unknown".to_string()
            } else {
                languages.join(", ")
            }
        )
    }

    fn extract_dependencies(manifest: &FileManifest) -> Vec<DependencyInfo> {
        let mut deps = Vec::new();

        for file in &manifest.files {
            let name = Path::new(&file.path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_lowercase();

            if name == "package.json" {
                if let Ok(content) = std::fs::read_to_string(&file.path) {
                    if let Ok(pkg) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(deps_obj) = pkg.get("dependencies").and_then(|d| d.as_object()) {
                            for (name, version) in deps_obj {
                                deps.push(DependencyInfo {
                                    name: name.clone(),
                                    version: version.as_str().unwrap_or("*").to_string(),
                                    category: "runtime".into(),
                                    popularity: None,
                                });
                            }
                        }
                        if let Some(dev_deps) = pkg.get("devDependencies").and_then(|d| d.as_object()) {
                            for (name, version) in dev_deps {
                                deps.push(DependencyInfo {
                                    name: name.clone(),
                                    version: version.as_str().unwrap_or("*").to_string(),
                                    category: "development".into(),
                                    popularity: None,
                                });
                            }
                        }
                    }
                }
            }

            if name == "cargo.toml" {
                if let Ok(content) = std::fs::read_to_string(&file.path) {
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if trimmed.starts_with('\"') && trimmed.contains('=') {
                            if let Some(eq_pos) = trimmed.find('=') {
                                let dep_name = trimmed[..eq_pos].trim().trim_matches('"').to_string();
                                let version = trimmed[eq_pos + 1..].trim().trim_matches('"').trim_matches(',').to_string();
                                if !dep_name.is_empty() && dep_name != "promoforge" {
                                    deps.push(DependencyInfo {
                                        name: dep_name,
                                        version,
                                        category: "runtime".into(),
                                        popularity: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        deps
    }

    fn infer_personas(project_type: &ProjectType, features: &[PkbFeature]) -> Vec<PkbPersona> {
        match project_type {
            ProjectType::SteamGame => vec![
                PkbPersona {
                    id: new_id(),
                    name: "Core Gamer".into(),
                    description: "Enthusiastic gamer who plays 10+ hours/week, follows gaming news, active on Steam".into(),
                    pain_points: vec![
                        "Can't find quality indie games".into(),
                        "Overwhelmed by Steam's catalog".into(),
                    ],
                    current_solutions: vec!["Steam browsing".into(), "YouTube/Twitch".into()],
                    value_propositions: vec!["Unique gameplay experience".into()],
                    preferred_channels: vec!["Steam".into(), "Reddit".into(), "YouTube".into(), "TikTok".into()],
                    confidence: 0.7,
                },
                PkbPersona {
                    id: new_id(),
                    name: "Content Creator".into(),
                    description: "Streamer/YouTuber looking for fresh indie games to feature".into(),
                    pain_points: vec!["Need new content ideas".into(), "Hard to find unique games".into()],
                    current_solutions: vec!["Press kits".into(), "Publisher contacts".into()],
                    value_propositions: vec!["Exciting new game to showcase".into()],
                    preferred_channels: vec!["Twitter".into(), "Discord".into(), "Email".into()],
                    confidence: 0.6,
                },
            ],
            ProjectType::CliTool | ProjectType::OpenSourceLibrary => vec![
                PkbPersona {
                    id: new_id(),
                    name: "Developer".into(),
                    description: "Software developer looking for efficient tools to improve workflow".into(),
                    pain_points: vec![
                        "Repetitive tasks".into(),
                        "Poor tooling".into(),
                        "Complex setup processes".into(),
                    ],
                    current_solutions: vec!["Manual scripts".into(), "Existing tools".into()],
                    value_propositions: vec!["Faster development workflow".into()],
                    preferred_channels: vec!["GitHub".into(), "Hacker News".into(), "Twitter".into(), "Dev.to".into()],
                    confidence: 0.8,
                },
            ],
            _ => vec![
                PkbPersona {
                    id: new_id(),
                    name: "Target User".into(),
                    description: format!("Primary user for this {}", match project_type {
                        ProjectType::WebApplication => "web application",
                        ProjectType::MobileApp => "mobile app",
                        ProjectType::SaasPlatform => "SaaS platform",
                        _ => "software product",
                    }),
                    pain_points: vec!["Needs a solution to their problem".into()],
                    current_solutions: vec!["Existing alternatives".into()],
                    value_propositions: features.iter().take(2).map(|f| f.name.clone()).collect(),
                    preferred_channels: vec!["Twitter".into(), "Reddit".into()],
                    confidence: 0.5,
                },
            ],
        }
    }

    fn infer_brand(manifest: &FileManifest, project_type: &ProjectType) -> PkbBrand {
        PkbBrand {
            inferred_voice: BrandVoice {
                tone: match project_type {
                    ProjectType::CliTool | ProjectType::OpenSourceLibrary => {
                        vec!["technical".into(), "concise".into(), "helpful".into()]
                    }
                    ProjectType::SteamGame => {
                        vec!["exciting".into(), "bold".into(), "playful".into()]
                    }
                    _ => {
                        vec!["professional".into(), "clear".into(), "confident".into()]
                    }
                },
                complexity: "moderate".into(),
                personality_traits: vec!["innovative".into(), "reliable".into()],
            },
            visual_signals: VisualSignals {
                dominant_colors: Vec::new(),
                style_preferences: Vec::new(),
            },
            vocabulary: VocabularyRules {
                preferred: Vec::new(),
                avoid: vec!["buzzwords".into(), "jargon without explanation".into()],
            },
        }
    }

    fn generate_tagline_candidates(name: &str, features: &[PkbFeature], project_type: &ProjectType) -> Vec<String> {
        let feature_summary = features.iter().take(3).map(|f| f.name.to_lowercase()).collect::<Vec<_>>().join(", ");
        vec![
            format!("{} - {}", name, feature_summary),
            format!("The future of {}", match project_type {
                ProjectType::SteamGame => "gaming",
                ProjectType::WebApplication => "web apps",
                ProjectType::MobileApp => "mobile",
                _ => "software",
            }),
            format!("{}: Built for builders", name),
        ]
    }

    fn generate_elevator_pitches(name: &str, features: &[PkbFeature], project_type: &ProjectType) -> Vec<String> {
        let feature_list: Vec<&str> = features.iter().take(3).map(|f| f.name.as_str()).collect();
        vec![
            format!(
                "{} is a {} that helps you {}.",
                name,
                match project_type {
                    ProjectType::SteamGame => "game",
                    ProjectType::WebApplication => "web application",
                    ProjectType::MobileApp => "mobile app",
                    ProjectType::SaasPlatform => "SaaS platform",
                    ProjectType::CliTool => "CLI tool",
                    _ => "software product",
                },
                if feature_list.is_empty() {
                    "achieve your goals"
                } else {
                    feature_list.join(", ")
                }
            ),
        ]
    }

    fn build_knowledge_graph(
        features: &[PkbFeature],
        personas: &[PkbPersona],
        project_type: &ProjectType,
    ) -> KnowledgeGraph {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        let product_id = new_id();
        nodes.push(GraphNode {
            id: product_id.clone(),
            node_type: "product".into(),
            label: "This Product".into(),
            properties: HashMap::new(),
        });

        for feature in features {
            nodes.push(GraphNode {
                id: feature.id.clone(),
                node_type: "feature".into(),
                label: feature.name.clone(),
                properties: HashMap::new(),
            });
            edges.push(GraphEdge {
                id: new_id(),
                source: product_id.clone(),
                target: feature.id.clone(),
                edge_type: "has_feature".into(),
                weight: 1.0,
            });
        }

        for persona in personas {
            nodes.push(GraphNode {
                id: persona.id.clone(),
                node_type: "persona".into(),
                label: persona.name.clone(),
                properties: HashMap::new(),
            });
            edges.push(GraphEdge {
                id: new_id(),
                source: product_id.clone(),
                target: persona.id.clone(),
                edge_type: "targets".into(),
                weight: persona.confidence,
            });
        }

        let type_node_id = new_id();
        nodes.push(GraphNode {
            id: type_node_id.clone(),
            node_type: "category".into(),
            label: format!("{:?}", project_type),
            properties: HashMap::new(),
        });
        edges.push(GraphEdge {
            id: new_id(),
            source: product_id.clone(),
            target: type_node_id,
            edge_type: "is_type".into(),
            weight: 1.0,
        });

        KnowledgeGraph { nodes, edges }
    }
}
