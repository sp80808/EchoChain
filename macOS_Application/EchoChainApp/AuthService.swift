import Foundation
import Combine

protocol AuthServiceProtocol: ObservableObject {
    var isAuthenticated: Bool { get }
    var authToken: String? { get }

    func login(email: String, password: String) async throws
    func register(email: String, password: String, referrerCode: String?) async throws
    func logout()
    func getToken() -> String?
}

class RealAuthService: AuthServiceProtocol {
    @Published var isAuthenticated: Bool = false
    @Published var authToken: String? = nil

    private let backendBaseURL = URL(string: "http://127.0.0.1:3001/api/auth")!
    private let tokenKey = "authToken"

    init() {
        // Attempt to load token from secure storage on initialization
        if let storedToken = UserDefaults.standard.string(forKey: tokenKey) { // Using UserDefaults for simplicity, SecureStorage is preferred for production
            self.authToken = storedToken
            self.isAuthenticated = true
        }
    }

    func login(email: String, password: String) async throws {
        guard let url = URL(string: backendBaseURL.absoluteString + "/login") else { throw APIError.invalidURL }

        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let body: [String: Any] = ["email": email, "password": password]
        request.httpBody = try JSONSerialization.data(withJSONObject: body, options: [])

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            let errorData = String(data: data, encoding: .utf8) ?? "Unknown error"
            throw APIError.requestFailed("Login failed: \(httpResponse.statusCode) - \(errorData)")
        }

        struct AuthResponse: Codable {
            let token: String
        }

        let decodedResponse = try JSONDecoder().decode(AuthResponse.self, from: data)
        storeToken(token: decodedResponse.token)
        self.isAuthenticated = true
        self.authToken = decodedResponse.token
    }

    func register(email: String, password: String, referrerCode: String?) async throws {
        guard let url = URL(string: backendBaseURL.absoluteString + "/register") else { throw APIError.invalidURL }

        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        var body: [String: Any] = ["email": email, "password": password]
        if let referrerCode = referrerCode, !referrerCode.isEmpty {
            body["referrerCode"] = referrerCode
        }
        request.httpBody = try JSONSerialization.data(withJSONObject: body, options: [])

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            let errorData = String(data: data, encoding: .utf8) ?? "Unknown error"
            throw APIError.requestFailed("Registration failed: \(httpResponse.statusCode) - \(errorData)")
        }

        struct AuthResponse: Codable {
            let token: String
        }

        let decodedResponse = try JSONDecoder().decode(AuthResponse.self, from: data)
        storeToken(token: decodedResponse.token)
        self.isAuthenticated = true
        self.authToken = decodedResponse.token
    }

    func logout() {
        clearToken()
        self.isAuthenticated = false
        self.authToken = nil
    }

    func getToken() -> String? {
        return authToken
    }

    private func storeToken(token: String) {
        UserDefaults.standard.set(token, forKey: tokenKey)
    }

    private func clearToken() {
        UserDefaults.standard.removeObject(forKey: tokenKey)
    }
}
