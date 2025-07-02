// src/hooks/usePolkadot.js

import { useState, useEffect } from 'react';
import { getApi, getAccounts } from '../lib/polkadotApi';
import { web3FromSource } from '@polkadot/extension-dapp';

export const usePolkadot = () => {
  const [api, setApi] = useState(null);
  const [accounts, setAccounts] = useState([]);
  const [selectedAccount, setSelectedAccount] = useState(null);
  const [balance, setBalance] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const init = async () => {
      try {
        const polkadotApi = await getApi();
        setApi(polkadotApi);

        const allAccounts = await getAccounts();
        setAccounts(allAccounts);
        if (allAccounts.length > 0) {
          setSelectedAccount(allAccounts[0]); // Auto-select first account
        }
      } catch (err) {
        console.error('Polkadot initialization error:', err);
        setError('Failed to connect to blockchain or load accounts.');
      } finally {
        setLoading(false);
      }
    };
    init();

    // No explicit disconnect in cleanup for now, as API is shared singleton
    // return () => { disconnectApi(); };
  }, []);

  useEffect(() => {
    const fetchBalance = async () => {
      if (api && selectedAccount) {
        try {
          const { data: { free } } = await api.query.system.account(selectedAccount.address);
          setBalance(free.toHuman());
        } catch (err) {
          console.error('Failed to fetch balance:', err);
          setBalance('Error');
        }
      }
    };
    fetchBalance();
  }, [api, selectedAccount]);

  const handleAccountChange = (account) => {
    setSelectedAccount(account);
  };

  // Function to sign and send a transaction
  const signAndSend = async (extrinsic) => {
    if (!api || !selectedAccount) {
      setError('No API connection or account selected.');
      return;
    }
    try {
      const injector = await web3FromSource(selectedAccount.meta.source);
      return new Promise((resolve, reject) => {
        extrinsic.signAndSend(selectedAccount.address, { signer: injector.signer }, ({ status, dispatchError }) => {
          if (status.isInBlock) {
            console.log(`Transaction included at blockHash ${status.asInBlock}`);
            resolve(status.asInBlock);
          } else if (status.isFinalized) {
            console.log(`Transaction finalized at blockHash ${status.asFinalized}`);
          } else if (dispatchError) {
            let errorMsg = dispatchError.toString();
            if (dispatchError.isModule) {
              // For module errors, we have the pallet index and error index
              const decoded = api.registry.findError(dispatchError.asModule);
              errorMsg = `${decoded.section}.${decoded.method}: ${decoded.docs || ''}`; 
            }
            console.error('Transaction dispatch error:', errorMsg);
            reject(new Error(`Transaction failed: ${errorMsg}`));
          }
        }).catch((err) => {
          console.error('Transaction signing/sending error:', err);
          reject(new Error(`Transaction failed: ${err.message}`));
        });
      });
    } catch (err) {
      console.error('Injector error:', err);
      throw new Error(`Failed to get signer: ${err.message}`);
    }
  };

  return { api, accounts, selectedAccount, balance, loading, error, handleAccountChange, signAndSend };
};
