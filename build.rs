const COMMANDS: &[&str] = &["ping", "authenticate"];

fn main() {
    // Only run this build script when building the plugin itself
    if std::env::var("CARGO_PKG_NAME").unwrap_or_default() != "tauri-plugin-plauth" {
        println!("Not building plugin");
        return;
    }

    println!("Building plugin with commands: {:?}", COMMANDS);

    // Build the main plugin
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();

    // Handle macOS Swift linking only when building for macOS target
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() == "macos" {
        println!("Building plugin for macOS");

        // Use swift-rs to link Swift code
        swift_rs::SwiftLinker::new("13.0")
            .with_package("tauri-plugin-plauth", "./macos/")
            .link();
    }
}
