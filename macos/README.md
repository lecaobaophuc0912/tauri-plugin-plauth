# Tauri Plugin PLAUTH - macOS Implementation

This macOS package implements the PLAUTH authentication plugin for Tauri applications using ASWebAuthenticationSession.

## Features

- **Secure Authentication**: Uses ASWebAuthenticationSession for secure web-based authentication flows
- **Cross-Platform Support**: Supports both iOS and macOS platforms
- **Callback Handling**: Properly handles authentication callbacks and URL schemes
- **Error Handling**: Comprehensive error handling for authentication failures

## Requirements

- iOS 13.0+ / macOS 10.15+
- Swift 5.3+
- Tauri API integration

## Implementation

### Authentication Flow

The plugin implements a secure authentication flow using ASWebAuthenticationSession:

1. **URL Validation**: Validates the authentication URL
2. **Session Creation**: Creates an ASWebAuthenticationSession with the provided URL and callback scheme
3. **Presentation**: Handles proper presentation context for the authentication session
4. **Callback Processing**: Processes authentication callbacks and errors
5. **Response Handling**: Returns structured responses with success/error information

### Data Models

- `AuthRequest`: Contains authentication URL, callback scheme, and presentation context provider
- `AuthResponse`: Structured response with success status, callback URL, and error information
- `PingRequest/PingResponse`: Basic ping functionality for testing

### Commands

- `ping`: Basic ping command for testing plugin functionality
- `authenticate`: Main authentication command using ASWebAuthenticationSession

## Usage

The plugin is automatically initialized when the Tauri application starts and provides authentication capabilities through the Tauri command system.

## Security

- Uses ASWebAuthenticationSession for secure authentication
- Properly validates URLs and callback schemes
- Implements secure presentation context handling
- Follows iOS/macOS security best practices
