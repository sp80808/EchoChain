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

## SubQuery Indexer Deployment & Custom Mapping for EchoChain

### 1. Bootstrap a SubQuery Project

```sh
npm install -g @subql/cli
subql init echochain-indexer
cd echochain-indexer
```

### 2. Configure `project.yaml`
- Set `network.endpoint` to your EchoChain node (e.g., `ws://localhost:9944`).
- Add the `sample-registry` pallet to the `dataSources` section.

### 3. Define the SampleRegistered Event Mapping
In `src/mappings/sampleRegistry.ts`:
```ts
import { SampleRegisteredEvent } from '../types';
import { Sample } from '../types/models/Sample';

export async function handleSampleRegistered(event: SampleRegisteredEvent): Promise<void> {
  const entity = new Sample(event.args.sampleId.toString());
  entity.owner = event.args.owner;
  entity.ipfsCid = Buffer.from(event.args.ipfsCid).toString('utf8');
  entity.blockNumber = event.block.block.header.number;
  entity.timestamp = event.block.timestamp;
  await entity.save();
}
```

### 4. Update `schema.graphql`
```graphql
type Sample @entity {
  id: ID!
  owner: String!
  ipfsCid: String!
  blockNumber: Int!
  timestamp: DateTime!
}
```

### 5. Register the Handler in `project.yaml`
```yaml
dataSources:
  - kind: substrate/Runtime
    ...
    mapping:
      handlers:
        - handler: handleSampleRegistered
          kind: substrate/EventHandler
          filter:
            module: sampleRegistry
            method: SampleRegistered
```

### 6. Codegen, Build, and Run
```sh
subql codegen
subql build
subql start
```

### 7. Query the GraphQL Endpoint
- The indexer exposes a GraphQL endpoint (default: `http://localhost:3000`).
- Example query:
```graphql
query {
  samples(orderBy: blockNumber_DESC, first: 10) {
    id
    owner
    ipfsCid
    blockNumber
    timestamp
  }
}
```

### 8. Customizing Mapping Logic
- Add more handlers for other events/extrinsics (e.g., `SampleStatusUpdated`).
- Extend the GraphQL schema and mapping files as needed.

---

For more details, see [SubQuery documentation](https://academy.subquery.network/).