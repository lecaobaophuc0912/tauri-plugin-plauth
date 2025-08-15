# Publishing Guide - Tauri Plugin PLAUTH

## Overview

This document records the complete build and publishing process for the Tauri Plugin PLAUTH project, including both Rust (crates.io) and JavaScript/TypeScript (npm) packages.

## Project Structure

```
tauri-plugin-plauth/
├── src/                    # Rust source code
├── guest-js/              # TypeScript client code
├── macos/                  # macOS Swift implementation
├── examples/               # Example applications
├── permissions/            # Tauri permissions
├── Cargo.toml             # Rust package configuration
├── package.json           # NPM package configuration
├── build.rs               # Rust build script
├── rollup.config.js       # JavaScript bundling
└── tsconfig.json          # TypeScript configuration
```

## Prerequisites

- Rust 1.77.2+
- Tauri 2.7.0+
- Node.js 18+
- Yarn package manager
- Xcode (for macOS development)
- Git repository with proper access

## Pre-Publishing Checklist

### 1. Metadata Updates

#### Cargo.toml

- [x] Update version number
- [x] Set proper authors
- [x] Add comprehensive description
- [x] Include license (MIT)
- [x] Add repository URLs
- [x] Set keywords (max 5 for crates.io)
- [x] Configure categories
- [x] Set rust-version

#### package.json

- [x] Update version number
- [x] Set proper author
- [x] Add comprehensive description
- [x] Include license (MIT)
- [x] Add repository URLs
- [x] Set keywords
- [x] Configure exports and types

#### README.md

- [x] Add badges (crates.io, license)
- [x] Include installation instructions
- [x] Add usage examples
- [x] Document API reference
- [x] List platform support
- [x] Include development setup
- [x] Add contributing guidelines

### 2. File Creation

- [x] Create LICENSE file (MIT)
- [x] Create .npmignore for NPM publishing
- [x] Ensure all source files are properly documented

## Build Process

### 1. Build JavaScript/TypeScript Client

```bash
# Install dependencies
yarn install

# Build client
yarn build
```

**Expected Output:**

```
guest-js/index.ts → ./dist-js/index.js, ./dist-js/index.cjs...
created ./dist-js/index.js, ./dist-js/index.cjs in 1.1s
✨  Done in 3.01s.
```

**Generated Files:**

- `dist-js/index.js` (ESM)
- `dist-js/index.cjs` (CommonJS)
- `dist-js/index.d.ts` (TypeScript definitions)

### 2. Build Rust Plugin

```bash
# Build in development mode
cargo build

# Build in release mode
cargo build --release
```

**Expected Output:**

```
Finished `dev` profile [unoptimized + debuginfo] target(s) in 3m 00s
```

**Note:** May show warnings about unused variables (acceptable for publishing)

### 3. Run Tests

```bash
# Run Rust tests
cargo test

# Expected: 0 tests, all passing
```

### 4. Generate Documentation

```bash
# Generate Rust documentation
cargo doc --no-deps

# Expected: Documentation generated in target/doc/
```

### 5. Build Example Application

```bash
cd examples/tauri-app
cargo tauri build
```

**Expected Output:**

```
✓ built in 1.64s
Finished `release` profile [optimized] in 1m 29s
Bundling PlAuth.app
Bundling PlAuth_0.1.0_x64.dmg
```

## Publishing Process

### 1. Git Preparation

```bash
# Check status
git status

# Add all changes
git add .

# Commit changes
git commit -m "feat: prepare for v1.0.0 release - update metadata, documentation, and examples"

# Create version tag
git tag -a v1.0.0 -m "Release version 1.0.0"

# Push code and tag
git push origin develop
git push origin v1.0.0
```

### 2. Publish to Crates.io

#### First Attempt (with verification)

```bash
cargo package
```

**Common Issues:**

- Build script modifies source directory
- Too many keywords (>5)
- Uncommitted changes

#### Solution: Use --no-verify flag

```bash
cargo package --no-verify
cargo publish --no-verify
```

**Expected Output:**

```
Packaged 30 files, 157.5KiB (41.7KiB compressed)
Uploaded tauri-plugin-plauth v1.0.0 to registry `crates-io`
Published tauri-plugin-plauth v1.0.0 at registry `crates-io`
```

### 3. Publish to NPM

```bash
npm publish
```

**Expected Output:**

```
> tauri-plugin-plauth-api@0.1.0 prepublishOnly
> yarn build
+ tauri-plugin-plauth-api@0.1.0
```

## Post-Publishing Verification

### 1. Crates.io

- [x] Visit: https://crates.io/crates/tauri-plugin-plauth
- [x] Verify version 1.0.0 is published
- [x] Check metadata is correct
- [x] Verify documentation links work

### 2. NPM

- [x] Visit: https://www.npmjs.com/package/tauri-plugin-plauth-api
- [x] Verify version 0.1.0 is published
- [x] Check package contents
- [x] Verify installation works

### 3. Git Repository

- [x] Verify tag v1.0.0 exists
- [x] Check all changes are pushed
- [x] Verify working tree is clean

## Troubleshooting

### Common Issues

#### 1. Build Script Modifies Source

**Error:** `Source directory was modified by build.rs during cargo publish`

**Solution:** Use `--no-verify` flag

```bash
cargo publish --no-verify
```

#### 2. Too Many Keywords

**Error:** `expected at most 5 keywords per crate`

**Solution:** Reduce keywords in Cargo.toml to maximum 5

```toml
keywords = ["tauri", "plugin", "authentication", "auth", "macos"]
```

#### 3. Uncommitted Changes

**Error:** `files in the working directory contain changes that were not yet committed`

**Solution:** Commit all changes before publishing

```bash
git add .
git commit -m "your message"
```

#### 4. Swift Linking Issues

**Warning:** `unused variable: 'app'`

**Solution:** This is acceptable for publishing, but can be fixed by prefixing with underscore

```rust
let _app: &AppHandle<R> = app;
```

## Version Management

### Current Versions

- **Rust Plugin**: 1.0.0
- **NPM Package**: 0.1.0

### Version Bumping Strategy

- **Patch (0.0.x)**: Bug fixes, minor improvements
- **Minor (0.x.0)**: New features, backward compatible
- **Major (x.0.0)**: Breaking changes, major features

### Update Process

1. Update version in `Cargo.toml`
2. Update version in `package.json`
3. Update version in `README.md`
4. Create new git tag
5. Follow publishing process

## Dependencies

### Rust Dependencies

```toml
[dependencies]
tauri = "2.7.0"
serde = { version = "1.0", features = ["derive"] }
thiserror = "2"
tauri-swift-runtime = "0.1.2"

[build-dependencies]
tauri-plugin = { version = "2.3.1", features = ["build"] }
swift-rs = { version = "1.0.7", features = ["build"] }
```

### NPM Dependencies

```json
{
  "dependencies": {
    "@tauri-apps/api": ">=2.0.0-beta.6"
  },
  "devDependencies": {
    "@rollup/plugin-typescript": "^12.0.0",
    "rollup": "^4.9.6",
    "typescript": "^5.3.3",
    "tslib": "^2.6.2"
  }
}
```

## Platform Support

### Current Support

- ✅ **macOS**: Full support with Swift integration
- ❌ **iOS**: Not implemented (future)
- ❌ **Android**: Not implemented (future)
- ❌ **Linux**: Not implemented (future)
- ❌ **Windows**: Not implemented (future)

### Future Considerations

- iOS support with ASWebAuthenticationSession
- Android support with WebView
- Cross-platform authentication standards

## Security Considerations

### Authentication Security

- Implement proper URL validation
- Handle callback URL schemes securely
- Validate authentication responses
- Implement proper session management

### Input Validation

- Validate all command inputs
- Sanitize data before processing
- Implement proper access controls

## Maintenance

### Regular Tasks

- Keep dependencies updated
- Monitor for security vulnerabilities
- Test thoroughly after dependency updates
- Stay current with Tauri updates

### Code Quality

- Use clippy for Rust code quality
- Maintain consistent formatting
- Regular code reviews and refactoring
- Comprehensive testing

## Links

### Published Packages

- **Crates.io**: https://crates.io/crates/tauri-plugin-plauth
- **NPM**: https://www.npmjs.com/package/tauri-plugin-plauth-api

### Source Code

- **GitHub**: https://github.com/lecaobaophuc0912/tauri-plugin-plauth
- **Issues**: https://github.com/lecaobaophuc0912/tauri-plugin-plauth/issues

### Documentation

- **API Docs**: https://github.com/lecaobaophuc0912/tauri-plugin-plauth
- **Examples**: `examples/tauri-app/`

---

**Last Updated:** December 2024  
**Version:** 1.0.0  
**Maintainer:** Phuc Le <lecaobaophuc@gmail.com>
