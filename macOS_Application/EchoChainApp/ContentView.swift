import SwiftUI

struct ContentView: View {
    var body: some View {
        NavigationView {
            VStack {
                Text("Welcome to EchoChain!")
                    .font(.largeTitle)
                    .padding()

                Spacer()

                NavigationLink(destination: WalletView()) {
                    Text("Go to Wallet")
                        .font(.title2)
                        .padding()
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                }
                .padding(.bottom, 20)

                NavigationLink(destination: SampleBrowserView()) {
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
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}