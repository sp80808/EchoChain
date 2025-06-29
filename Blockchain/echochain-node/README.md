# EchoChain Node Development

This directory contains the source code for the EchoChain blockchain node, built using the Substrate framework.

## **IMPORTANT: Rust Toolchain Setup Required**

Before you can build or run this node, you must ensure your Rust development environment is correctly set up. If you encounter `cargo: command not found` or similar errors, please follow these steps:

1.  **Install Rustup (if not already installed):**
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2.  **Configure Rust Toolchain:**
    ```bash
    rustup default stable
    rustup update
    rustup target add wasm32-unknown-unknown --toolchain stable
    ```

3.  **Install Substrate Build Dependencies (for macOS):**
    ```bash
    brew install cmake pkg-config openssl llvm
    ```
    (Ensure Xcode Command Line Tools are installed: `xcode-select --install`)

**After running these commands, please restart your terminal application to ensure the changes to your PATH are loaded.**

Once the Rust toolchain is correctly set up, you can build the node by navigating to this directory and running:

```bash
cargo build --release
```

## Project Structure

*   `node/`: Contains the main node executable.
*   `runtime/`: Defines the blockchain's state transition function and includes various FRAME pallets.
*   `pallets/`: Custom pallets for EchoChain's unique logic (e.g., sample registry, proof-of-contribution).

## Development Plan

1.  **Initial Build & Run:** Verify the basic node template compiles and runs.
2.  **Custom Pallet Development:** Implement `pallet-sample-registry` and `pallet-proof-of-contribution` as outlined in the main project specification.
3.  **Runtime Integration:** Integrate custom pallets into the `runtime/src/lib.rs`.
4.  **Testing:** Develop unit and integration tests for the custom pallets and runtime logic.
5.  **Deployment:** Prepare the node for testnet and mainnet deployment.