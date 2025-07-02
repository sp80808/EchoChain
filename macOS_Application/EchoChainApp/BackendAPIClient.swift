import Foundation

protocol BackendAPIClientProtocol {
    func fetchSamples(query: String?, category: String?) async throws -> [Sample]
}

class RealBackendAPIClient: BackendAPIClientProtocol {
    private let baseURL = URL(string: "http://127.0.0.1:3001/api")!
    private let authService: AuthServiceProtocol

    init(authService: AuthServiceProtocol = RealAuthService()) {
        self.authService = authService
    }

    func fetchSamples(query: String?, category: String?, bpm: String?, key: String?, tags: String?, sortBy: String?, order: String?) async throws -> [Sample] {
        var urlComponents = URLComponents(url: baseURL.appendingPathComponent("samples"), resolvingAgainstBaseURL: false)!
        var queryItems: [URLQueryItem] = []

        if let query = query, !query.isEmpty {
            queryItems.append(URLQueryItem(name: "search", value: query))
        }

        if let category = category, !category.isEmpty {
            queryItems.append(URLQueryItem(name: "category", value: category))
        }

        if let bpm = bpm, !bpm.isEmpty {
            queryItems.append(URLQueryItem(name: "bpm", value: bpm))
        }

        if let key = key, !key.isEmpty {
            queryItems.append(URLQueryItem(name: "key", value: key))
        }

        if let tags = tags, !tags.isEmpty {
            queryItems.append(URLQueryItem(name: "tags", value: tags))
        }

        if let sortBy = sortBy, !sortBy.isEmpty {
            queryItems.append(URLQueryItem(name: "sortBy", value: sortBy))
        }

        if let order = order, !order.isEmpty {
            queryItems.append(URLQueryItem(name: "order", value: order))
        }

        urlComponents.queryItems = queryItems

        guard let url = urlComponents.url else {
            throw APIError.invalidURL
        }

        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        if let token = authService.getToken() {
            request.setValue("Bearer \(token)", forHTTPHeaderField: "x-auth-token")
        }

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            throw APIError.requestFailed
        }

        let samples = try JSONDecoder().decode([Sample].self, from: data)
        return samples
    }

    func fetchUserAndSamples(userId: String) async throws -> (User, [Sample]) {
        guard let url = URL(string: baseURL.absoluteString + "/users/" + userId) else {
            throw APIError.invalidURL
        }

        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        if let token = authService.getToken() {
            request.setValue("Bearer \(token)", forHTTPHeaderField: "x-auth-token")
        }

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw APIError.requestFailed
        }

        if httpResponse.statusCode == 404 {
            throw APIError.userNotFound
        } else if httpResponse.statusCode != 200 {
            throw APIError.requestFailed
        }

        struct UserAndSamplesResponse: Codable {
            let user: User
            let samples: [Sample]
        }

        let decodedResponse = try JSONDecoder().decode(UserAndSamplesResponse.self, from: data)
        return (decodedResponse.user, decodedResponse.samples)
    }

    func fetchReferredUsersCount(userId: String) async throws -> Int {
        guard let url = URL(string: baseURL.absoluteString + "/users/" + userId + "/referred-count") else {
            throw APIError.invalidURL
        }

        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        if let token = authService.getToken() {
            request.setValue("Bearer \(token)", forHTTPHeaderField: "x-auth-token")
        }

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLRESPONSE, httpResponse.statusCode == 200 else {
            throw APIError.requestFailed
        }

        struct ReferredCountResponse: Codable {
            let count: Int
        }

        let decodedResponse = try JSONDecoder().decode(ReferredCountResponse.self, from: data)
        return decodedResponse.count
    }
}

struct Sample: Codable, Identifiable {
    let id: String
    let title: String
    let artist: String
    let duration: String
    let category: String
    let p2pContentId: String
    let price: Double
    let ownerAddress: String
    let usageCount: Int // Added for incentive system
    let bpm: Int?
    let key: String?
    let creatorId: String // Added for linking to creator profiles
}

enum APIError: Error, LocalizedError {
    case invalidURL
    case requestFailed(String)
    case userNotFound

    var errorDescription: String? {
        switch self {
        case .invalidURL:
            return "Invalid URL for API request."
        case .requestFailed(let message):
            return "API request failed: \(message)"
        case .userNotFound:
            return "The requested user could not be found."
        }
    }
}
