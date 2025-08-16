fn main() {
    // Build the plugin first
    println!("Building tauri-plugin-plauth...");
    
    // Set the plugin build environment
    std::env::set_var("CARGO_PKG_NAME", "tauri-plugin-plauth");
    
    // Build the main Tauri app
    tauri_build::build()
}
