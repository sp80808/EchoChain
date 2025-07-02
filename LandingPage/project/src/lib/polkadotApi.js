// src/lib/polkadotApi.js

import { ApiPromise, WsProvider } from '@polkadot/api';
import { web3Accounts, web3Enable } from '@polkadot/extension-dapp';
import { ECHOCHAIN_NODE_RPC_URL } from './constants';

let api = null;

export const getApi = async () => {
  if (!api) {
    const provider = new WsProvider(ECHOCHAIN_NODE_RPC_URL);
    api = await ApiPromise.create({ provider });
    console.log('Polkadot API connected:', api.genesisHash.toHex());
  }
  return api;
};

export const disconnectApi = async () => {
  if (api) {
    await api.disconnect();
    api = null;
    console.log('Polkadot API disconnected.');
  }
};

export const getAccounts = async () => {
  const extensions = await web3Enable('EchoChain Dashboard');
  if (extensions.length === 0) {
    console.warn('No Polkadot.js extension found. Please install it.');
    return [];
  }
  const accounts = await web3Accounts();
  return accounts;
};
