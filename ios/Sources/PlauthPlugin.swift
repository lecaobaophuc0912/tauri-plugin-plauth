
import Foundation
import Tauri
import AuthenticationServices
import UIKit

// MARK: - Data Models
class AuthRequest: Decodable {
    let url: String
    let callbackUrlScheme: String
}

class AuthResponse: Encodable {
    let success: Bool
    let callbackUrl: String?
    let error: String?
    
    init(success: Bool, callbackUrl: String? = nil, error: String? = nil) {
        self.success = success
        self.callbackUrl = callbackUrl
        self.error = error
    }
}

class PingRequest: Decodable {
    let value: String?
}

class PingResponse: Encodable {
    let value: String?
    
    init(value: String?) {
        self.value = value
    }
}

// MARK: - Authentication Session Delegate
class AuthSessionDelegate: NSObject, ASWebAuthenticationPresentationContextProviding {
    let semaphore = DispatchSemaphore(value: 0)
    var callbackURL: String?
    var error: Error?
    
    func presentationAnchor(for session: ASWebAuthenticationSession) -> ASPresentationAnchor {
        // iOS: Get the key window from UIApplication
        if let windowScene = UIApplication.shared.connectedScenes
            .compactMap({ $0 as? UIWindowScene })
            .first(where: { $0.activationState == .foregroundActive }),
           let window = windowScene.windows.first(where: { $0.isKeyWindow }) {
            return window
        }
        
        // Fallback: return the first available window
        if let window = UIApplication.shared.windows.first {
            return window
        }
        
        // Last resort: create a default anchor
        return UIWindow()
    }
}

// MARK: - PLAUTH Plugin
class PlauthPlugin: Plugin {
    // MARK: - Ping Command
    @objc public func ping(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(PingRequest.self)
        let response = PingResponse(value: args.value ?? "")
        invoke.resolve(response)
    }
    
    // MARK: - Authenticate Command
    @objc public func authenticate(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(AuthRequest.self)
        
        guard let url = URL(string: args.url) else {
            let errorResponse = AuthResponse(success: false, error: "Invalid URL")
            invoke.resolve(errorResponse)
            return
        }
        
        let delegate = AuthSessionDelegate()
        var result: AuthResponse?
        
        let session = ASWebAuthenticationSession(
            url: url,
            callbackURLScheme: args.callbackUrlScheme
        ) { callbackURL, error in
            if let callbackURL = callbackURL {
                result = AuthResponse(success: true, callbackUrl: callbackURL.absoluteString)
            } else if let error = error {
                result = AuthResponse(success: false, error: error.localizedDescription)
            } else {
                result = AuthResponse(success: false, error: "Unknown error")
            }
            delegate.semaphore.signal()
        }
        
        // iOS: Always set presentation context provider
        session.presentationContextProvider = delegate
        
        session.start()
        delegate.semaphore.wait()
        
        invoke.resolve(result ?? AuthResponse(success: false, error: "No result"))
    }
}

// MARK: - Plugin Initialization
@_cdecl("init_plugin_plauth")
func initPlugin() -> Plugin {
    return PlauthPlugin()
}
