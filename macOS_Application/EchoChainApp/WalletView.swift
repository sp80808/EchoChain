import SwiftUI
import SubstrateKeychain
import LocalAuthentication

struct WalletView: View {
    @EnvironmentObject private var authService: RealAuthService // Inject AuthService
    @StateObject private var blockchainClient = RealBlockchainClient(authService: RealAuthService()) // Pass AuthService
    @State private var showingImportAlert = false
    @State private var biometricsEnabled = false
    @State private var isAuthenticating = false
    @State private var importMnemonicInput: String = "" // Changed to mnemonic input
    @State private var showingErrorAlert = false
    @State private var errorMessage: String = ""
    @State private var showingSendView = false
    @State private var sendAmount: String = ""
    @State private var sendAddress: String = ""
    @State private var emailInput: String = ""
    @State private var passwordInput: String = ""
    @State private var referrerCodeInput: String = "" // New state for referrer code input
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
                // Existing biometrics logic
            }

            Divider()

            // Referral System Section
            Text("Referral Program")
                .font(.title)
                .padding(.top, 20)

            HStack {
                Text("Your Referral Code:")
                    .font(.title3)
                Spacer()
                Text(blockchainClient.referralCode) // Assuming blockchainClient will provide this
                    .font(.title3)
                    .foregroundColor(.blue)
                Button(action: {
                    NSPasteboard.general.clearContents()
                    NSPasteboard.general.setString(blockchainClient.referralCode, forType: .string)
                    errorMessage = "Referral code copied to clipboard!"
                    showingErrorAlert = true
                }) {
                    Image(systemName: "doc.on.doc")
                }
                .buttonStyle(PlainButtonStyle())
            }
            .padding(.horizontal)

            HStack {
                Text("Referred Users:")
                    .font(.title3)
                Spacer()
                Text("\(blockchainClient.referredUsersCount)") // Assuming blockchainClient will provide this
                    .font(.title3)
                    .foregroundColor(.green)
            }
            .padding(.horizontal)

            // Faucet Rewards Section
            Text("Faucet Rewards")
                .font(.title)
                .padding(.top, 20)

            HStack {
                Text("Available Faucet:")
                    .font(.title3)
                Spacer()
                Text("\(blockchainClient.faucetAmount, specifier: "%.4f") ECHO") // Assuming blockchainClient will provide this
                    .font(.title3)
                    .foregroundColor(.green)
            }
            .padding(.horizontal)

            Button(action: {
                Task {
                    isBlockchainActionLoading = true
                    defer { isBlockchainActionLoading = false }
                    do {
                        let result = try await blockchainClient.claimRewards(requireBiometrics: biometricsEnabled)
                        errorMessage = "Rewards claimed! Tx Hash: \(result)"
                        showingErrorAlert = true
                        // Refresh balance and faucet amount after claiming
                        try await blockchainClient.fetchBalance(requireBiometrics: biometricsEnabled)
                        try await blockchainClient.fetchFaucetAmount(requireBiometrics: biometricsEnabled)
                    } catch {
                        errorMessage = error.localizedDescription
                        showingErrorAlert = true
                    }
                }
            }) {
                Label("Claim Faucet Rewards", systemImage: "giftcard.fill")
                    .font(.title3)
                    .padding()
                    .frame(maxWidth: .infinity)
                    .background(Color.green)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
            .padding(.horizontal)

            Divider()

            Text("Wallet Actions")
                .font(.title)
                .padding(.top, 20)

            if authService.isAuthenticated {
                // Display wallet details if authenticated
                Group {
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
                            await handleBiometricsToggle(newValue: newValue)
                        }
                    }

                    Divider()

                    // Referral System Section
                    Text("Referral Program")
                        .font(.title)
                        .padding(.top, 20)

                    HStack {
                        Text("Your Referral Code:")
                            .font(.title3)
                        Spacer()
                        Text(blockchainClient.referralCode) // Assuming blockchainClient will provide this
                            .font(.title3)
                            .foregroundColor(.blue)
                        Button(action: {
                            NSPasteboard.general.clearContents()
                            NSPasteboard.general.setString(blockchainClient.referralCode, forType: .string)
                            errorMessage = "Referral code copied to clipboard!"
                            showingErrorAlert = true
                        }) {
                            Image(systemName: "doc.on.doc")
                        }
                        .buttonStyle(PlainButtonStyle())
                    }
                    .padding(.horizontal)

                    HStack {
                        Text("Referred Users:")
                            .font(.title3)
                        Spacer()
                        Text("\(blockchainClient.referredUsersCount)") // Assuming blockchainClient will provide this
                            .font(.title3)
                            .foregroundColor(.green)
                    }
                    .padding(.horizontal)

                    // Faucet Rewards Section
                    Text("Faucet Rewards")
                        .font(.title)
                        .padding(.top, 20)

                    HStack {
                        Text("Available Faucet:")
                            .font(.title3)
                        Spacer()
                        Text("\(blockchainClient.faucetAmount, specifier: "%.4f") ECHO") // Assuming blockchainClient will provide this
                            .font(.title3)
                            .foregroundColor(.green)
                    }
                    .padding(.horizontal)

                    Button(action: {
                        Task {
                            isBlockchainActionLoading = true
                            defer { isBlockchainActionLoading = false }
                            do {
                                let result = try await blockchainClient.claimRewards(requireBiometrics: biometricsEnabled)
                                errorMessage = "Rewards claimed! Tx Hash: \(result)"
                                showingErrorAlert = true
                                // Refresh balance and faucet amount after claiming
                                try await blockchainClient.fetchBalance(requireBiometrics: biometricsEnabled)
                                try await blockchainClient.fetchFaucetAmount(requireBiometrics: biometricsEnabled)
                            } catch {
                                errorMessage = error.localizedDescription
                                showingErrorAlert = true
                            }
                        }
                    }) {
                        if isBlockchainActionLoading {
                            ProgressView()
                        } else {
                            Label("Claim Faucet Rewards", systemImage: "giftcard.fill")
                                .font(.title3)
                                .padding()
                                .frame(maxWidth: .infinity)
                                .background(Color.green)
                                .foregroundColor(.white)
                                .cornerRadius(10)
                        }
                    }
                    .disabled(isBlockchainActionLoading)

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
                        if isBlockchainActionLoading {
                            ProgressView()
                        } else {
                            Label("Create New Wallet", systemImage: "plus.circle.fill")
                                .font(.title3)
                                .padding()
                                .frame(maxWidth: .infinity)
                                .background(Color.accentColor)
                                .foregroundColor(.white)
                                .cornerRadius(10)
                        }
                    }
                    .disabled(isBlockchainActionLoading)

                    TextField("Referrer Code (Optional)", text: $referrerCodeInput)
                        .textFieldStyle(RoundedBorderTextFieldStyle())
                        .padding(.horizontal)
                        .padding(.bottom, 5)

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
                        TextField("Referrer Code (Optional)", text: $referrerCodeInput) // Add referrer code to import alert
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
                                    try await blockchainClient.importWallet(mnemonic: importMnemonicInput, requireBiometrics: biometricsEnabled, referrerCode: referrerCodeInput)
                                    importMnemonicInput = "" // Clear input
                                    referrerCodeInput = "" // Clear input after use
                                    errorMessage = "Wallet imported successfully!"
                                    showingErrorAlert = true
                                } catch {
                                    errorMessage = error.localizedDescription
                                    showingErrorAlert = true
                                }
                            }
                        }
                        .disabled(isBlockchainActionLoading)
                        Button("Cancel", role: .cancel) {
                            importMnemonicInput = ""
                            referrerCodeInput = ""
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
                        if isBlockchainActionLoading {
                            ProgressView()
                        } else {
                            Label("Refresh Balance", systemImage: "arrow.clockwise.circle.fill")
                                .font(.title3)
                                .padding()
                                .frame(maxWidth: .infinity)
                                .background(Color.orange)
                                .foregroundColor(.white)
                                .cornerRadius(10)
                        }
                    }
                    .disabled(isBlockchainActionLoading)

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
                                .disabled(isBlockchainActionLoading)
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
                                // Refresh balance and faucet amount after claiming
                                try await blockchainClient.fetchBalance(requireBiometrics: biometricsEnabled)
                                try await blockchainClient.fetchFaucetAmount(requireBiometrics: biometricsEnabled)
                            } catch {
                                errorMessage = error.localizedDescription
                                showingErrorAlert = true
                            }
                        }
                    }) {
                        if isBlockchainActionLoading {
                            ProgressView()
                        } else {
                            Label("Claim Faucet Rewards", systemImage: "giftcard.fill")
                                .font(.title3)
                                .padding()
                                .frame(maxWidth: .infinity)
                                .background(Color.green)
                                .foregroundColor(.white)
                                .cornerRadius(10)
                        }
                    }
                    .disabled(isBlockchainActionLoading)

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
                        if isBlockchainActionLoading {
                            ProgressView()
                        } else {
                            Label("Submit Network Contribution", systemImage: "arrow.up.arrow.down")
                                .font(.title3)
                                .padding()
                                .frame(maxWidth: .infinity)
                                .background(Color.purple)
                                .foregroundColor(.white)
                                .cornerRadius(10)
                        }
                    }
                    .disabled(isBlockchainActionLoading)

                    Button(action: {
                Task {
                    isBlockchainActionLoading = true
                    defer { isBlockchainActionLoading = false }
                    do {
                        authService.logout()
                        errorMessage = "Logged out successfully!"
                        showingErrorAlert = true
                    } catch {
                        errorMessage = error.localizedDescription
                        showingErrorAlert = true
                    }
                }
            }) {
                if isBlockchainActionLoading {
                    ProgressView()
                } else {
                    Label("Logout", systemImage: "arrow.right.square.fill")
                        .font(.title3)
                        .padding()
                        .frame(maxWidth: .infinity)
                        .background(Color.red)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }
            }
            .disabled(isBlockchainActionLoading)

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
            } else {
                // Display login/registration form if not authenticated
                VStack(spacing: 20) {
                    Text("Welcome to EchoChain")
                        .font(.largeTitle)
                        .bold()

                    TextField("Email", text: $emailInput)
                        .textFieldStyle(RoundedBorderTextFieldStyle())
                        .keyboardType(.emailAddress)
                        .autocapitalization(.none)

                    SecureField("Password", text: $passwordInput)
                        .textFieldStyle(RoundedBorderTextFieldStyle())

                    TextField("Referrer Code (Optional)", text: $referrerCodeInput)
                        .textFieldStyle(RoundedBorderTextFieldStyle())
                        .autocapitalization(.none)

                    Button(action: {
                        Task {
                            isAuthenticating = true
                            defer { isAuthenticating = false }
                            do {
                                try await authService.login(email: emailInput, password: passwordInput)
                                errorMessage = "Logged in successfully!"
                                showingErrorAlert = true
                            } catch {
                                errorMessage = error.localizedDescription
                                showingErrorAlert = true
                            }
                        }
                    }) {
                        if isAuthenticating {
                            ProgressView()
                        } else {
                            Text("Login")
                                .font(.title2)
                                .padding()
                                .frame(maxWidth: .infinity)
                                .background(Color.blue)
                                .foregroundColor(.white)
                                .cornerRadius(10)
                        }
                    }
                    .disabled(isAuthenticating)

                    Button(action: {
                        Task {
                            isAuthenticating = true
                            defer { isAuthenticating = false }
                            do {
                                try await authService.register(email: emailInput, password: passwordInput, referrerCode: referrerCodeInput)
                                errorMessage = "Registered successfully!"
                                showingErrorAlert = true
                            } catch {
                                errorMessage = error.localizedDescription
                                showingErrorAlert = true
                            }
                        }
                    }) {
                        if isAuthenticating {
                            ProgressView()
                        } else {
                            Text("Register")
                                .font(.title2)
                                .padding()
                                .frame(maxWidth: .infinity)
                                .background(Color.purple)
                                .foregroundColor(.white)
                                .cornerRadius(10)
                        }
                    }
                    .disabled(isAuthenticating)
                }
                .padding()
            }

private let itemFormatter: DateFormatter = {
    let formatter = DateFormatter()
    formatter.dateStyle = .short
    formatter.timeStyle = .short
    return formatter
}()
