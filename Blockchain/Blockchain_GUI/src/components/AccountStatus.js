import React, { useEffect, useState } from 'react';
import { web3Enable, web3Accounts, web3FromSource } from '@polkadot/extension-dapp';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { useAccount } from '../contexts/AccountContext';
import { useNotification } from '../contexts/NotificationContext';
import LoadingSpinner from './LoadingSpinner';
import { Button, Select, MenuItem, Typography, Box } from '@mui/material';

const WS_ENDPOINT = 'ws://127.0.0.1:9944'; // Updated for local node

function AccountStatus() {
  const [accounts, setAccounts] = useState([]);
  const [api, setApi] = useState(null);
  const [balance, setBalance] = useState(null);
  const [status, setStatus] = useState('idle'); // idle, connecting, connected, no-extension, error
  const [error, setError] = useState(null);
  const { account, setAccount } = useAccount();
  const { showSuccess, showError, showInfo, showWarning } = useNotification();

  useEffect(() => {
    const provider = new WsProvider(WS_ENDPOINT);
    ApiPromise.create({ provider })
      .then((api) => {
        setApi(api);
        showSuccess('Connected to EchoChain blockchain node');
      })
      .catch((e) => {
        setError('Failed to connect to local node.');
        setStatus('error');
        showError('Failed to connect to blockchain node. Please ensure the node is running.');
      });
  }, [showSuccess, showError]);

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
    showInfo('Connecting to wallet extension...');
    
    try {
      const extensions = await web3Enable('Echochain GUI');
      if (extensions.length === 0) {
        setStatus('no-extension');
        showWarning('Polkadot.js extension not found. Please install it to continue.');
        return;
      }
      const accs = await web3Accounts();
      if (accs.length === 0) {
        setError('No accounts found in Polkadot.js extension.');
        setStatus('error');
        showError('No accounts found in Polkadot.js extension. Please create an account first.');
        return;
      }
      setAccounts(accs);
      setStatus('connected');
      showSuccess(`Successfully connected! Found ${accs.length} account(s).`);
    } catch (e) {
      setError('Failed to connect to wallet.');
      setStatus('error');
      showError('Failed to connect to wallet extension. Please try again.');
    }
  };

  if (status === 'no-extension') {
    return (
      <Box sx={{ color: 'error.main', mt: 2 }}>
        <Typography variant="body1">
          Polkadot.js extension not found. <a href="https://polkadot.js.org/extension/" target="_blank" rel="noopener noreferrer">Install it here</a>.
        </Typography>
      </Box>
    );
  }

  if (status === 'idle' || status === 'error') {
    return (
      <Box sx={{ mt: 2, display: 'flex', alignItems: 'center' }}>
        <Button variant="contained" onClick={connectWallet}>Connect Wallet</Button>
        {error && (
          <Typography variant="body2" color="error" sx={{ ml: 2 }}>
            {error}
          </Typography>
        )}
      </Box>
    );
  }

  if (status === 'connecting') {
    return (
      <Box sx={{ display: 'flex', alignItems: 'center', mt: 2 }}>
        <LoadingSpinner />
        <Typography variant="body1" sx={{ ml: 2 }}>
          Connecting to wallet. Please approve the connection in your extension...
        </Typography>
      </Box>
    );
  }

  return (
    <Box sx={{ mt: 2, display: 'flex', alignItems: 'center' }}>
      <Typography variant="body1" sx={{ mr: 1 }}>Account:</Typography>
      <Select
        id="account-select"
        value={account ? account.address : ''}
        onChange={e => {
          const acc = accounts.find(a => a.address === e.target.value);
          setAccount(acc);
          if (acc) {
            showInfo(`Switched to account: ${acc.meta.name || acc.address.slice(0, 8)}...`);
          }
        }}
        sx={{ minWidth: 200 }}
      >
        <MenuItem value="">Select account</MenuItem>
        {accounts.map(acc => (
          <MenuItem key={acc.address} value={acc.address}>
            {acc.meta.name || acc.address}
          </MenuItem>
        ))}
      </Select>
      {account && (
        <Typography variant="body2" sx={{ ml: 2 }}>
          Balance: {balance || '...'}
        </Typography>
      )}
    </Box>
  );
}

export default AccountStatus;
