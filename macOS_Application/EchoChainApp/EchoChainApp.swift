import SwiftUI

@main
struct EchoChainApp: App {
    @StateObject private var authService = RealAuthService()

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(authService)
        }
    }
}