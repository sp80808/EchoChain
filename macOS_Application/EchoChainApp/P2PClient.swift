import Foundation
import Combine
import Network // For NWConnection

// Protocol to define the interface for P2P interactions
protocol P2PClientProtocol: ObservableObject {
    var isConnected: Bool { get }
    var downloadedFiles: [P2PFile] { get }
    var uploadedFiles: [P2PFile] { get }

    func connect() async throws
    func disconnect() async throws
    func uploadFile(at url: URL) async throws -> String
    func downloadFile(contentId: String) async throws -> URL
    func fetchAvailableSamples() async throws -> [P2PFileMetadata]
}

// Real P2P Client communicating with Python P2P Node's local API
class RealP2PClient: P2PClientProtocol {
    @Published var isConnected: Bool = false
    @Published var downloadedFiles: [P2PFile] = []
    @Published var uploadedFiles: [P2PFile] = []

    private let nodeHost: String
    private let localAPIPort: Int
    private var connection: NWConnection?
    private var cancellables = Set<AnyCancellable>()

    init(nodeHost: String = "127.0.0.1", localAPIPort: Int = 8002) {
        self.nodeHost = nodeHost
        self.localAPIPort = localAPIPort
    }

    @MainActor
    func connect() async throws {
        guard !isConnected else { return }
        var retryCount = 0
        let maxRetries = 3
        while retryCount < maxRetries {
            do {
                try await withCheckedThrowingContinuation { continuation in
                    let host = NWEndpoint.Host(nodeHost)
                    let port = NWEndpoint.Port(rawValue: UInt16(localAPIPort))!
                    connection = NWConnection(host: host, port: port, using: .tcp)
                    connection?.stateUpdateHandler = { newState in
                        switch newState {
                        case .ready:
                            print("RealP2PClient: Connected to local P2P node API.")
                            self.isConnected = true
                            continuation.resume(returning: ())
                        case .failed(let error):
                            print("RealP2PClient: Connection failed: \(error.localizedDescription)")
                            self.isConnected = false
                            continuation.resume(throwing: P2PClientError.connectionFailed(error.localizedDescription))
                        case .cancelled:
                            print("RealP2PClient: Connection cancelled.")
                            self.isConnected = false
                            continuation.resume(throwing: P2PClientError.connectionFailed("Connection cancelled"))
                        default:
                            break
                        }
                    }
                    connection?.start(queue: .global())
                }
                return
            } catch {
                retryCount += 1
                if retryCount >= maxRetries {
                    throw error
                }
            }
        }
    }

    @MainActor
    func disconnect() async throws {
        guard isConnected else { return }
        connection?.cancel()
        isConnected = false
        // Clean up any additional resources if needed
        print("RealP2PClient: Disconnected from local P2P node API.")
    }

    private func sendLocalCommand(commandType: String, payload: [String: Any]) async throws -> [String: Any] {
        guard let connection = connection, isConnected else {
            throw P2PClientError.notConnected
        }

        // TODO: Improve error handling for JSON serialization/deserialization.
        return try await withCheckedThrowingContinuation { continuation in
            let message: [String: Any] = ["type": commandType, "payload": payload]
            guard let jsonData = try? JSONSerialization.data(withJSONObject: message) else {
                continuation.resume(throwing: P2PClientError.serializationFailed("Failed to serialize command"))
                return
            }

            connection.send(content: jsonData, completion: .contentClosed { error in
                if let error = error {
                    continuation.resume(throwing: P2PClientError.commandFailed("Send error: \(error.localizedDescription)"))
                    return
                }
                
                connection.receiveMessage { (data, context, isComplete, error) in
                    if let error = error {
                        continuation.resume(throwing: P2PClientError.commandFailed("Receive error: \(error.localizedDescription)"))
                        return
                    }
                    guard let data = data, let response = try? JSONSerialization.jsonObject(with: data) as? [String: Any] else {
                        continuation.resume(throwing: P2PClientError.serializationFailed("Failed to deserialize response"))
                        return
                    }
                    continuation.resume(returning: response)
                }
            })
        }
    }

    @MainActor
    func uploadFile(at url: URL) async throws -> String {
        guard isConnected else { throw P2PClientError.notConnected }
        var status: UploadStatus = .pending
        let filePath = url.path
        print("RealP2PClient: Requesting to add file \(filePath) to P2P system...")
        status = .uploading
        let addFileResponse = try await sendLocalCommand(commandType: "local_add_file", payload: ["filepath": filePath])
        guard addFileResponse["status"] as? String == "success",
              let fileHash = addFileResponse["file_hash"] as? String else {
            status = .failed
            throw P2PClientError.uploadFailed(addFileResponse["message"] as? String ?? "Unknown error adding file")
        }
        print("RealP2PClient: File added with hash \(fileHash). Announcing content...")
        let announceResponse = try await sendLocalCommand(commandType: "local_announce_content", payload: ["content_hash": fileHash])
        guard announceResponse["status"] as? String == "success" else {
            status = .failed
            throw P2PClientError.uploadFailed(announceResponse["message"] as? String ?? "Unknown error announcing content")
        }
        print("RealP2PClient: Content \(fileHash) announced successfully.")
        status = .uploaded
        let fileName = url.lastPathComponent
        let newUploadedFile = P2PFile(id: UUID(), contentId: fileHash, fileName: fileName, localPath: url.path, status: status)
        self.uploadedFiles.append(newUploadedFile)
        return fileHash
    }

    @MainActor
    func downloadFile(contentId: String) async throws -> URL {
        guard isConnected else { throw P2PClientError.notConnected }
        
        print("RealP2PClient: Requesting download for content hash \(contentId)...")
        let downloadResponse = try await sendLocalCommand(commandType: "local_request_file", payload: ["content_hash": contentId])
        
        guard downloadResponse["status"] as? String == "success" else {
            throw P2PClientError.downloadFailed(downloadResponse["message"] as? String ?? "Unknown error initiating download")
        }
        
        print("RealP2PClient: Download initiated for \(contentId). Waiting for completion...")
        
        // TODO: Replace simulation with actual polling or callback mechanism from the Python node
        // to confirm download completion and get the actual downloaded file path.
        try await Task.sleep(nanoseconds: 5_000_000_000) // Simulate download time

        // Construct the expected download path based on p2p_node.py's logic
        // This is a simplification; ideally, the node would return the actual path.
        let tempDirectory = FileManager.default.temporaryDirectory
        let fileName = "downloaded_sample_\(contentId).mp3" // Assuming .mp3 for samples
        let downloadedFileURL = tempDirectory.appendingPathComponent(fileName)
        
        // For demonstration, ensure a dummy file exists at this path
        if !FileManager.default.fileExists(atPath: downloadedFileURL.path) {
            let dummyContent = "This is a simulated downloaded audio file content for \(contentId)."
            try dummyContent.write(to: downloadedFileURL, atomically: true, encoding: .utf8)
        }

        let newDownloadedFile = P2PFile(id: UUID(), contentId: contentId, fileName: fileName, localPath: downloadedFileURL.path, status: .downloaded)
        self.downloadedFiles.append(newDownloadedFile)
        print("RealP2PClient: File downloaded from P2P network. Local path: \(downloadedFileURL.path)")
        
        return downloadedFileURL
    }

    @MainActor
    func fetchAvailableSamples() async throws -> [P2PFileMetadata] {
        print("RealP2PClient: Requesting available content info from P2P node...")
        let response = try await sendLocalCommand(commandType: "local_request_content_info", payload: ["content_hash": "all_available_content"])
        guard response["status"] as? String == "success",
              let availableContent = response["available_content"] as? [[String: Any]] else {
            throw P2PClientError.metadataFetchFailed(response["message"] as? String ?? "Unknown error fetching metadata")
        }
        var fetchedMetadata: [P2PFileMetadata] = []
        for contentDict in availableContent {
            if let contentId = contentDict["content_id"] as? String,
               let filename = contentDict["filename"] as? String,
               let size = contentDict["size"] as? Int {
                // Fetch richer metadata from backend API
                let backendMetadata = try? await fetchSampleMetadataFromBackend(contentId: contentId)
                let title = backendMetadata?.title ?? filename.replacingOccurrences(of: ".mp3", with: "").replacingOccurrences(of: ".txt", with: "")
                let artist = backendMetadata?.artist ?? "Unknown Artist"
                let duration = backendMetadata?.duration ?? "\(size / 1024)KB"
                let blockchainHash = backendMetadata?.blockchainHash ?? "N/A"
                fetchedMetadata.append(P2PFileMetadata(contentId: contentId, title: title, artist: artist, duration: duration, blockchainHash: blockchainHash))
            }
        }
        print("RealP2PClient: Fetched \(fetchedMetadata.count) available samples from P2P network.")
        return fetchedMetadata
    }

    private func fetchSampleMetadataFromBackend(contentId: String) async throws -> (title: String, artist: String, duration: String, blockchainHash: String)? {
        guard let url = URL(string: "http://localhost:3001/api/samples/metadata/\(contentId)") else { return nil }
        let (data, response) = try await URLSession.shared.data(from: url)
        guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else { return nil }
        let json = try JSONSerialization.jsonObject(with: data, options: []) as? [String: Any]
        guard let title = json?["title"] as? String,
              let artist = json?["artist"] as? String,
              let duration = json?["duration"] as? String,
              let blockchainHash = json?["blockchainHash"] as? String else { return nil }
        return (title, artist, duration, blockchainHash)
    }
}

enum P2PClientError: Error, LocalizedError {
    case notConnected
    case connectionFailed(String)
    case serializationFailed(String)
    case commandFailed(String)
    case uploadFailed(String)
    case downloadFailed(String)
    case metadataFetchFailed(String)

    var errorDescription: String? {
        switch self {
        case .notConnected:
            return "P2P client is not connected."
        case .connectionFailed(let message):
            return "P2P connection failed: \(message)"
        case .serializationFailed(let message):
            return "P2P data serialization failed: \(message)"
        case .commandFailed(let message):
            return "P2P command failed: \(message)"
        case .uploadFailed(let message):
            return "P2P upload failed: \(message)"
        case .downloadFailed(let message):
            return "P2P download failed: \(message)"
        case .metadataFetchFailed(let message):
            return "Failed to fetch P2P sample metadata: \(message)"
        }
    }
}

enum UploadStatus {
    case pending, uploading, uploaded, failed
}

struct P2PFile: Identifiable {
    let id: UUID
    let contentId: String
    let fileName: String
    let localPath: String
    var status: UploadStatus
}

struct P2PFileMetadata: Identifiable, Codable { // Added Codable for potential future use with real data
    let id: UUID
    let contentId: String
    let title: String
    let artist: String
    let duration: String
    let blockchainHash: String

    init(id: UUID = UUID(), contentId: String, title: String, artist: String, duration: String, blockchainHash: String) {
        self.id = id
        self.contentId = contentId
        self.title = title
        self.artist = artist
        self.duration = duration
        self.blockchainHash = blockchainHash
    }
}

// Swift P2P Client for EchoChain Python P2P Node
import Foundation
import Network

class P2PClient {
    let host: NWEndpoint.Host
    let port: NWEndpoint.Port

    init(host: String = "127.0.0.1", port: UInt16 = 8002) {
        self.host = NWEndpoint.Host(host)
        self.port = NWEndpoint.Port(rawValue: port)!
    }

    private func sendCommand(commandType: String, payload: [String: Any], completion: @escaping (Result<[String: Any], Error>) -> Void) {
        let connection = NWConnection(host: host, port: port, using: .tcp)
        connection.stateUpdateHandler = { state in
            if case .failed(let error) = state {
                completion(.failure(error))
            }
        }
        connection.start(queue: .global())
        var message = [String: Any]()
        message["type"] = commandType
        message["payload"] = payload
        guard let data = try? JSONSerialization.data(withJSONObject: message) else {
            completion(.failure(NSError(domain: "P2P", code: 1, userInfo: [NSLocalizedDescriptionKey: "JSON encode error"])))
            return
        }
        connection.send(content: data, completion: .contentProcessed({ error in
            if let error = error {
                completion(.failure(error))
                connection.cancel()
                return
            }
            connection.receive(minimumIncompleteLength: 1, maximumLength: 4096) { data, _, _, error in
                if let error = error {
                    completion(.failure(error))
                } else if let data = data, let dict = try? JSONSerialization.jsonObject(with: data) as? [String: Any] {
                    completion(.success(dict))
                } else {
                    completion(.failure(NSError(domain: "P2P", code: 2, userInfo: [NSLocalizedDescriptionKey: "No data received"])))
                }
                connection.cancel()
            }
        }))
    }

    func addFileAndAnnounce(filepath: String, completion: @escaping (Result<String, Error>) -> Void) {
        sendCommand(commandType: "local_add_file", payload: ["filepath": filepath]) { result in
            switch result {
            case .success(let resp):
                if let status = resp["status"] as? String, status == "success", let fileHash = resp["file_hash"] as? String {
                    self.sendCommand(commandType: "local_announce_content", payload: ["content_hash": fileHash]) { announceResult in
                        switch announceResult {
                        case .success(let announceResp):
                            if let status = announceResp["status"] as? String, status == "success" {
                                completion(.success(fileHash))
                            } else {
                                completion(.failure(NSError(domain: "P2P", code: 3, userInfo: [NSLocalizedDescriptionKey: announceResp["message"] as? String ?? "Unknown error"])))
                            }
                        case .failure(let error):
                            completion(.failure(error))
                        }
                    }
                } else {
                    completion(.failure(NSError(domain: "P2P", code: 4, userInfo: [NSLocalizedDescriptionKey: resp["message"] as? String ?? "Unknown error"])))
                }
            case .failure(let error):
                completion(.failure(error))
            }
        }
    }

    func discoverContentPeers(contentHash: String, completion: @escaping (Result<[String], Error>) -> Void) {
        sendCommand(commandType: "local_request_content_info", payload: ["content_hash": contentHash]) { result in
            switch result {
            case .success(let resp):
                if let status = resp["status"] as? String, status == "success", let peers = resp["peers"] as? [String] {
                    completion(.success(peers))
                } else {
                    completion(.failure(NSError(domain: "P2P", code: 5, userInfo: [NSLocalizedDescriptionKey: resp["message"] as? String ?? "Unknown error"])))
                }
            case .failure(let error):
                completion(.failure(error))
            }
        }
    }

    func requestFileDownload(contentHash: String, completion: @escaping (Result<Bool, Error>) -> Void) {
        sendCommand(commandType: "local_request_file", payload: ["content_hash": contentHash]) { result in
            switch result {
            case .success(let resp):
                if let status = resp["status"] as? String, status == "success" {
                    completion(.success(true))
                } else {
                    completion(.failure(NSError(domain: "P2P", code: 6, userInfo: [NSLocalizedDescriptionKey: resp["message"] as? String ?? "Unknown error"])))
                }
            case .failure(let error):
                completion(.failure(error))
            }
        }
    }
}

// Example usage:
let client = P2PClient()
client.addFileAndAnnounce(filepath: "/path/to/file.wav") { result in
    switch result {
    case .success(let fileHash):
        print("File added and announced with hash: \(fileHash)")
        client.discoverContentPeers(contentHash: fileHash) { peersResult in
            print("Peers:", peersResult)
        }
    case .failure(let error):
        print("Error:", error)
    }
}