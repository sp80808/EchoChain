import React, { useEffect, useState } from 'react';
import { web3Enable, web3Accounts, web3FromSource } from '@polkadot/extension-dapp';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { useAccount } from '../contexts/AccountContext';
import LoadingSpinner from './LoadingSpinner';

const WS_ENDPOINT = 'ws://127.0.0.1:9944'; // Updated for local node

function AccountStatus() {
  const [accounts, setAccounts] = useState([]);
  const [api, setApi] = useState(null);
  const [balance, setBalance] = useState(null);
  const [status, setStatus] = useState('idle'); // idle, connecting, connected, no-extension, error
  const [error, setError] = useState(null);
  const { account, setAccount } = useAccount();

  useEffect(() => {
    const provider = new WsProvider(WS_ENDPOINT);
    ApiPromise.create({ provider })
      .then(setApi)
      .catch((e) => {
        setError('Failed to connect to local node.');
        setStatus('error');
      });
  }, []);

  useEffect(() => {
    if (api && account) {
      api.query.system.account(account.address)
        .then(({ data }) => setBalance(data.free.toHuman()))
        .catch(() => setBalance('Error'));
    }
  }, [api, account]);

  const connectWallet = async () => {
    setStatus('connecting');
    setError(null);
    try {
      const extensions = await web3Enable('Echochain GUI');
      if (extensions.length === 0) {
        setStatus('no-extension');
        return;
      }
      const accs = await web3Accounts();
      if (accs.length === 0) {
        setError('No accounts found in Polkadot.js extension.');
        setStatus('error');
        return;
      }
      setAccounts(accs);
      setStatus('connected');
    } catch (e) {
      setError('Failed to connect to wallet.');
      setStatus('error');
    }
  };

  if (status === 'no-extension') {
    return <div style={{ color: 'red' }}>Polkadot.js extension not found. <a href="https://polkadot.js.org/extension/" target="_blank" rel="noopener noreferrer">Install it here</a>.</div>;
  }

  if (status === 'idle' || status === 'error') {
    return (
      <div style={{ margin: '1em 0' }}>
        <button onClick={connectWallet}>Connect Wallet</button>
        {error && <span style={{ color: 'red', marginLeft: '1em' }}>{error}</span>}
      </div>
    );
  }

  if (status === 'connecting') {
    return <><LoadingSpinner /><div>Connecting to wallet. Please approve the connection in your extension...</div></>;
  }

  return (
    <div style={{ margin: '1em 0' }}>
      <label htmlFor="account-select">Account: </label>
      <select
        id="account-select"
        value={account ? account.address : ''}
        onChange={e => {
          const acc = accounts.find(a => a.address === e.target.value);
          setAccount(acc);
        }}
      >
        <option value="">Select account</option>
        {accounts.map(acc => (
          <option key={acc.address} value={acc.address}>
            {acc.meta.name || acc.address}
          </option>
        ))}
      </select>
      {account && (
        <span style={{ marginLeft: '1em' }}>
          Balance: {balance || '...'}
        </span>
      )}
    </div>
  );
}

export default AccountStatus; 