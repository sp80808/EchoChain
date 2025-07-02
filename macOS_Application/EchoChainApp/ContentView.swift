import SwiftUI

struct ContentView: View {
    @StateObject private var p2pClient = RealP2PClient()
    @EnvironmentObject var authService: RealAuthService // Inject AuthService

    var body: some View {
        NavigationView {
            VStack {
                Text("Welcome to EchoChain!")
                    .font(.largeTitle)
                    .padding()

                Spacer()

                HStack {
                    Text("P2P Status:")
                        .font(.headline)
                    Text(p2pClient.isConnected ? "Connected" : "Disconnected")
                        .foregroundColor(p2pClient.isConnected ? .green : .red)
                }
                .padding(.bottom, 10)

                NavigationLink(destination: WalletView().environmentObject(authService)) { // Pass authService
                    Text("Go to Wallet")
                        .font(.title2)
                        .padding()
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }
                .padding(.bottom, 20)

                NavigationLink(destination: SampleBrowserView().environmentObject(authService)) { // Pass authService
                    Text("Browse Samples")
                        .font(.title2)
                        .padding()
                        .background(Color.green)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }
                .padding(.bottom, 20)
            }
            .navigationTitle("EchoChain")
            .onAppear {
                Task {
                    do {
                        try await p2pClient.connect()
                    } catch {
                        print("Failed to connect to P2P client: \(error.localizedDescription)")
                    }
                }
            }
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}