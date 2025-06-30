import SwiftUI

struct WalletView: View {
    @StateObject private var blockchainClient = RealBlockchainClient()
    @StateObject private var secureStorage = SecureStorage() // TODO: SecureStorage needs to be fully implemented for secure key management.
    @State private var walletAddress: String = "Loading..."
    @State private var showingImportAlert = false
    @State private var importPrivateKeyInput: String = ""
    @State private var showingErrorAlert = false
    @State private var errorMessage: String = ""
    @State private var showingSendView = false
    @State private var sendAmount: String = ""
    @State private var sendAddress: String = ""

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
                        // TODO: Implement actual wallet creation on the blockchain and secure storage.
                        try await blockchainClient.createWallet()
                        walletAddress = blockchainClient.walletAddress
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

            /*
            // TODO: Re-enable and implement robust wallet import functionality, considering security best practices.
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
                TextField("Private Key", text: $importPrivateKeyInput)
                Button("Import") {
                    Task {
                        do {
                            try await blockchainClient.importWallet(privateKey: importPrivateKeyInput)
                            walletAddress = blockchainClient.walletAddress
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
                Text("Please enter your private key to import your wallet.")
            }
            */

            Button(action: {
                Task {
                    do {
                        // TODO: Ensure this fetches the latest balance from the blockchain.
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
                                    
                                    // TODO: Implement actual transaction sending via blockchainClient.
                                    // This will involve signing the transaction with the secureStorage and broadcasting it.
                                    try await blockchainClient.sendTransaction(
                                        to: sendAddress,
                                        amount: amount,
                                        signer: secureStorage // TODO: Ensure secureStorage provides a proper signer interface.
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

            Divider()

            Text("Transaction History")
                .font(.title)
                .padding(.top, 20)

            // TODO: Ensure transaction history is fetched and displayed accurately from the blockchain.
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
            Task {
                // TODO: Handle initial wallet loading (create new or import) more gracefully.
                walletAddress = blockchainClient.walletAddress
                do {
                    try await blockchainClient.fetchBalance()
                    try await blockchainClient.fetchTransactionHistory()
                } catch {
                    errorMessage = error.localizedDescription
                    showingErrorAlert = true
                }
            }
        }
        .alert("Error", isPresented: $showingErrorAlert) {
            Button("OK") { }
        } message: {
            Text(errorMessage)
        }
    }

    private var itemFormatter: DateFormatter {
        let formatter = DateFormatter()
        formatter.dateStyle = .short
        formatter.timeStyle = .medium
        return formatter
    }
}

struct WalletView_Previews: PreviewProvider {
    static var previews: some View {
        WalletView()
    }
}
