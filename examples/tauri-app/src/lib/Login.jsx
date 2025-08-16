import React, { useState, useEffect } from 'react'
import { ping, authenticate } from 'tauri-plugin-plauth-api'

function Login() {
  const [isLoading, setIsLoading] = useState(false)
  const [message, setMessage] = useState('')
  const [isAuthenticated, setIsAuthenticated] = useState(false)
  const [isMobile, setIsMobile] = useState(false)

  // Check if device is mobile
  useEffect(() => {
    const checkMobile = () => {
      const mobile = window.innerWidth <= 768
      setIsMobile(mobile)
      console.log('Screen width:', window.innerWidth, 'Is mobile:', mobile)
    }

    checkMobile()
    window.addEventListener('resize', checkMobile)

    return () => window.removeEventListener('resize', checkMobile)
  }, [])

  const handleGoogleLogin = async () => {
    setIsLoading(true)
    setMessage('')

    try {
      // Now perform actual authentication
      const authResult = await authenticate({
        url: "https://preview.mojo-platform.pages.dev/redirecting?uri=https%3A%2F%2Faccounts.google.com%2Fo%2Foauth2%2Fv2%2Fauth%3Fclient_id%3D481733480788-t19u97j3tgrbnmr2i3hdl3k0b1ooe4nl.apps.googleusercontent.com%26scope%3Dopenid%2520email%2520profile%26response_type%3Dcode%26redirect_uri%3Dhttps%253A%252F%252Fpreview.mojo-platform.pages.dev%252Flogin%252Fredirect%26state%3Dtoken%25253Dmojoplatform%252526name%25253Dgoogle_stg%252526app%25253Dmain%252526redirect%25253D&type=login_google",
        callbackUrlScheme: "mojoapp"
      })
      console.log(authResult)
      const urlObj = new URL(authResult.callbackUrl);
      const params = urlObj.searchParams;

      const code = params.get('code');
      const state = params.get('state');
      const token = params.get('token');
      console.log(code, state, token)
      if (authResult.success) {
        setIsAuthenticated(true)
        setMessage(`Authentication successful! Callback URL: ${authResult.callbackUrl}`)
      } else {
        setMessage(`Authentication failed: ${authResult.error}`)
      }

      setIsLoading(false)

    } catch (error) {
      console.log(error)
      setMessage(`Google auth error: ${error}`)
      setIsLoading(false)
    }
  }

  const handleSubmit = async () => {
    setIsLoading(true)
    setMessage('')

    try {
      const pingResult = await ping("Testing authentication...")
      setMessage(`Ping successful: ${pingResult}`)
      setIsLoading(false)
    } catch (error) {
      setMessage(`Error: ${error}`)
      setIsLoading(false)
    }
  }

  const handleLogout = () => {
    setIsAuthenticated(false)
    setMessage('Logged out successfully')
  }

  // Debug info
  console.log('Current mobile state:', isMobile)

  return (
    <div className={`login-container ${isMobile ? 'mobile' : ''}`}>
      <div className={`login-card ${isMobile ? 'mobile' : ''}`}>
        <h2 className={isMobile ? 'mobile-title' : ''}>🔐 Login</h2>
        <p className={`subtitle ${isMobile ? 'mobile-subtitle' : ''}`}>
          Test your Tauri authentication plugin
        </p>

        {!isAuthenticated ? (
          <>
            {/* Google Login Button */}
            <div className={`google-login-section ${isMobile ? 'mobile' : ''}`}>
              <button
                className={`google-login-button ${isMobile ? 'mobile' : ''}`}
                onClick={handleGoogleLogin}
                disabled={isLoading}
              >
                <svg className={`google-icon ${isMobile ? 'mobile' : ''}`} viewBox="0 0 24 24">
                  <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" />
                  <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" />
                  <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" />
                  <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" />
                </svg>
                <span className={isMobile ? 'mobile-text' : ''}>
                  {isLoading ? 'Connecting...' : 'Continue with Google'}
                </span>
              </button>
            </div>

            {/* Submit Button */}
            <div className={`submit-section ${isMobile ? 'mobile' : ''}`}>
              <button
                className={`submit-button ${isMobile ? 'mobile' : ''}`}
                onClick={handleSubmit}
                disabled={isLoading}
              >
                <span className={isMobile ? 'mobile-text' : ''}>
                  {isLoading ? 'Testing...' : 'Submit Test'}
                </span>
              </button>
            </div>
          </>
        ) : (
          <div className={`authenticated-section ${isMobile ? 'mobile' : ''}`}>
            <div className={`success-message ${isMobile ? 'mobile' : ''}`}>
              ✅ Successfully authenticated!
            </div>
            <button
              className={`logout-button ${isMobile ? 'mobile' : ''}`}
              onClick={handleLogout}
            >
              <span className={isMobile ? 'mobile-text' : ''}>Logout</span>
            </button>
          </div>
        )}

        {message && (
          <div className={`message ${isMobile ? 'mobile' : ''}`}>
            {message}
          </div>
        )}
      </div>
    </div>
  )
}

export default Login
