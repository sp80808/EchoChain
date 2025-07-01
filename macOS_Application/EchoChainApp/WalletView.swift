import SwiftUI
import SubstrateKeychain
import LocalAuthentication

struct WalletView: View {
    @StateObject private var blockchainClient = RealBlockchainClient()
    @State private var showingImportAlert = false
    @State private var biometricsEnabled = false
    @State private var isAuthenticating = false
    @State private var importMnemonicInput: String = "" // Changed to mnemonic input
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
                Text("\(blockchainClient.balance, specifier: "%.4f") ECHO")
                    .font(.title2)
                    .foregroundColor(.green)
            }
            .padding(.horizontal)

            HStack {
                Text("Address:")
                    .font(.title2)
                Spacer()
                Text(blockchainClient.walletAddress) // Directly use blockchainClient.walletAddress
                    .font(.title3)
                    .lineLimit(1)
                    .truncationMode(.middle)
                    .foregroundColor(.gray)
                Button(action: {
                    NSPasteboard.general.clearContents()
                    NSPasteboard.general.setString(blockchainClient.walletAddress, forType: .string)
                    errorMessage = "Address copied to clipboard!"
                    showingErrorAlert = true
                }) {
                    Image(systemName: "doc.on.doc")
                }
                .buttonStyle(PlainButtonStyle())
            }
            .padding(.horizontal)

            HStack {
                Text("Node Connection:")
                    .font(.title2)
                Spacer()
                Text(blockchainClient.isConnected ? "Connected" : "Disconnected")
                    .font(.title3)
                    .foregroundColor(blockchainClient.isConnected ? .green : .red)
            }
            .padding(.horizontal)

            Toggle(isOn: $biometricsEnabled) {
                Text("Enable Biometric Authentication")
                    .font(.title2)
            }
            .padding(.horizontal)
            .onChange(of: biometricsEnabled) { newValue in
                Task {
                    if newValue {
                        // Attempt to enable biometrics
                        do {
                            // Re-save the existing mnemonic with biometrics enabled
                            if let mnemonic = try await SecureStorage().getMnemonic(requireBiometrics: false) {
                                try await SecureStorage().saveMnemonic(mnemonic, requireBiometrics: true)
                                errorMessage = "Biometric authentication enabled."
                            } else {
                                errorMessage = "No wallet found to enable biometrics for."
                                biometricsEnabled = false // Revert toggle if no wallet
                            }
                        } catch {
                            errorMessage = "Failed to enable biometrics: \(error.localizedDescription)"
                            biometricsEnabled = false // Revert toggle on failure
                        }
                    } else {
                        // Attempt to disable biometrics
                        do {
                            // Re-save the existing mnemonic without biometrics
                            if let mnemonic = try await SecureStorage().getMnemonic(requireBiometrics: true) {
                                try await SecureStorage().saveMnemonic(mnemonic, requireBiometrics: false)
                                errorMessage = "Biometric authentication disabled."
                            } else {
                                errorMessage = "No wallet found to disable biometrics for."
                                biometricsEnabled = true // Revert toggle if no wallet
                            }
                        } catch {
                            errorMessage = "Failed to disable biometrics: \(error.localizedDescription)"
                            biometricsEnabled = true // Revert toggle on failure
                        }
                    }
                    showingErrorAlert = true
                }
            }

            Divider()

            Text("Wallet Actions")
                .font(.title)
                .padding(.top, 20)

            Button(action: {
                Task {
                    isBlockchainActionLoading = true
                    defer { isBlockchainActionLoading = false }
                    do {
                        try await blockchainClient.createWallet(requireBiometrics: biometricsEnabled)
                        errorMessage = "New wallet created successfully!"
                        showingErrorAlert = true
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
                TextField("Mnemonic Phrase (12 or 24 words)", text: $importMnemonicInput)
                Button("Import") {
                    Task {
                        isBlockchainActionLoading = true
                        defer { isBlockchainActionLoading = false }
                        do {
                            guard !importMnemonicInput.isEmpty else {
                                errorMessage = "Mnemonic phrase cannot be empty."
                                showingErrorAlert = true
                                return
                            }
                            try await blockchainClient.importWallet(mnemonic: importMnemonicInput, requireBiometrics: biometricsEnabled)
                            importMnemonicInput = "" // Clear input
                            errorMessage = "Wallet imported successfully!"
                            showingErrorAlert = true
                        } catch {
                            errorMessage = error.localizedDescription
                            showingErrorAlert = true
                        }
                    }
                }
                Button("Cancel", role: .cancel) {
                    importMnemonicInput = ""
                }
            } message: {
                Text("WARNING: Importing a mnemonic phrase directly is risky. Ensure you understand the security implications. Please enter your 12 or 24 word mnemonic phrase.")
            }

            Button(action: {
                Task {
                    isBlockchainActionLoading = true
                    defer { isBlockchainActionLoading = false }
                    do {
                        try await blockchainClient.fetchBalance(requireBiometrics: biometricsEnabled)
                        errorMessage = "Balance refreshed."
                        showingErrorAlert = true
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

                    TextField("Recipient Address (SS58)", text: $sendAddress)
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
                                isBlockchainActionLoading = true
                                defer { isBlockchainActionLoading = false }
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

                                    let txHash = try await blockchainClient.sendTransaction(
                                        to: sendAddress,
                                        amount: amount,
                                        requireBiometrics: biometricsEnabled
                                    )
                                    showingSendView = false
                                    sendAmount = ""
                                    sendAddress = ""
                                    errorMessage = "Transaction sent! Tx Hash: \(txHash)"
                                    showingErrorAlert = true
                                    try await blockchainClient.fetchBalance(requireBiometrics: biometricsEnabled)
                                    try await blockchainClient.fetchTransactionHistory(requireBiometrics: biometricsEnabled)
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
                        let result = try await blockchainClient.claimRewards(requireBiometrics: biometricsEnabled)
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
                        let uploaded = p2pClient.totalBytesUploaded
                        let downloaded = p2pClient.totalBytesDownloaded
                        let result = try await blockchainClient.submitNetworkContribution(uploaded: uploaded, downloaded: downloaded, requireBiometrics: biometricsEnabled)
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
            
            Button(action: {
                Task {
                    isBlockchainActionLoading = true
                    defer { isBlockchainActionLoading = false }
                    do {
                        try blockchainClient.deleteWallet()
                        errorMessage = "Wallet deleted successfully!"
                        showingErrorAlert = true
                    } catch {
                        errorMessage = error.localizedDescription
                        showingErrorAlert = true
                    }
                }
            }) {
                Label("Delete Wallet", systemImage: "trash.fill")
                    .font(.title3)
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(Color.red)
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
            // Initial fetch when view appears
            Task {
                do {
                    // Check if biometrics are enabled for the stored mnemonic
                    if let mnemonic = try await SecureStorage().getMnemonic(requireBiometrics: true) {
                        biometricsEnabled = true
                    } else {
                        biometricsEnabled = false
                    }
                    try await blockchainClient.fetchBalance(requireBiometrics: biometricsEnabled)
                    try await blockchainClient.fetchTransactionHistory(requireBiometrics: biometricsEnabled)
                } catch {
                    errorMessage = error.localizedDescription
                    showingErrorAlert = true
                }
            }
        }
        .onChange(of: blockchainClient.walletAddress) { _ in
            // When wallet address changes (e.g., after create/import), refresh balance and history
            Task {
                do {
                    try await blockchainClient.fetchBalance(requireBiometrics: biometricsEnabled)
                    try await blockchainClient.fetchTransactionHistory(requireBiometrics: biometricsEnabled)
                } catch {
                    errorMessage = error.localizedDescription
                    showingErrorAlert = true
                }
            }
        }
        .alert("Blockchain Action Status", isPresented: $showingErrorAlert) {
            Button("OK") { }
        } message: {
            Text(errorMessage)
        }
    }
}

private let itemFormatter: DateFormatter = {
    let formatter = DateFormatter()
    formatter.dateStyle = .short
    formatter.timeStyle = .short
    return formatter
}()
