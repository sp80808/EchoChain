const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise } = require('@polkadot/api-contract');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

async function main() {
    const provider = new WsProvider('ws://127.0.0.1:9945'); // Alice's WS port
    const api = await ApiPromise.create({ provider });

    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    const charlie = keyring.addFromUri('//Charlie');

    console.log('Connected to EchoChain node (Alice).');
    console.log(`Alice's address: ${alice.address}`);
    console.log(`Bob's address: ${bob.address}`);
    console.log(`Charlie's address: ${charlie.address}`);

    // --- Initial Asset Distribution ---
    console.log('\n--- Initial Asset Distribution ---');
    const initialAmount = api.createType('Balance', '1000000000000000000000'); // 1000 units (adjust as needed)

    const transferToBob = api.tx.balances.transfer(bob.address, initialAmount);
    const transferToCharlie = api.tx.balances.transfer(charlie.address, initialAmount);

    console.log(`Transferring ${initialAmount.toString()} to Bob and Charlie...`);
    await new Promise(async (resolve, reject) => {
        await api.tx.utility
            .batchAll([transferToBob, transferToCharlie])
            .signAndSend(alice, ({ status }) => {
                if (status.isInBlock) {
                    console.log(`Transaction included at block hash ${status.asInBlock}`);
                } else if (status.isFinalized) {
                    console.log('Asset distribution finalized.');
                    resolve();
                }
            });
    });

    // --- Smart Contract Compilation and Deployment ---
    console.log('\n--- Smart Contract Compilation and Deployment ---');

    const contractPath = '../../contracts/flipper';
    const contractTargetDir = path.join(contractPath, 'target');
    const contractBuildDir = path.join(contractTargetDir, 'ink');

    // Ensure cargo-contract is installed
    try {
        execSync('cargo install cargo-contract --force', { stdio: 'inherit' });
    } catch (error) {
        console.error('Failed to install cargo-contract. Please ensure Rust toolchain is set up correctly.');
        process.exit(1);
    }

    // Compile the Flipper contract
    console.log(`Compiling Flipper contract in ${contractPath}...`);
    try {
        execSync(`cd ${contractPath} && cargo contract build`, { stdio: 'inherit' });
        console.log('Flipper contract compiled successfully.');
    } catch (error) {
        console.error('Failed to compile Flipper contract:', error.message);
        process.exit(1);
    }

    // Load the contract WASM and ABI
    const flipperWasm = fs.readFileSync(path.join(contractBuildDir, 'flipper.wasm'));
    const flipperAbi = JSON.parse(fs.readFileSync(path.join(contractBuildDir, 'flipper.json'), 'utf8'));

    // Deploy the Flipper contract
    console.log('Deploying Flipper contract...');
    const contract = new ContractPromise(api, flipperAbi, flipperWasm);

    const gasLimit = api.registry.createType('WeightV2', {
        refTime: api.consts.system.blockWeights.maxBlock.refTime.muln(2),
        proofSize: api.consts.system.blockWeights.maxBlock.proofSize.muln(2),
    });
    const value = api.createType('Balance', 0); // No value transferred with deployment

    const tx = contract.tx.new(
        { gasLimit, value, storageDepositLimit: null },
        false // Initial value for the Flipper contract
    );

    await new Promise(async (resolve, reject) => {
        await tx.signAndSend(alice, ({ status, contractEvents, dispatchError }) => {
            if (status.isInBlock) {
                console.log(`Contract deployment transaction in block: ${status.asInBlock}`);
            } else if (status.isFinalized) {
                if (dispatchError) {
                    if (dispatchError.isModule) {
                        const decoded = api.registry.findError(dispatchError.asModule);
                        const { docs, name, pallet } = decoded;
                        console.error(`Contract deployment failed: ${pallet}.${name}: ${docs.join(' ')}`);
                    } else {
                        console.error(`Contract deployment failed: ${dispatchError.toString()}`);
                    }
                    reject(new Error('Contract deployment failed.'));
                } else {
                    const contractAddress = contractEvents[0].asV1.contract.toString();
                    console.log(`Flipper contract deployed at: ${contractAddress}`);
                    // Store contract address for future interactions if needed
                    resolve();
                }
            }
        });
    });

    console.log('\n--- All deployment and funding steps completed. ---');
    process.exit(0);
}

main().catch(console.error);