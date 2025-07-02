import React, { useEffect, useState } from 'react';
import { web3Enable, web3Accounts, web3FromSource } from '@polkadot/extension-dapp';
import { ApiPromise, WsProvider } from '@polkadot/api';

const WS_ENDPOINT = 'wss://rpc.polkadot.io'; // Change to your chain's endpoint if needed

function AccountStatus() {
  const [accounts, setAccounts] = useState([]);
  const [selected, setSelected] = useState(null);
  const [api, setApi] = useState(null);
  const [balance, setBalance] = useState(null);
  const [status, setStatus] = useState('disconnected');

  useEffect(() => {
    web3Enable('Echochain GUI').then((extensions) => {
      if (extensions.length === 0) {
        setStatus('no-extension');
        return;
      }
      web3Accounts().then((accs) => {
        setAccounts(accs);
        setStatus('connected');
      });
    });
    const provider = new WsProvider(WS_ENDPOINT);
    ApiPromise.create({ provider }).then(setApi);
  }, []);

  useEffect(() => {
    if (api && selected) {
      api.query.system.account(selected.address).then(({ data }) => {
        setBalance(data.free.toHuman());
      });
    }
  }, [api, selected]);

  if (status === 'no-extension') {
    return <div style={{ color: 'red' }}>Polkadot.js extension not found. <a href="https://polkadot.js.org/extension/" target="_blank" rel="noopener noreferrer">Install it here</a>.</div>;
  }

  if (status === 'disconnected') {
    return <div>Connecting to wallet...</div>;
  }

  return (
    <div style={{ margin: '1em 0' }}>
      <label htmlFor="account-select">Account: </label>
      <select
        id="account-select"
        value={selected ? selected.address : ''}
        onChange={e => {
          const acc = accounts.find(a => a.address === e.target.value);
          setSelected(acc);
        }}
      >
        <option value="">Select account</option>
        {accounts.map(acc => (
          <option key={acc.address} value={acc.address}>
            {acc.meta.name || acc.address}
          </option>
        ))}
      </select>
      {selected && (
        <span style={{ marginLeft: '1em' }}>
          Balance: {balance || '...'}
        </span>
      )}
    </div>
  );
}

export default AccountStatus; 