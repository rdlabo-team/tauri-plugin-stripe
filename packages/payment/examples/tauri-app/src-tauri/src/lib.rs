// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri_plugin_log::{Target, TargetKind};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .plugin(tauri_plugin_log::Builder::new().targets([
            Target::new(TargetKind::Stdout),
            Target::new(TargetKind::LogDir { file_name: None }),
            Target::new(TargetKind::Webview),
        ]).build())
        .plugin(tauri_plugin_stripe_payment::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
