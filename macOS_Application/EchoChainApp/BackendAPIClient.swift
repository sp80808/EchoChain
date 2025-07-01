import Foundation

protocol BackendAPIClientProtocol {
    func fetchSamples(query: String?, category: String?) async throws -> [Sample]
}

class RealBackendAPIClient: BackendAPIClientProtocol {
    private let baseURL = URL(string: "http://127.0.0.1:3000/api")!

    func fetchSamples(query: String?, category: String?) async throws -> [Sample] {
        var urlComponents = URLComponents(url: baseURL.appendingPathComponent("samples"), resolvingAgainstBaseURL: false)!
        var queryItems: [URLQueryItem] = []

        if let query = query, !query.isEmpty {
            queryItems.append(URLQueryItem(name: "search", value: query))
        }

        if let category = category, !category.isEmpty {
            queryItems.append(URLQueryItem(name: "category", value: category))
        }

        urlComponents.queryItems = queryItems

        guard let url = urlComponents.url else {
            throw APIError.invalidURL
        }

        let (data, response) = try await URLSession.shared.data(from: url)

        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
            throw APIError.requestFailed
        }

        let samples = try JSONDecoder().decode([Sample].self, from: data)
        return samples
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
}

enum APIError: Error {
    case invalidURL
    case requestFailed
}
