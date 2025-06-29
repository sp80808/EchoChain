import SwiftUI

struct WalletView: View {
    @StateObject private var blockchainClient = RealBlockchainClient()
    @StateObject private var secureStorage = SecureStorage()
    @State private var walletAddress: String = "Loading..."
    @State private var showingImportAlert = false
    @State private var importPrivateKeyInput: String = ""
    @State private var showingErrorAlert = false
    @State private var errorMessage: String = ""
    @State private var showingSendView = false
    @State private var sendAmount: String = ""
    @State private var sendAddress: String = ""
    @StateObject private var p2pClient = RealP2PClient()
    @State private var isBlockchainActionLoading = false

    var body: some View {
        VStack(alignment: .leading, spacing: 20) {
            Text("Your Wallet")
                .font(.largeTitle)
                .bold()
                .padding(.bottom, 10)

            HStack {
                Text("Balance:")
                    .font(.title2)
                Spacer()
                // TODO: Ensure balance is fetched and displayed in real-time from the blockchain.
                Text("\(blockchainClient.balance, specifier: "%.4f") ECHO")
                    .font(.title2)
                    .foregroundColor(.green)
            }
            .padding(.horizontal)

            HStack {
                Text("Address:")
                    .font(.title2)
                Spacer()
                // TODO: Display the actual wallet address derived from the blockchain client.
                Text(walletAddress)
                    .font(.title3)
                    .lineLimit(1)
                    .truncationMode(.middle)
                    .foregroundColor(.gray)
            }
            .padding(.horizontal)

            Divider()

            Text("Wallet Actions")
                .font(.title)
                .padding(.top, 20)

            Button(action: {
                Task {
                    do {
                        try await blockchainClient.createWallet()
                    } catch {
                        errorMessage = error.localizedDescription
                        showingErrorAlert = true
                    }
                }
            }) {
                Label("Create New Wallet", systemImage: "plus.circle.fill")
                    .font(.title3)
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(Color.accentColor)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }

            Button(action: {
                showingImportAlert = true
            }) {
                Label("Import Existing Wallet", systemImage: "square.and.arrow.down.fill")
                    .font(.title3)
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(Color.gray)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .alert("Import Wallet", isPresented: $showingImportAlert) {
                TextField("Private Key (Hex or Base64)", text: $importPrivateKeyInput)
                Button("Import") {
                    Task {
                        do {
                            guard let privateKeyData = Data(hex: importPrivateKeyInput) ?? Data(base64Encoded: importPrivateKeyInput) else {
                                errorMessage = "Invalid private key format. Please use Hex or Base64."
                                showingErrorAlert = true
                                return
                            }
                            try await blockchainClient.importWallet(privateKeyData: privateKeyData)
                            importPrivateKeyInput = "" // Clear input
                        } catch {
                            errorMessage = error.localizedDescription
                            showingErrorAlert = true
                        }
                    }
                }
                Button("Cancel", role: .cancel) {
                    importPrivateKeyInput = ""
                }
            } message: {
                Text("WARNING: Importing a private key directly is risky. Ensure you understand the security implications. Please enter your private key in Hex or Base64 format.")
            }

            Button(action: {
                Task {
                    do {
                        try await blockchainClient.fetchBalance()
                    } catch {
                        errorMessage = error.localizedDescription
                        showingErrorAlert = true
                    }
                }
            }) {
                Label("Refresh Balance", systemImage: "arrow.clockwise.circle.fill")
                    .font(.title3)
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(Color.orange)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }

            Button(action: {
                showingSendView = true
            }) {
                Label("Send ECHO", systemImage: "paperplane.fill")
                    .font(.title3)
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(Color.blue)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .sheet(isPresented: $showingSendView) {
                VStack(spacing: 20) {
                    Text("Send ECHO Tokens")
                        .font(.title)
                        .padding(.top)
                    
                    TextField("Recipient Address", text: $sendAddress)
                        .textFieldStyle(RoundedBorderTextFieldStyle())
                        .padding(.horizontal)
                    
                    TextField("Amount", text: $sendAmount)
                        .keyboardType(.decimalPad)
                        .textFieldStyle(RoundedBorderTextFieldStyle())
                        .padding(.horizontal)
                    
                    HStack(spacing: 20) {
                        Button("Cancel") {
                            showingSendView = false
                            sendAmount = ""
                            sendAddress = ""
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.gray)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                        
                        Button("Send") {
                            Task {
                                do {
                                    guard let amount = Double(sendAmount), amount > 0 else {
                                        errorMessage = "Please enter a valid amount"
                                        showingErrorAlert = true
                                        return
                                    }
                                    
                                    guard !sendAddress.isEmpty else {
                                        errorMessage = "Please enter a recipient address"
                                        showingErrorAlert = true
                                        return
                                    }
                                    
                                    try await blockchainClient.sendTransaction(
                                        to: sendAddress,
                                        amount: amount
                                    )
                                    showingSendView = false
                                    sendAmount = ""
                                    sendAddress = ""
                                    try await blockchainClient.fetchBalance()
                                    try await blockchainClient.fetchTransactionHistory()
                                } catch {
                                    errorMessage = error.localizedDescription
                                    showingErrorAlert = true
                                }
                            }
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                    }
                    .padding(.horizontal)
                    
                    Spacer()
                }
                .padding()
            }

            Button(action: {
                Task {
                    isBlockchainActionLoading = true
                    defer { isBlockchainActionLoading = false }
                    do {
                        let result = try await blockchainClient.claimRewards()
                        errorMessage = "Rewards claimed! Tx Hash: \(result)"
                        showingErrorAlert = true
                    } catch {
                        errorMessage = error.localizedDescription
                        showingErrorAlert = true
                    }
                }
            }) {
                Label("Claim Rewards", systemImage: "gift.fill")
                    .font(.title3)
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(Color.green)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }

            Button(action: {
                Task {
                    isBlockchainActionLoading = true
                    defer { isBlockchainActionLoading = false }
                    do {
                        let uploaded = UInt64(p2pClient.uploadedFiles.count)
                        let downloaded = UInt64(p2pClient.downloadedFiles.count)
                        let result = try await blockchainClient.submitNetworkContribution(uploaded: uploaded, downloaded: downloaded)
                        errorMessage = "Network contribution submitted! Tx Hash: \(result)"
                        showingErrorAlert = true
                    } catch {
                        errorMessage = error.localizedDescription
                        showingErrorAlert = true
                    }
                }
            }) {
                Label("Submit Network Contribution", systemImage: "arrow.up.arrow.down")
                    .font(.title3)
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(Color.purple)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }

            if isBlockchainActionLoading {
                ProgressView("Processing blockchain action...")
                    .padding()
            }

            Divider()

            Text("Transaction History")
                .font(.title)
                .padding(.top, 20)

            List(blockchainClient.transactionHistory) { transaction in
                VStack(alignment: .leading) {
                    Text("Type: \(transaction.type.rawValue.capitalized)")
                        .font(.headline)
                    Text("Amount: \(transaction.amount, specifier: "%.4f") ECHO")
                        .font(.subheadline)
                    Text("From: \(transaction.from)")
                        .font(.caption)
                    Text("To: \(transaction.to)")
                        .font(.caption)
                    Text("Date: \(transaction.date, formatter: itemFormatter)")
                        .font(.caption)
                }
                .padding(.vertical, 2)
            }
            .listStyle(PlainListStyle())

            Spacer()
        }
        .padding()
        .navigationTitle("Wallet")
        .onAppear {
            walletAddress = blockchainClient.walletAddress
            Task {
                do {
                    try await blockchainClient.fetchBalance()
                    try await blockchainClient.fetchTransactionHistory()
                } catch {
                    errorMessage = error.localizedDescription
                    showingErrorAlert = true
                }
            }
        }
        .onChange(of: blockchainClient.walletAddress) { newAddress in
            walletAddress = newAddress
            // When wallet address changes (e.g., after create/import), refresh balance and history
            Task {
                do {
                    try await blockchainClient.fetchBalance()
                    try await blockchainClient.fetchTransactionHistory()
                } catch {
                    errorMessage = error.localizedDescription
                    showingErrorAlert = true
                }
            }
        }
    }
}
