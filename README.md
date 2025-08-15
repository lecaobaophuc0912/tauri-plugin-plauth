# Tauri Plugin PLAUTH

A Tauri plugin for authentication (PLAUTH) that currently supports **macOS and iOS platforms only**. The plugin implements ASWebAuthenticationSession for secure web-based authentication flows.

[![Crates.io](https://img.shields.io/crates/v/tauri-plugin-plauth)](https://crates.io/crates/tauri-plugin-plauth)
[![Crates.io](https://img.shields.io/crates/d/tauri-plugin-plauth)](https://crates.io/crates/tauri-plugin-plauth)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **macOS**: Full support with web view integration
- **iOS**: Full support with ASWebAuthenticationSession
- **Android**: Not currently supported
- **Linux**: Not currently supported
- **Windows**: Not currently supported

## Installation

### Rust Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-plauth = "0.1.0"
```

### JavaScript/TypeScript Dependencies

```bash
npm install tauri-plugin-plauth-api
# or
yarn add tauri-plugin-plauth-api
```

## Usage

### Rust Setup

In your `src-tauri/build.rs`:

```rust
fn main() {
    tauri_plugin_plauth::build();
}
```

In your `src-tauri/src/main.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_plauth::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### JavaScript/TypeScript Usage

```typescript
import { authenticate } from "tauri-plugin-plauth-api";

// Basic authentication
try {
  const result = await authenticate({
    url: "https://example.com/auth",
    callback_url_scheme: "myapp://callback",
  });
  console.log("Authentication successful:", result);
} catch (error) {
  console.error("Authentication failed:", error);
}
```

## API Reference

### `authenticate(request: AuthRequest): Promise<AuthResponse>`

Initiates an authentication flow using ASWebAuthenticationSession (iOS) or web view (macOS).

#### Parameters

- `url` (string): The authentication URL to navigate to
- `callback_url_scheme` (string): The URL scheme to handle the callback
- `presentation_context_provider` (optional): Custom presentation context provider

#### Returns

- `AuthResponse`: Object containing the authentication result

## Examples

- `examples/tauri-app/` - Example Tauri application demonstrating plugin usage on macOS with **React** frontend

## Platform Support

| Platform | Status           | Notes                      |
| -------- | ---------------- | -------------------------- |
| macOS    | ✅ Full          | Web view integration       |
| iOS      | ❌ Full          | ASWebAuthenticationSession |
| Android  | ❌ Not supported | Future implementation      |
| Linux    | ❌ Not supported | Not planned                |
| Windows  | ❌ Not supported | Not planned                |

## Development

### Prerequisites

- Rust 1.77.2+
- Tauri 2.7.0+
- Xcode (for iOS development)
- macOS (for development and testing)

### Building

```bash
# Build Rust plugin
cargo build

# Build JavaScript/TypeScript client
yarn build
```

### Testing

```bash
# Run Rust tests
cargo test

# Run example app
cd examples/tauri-app
cargo tauri dev
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

If you encounter any issues or have questions, please:

1. Check the [examples](./examples/tauri-app/) for usage patterns
2. Search existing [issues](https://github.com/lecaobaophuc0912/tauri-plugin-plauth/issues)
3. Create a new issue with detailed information about your problem

## Changelog

### 0.1.0

- Initial release
- macOS and iOS support
- ASWebAuthenticationSession integration
- Basic authentication flow
