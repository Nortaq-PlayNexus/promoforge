pub mod commands;

use tracing_subscriber::{fmt, EnvFilter};

pub fn run() {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("Starting PromoForge AI v0.1.0");

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::analyze_project,
            commands::get_pkb,
            commands::generate_strategy,
            commands::generate_text_assets,
            commands::get_projects,
            commands::get_project,
            commands::create_project,
            commands::get_settings,
            commands::update_settings,
            commands::get_campaign,
            commands::list_campaigns,
            commands::approve_content,
            commands::export_package,
            commands::health_check,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = commands::init_storage(&app_handle).await {
                    tracing::error!("Failed to initialize storage: {}", e);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running PromoForge AI");
}
