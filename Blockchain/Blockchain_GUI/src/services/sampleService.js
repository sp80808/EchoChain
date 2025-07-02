import axios from 'axios';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

const API_BASE = process.env.REACT_APP_API_URL || 'http://localhost:3000/api';
const WS_PROVIDER = process.env.REACT_APP_WS_PROVIDER || 'ws://localhost:9944';

export async function registerSample(sample, token, account) {
  // sample: { id, title, artist, duration, category, p2pContentId, price, ownerAddress, blockchainHash }
  const res = await axios.post(`${API_BASE}/samples`, sample, {
    headers: { Authorization: `Bearer ${token}` }
  });
  
  // On-chain registration logic
  if (account) {
    try {
      const provider = new WsProvider(WS_PROVIDER);
      const api = await ApiPromise.create({ provider });
      const keyring = new Keyring({ type: 'sr25519' });
      const signer = keyring.addFromJson(account.json, account.password);
      
      // Assuming a pallet named 'sampleRegistry' with a function 'registerSample'
      const tx = api.tx.sampleRegistry.registerSample(
        sample.id,
        sample.blockchainHash,
        sample.title,
        sample.artist
      );
      
      await tx.signAndSend(signer, ({ status }) => {
        if (status.isInBlock) {
          console.log(`Sample ${sample.id} registered on-chain in block ${status.asInBlock}`);
        }
      });
    } catch (error) {
      console.error('Error registering sample on-chain:', error);
    }
  }
  
  return res.data;
}

export async function fetchSamples(params = {}) {
  const res = await axios.get(`${API_BASE}/samples`, { params });
  return res.data;
}

export async function getUserSamples(address) {
  try {
    const provider = new WsProvider(WS_PROVIDER);
    const api = await ApiPromise.create({ provider });
    // Assuming a pallet named 'sampleRegistry' with a storage item 'samplesByOwner'
    const samples = await api.query.sampleRegistry.samplesByOwner(address);
    return samples.map((sample, index) => ({
      id: index,
      description: sample.description.toString(),
      hash: sample.hash.toString(),
      registrationDate: new Date().toISOString() // Placeholder, actual date would come from blockchain
    }));
  } catch (error) {
    console.error('Error fetching user samples from blockchain:', error);
    return [];
  }
}
