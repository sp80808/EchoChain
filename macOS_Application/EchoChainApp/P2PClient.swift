import Foundation
import Combine
import Network // For NWConnection

// Protocol to define the interface for P2P interactions
protocol P2PClientProtocol: ObservableObject {
    var isConnected: Bool { get }
    var downloadedFiles: [P2PFile] { get }
    var uploadedFiles: [P2PFile] { get }
    var totalBytesUploaded: UInt64 { get }
    var totalBytesDownloaded: UInt64 { get }

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
    @Published var totalBytesUploaded: UInt64 = 0
    @Published var totalBytesDownloaded: UInt64 = 0

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
                    try await Task.sleep(nanoseconds: 1_000_000_000) // Wait 1 second before retrying
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
        // In a real-world scenario, ensure all P2P resources (e.g., open sockets, background tasks)
        // are properly released and cleaned up upon disconnection to prevent resource leaks.
        print("RealP2PClient: Disconnected from local P2P node API.")
    }

    private func sendLocalCommand(commandType: String, payload: [String: Any]) async throws -> [String: Any] {
        guard let connection = connection, isConnected else {
            throw P2PClientError.notConnected
        }

        // The current implementation assumes the Python node returns a simple JSON dictionary.
        // TODO: For production, consider a more robust JSON-RPC client or a dedicated P2P communication protocol.
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
        // Increment totalBytesUploaded
        if let fileSize = try? FileManager.default.attributesOfItem(atPath: url.path)[.size] as? UInt64 {
            self.totalBytesUploaded += fileSize
        }
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
        
        print("RealP2PClient: Download initiated for \(contentId). Polling for completion...")
        
        // Poll for download status until complete
        var downloadStatus: String = "pending"
        var downloadedFilePath: String? = nil
        let maxPollAttempts = 60 // Poll for up to 60 seconds
        var attempt = 0
        
        while downloadStatus != "completed" && attempt < maxPollAttempts {
            try await Task.sleep(nanoseconds: 1_000_000_000) // Poll every 1 second
            attempt += 1
            
            let statusResponse = try await sendLocalCommand(commandType: "local_get_download_status", payload: ["content_hash": contentId])
            
            if let status = statusResponse["status"] as? String, status == "success",
               let currentDownloadStatus = statusResponse["download_status"] as? String {
                downloadStatus = currentDownloadStatus
                downloadedFilePath = statusResponse["local_path"] as? String
                print("Download status for \(contentId): \(downloadStatus)")
            } else {
                print("Failed to get download status for \(contentId): \(statusResponse["message"] as? String ?? "Unknown error")")
            }
        }
        
        guard downloadStatus == "completed", let finalPath = downloadedFilePath else {
            throw P2PClientError.downloadFailed("Download for \(contentId) did not complete in time or failed.")
        }
        
        let downloadedFileURL = URL(fileURLWithPath: finalPath)
        
        let newDownloadedFile = P2PFile(id: UUID(), contentId: contentId, fileName: downloadedFileURL.lastPathComponent, localPath: downloadedFileURL.path, status: .downloaded)
        self.downloadedFiles.append(newDownloadedFile)
        // Increment totalBytesDownloaded
        if let fileSize = try? FileManager.default.attributesOfItem(atPath: downloadedFileURL.path)[.size] as? UInt64 {
            self.totalBytesDownloaded += fileSize
        }
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