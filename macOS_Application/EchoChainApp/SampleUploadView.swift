import SwiftUI
import UniformTypeIdentifiers
import CryptoKit

struct SampleUploadView: View {
    @StateObject private var blockchainClient = RealBlockchainClient()
    @StateObject private var p2pClient = RealP2PClient() // Use RealP2PClient

    @State private var sampleTitle: String = ""
    @State private var artistName: String = ""
    @State private var selectedFile: URL? = nil
    @State private var showingFileImporter: Bool = false
    @State private var isUploading: Bool = false
    @State private var showingSuccessAlert: Bool = false
    @State private var showingErrorAlert: Bool = false
    @State private var errorMessage: String = ""
    @State private var isContributionSubmitting = false

    private let maxFileSize: Int = 100 * 1024 * 1024 // 100 MB

    var body: some View {
        VStack(alignment: .leading, spacing: 20) {
            Text("Upload New Music Sample")
                .font(.largeTitle)
                .bold()
                .padding(.bottom, 20)

            VStack(alignment: .leading) {
                Text("Sample Title:")
                    .font(.headline)
                TextField("Enter title", text: $sampleTitle)
                    .textFieldStyle(RoundedBorderTextFieldStyle())
            }

            VStack(alignment: .leading) {
                Text("Artist Name:")
                    .font(.headline)
                TextField("Enter artist name", text: $artistName)
                    .textFieldStyle(RoundedBorderTextFieldStyle())
            }

            VStack(alignment: .leading) {
                Text("Select Audio File:")
                    .font(.headline)
                Button(action: {
                    showingFileImporter = true
                }) {
                    Label(selectedFile?.lastPathComponent ?? "Choose File", systemImage: "doc.fill")
                        .font(.body)
                        .padding(.vertical, 10)
                        .frame(maxWidth: .infinity)
                        .background(selectedFile == nil ? Color.gray.opacity(0.2) : Color.green.opacity(0.2))
                        .foregroundColor(selectedFile == nil ? .black : .green)
                        .cornerRadius(5)
                }
                .fileImporter(
                    isPresented: $showingFileImporter,
                    allowedContentTypes: [.audio],
                    allowsMultipleSelection: false
                ) { result in
                    do {
                        let fileURL = try result.get().first
                        if let url = fileURL {
                            try validateFile(url)
                            selectedFile = url
                        }
                        // TODO: Properly handle security-scoped bookmarks for persistent access if needed.
                        // fileURL.startAccessingSecurityScopedResource() and fileURL.stopAccessingSecurityScopedResource()
                        print("Selected file: \(fileURL?.lastPathComponent ?? "N/A")")
                    } catch {
                        errorMessage = error.localizedDescription
                        showingErrorAlert = true
                    }
                }
            }

            Spacer()

            Button(action: {
                Task {
                    await uploadSample()
                }
            }) {
                if isUploading || isContributionSubmitting {
                    ProgressView()
                        .progressViewStyle(CircularProgressViewStyle(tint: .white))
                        .padding()
                        .frame(maxWidth: .infinity)
                        .background(Color.blue.opacity(0.6))
                        .foregroundColor(.white)
                        .cornerRadius(10)
                } else {
                    Text("Upload Sample")
                        .font(.title2)
                        .padding()
                        .frame(maxWidth: .infinity)
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }
            }
            .disabled(sampleTitle.isEmpty || artistName.isEmpty || selectedFile == nil || isUploading)
        }
        .padding()
        .navigationTitle("Upload Sample")
        .alert("Upload Successful!", isPresented: $showingSuccessAlert) {
            Button("OK") {
                // Reset form or navigate away
                sampleTitle = ""
                artistName = ""
                selectedFile = nil
            }
        } message: {
            Text("Your music sample has been successfully uploaded and registered.")
        }
        .alert("Error", isPresented: $showingErrorAlert) {
            Button("OK") { }
        } message: {
            Text(errorMessage)
        }
        .onAppear {
            Task {
                do {
                    if !p2pClient.isConnected {
                        // TODO: Implement robust connection retry logic for P2P client.
                        try await p2pClient.connect()
                    }
                } catch {
                    errorMessage = error.localizedDescription
                    showingErrorAlert = true
                }
            }
        }
    }

    private func validateFile(_ fileURL: URL) throws {
        guard fileURL.startAccessingSecurityScopedResource() else {
            throw SampleUploadError.fileAccessDenied
        }
        defer { fileURL.stopAccessingSecurityScopedResource() }

        guard fileURL.isFileURL else {
            throw SampleUploadError.invalidFileType("Selected item is not a file.")
        }

        guard let fileType = fileURL.contentType, fileType.conforms(to: .audio) else {
            throw SampleUploadError.invalidFileType("Only audio files are allowed.")
        }

        let fileAttributes = try FileManager.default.attributesOfItem(atPath: fileURL.path)
        guard let fileSize = fileAttributes[.size] as? Int else {
            throw SampleUploadError.fileSizeError("Could not determine file size.")
        }

        guard fileSize <= maxFileSize else {
            throw SampleUploadError.fileSizeError("File size exceeds the maximum allowed (
\(maxFileSize / 1024 / 1024) MB).")
        }
    }

    private func uploadSample() async {
        guard let fileURL = selectedFile, !sampleTitle.isEmpty, !artistName.isEmpty else {
            errorMessage = "Please fill all fields and select a file."
            showingErrorAlert = true
            return
        }

        isUploading = true
        do {
            // Validation already done in validateFile, but re-access for upload
            let accessed = fileURL.startAccessingSecurityScopedResource()
            defer {
                if accessed {
                    fileURL.stopAccessingSecurityScopedResource()
                }
            }

            // 1. Hash the audio file
            let fileData = try Data(contentsOf: fileURL)
            let fileHash = SHA256.hash(data: fileData).compactMap { String(format: "%02x", $0) }.joined()
            print("File Hashed: \(fileHash)")

            // 2. Initiate P2P file sharing (upload)
            let p2pContentId = try await p2pClient.uploadFile(at: fileURL)
            print("P2P Uploaded with Content ID: \(p2pContentId)")

            // 3. Register metadata on the blockchain
            let blockchainTxHash = try await blockchainClient.registerSampleMetadata(
                title: sampleTitle,
                artist: artistName,
                p2pContentId: p2pContentId,
                blockchainHash: fileHash
            )
            print("Blockchain Registration Tx Hash: \(blockchainTxHash)")

            // 4. Submit content contribution (real file size as amount)
            isContributionSubmitting = true
            defer { isContributionSubmitting = false }
            do {
                let contributionTxHash = try await blockchainClient.submitContentContribution(amount: UInt64(fileData.count))
                print("Content Contribution Tx Hash: \(contributionTxHash)")
            } catch {
                errorMessage = "Contribution submission failed: \(error.localizedDescription)"
                showingErrorAlert = true
            }

            showingSuccessAlert = true
        } catch {
            errorMessage = "Upload failed: \(error.localizedDescription)"
            showingErrorAlert = true
        }
        isUploading = false
    }
}

enum SampleUploadError: Error, LocalizedError {
    case fileAccessDenied
    case invalidFileType(String)
    case fileSizeError(String)

    var errorDescription: String? {
        switch self {
        case .fileAccessDenied:
            return "Access to the selected file was denied."
        case .invalidFileType(let message):
            return "Invalid file type: \(message)"
        case .fileSizeError(let message):
            return "File size error: \(message)"
        }
    }
}

struct SampleUploadView_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            SampleUploadView()
        }
    }
}