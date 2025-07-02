# EchoChain Web Dashboard

This document outlines the architecture, setup, and functionality of the EchoChain Web Dashboard, integrated into the project's landing page. This dashboard provides users with a comprehensive interface to interact with the EchoChain blockchain, manage their assets, and utilize core platform features.

## Table of Contents
1.  [Overview](#1-overview)
2.  [Architecture and Technology Stack](#2-architecture-and-technology-stack)
3.  [Project Structure](#3-project-structure)
4.  [Setup and Installation](#4-setup-and-installation)
5.  [Core Functionalities](#5-core-functionalities)
    *   [Wallet Integration](#wallet-integration)
    *   [Sample Uploading](#sample-uploading)
    *   [Royalty Collection Management](#royalty-collection-management)
    *   [Transaction Execution](#transaction-execution)
6.  [Extensibility and Maintainability](#6-extensibility-and-maintainability)
7.  [Troubleshooting](#7-troubleshooting)

---

## 1. Overview

The EchoChain Web Dashboard is a user-friendly interface designed to bridge users with the EchoChain blockchain. It aims to mirror the intuitive aesthetic of the macOS application, providing a seamless experience for managing digital assets (ECHO tokens), uploading music samples, claiming royalties, and executing blockchain transactions directly from a web browser.

## 2. Architecture and Technology Stack

The dashboard is built upon a modern and scalable web stack:

*   **Frontend Framework:** [**Next.js**](https://nextjs.org/) (React)
    *   Chosen for its server-side rendering (SSR) capabilities, file-system based routing, and robust development experience.
*   **Styling:** [**Tailwind CSS**](https://tailwindcss.com/)
    *   A utility-first CSS framework for rapid UI development and consistent styling, aligning with existing project conventions.
*   **Blockchain Interaction:** [**@polkadot/api**](https://polkadot.js.org/docs/api/) (Polkadot-JS API)
    *   The official JavaScript API for interacting with Substrate-based blockchains, enabling direct RPC communication with the EchoChain node.
*   **Wallet Integration:** [**Polkadot.js Browser Extension**](https://polkadot.js.org/extension/)
    *   The standard and most secure method for web applications to manage user accounts and sign transactions without exposing private keys.
*   **IPFS Interaction:** [**ipfs-http-client**](https://www.npmjs.com/package/ipfs-http-client)
    *   A JavaScript client library for interacting with a local or remote IPFS daemon's HTTP API, used for decentralized file storage.
*   **Backend (As Needed):** Existing `Backend_API_Services` (Node.js/FastAPI)
    *   Can be extended to serve as an API gateway for complex operations, data aggregation, or sensitive logic that should not be handled purely client-side. For most dashboard functionalities, direct blockchain interaction is prioritized for responsiveness.

## 3. Project Structure

The dashboard's code resides within the `LandingPage/project` directory, leveraging Next.js's conventions for routing and component organization.

```
LandingPage/project/
├───package.json
├───next.config.js
├───tailwind.config.js
├───postcss.config.js
├───src/
│   ├───pages/
│   │   ├───_app.js             // Custom App component for global layout/state
│   │   ├───index.js            // Existing Landing Page (can link to dashboard)
│   │   ├───dashboard.js        // Main Dashboard Entry Page
│   │   ├───dashboard/
│   │   │   ├───upload.js       // Sample Upload Page
│   │   │   ├───royalty.js      // Royalty Management Page
│   │   │   ├───wallet.js       // Wallet Overview Page
│   │   │   └───transactions.js // Transaction History/Execution Page
│   │
│   ├───components/
│   │   ├───Layout.js           // Dashboard layout (sidebar, header, navigation)
│   │   ├───WalletConnect.js    // Component for connecting to Polkadot.js extension
│   │   ├───SampleCard.js       // Displays individual sample details (for browsing)
│   │   ├───TransactionTable.js // Displays transaction history
│   │   ├───Input.js            // Reusable form input field
│   │   ├───Button.js           // Reusable button component
│   │   └───... (other shared UI components)
│   │
│   ├───lib/
│   │   ├───polkadotApi.js      // Initializes and manages Polkadot-JS API connection
│   │   ├───ipfsClient.js       // IPFS interaction logic (upload, download)
│   │   ├───utils.js            // General utility functions (e.g., data formatters)
│   │   ├───constants.js        // Frontend-specific constants (e.g., RPC URL)
│   │
│   ├───styles/
│   │   ├───globals.css         // Tailwind CSS imports and custom global styles
│   │
│   └───hooks/
│       ├───usePolkadot.js      // Custom React hook for managing blockchain state and interactions
│       └───useIPFS.js          // Custom React hook for managing IPFS state and interactions
```

## 4. Setup and Installation

To set up and run the EchoChain Web Dashboard:

1.  **Navigate to the project directory:**
    ```bash
    cd /Volumes/Harry/DEV/Echochain/LandingPage/project
    ```

2.  **Install Dependencies:**
    ```bash
    npm install
    # or yarn install
    ```
    This will install Next.js, React, Tailwind CSS, Polkadot-JS API, ipfs-http-client, and other necessary packages.

3.  **Ensure EchoChain Node is Running:**
    The dashboard connects to an EchoChain blockchain node via WebSocket. Make sure your local or remote EchoChain node is running and accessible at the configured RPC URL (default: `ws://127.00.1:9944`).

4.  **Install Polkadot.js Browser Extension:**
    For wallet integration and transaction signing, you need the [Polkadot.js Browser Extension](https://polkadot.js.org/extension/) installed in your browser (Chrome, Firefox, Brave). Create or import an account within the extension.

5.  **Ensure IPFS Daemon is Running:**
    For sample uploading and downloading, the dashboard interacts with an IPFS daemon. Ensure your local IPFS daemon (`go-ipfs`) is running and accessible at its HTTP API endpoint (default: `http://127.0.0.1:5001`).

6.  **Run the Development Server:**
    ```bash
    npm run dev
    # or yarn dev
    ```
    The dashboard will typically be accessible at `http://localhost:3000`.

## 5. Core Functionalities

### Wallet Integration

*   **Purpose:** Connects the user's browser-based Polkadot.js wallet to the dashboard.
*   **Features:**
    *   Detects and prompts for Polkadot.js extension.
    *   Lists available accounts from the extension.
    *   Allows selection of an active account.
    *   Displays the selected account's SS58 address and current balance (fetched from `pallet-balances`).
*   **Components:** `src/components/WalletConnect.js`, `src/lib/polkadotApi.js`, `src/hooks/usePolkadot.js`

### Sample Uploading

*   **Purpose:** Enables users to upload audio files and their associated metadata to IPFS, and register the IPFS CIDs on the EchoChain blockchain.
*   **Features:**
    *   File input for selecting audio files.
    *   Text fields for sample title, artist, etc.
    *   Uploads audio and metadata to a configured IPFS daemon.
    *   Constructs and signs a `pallet-sample-registry::register_sample` extrinsic.
    *   Submits the transaction to the EchoChain.
    *   Provides real-time feedback on upload and transaction status.
*   **Components:** `src/pages/dashboard/upload.js`, `src/lib/ipfsClient.js`

### Royalty Collection Management

*   **Purpose:** Allows users to view their earned content and network rewards and initiate claims for these rewards.
*   **Features:**
    *   Displays unclaimed rewards from `pallet-content-rewards` and `pallet-network-rewards`.
    *   Provides buttons to trigger `claim_rewards` extrinsics for each reward type.
    *   Shows transaction status for reward claims.
*   **Components:** `src/pages/dashboard/royalty.js`

### Transaction Execution

*   **Purpose:** Provides a general interface for viewing recent transactions and executing custom blockchain calls.
*   **Features:**
    *   Lists recent transactions associated with the connected account (conceptual, typically requires a backend indexer).
    *   Allows users to specify a pallet module and method, along with arguments (in JSON format), to construct and send custom extrinsics.
    *   Useful for advanced users or debugging.
*   **Components:** `src/pages/dashboard/transactions.js`

## 6. Extensibility and Maintainability

The dashboard is designed with scalability and maintainability in mind:

*   **Component-Based Architecture:** UI is broken down into reusable React components, promoting modularity.
*   **Clear Separation of Concerns:** Logic for UI, blockchain interaction, and IPFS operations are kept distinct.
*   **Custom Hooks:** `usePolkadot` and `useIPFS` encapsulate complex logic and state, making components cleaner and more readable.
*   **Next.js Features:** Leverages file-system routing, API routes (for future backend needs), and potential SSR/SSG for performance.
*   **Tailwind CSS:** Provides a consistent and easily customizable styling system.
*   **Error Handling:** Includes robust error handling and user-friendly feedback mechanisms.
*   **Future Enhancements:** Easily extendable to include new pallets, features, or integrate with more complex backend services.

## 7. Troubleshooting

*   **"Polkadot.js extension not found"**: Ensure the browser extension is installed and enabled.
*   **"Failed to connect to blockchain"**: Verify that your EchoChain node is running and accessible at the configured RPC URL (`ws://127.0.0.1:9944` by default). Check your network connection.
*   **"Failed to upload file to IPFS"**: Ensure your local IPFS daemon is running and accessible at its HTTP API endpoint (`http://127.0.0.1:5001` by default).
*   **Transaction failures**: Check the browser console for detailed error messages from the Polkadot.js extension or the blockchain node. Ensure you have sufficient funds and the correct permissions for the transaction.
*   **Build errors**: Run `npm install` (or `yarn install`) again to ensure all dependencies are correctly installed. Clear your `node_modules` and `package-lock.json` (or `yarn.lock`) and reinstall if issues persist.
