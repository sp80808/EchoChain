import SwiftUI

struct CreatorProfileView: View {
    let creatorId: String
    @State private var creator: User? // Assuming a User struct exists in the frontend
    @State private var samples: [Sample] = []
    @State private var isLoading = true
    @State private var errorMessage: String = ""
    @State private var showingErrorAlert = false

    @StateObject private var backendAPIClient = RealBackendAPIClient()

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
