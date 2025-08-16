import XCTest
@testable import PlauthPlugin

final class PlauthPluginTests: XCTestCase {
    func testPlauthPluginInitialization() throws {
        let plugin = PlauthPlugin()
        XCTAssertNotNil(plugin)
    }
    
    func testPingRequestParsing() throws {
        let pingRequest = PingRequest(value: "test")
        XCTAssertEqual(pingRequest.value, "test")
    }
    
    func testPingResponseCreation() throws {
        let pingResponse = PingResponse(value: "test")
        XCTAssertEqual(pingResponse.value, "test")
    }
    
    func testAuthRequestParsing() throws {
        let authRequest = AuthRequest(
            url: "https://example.com",
            callbackUrlScheme: "myapp",
            presentationContextProvider: nil
        )
        XCTAssertEqual(authRequest.url, "https://example.com")
        XCTAssertEqual(authRequest.callbackUrlScheme, "myapp")
        XCTAssertNil(authRequest.presentationContextProvider)
    }
    
    func testAuthResponseCreation() throws {
        let successResponse = AuthResponse(success: true, callbackUrl: "myapp://callback")
        XCTAssertTrue(successResponse.success)
        XCTAssertEqual(successResponse.callbackUrl, "myapp://callback")
        XCTAssertNil(successResponse.error)
        
        let errorResponse = AuthResponse(success: false, error: "Authentication failed")
        XCTAssertFalse(errorResponse.success)
        XCTAssertNil(errorResponse.callbackUrl)
        XCTAssertEqual(errorResponse.error, "Authentication failed")
    }
}
