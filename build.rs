const COMMANDS: &[&str] = &["ping", "authenticate"];

fn main() {
    println!("Building plugin with commands: {:?}", COMMANDS);

    // Build the main plugin
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();

    // Handle macOS Swift linking
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() == "macos" {
        println!("Building plugin for macOS");

        // Use swift-rs to link Swift code
        swift_rs::SwiftLinker::new("13.0")
            .with_package("tauri-plugin-plauth", "./macos/")
            .link();
    }
}
