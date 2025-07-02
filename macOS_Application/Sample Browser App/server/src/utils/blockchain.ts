import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { cryptoWaitReady } from '@polkadot/util-crypto';
import { logger } from './logger';

// Global API instance for reuse
let globalApi: ApiPromise | null = null;
let globalKeyring: Keyring | null = null;

function getWsEndpoint() {
  const username = process.env.POLKADOT_NODE_USERNAME;
  const password = process.env.POLKADOT_NODE_PASSWORD;
  const wsAuth = process.env.POLKADOT_WS_ENDPOINT_AUTH;
  const wsPublic = process.env.POLKADOT_WS_ENDPOINT;
  if (username && password && wsAuth) {
    // Insert credentials into the URL: wss://username:password@host
    const url = new URL(wsAuth);
    url.username = username;
    url.password = password;
    return url.toString();
  }
  return wsPublic || 'ws://127.0.0.1:9944';
}

// Initialize blockchain connection
async function getBlockchainApi(): Promise<ApiPromise> {
  if (globalApi && globalApi.isConnected) {
    return globalApi;
  }

  try {
    await cryptoWaitReady();
    const provider = new WsProvider(getWsEndpoint());
    globalApi = await ApiPromise.create({ provider });
    logger.info(`Connected to EchoChain blockchain node at ${getWsEndpoint()}`);
    return globalApi;
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    logger.error(`Failed to connect to blockchain node: ${errorMessage}`);
    throw new Error(`Blockchain connection failed: ${errorMessage}`);
  }
}

// Initialize keyring
function getKeyring(): Keyring {
  if (!globalKeyring) {
    globalKeyring = new Keyring({ type: 'sr25519' });
  }
  return globalKeyring;
}

// Get service account for blockchain operations
function getServiceAccount(): KeyringPair {
  const seed = process.env.POLKADOT_SERVICE_ACCOUNT_SEED;
  if (!seed) {
    throw new Error('POLKADOT_SERVICE_ACCOUNT_SEED environment variable not set');
  }
  return getKeyring().addFromUri(seed);
}

// Helper function to handle blockchain transactions
async function executeTransaction(
  txName: string,
  tx: any,
  serviceAccount: KeyringPair
): Promise<boolean> {
  return new Promise<boolean>((resolve) => {
    tx.signAndSend(serviceAccount, ({ status, dispatchError }: any) => {
      if (dispatchError) {
        if (dispatchError.isModule) {
          try {
            const decoded = globalApi!.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;
            logger.error(`${txName} blockchain error: ${section}.${name}: ${docs.join(' ')}`);
          } catch (e) {
            logger.error(`${txName} blockchain error: ${dispatchError.toString()}`);
          }
        } else {
          logger.error(`${txName} blockchain error: ${dispatchError.toString()}`);
        }
        resolve(false);
      } else if (status.isInBlock) {
        logger.info(`${txName} completed in block ${status.asInBlock}`);
        resolve(true);
      } else if (status.isFinalized) {
        logger.info(`${txName} finalized at block ${status.asFinalized}`);
      }
    }).catch((err: any) => {
      logger.error(`${txName} transaction failed: ${err}`);
      resolve(false);
    });
  });
}

export const registerSampleOnBlockchain = async (
  ipfsCid: string,
  metadataIpfsCid: string,
  creatorId: string,
  additionalData?: any
): Promise<boolean> => {
  try {
    const api = await getBlockchainApi();
    const serviceAccount = getServiceAccount();

    logger.info(`Registering sample on blockchain: IPFS CID ${ipfsCid}, Creator ${creatorId}`);

    // Construct the extrinsic for sample registration
    const tx = api.tx.sampleRegistry.registerSample(
      ipfsCid,
      metadataIpfsCid,
      creatorId,
      additionalData || null
    );

    return await executeTransaction('Sample registration', tx, serviceAccount);
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    logger.error(`Failed to register sample on blockchain: ${errorMessage}`);
    return false;
  }
};

export const getWalletBalanceFromBlockchain = async (walletAddress: string): Promise<number> => {
  try {
    const api = await getBlockchainApi();
    logger.info(`Fetching balance for wallet: ${walletAddress}`);

    const accountInfo = await api.query.system.account(walletAddress);
    const balance = (accountInfo as any).data;
    const balanceValue = balance.free.toBn().toNumber() / 1000000000000; // Convert from planck to ECHO (assuming 12 decimals)

    logger.info(`Balance for ${walletAddress}: ${balanceValue} ECHO`);
    return balanceValue;
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    logger.error(`Failed to fetch wallet balance: ${errorMessage}`);
    return 0;
  }
};

export const approveSampleOnBlockchain = async (sampleId: string): Promise<boolean> => {
  try {
    const api = await getBlockchainApi();
    const serviceAccount = getServiceAccount();

    logger.info(`Approving sample ${sampleId} on blockchain`);

    const tx = api.tx.sampleRegistry.approveSample(sampleId);
    return await executeTransaction('Sample approval', tx, serviceAccount);
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    logger.error(`Failed to approve sample ${sampleId}: ${errorMessage}`);
    return false;
  }
};

export const transferTokensOnBlockchain = async (
  fromAddress: string,
  toAddress: string,
  amount: number
): Promise<boolean> => {
  try {
    const api = await getBlockchainApi();
    const keyring = getKeyring();
    
    // For this example, we'll use the service account as the sender
    // In a real implementation, you'd need the private key/seed for fromAddress
    const serviceAccount = getServiceAccount();

    logger.info(`Transferring ${amount} ECHO from ${fromAddress} to ${toAddress}`);

    const amountInPlanck = Math.floor(amount * 1000000000000); // Convert to planck (12 decimals)
    const tx = api.tx.balances.transfer(toAddress, amountInPlanck);

    return await executeTransaction('Token transfer', tx, serviceAccount);
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    logger.error(`Failed to transfer tokens: ${errorMessage}`);
    return false;
  }
};

export const reportNetworkContributionToBlockchain = async (
  contributorAddress: string,
  storageBytes: number,
  bandwidthBytes: number
): Promise<boolean> => {
  try {
    const api = await getBlockchainApi();
    const serviceAccount = getServiceAccount();

    logger.info(`Reporting network contribution for ${contributorAddress}: Storage=${storageBytes} bytes, Bandwidth=${bandwidthBytes} bytes`);

    const tx = api.tx.networkRewards.reportContribution(
      contributorAddress,
      storageBytes,
      bandwidthBytes
    );

    return await executeTransaction('Network contribution report', tx, serviceAccount);
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    logger.error(`Failed to report network contribution: ${errorMessage}`);
    return false;
  }
};

export const getSampleMetadataFromBlockchain = async (sampleId: string): Promise<any | null> => {
  try {
    const api = await getBlockchainApi();
    logger.info(`Fetching metadata for sample ID: ${sampleId} from blockchain`);

    const sampleData = await api.query.sampleRegistry.samples(sampleId);
    const sampleOption = sampleData as any;
    
    if (sampleOption.isNone) {
      logger.warn(`Sample ${sampleId} not found on blockchain`);
      return null;
    }

    const metadata = sampleOption.unwrap().toJSON();
    logger.info(`Retrieved metadata for sample ${sampleId}`);
    return metadata;
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    logger.error(`Failed to fetch sample metadata: ${errorMessage}`);
    return null;
  }
};

export const distributeContentRewardsOnBlockchain = async (
  creatorId: string,
  amount: number
): Promise<boolean> => {
  try {
    const api = await getBlockchainApi();
    const serviceAccount = getServiceAccount();

    logger.info(`Distributing content reward of ${amount} ECHO to creator ${creatorId}`);

    const amountInPlanck = Math.floor(amount * 1000000000000); // Convert to planck
    const tx = api.tx.contentRewards.distributeReward(creatorId, amountInPlanck);

    return await executeTransaction('Content reward distribution', tx, serviceAccount);
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    logger.error(`Failed to distribute content reward: ${errorMessage}`);
    return false;
  }
};

export const distributeNetworkRewardsOnBlockchain = async (
  contributorAddress: string,
  amount: number
): Promise<boolean> => {
  try {
    const api = await getBlockchainApi();
    const serviceAccount = getServiceAccount();

    logger.info(`Distributing network reward of ${amount} ECHO to contributor ${contributorAddress}`);

    const amountInPlanck = Math.floor(amount * 1000000000000); // Convert to planck
    const tx = api.tx.networkRewards.distributeReward(contributorAddress, amountInPlanck);

    return await executeTransaction('Network reward distribution', tx, serviceAccount);
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    logger.error(`Failed to distribute network reward: ${errorMessage}`);
    return false;
  }
};

export const getBlockchainStats = async (): Promise<{ 
  totalSamples: number; 
  totalCreators: number; 
  totalNetworkStorage: string; 
  monthlyRewardsDistributed: number 
}> => {
  try {
    const api = await getBlockchainApi();
    logger.info('Fetching blockchain statistics');

    // Query multiple storage items in parallel
    const [samplesCount, creatorsCount, storageStats, rewardStats] = await Promise.all([
      api.query.sampleRegistry.sampleCount(),
      api.query.sampleRegistry.creatorCount(),
      api.query.networkRewards.totalNetworkStorage(),
      api.query.contentRewards.monthlyRewardsDistributed()
    ]);

    const stats = {
      totalSamples: (samplesCount as any).toNumber(),
      totalCreators: (creatorsCount as any).toNumber(),
      totalNetworkStorage: `${((storageStats as any).toNumber() / (1024 * 1024 * 1024 * 1024)).toFixed(2)} TB`,
      monthlyRewardsDistributed: (rewardStats as any).toNumber() / 1000000000000 // Convert from planck
    };

    logger.info(`Retrieved blockchain statistics: ${JSON.stringify(stats)}`);
    return stats;
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    logger.error(`Failed to fetch blockchain stats: ${errorMessage}`);
    
    // Return default stats if blockchain is not available
    return {
      totalSamples: 0,
      totalCreators: 0,
      totalNetworkStorage: '0.00 TB',
      monthlyRewardsDistributed: 0
    };
  }
};

export const submitProposalToBlockchain = async (
  proposerAddress: string,
  title: string,
  description: string
): Promise<boolean> => {
  try {
    const api = await getBlockchainApi();
    const serviceAccount = getServiceAccount();

    logger.info(`Submitting proposal from ${proposerAddress}: ${title}`);

    const tx = api.tx.democracy.propose(
      title,
      description,
      proposerAddress
    );

    return await executeTransaction('Proposal submission', tx, serviceAccount);
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    logger.error(`Failed to submit proposal: ${errorMessage}`);
    return false;
  }
};

export const voteOnProposalOnBlockchain = async (
  voterAddress: string,
  proposalId: string,
  vote: 'aye' | 'nay'
): Promise<boolean> => {
  try {
    const api = await getBlockchainApi();
    const serviceAccount = getServiceAccount();

    logger.info(`${voterAddress} voting ${vote} on proposal ${proposalId}`);

    const tx = api.tx.democracy.vote(proposalId, vote === 'aye');
    return await executeTransaction('Proposal vote', tx, serviceAccount);
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    logger.error(`Failed to cast vote: ${errorMessage}`);
    return false;
  }
};

export const getProposalsFromBlockchain = async (): Promise<any[]> => {
  try {
    const api = await getBlockchainApi();
    logger.info('Fetching proposals from blockchain');

    const proposals = await api.query.democracy.proposals();
    const proposalList = proposals.toJSON() as any[];

    logger.info(`Retrieved ${proposalList.length} proposals from blockchain`);
    return proposalList;
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    logger.error(`Failed to fetch proposals: ${errorMessage}`);
    return [];
  }
};

// Utility function to disconnect from blockchain
export const disconnectFromBlockchain = async (): Promise<void> => {
  if (globalApi) {
    await globalApi.disconnect();
    globalApi = null;
    logger.info('Disconnected from blockchain');
  }
};