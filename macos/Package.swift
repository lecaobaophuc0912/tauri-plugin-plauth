// swift-tools-version:5.7
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "tauri-plugin-plauth",
    platforms: [
        .macOS(.v13),
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "tauri-plugin-plauth",
            type: .static,
            targets: ["tauri-plugin-plauth"]),
    ],
    dependencies: [
        .package(url: "https://github.com/Choochmeque/tauri-swift-runtime.git", from: "0.1.1")
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "tauri-plugin-plauth",
            dependencies: [
                .product(name: "TauriSwiftRuntime", package: "tauri-swift-runtime")
            ],
            path: "Sources"
        ),
    ]
)
