import axios from 'axios';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

const API_BASE = process.env.REACT_APP_API_URL || 'http://localhost:3000/api';
const WS_PROVIDER = process.env.REACT_APP_WS_PROVIDER || 'ws://localhost:9944';

export async function postCommission(commissionData, token, account) {
  // commissionData: { id, bounty, description, durationBlocks }
  const res = await axios.post(`${API_BASE}/commissions`, commissionData, {
    headers: { Authorization: `Bearer ${token}` }
  });
  
  // On-chain commission posting logic
  if (account) {
    try {
      const provider = new WsProvider(WS_PROVIDER);
      const api = await ApiPromise.create({ provider });
      const keyring = new Keyring({ type: 'sr25519' });
      const signer = keyring.addFromJson(account.json, account.password);
      
      // Assuming a pallet named 'echochainMarketplace' with a function 'postCommission'
      const tx = api.tx.echochainMarketplace.postCommission(
        commissionData.id,
        commissionData.bounty,
        commissionData.description,
        commissionData.durationBlocks || null
      );
      
      await tx.signAndSend(signer, ({ status }) => {
        if (status.isInBlock) {
          console.log(`Commission ${commissionData.id} posted on-chain in block ${status.asInBlock}`);
        }
      });
    } catch (error) {
      console.error('Error posting commission on-chain:', error);
    }
  }
  
  return res.data;
}

export async function fetchCommissions(params = {}) {
  const res = await axios.get(`${API_BASE}/commissions`, { params });
  return res.data;
}

export async function submitForCommission(commissionId, submissionData, token, account) {
  // submissionData: { id, contentHash }
  const res = await axios.post(`${API_BASE}/commissions/${commissionId}/submissions`, submissionData, {
    headers: { Authorization: `Bearer ${token}` }
  });
  
  // On-chain submission logic
  if (account) {
    try {
      const provider = new WsProvider(WS_PROVIDER);
      const api = await ApiPromise.create({ provider });
      const keyring = new Keyring({ type: 'sr25519' });
      const signer = keyring.addFromJson(account.json, account.password);
      
      // Assuming a pallet named 'echochainMarketplace' with a function 'submitForCommission'
      const tx = api.tx.echochainMarketplace.submitForCommission(
        commissionId,
        submissionData.id,
        submissionData.contentHash
      );
      
      await tx.signAndSend(signer, ({ status }) => {
        if (status.isInBlock) {
          console.log(`Submission ${submissionData.id} for commission ${commissionId} posted on-chain in block ${status.asInBlock}`);
        }
      });
    } catch (error) {
      console.error('Error submitting for commission on-chain:', error);
    }
  }
  
  return res.data;
}

export async function fetchSubmissions(commissionId, params = {}) {
  const res = await axios.get(`${API_BASE}/commissions/${commissionId}/submissions`, { params });
  return res.data;
}

export async function selectSubmission(commissionId, submissionId, token, account) {
  const res = await axios.post(`${API_BASE}/commissions/${commissionId}/select`, { submissionId }, {
    headers: { Authorization: `Bearer ${token}` }
  });
  
  // On-chain selection logic
  if (account) {
    try {
      const provider = new WsProvider(WS_PROVIDER);
      const api = await ApiPromise.create({ provider });
      const keyring = new Keyring({ type: 'sr25519' });
      const signer = keyring.addFromJson(account.json, account.password);
      
      // Assuming a pallet named 'echochainMarketplace' with a function 'selectSubmission'
      const tx = api.tx.echochainMarketplace.selectSubmission(
        commissionId,
        submissionId
      );
      
      await tx.signAndSend(signer, ({ status }) => {
        if (status.isInBlock) {
          console.log(`Submission ${submissionId} selected for commission ${commissionId} on-chain in block ${status.asInBlock}`);
        }
      });
    } catch (error) {
      console.error('Error selecting submission on-chain:', error);
    }
  }
  
  return res.data;
}

export async function getUserCommissions(address) {
  try {
    const provider = new WsProvider(WS_PROVIDER);
    const api = await ApiPromise.create({ provider });
    // Assuming a pallet named 'echochainMarketplace' with a storage item 'commissionsByRequester'
    const commissions = await api.query.echochainMarketplace.commissionsByRequester(address);
    return commissions.map((commission, index) => ({
      id: index,
      description: commission.description.toString(),
      bounty: commission.bounty.toString(),
      status: commission.status.toString(),
      listingDate: new Date().toISOString() // Placeholder, actual date would come from blockchain
    }));
  } catch (error) {
    console.error('Error fetching user commissions from blockchain:', error);
    return [];
  }
}
