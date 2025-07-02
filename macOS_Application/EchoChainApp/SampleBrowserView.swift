import SwiftUI
import AVFoundation

struct SampleBrowserView: View {
    @StateObject private var backendAPIClient = RealBackendAPIClient()
    @EnvironmentObject private var blockchainClient: RealBlockchainClient
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
                List(filteredSamples) { sample in
                    SampleRow(sample: sample, playAction: {
                        Task {
                            do {
                                await playSample(contentId: sample.p2pContentId)
                            } catch {
                                errorMessage = error.localizedDescription
                                showingErrorAlert = true
                            }
                        }
                    }, purchaseAction: {
                        Task {
                            do {
                                try await purchaseSample(sample: sample)
                            } catch {
                                errorMessage = error.localizedDescription
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

    private func purchaseSample(sample: Sample) async throws {
        print("Purchasing sample: \(sample.title)")
        let txHash = try await blockchainClient.purchaseSample(sampleId: sample.id, price: sample.price, recipientAddress: sample.ownerAddress)
        errorMessage = "Successfully purchased \(sample.title)! Transaction Hash: \(txHash)"
        showingErrorAlert = true
    }
}

struct SampleRow: View {
    let sample: Sample
    let playAction: () -> Void
    let purchaseAction: () -> Void

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
                        Image(systemName: "play.circle.fill")
                            .font(.title)
                            .foregroundColor(.blue)
                    }
                    Button(action: purchaseAction) {
                        Image(systemName: "cart.fill")
                            .font(.title)
                            .foregroundColor(.green)
                    }
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
}

struct SampleBrowserView_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            SampleBrowserView()
                .environmentObject(RealBlockchainClient())
        }
    }
}