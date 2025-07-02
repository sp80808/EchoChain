import SwiftUI
import AVFoundation

struct SampleBrowserView: View {
    @StateObject private var backendAPIClient = RealBackendAPIClient(authService: RealAuthService()) // Pass authService
    @EnvironmentObject private var blockchainClient: RealBlockchainClient
    @EnvironmentObject var authService: RealAuthService // Inject AuthService
    @State private var samples: [Sample] = []
    @State private var showingErrorAlert = false
    @State private var errorMessage: String = ""
    @State private var audioPlayer: AVPlayer?
    @State private var searchQuery: String = ""
    @State private var filterCategory: String = ""
    @State private var filterBPM: String = "" // New BPM filter
    @State private var filterKey: String = "" // New Key filter
    @State private var isLoadingSamples: Bool = false // New loading state

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
            HStack {
                TextField("BPM (e.g., 120)", text: $filterBPM)
                    .textFieldStyle(RoundedBorderTextFieldStyle())
                    .keyboardType(.numberPad)
                    .padding(.horizontal)
                Picker("Key", selection: $filterKey) {
                    Text("All").tag("")
                    Text("C").tag("C")
                    Text("C#").tag("C#")
                    Text("D").tag("D")
                    Text("D#").tag("D#")
                    Text("E").tag("E")
                    Text("F").tag("F")
                    Text("F#").tag("F#")
                    Text("G").tag("G")
                    Text("G#").tag("G#")
                    Text("A").tag("A")
                    Text("A#").tag("A#")
                    Text("B").tag("B")
                }
                .pickerStyle(MenuPickerStyle())
                .padding(.horizontal)
            }
            .padding(.top)

            if isLoadingSamples { // Use new loading state
                ProgressView("Loading Samples...")
                    .padding()
            } else if samples.isEmpty {
                Text("No samples available. Try uploading one!")
                    .foregroundColor(.gray)
                    .padding()
            } else {
                List(samples) { sample in
                    SampleRow(sample: sample,
                              playAction: {
                                  Task {
                                      await playSample(contentId: sample.p2pContentId, sampleId: sample.id)
                                  }
                              },
                              purchaseAction: {
                                  Task {
                                      await purchaseSample(sample: sample)
                                  }
                              },
                              isPlaying: self.isPlayingSample == sample.id,
                              isPurchasing: self.isPurchasingSample == sample.id
                    )
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
        isLoadingSamples = true
        defer { isLoadingSamples = false }
        do {
            samples = try await backendAPIClient.fetchSamples(query: searchQuery, category: filterCategory, bpm: filterBPM, key: filterKey)
        } catch {
            errorMessage = error.localizedDescription
            showingErrorAlert = true
        }
    }

    private func purchaseSample(sample: Sample) async {
        self.isPurchasingSample = sample.id
        do {
            print("Purchasing sample: \(sample.title)")
            let txHash = try await blockchainClient.purchaseSample(sampleId: sample.id, price: sample.price, recipientAddress: sample.ownerAddress)
            errorMessage = "Successfully purchased \(sample.title)! Transaction Hash: \(txHash)"
            showingErrorAlert = true
        } catch {
            errorMessage = "Failed to purchase sample: \(error.localizedDescription)"
            showingErrorAlert = true
        }
        self.isPurchasingSample = nil
    }

    private func playSample(contentId: String, sampleId: String) async {
        // Stop any currently playing sample
        stopCurrentPlayback()

        self.currentPlayingSampleId = sampleId

        // In a real application, you would fetch the audio data from IPFS using p2pContentId
        // For demonstration, we'll use a dummy URL.
        guard let url = URL(string: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3") else {
            errorMessage = "Invalid audio URL."
            showingErrorAlert = true
            self.currentPlayingSampleId = nil
            return
        }

        audioPlayer = AVPlayer(url: url)
        audioPlayer?.play()

        // Observe when the audio finishes playing
        NotificationCenter.default.addObserver(forName: .AVPlayerItemDidPlayToEndTime, object: audioPlayer?.currentItem, queue: .main) {
            _ in
            self.currentPlayingSampleId = nil
            self.audioPlayer = nil // Release player resources
        }

        errorMessage = "Playing sample: \(contentId)"
        showingErrorAlert = true
    }

    private func stopCurrentPlayback() {
        audioPlayer?.pause()
        audioPlayer = nil
        currentPlayingSampleId = nil
    }
}

struct SampleRow: View {
    let sample: Sample
    let playAction: () -> Void
    let purchaseAction: () -> Void
    let isPlaying: Bool
    let isPurchasing: Bool

    var body: some View {
        HStack(alignment: .center) {
            // Left section: Info and Waveform Placeholder
            VStack(alignment: .leading, spacing: 5) {
                Text(sample.title)
                    .font(.headline)
                    .lineLimit(1)

                NavigationLink(destination: CreatorProfileView(creatorId: sample.creatorId)) {
                    Text(sample.artist)
                        .font(.subheadline)
                        .foregroundColor(.gray)
                        .lineLimit(1)
                }

                HStack {
                    if let bpm = sample.bpm {
                        Text("BPM: \(bpm)")
                            .font(.caption)
                            .padding(.horizontal, 6)
                            .padding(.vertical, 3)
                            .background(Capsule().fill(Color.blue.opacity(0.2)))
                    }
                    if let key = sample.key {
                        Text("Key: \(key)")
                            .font(.caption)
                            .padding(.horizontal, 6)
                            .padding(.vertical, 3)
                            .background(Capsule().fill(Color.green.opacity(0.2)))
                    }
                    Text(sample.category)
                        .font(.caption)
                        .padding(.horizontal, 6)
                        .padding(.vertical, 3)
                        .background(Capsule().fill(Color.orange.opacity(0.2)))
                }

                // Waveform Placeholder
                RoundedRectangle(cornerRadius: 5)
                    .fill(Color.gray.opacity(0.3))
                    .frame(height: 40)
                    .overlay(
                        Text("Waveform Placeholder")
                            .font(.caption2)
                            .foregroundColor(.white)
                    )
            }

            Spacer()

            // Right section: Actions and Stats
            VStack(alignment: .trailing, spacing: 5) {
                HStack {
                    Button(action: playAction) {
                        if isPlaying {
                            Image(systemName: "stop.circle.fill") // Stop icon when playing
                                .font(.title)
                                .foregroundColor(.red)
                        } else {
                            Image(systemName: "play.circle.fill")
                                .font(.title)
                                .foregroundColor(.blue)
                        }
                    }
                    .disabled(isPurchasing)

                    Button(action: purchaseAction) {
                        if isPurchasing {
                            ProgressView()
                        } else {
                            Image(systemName: "cart.fill")
                                .font(.title)
                                .foregroundColor(.green)
                        }
                    }
                    .disabled(isPlaying || isPurchasing)
                }

                Text("Price: \(sample.price, specifier: "%.4f")")
                    .font(.subheadline)

                Text("Used: \(sample.usageCount) times")
                    .font(.caption)
                    .foregroundColor(.secondary)

                Text("Duration: \(sample.duration)")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
        .padding(.vertical, 10)
        .padding(.horizontal)
        .background(Color.white)
        .cornerRadius(10)
        .shadow(radius: 2)
    }

struct SampleBrowserView_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            SampleBrowserView()
                .environmentObject(RealBlockchainClient())
        }
    }
}