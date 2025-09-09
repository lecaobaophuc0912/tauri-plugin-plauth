// Learn more about Tauri commands at https://v2.tauri.app/develop/calling-rust/#commands
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Running tauri application");
    let mut builder = tauri::Builder::default();
    #[cfg(target_os = "ios")]
    {
        println!("Building plugin for iOS");
        builder = builder.plugin(tauri_plugin_plauth::init());
    }
    #[cfg(target_os = "macos")]
    {
        println!("Building plugin for macOS");
        builder = builder.plugin(tauri_plugin_plauth::init());
    }
    builder
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
