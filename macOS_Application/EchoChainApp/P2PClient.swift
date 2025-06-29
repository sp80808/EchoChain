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
        
        return try await withCheckedThrowingContinuation { continuation in
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
    }

    @MainActor
    func disconnect() async throws {
        guard isConnected else { return }
        connection?.cancel()
        isConnected = false
        print("RealP2PClient: Disconnected from local P2P node API.")
    }

    private func sendLocalCommand(commandType: String, payload: [String: Any]) async throws -> [String: Any] {
        guard let connection = connection, isConnected else {
            throw P2PClientError.notConnected
        }

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
        
        // Ensure the file URL can be accessed by the Python script
        // For local development, we might need to pass the absolute path
        let filePath = url.path
        print("RealP2PClient: Requesting to add file \(filePath) to P2P system...")
        
        let addFileResponse = try await sendLocalCommand(commandType: "local_add_file", payload: ["filepath": filePath])
        
        guard addFileResponse["status"] as? String == "success",
              let fileHash = addFileResponse["file_hash"] as? String else {
            throw P2PClientError.uploadFailed(addFileResponse["message"] as? String ?? "Unknown error adding file")
        }
        
        print("RealP2PClient: File added with hash \(fileHash). Announcing content...")
        let announceResponse = try await sendLocalCommand(commandType: "local_announce_content", payload: ["content_hash": fileHash])
        
        guard announceResponse["status"] as? String == "success" else {
            throw P2PClientError.uploadFailed(announceResponse["message"] as? String ?? "Unknown error announcing content")
        }
        
        print("RealP2PClient: Content \(fileHash) announced successfully.")
        
        // Add to uploaded files list (simplified, actual status tracking would be more complex)
        let fileName = url.lastPathComponent
        let newUploadedFile = P2PFile(id: UUID(), contentId: fileHash, fileName: fileName, localPath: url.path, status: .uploaded)
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
        
        // In a real scenario, the Python node would notify the client upon completion,
        // or the client would poll for status. For now, we'll simulate waiting
        // and then assume success and construct a dummy path.
        // A more robust solution would involve the Python node returning the final path.
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
        guard isConnected else { throw P2PClientError.notConnected }
        
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
                // Placeholder for title, artist, duration, blockchainHash
                // In a real system, this metadata would come from the blockchain or a more detailed P2P metadata exchange
                let title = filename.replacingOccurrences(of: ".mp3", with: "").replacingOccurrences(of: ".txt", with: "")
                let artist = "Unknown Artist"
                let duration = "\(size / 1024)KB" // Simple size-based duration for now
                let blockchainHash = "N/A" // This would come from blockchain integration
                
                fetchedMetadata.append(P2PFileMetadata(contentId: contentId, title: title, artist: artist, duration: duration, blockchainHash: blockchainHash))
            }
        }
        
        print("RealP2PClient: Fetched \(fetchedMetadata.count) available samples from P2P network.")
        return fetchedMetadata
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

struct P2PFile: Identifiable {
    let id: UUID
    let contentId: String
    let fileName: String
    let localPath: String
    let status: P2PFileStatus
}

enum P2PFileStatus {
    case uploaded
    case downloaded
    case pendingUpload
    case pendingDownload
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