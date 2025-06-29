import SwiftUI
import AVFoundation

struct SampleBrowserView: View {
    @StateObject private var p2pClient = RealP2PClient() // Use RealP2PClient
    @State private var samples: [P2PFileMetadata] = []
    @State private var showingErrorAlert = false
    @State private var errorMessage: String = ""
    @State private var audioPlayer: AVPlayer?

    var body: some View {
        VStack {
            Text("Browse Music Samples")
                .font(.largeTitle)
                .bold()
                .padding(.bottom, 20)

            // TODO: Implement search bar and filtering options for samples.
            // This will require updating fetchAvailableSamples to accept search parameters.

            if samples.isEmpty {
                ProgressView("Loading Samples...")
                    .padding()
            } else {
                List(samples) { sample in
                    SampleRow(sample: sample, playAction: {
                        Task {
                            // TODO: Add proper error handling and UI feedback for sample playback.
                            await playSample(contentId: sample.contentId)
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

    private func fetchSamples() async {
        do {
            if !p2pClient.isConnected {
                // TODO: Implement robust connection retry logic for P2P client.
                try await p2pClient.connect()
            }
            // TODO: Integrate with backend API to fetch richer sample metadata, not just P2P metadata.
            samples = try await p2pClient.fetchAvailableSamples()
        } catch {
            errorMessage = error.localizedDescription
            showingErrorAlert = true
        }
    }

    private func playSample(contentId: String) async {
        do {
            // TODO: Implement proper audio streaming or progressive download for large files.
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