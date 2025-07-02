import SwiftUI

struct CreatorProfileView: View {
    let creatorId: String
    @State private var creator: User? // Assuming a User struct exists in the frontend
    @State private var samples: [Sample] = []
    @State private var isLoading = true
    @State private var errorMessage: String = ""
    @State private var showingErrorAlert = false
    @State private var isPlayingSample: String? = nil
    @State private var isPurchasingSample: String? = nil

    @StateObject private var backendAPIClient = RealBackendAPIClient(authService: RealAuthService()) // Pass authService
    @EnvironmentObject var authService: RealAuthService // Inject AuthService
    @EnvironmentObject private var blockchainClient: RealBlockchainClient // Inject BlockchainClient for play/purchase actions

    var body: some View {
        VStack {
            if isLoading {
                ProgressView("Loading Creator Profile...")
            } else if showingErrorAlert {
                Text("Error: \(errorMessage)")
                    .foregroundColor(.red)
            } else if let creator = creator {
                Text(creator.email) // Display creator's email or name
                    .font(.largeTitle)
                    .bold()
                    .padding()

                Text("Wallet Address: \(creator.walletAddress)")
                    .font(.subheadline)
                    .foregroundColor(.gray)
                    .padding(.bottom)

                Divider()

                Text("Uploaded Samples")
                    .font(.title2)
                    .padding(.vertical)

                if samples.isEmpty {
                    Text("No samples uploaded by this creator.")
                        .foregroundColor(.gray)
                } else {
                    List(samples) { sample in
                        // Re-use SampleRow or create a simplified version
                        SampleRow(sample: sample, playAction: { /* Implement play action */ }, purchaseAction: { /* Implement purchase action */ })
                    }
                }
            } else {
                Text("Creator not found.")
                    .foregroundColor(.gray)
            }
        }
        .navigationTitle("Creator Profile")
        .onAppear(perform: fetchCreatorProfile)
        .alert("Error", isPresented: $showingErrorAlert) {
            Button("OK") { }
        } message: {
            Text(errorMessage)
        }
    }

    private func playSample(contentId: String, sampleId: String) async {
        self.isPlayingSample = sampleId
        do {
            // Simulate audio playback
            try await Task.sleep(nanoseconds: 2_000_000_000) // Simulate 2 seconds of playback
            errorMessage = "Playing sample: \(contentId)"
            showingErrorAlert = true
        } catch {
            errorMessage = "Failed to play sample: \(error.localizedDescription)"
            showingErrorAlert = true
        }
        self.isPlayingSample = nil
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

    private func fetchCreatorProfile() {
        isLoading = true
        Task {
            do {
                // Assuming BackendAPIClient has a method to fetch user by ID
                let (fetchedUser, fetchedSamples) = try await backendAPIClient.fetchUserAndSamples(userId: creatorId)
                self.creator = fetchedUser
                self.samples = fetchedSamples
            } catch {
                errorMessage = error.localizedDescription
                showingErrorAlert = true
            }
            isLoading = false
        }
    }
}
