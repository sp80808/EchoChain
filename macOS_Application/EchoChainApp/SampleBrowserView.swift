import SwiftUI
import AVFoundation

struct SampleBrowserView: View {
    @StateObject private var p2pClient = RealP2PClient() // Use RealP2PClient
    @State private var samples: [P2PFileMetadata] = []
    @State private var showingErrorAlert = false
    @State private var errorMessage: String = ""
    @State private var audioPlayer: AVPlayer?
    @State private var searchQuery: String = ""
    @State private var filterCategory: String = ""

    var body: some View {
        VStack {
            Text("Browse Music Samples")
                .font(.largeTitle)
                .bold()
                .padding(.bottom, 20)

            HStack {
                TextField("Search samples...", text: $searchQuery)
                    .textFieldStyle(RoundedBorderTextFieldStyle())
                    .padding(.horizontal)
                Picker("Category", selection: $filterCategory) {
                    Text("All").tag("")
                    Text("Drums").tag("Drums")
                    Text("Vocals").tag("Vocals")
                    Text("Synths").tag("Synths")
                    Text("FX").tag("FX")
                }
                .pickerStyle(MenuPickerStyle())
                .padding(.horizontal)
            }
            .padding(.top)

            if samples.isEmpty {
                ProgressView("Loading Samples...")
                    .padding()
            } else {
                List(filteredSamples) { sample in
                    SampleRow(sample: sample, playAction: {
                        Task {
                            do {
                                await playSample(contentId: sample.contentId)
                            } catch {
                                errorMessage = "Failed to play sample: \(error.localizedDescription)"
                                showingErrorAlert = true
                            }
                        }
                    })
                }
                .listStyle(PlainListStyle())
            }

            Spacer()

            NavigationLink(destination: SampleUploadView()) {
                Text("Upload New Sample")
                    .font(.title2)
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(Color.purple)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .padding()
        }
        .navigationTitle("Samples")
        .onAppear {
            Task {
                // TODO: Ensure samples are fetched efficiently and refreshed as needed.
                await fetchSamples()
            }
        }
        .alert("Error", isPresented: $showingErrorAlert) {
            Button("OK") { }
        } message: {
            Text(errorMessage)
        }
    }

    var filteredSamples: [P2PFileMetadata] {
        samples.filter { sample in
            (searchQuery.isEmpty || sample.title.localizedCaseInsensitiveContains(searchQuery)) &&
            (filterCategory.isEmpty || sample.category == filterCategory)
        }
    }

    private func fetchSamples() async {
        var retryCount = 0
        let maxRetries = 3
        do {
            while !p2pClient.isConnected && retryCount < maxRetries {
                do {
                    try await p2pClient.connect()
                } catch {
                    retryCount += 1
                    if retryCount >= maxRetries {
                        throw error
                    }
                }
            }
            samples = try await p2pClient.fetchAvailableSamples()
        } catch {
            errorMessage = error.localizedDescription
            showingErrorAlert = true
        }
    }

    private func playSample(contentId: String) async {
        do {
            // TODO: Implement proper audio streaming or progressive download for large files.
            // The current implementation downloads the entire file before playing, which is inefficient for large samples.
            // Consider using AVPlayerItem with AVAssetResourceLoaderDelegate for custom loading,
            // or integrating a dedicated streaming library that works with the P2P client.
            let fileURL = try await p2pClient.downloadFile(contentId: contentId)
            audioPlayer = AVPlayer(url: fileURL)
            audioPlayer?.play()
            print("Playing sample from: \(fileURL.lastPathComponent)")
        } catch {
            errorMessage = "Failed to play sample: \(error.localizedDescription)"
            showingErrorAlert = true
        }
    }
}

struct SampleRow: View {
    let sample: P2PFileMetadata
    let playAction: () -> Void

    var body: some View {
        HStack {
            VStack(alignment: .leading) {
                Text(sample.title)
                    .font(.headline)
                Text(sample.artist)
                    .font(.subheadline)
                    .foregroundColor(.gray)
            }
            Spacer()
            Text(sample.duration)
                .font(.subheadline)
            Button(action: playAction) {
                Image(systemName: "play.circle.fill")
                    .font(.title)
                    .foregroundColor(.blue)
            }
        }
        .padding(.vertical, 5)
    }
}

struct SampleBrowserView_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            SampleBrowserView()
        }
    }
}