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

    var filteredSamples: [Sample] {
        samples.filter { sample in
            (searchQuery.isEmpty || sample.title.localizedCaseInsensitiveContains(searchQuery)) &&
            (filterCategory.isEmpty || sample.category == filterCategory)
        }
    }

    private func fetchSamples() async {
        isLoadingSamples = true
        defer { isLoadingSamples = false }
        do {
            samples = try await backendAPIClient.fetchSamples(query: searchQuery, category: filterCategory)
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
            Text("Price: \(sample.price, specifier: "%.4f")")
                .font(.subheadline)
            Text("ID: \(sample.p2pContentId)")
                .font(.caption)
                .lineLimit(1)
                .truncationMode(.middle)
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
        .padding(.vertical, 5)
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