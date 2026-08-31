use promoforge_core::*;
use std::collections::HashMap;
use std::path::Path;

pub struct ProjectClassifier;

impl ProjectClassifier {
    pub fn classify(manifest: &FileManifest) -> (ProjectType, f64) {
        let mut signals: HashMap<ProjectType, f64> = HashMap::new();

        for file in &manifest.files {
            let name = Path::new(&file.path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_lowercase();
            let path_lower = file.relative_path.to_lowercase();

            // Steam / Game signals
            if name == "steam_appid.txt" || name == "steamworks_sdk" || path_lower.contains("steamworks") {
                *signals.entry(ProjectType::SteamGame).or_insert(0.0) += 50.0;
            }
            if name == "projectsettings.asset" || (name == "project.godot") || (name == "uproject" && path_lower.ends_with(".uproject")) {
                *signals.entry(ProjectType::SteamGame).or_insert(0.0) += 40.0;
            }
            if name.contains("game") && (file.language.as_deref() == Some("C#") || file.language.as_deref() == Some("C++")) {
                *signals.entry(ProjectType::SteamGame).or_insert(0.0) += 10.0;
            }

            // Mobile App signals
            if name == "info.plist" || name == "androidmanifest.xml" || name == "pubspec.yaml" {
                *signals.entry(ProjectType::MobileApp).or_insert(0.0) += 40.0;
            }
            if path_lower.contains("/android/") || path_lower.contains("/ios/") {
                *signals.entry(ProjectType::MobileApp).or_insert(0.0) += 20.0;
            }

            // Roblox signals
            if name == "default.project.json" || name == "*.rbxl" || path_lower.contains("roblox") {
                *signals.entry(ProjectType::RobloxExperience).or_insert(0.0) += 50.0;
            }

            // Web Application signals
            if name == "next.config.js" || name == "next.config.mjs" || name == "nuxt.config.ts" || name == "nuxt.config.js" {
                *signals.entry(ProjectType::WebApplication).or_insert(0.0) += 40.0;
            }
            if name == "index.html" && path_lower.contains("/public/") {
                *signals.entry(ProjectType::WebApplication).or_insert(0.0) += 10.0;
            }

            // Desktop Application signals
            if name == "tauri.conf.json" || name == "electron-builder.yml" || name == "tauri.conf.json5" {
                *signals.entry(ProjectType::DesktopApplication).or_insert(0.0) += 50.0;
            }

            // CLI Tool signals
            if name == "clap" || path_lower.contains("/cli/") || path_lower.contains("/bin/") {
                *signals.entry(ProjectType::CliTool).or_insert(0.0) += 15.0;
            }
            if file.language.as_deref() == Some("Rust") && name == "main.rs" {
                *signals.entry(ProjectType::CliTool).or_insert(0.0) += 5.0;
            }

            // Browser Extension signals
            if name == "manifest.json" && path_lower.contains("extension") {
                *signals.entry(ProjectType::BrowserExtension).or_insert(0.0) += 40.0;
            }
            if name == "background.js" || name == "content.js" || name == "popup.html" {
                *signals.entry(ProjectType::BrowserExtension).or_insert(0.0) += 20.0;
            }

            // SaaS signals
            if name == "stripe" || name == "billing" || path_lower.contains("/billing/") || path_lower.contains("/subscription/") {
                *signals.entry(ProjectType::SaasPlatform).or_insert(0.0) += 20.0;
            }

            // API Service signals
            if name == "openapi" || name == "swagger" || path_lower.contains("/api/") || name == "grpc") {
                *signals.entry(ProjectType::ApiService).or_insert(0.0) += 15.0;
            }

            // Open Source Library signals
            if name == "readme.md" && manifest.files.iter().any(|f| f.relative_path.to_lowercase() == "contributing.md") {
                *signals.entry(ProjectType::OpenSourceLibrary).or_insert(0.0) += 10.0;
            }

            // Dependency-based signals
            if name == "package.json" {
                if let Ok(content) = std::fs::read_to_string(&file.path) {
                    if content.contains("react-native") || content.contains("expo") {
                        *signals.entry(ProjectType::MobileApp).or_insert(0.0) += 30.0;
                    }
                    if content.contains("next") || content.contains("nuxt") || content.contains("remix") {
                        *signals.entry(ProjectType::WebApplication).or_insert(0.0) += 25.0;
                    }
                    if content.contains("electron") {
                        *signals.entry(ProjectType::DesktopApplication).or_insert(0.0) += 25.0;
                    }
                    if content.includes("express") || content.includes("fastify") || content.includes("hono") || content.includes("nestjs") {
                        *signals.entry(ProjectType::ApiService).or_insert(0.0) += 20.0;
                    }
                }
            }

            if name == "pubspec.yaml" {
                if let Ok(content) = std::fs::read_to_string(&file.path) {
                    if content.contains("flutter") {
                        *signals.entry(ProjectType::MobileApp).or_insert(0.0) += 30.0;
                    }
                }
            }
        }

        if signals.is_empty() {
            return (ProjectType::Unknown, 0.3);
        }

        let mut best = signals.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
        let confidence = (best.1 / 100.0).min(1.0).max(0.1);
        (best.0, confidence)
    }

    pub fn infer_name(manifest: &FileManifest, project_type: &ProjectType) -> String {
        let root = Path::new(&manifest.root);
        root.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled Project")
            .to_string()
    }
}
