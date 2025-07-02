import React, { useState, useEffect } from 'react';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Button, TextField, Typography, Box } from '@mui/material';

function NodeConnection({ onConnect }) {
  const [nodeUrl, setNodeUrl] = useState('ws://127.0.0.1:9944');
  const [api, setApi] = useState(null);
  const [connectionStatus, setConnectionStatus] = useState('Disconnected');
  const [chainInfo, setChainInfo] = useState(null);

  useEffect(() => {
    if (api) {
      api.on('connected', () => {
        setConnectionStatus('Connected');
        console.log('API connected');
      });
      api.on('disconnected', () => {
        setConnectionStatus('Disconnected');
        setChainInfo(null);
        console.log('API disconnected');
      });
      api.on('ready', async () => {
        setConnectionStatus('Ready');
        console.log('API ready');
        try {
          const [chain, nodeName, nodeVersion] = await Promise.all([
            api.rpc.system.chain(),
            api.rpc.system.name(),
            api.rpc.system.version()
          ]);
          setChainInfo(`${chain} (Node: ${nodeName} v${nodeVersion})`);
          onConnect(api); // Pass the connected API to the parent component
        } catch (error) {
          console.error('Error fetching chain info:', error);
          setChainInfo('Error fetching chain info');
        }
      });
      api.on('error', (err) => {
        setConnectionStatus(`Error: ${err.message}`);
        console.error('API error:', err);
      });
    }
  }, [api, onConnect]);

  const connectToNode = async () => {
    if (api) {
      await api.disconnect();
    }
    setConnectionStatus('Connecting...');
    try {
      const provider = new WsProvider(nodeUrl);
      const newApi = await ApiPromise.create({ provider });
      setApi(newApi);
    } catch (error) {
      setConnectionStatus(`Failed to connect: ${error.message}`);
      console.error('Connection error:', error);
    }
  };

  const disconnectFromNode = async () => {
    if (api) {
      await api.disconnect();
      setApi(null);
      setConnectionStatus('Disconnected');
      setChainInfo(null);
    }
  };

  return (
    <Box sx={{ p: 2, border: '1px solid #ccc', borderRadius: '8px', mb: 2 }}>
      <Typography variant="h6" gutterBottom>Node Connection</Typography>
      <TextField
        label="Node URL"
        variant="outlined"
        fullWidth
        value={nodeUrl}
        onChange={(e) => setNodeUrl(e.target.value)}
        sx={{ mb: 2 }}
      />
      <Button
        variant="contained"
        onClick={connectToNode}
        disabled={connectionStatus === 'Connecting...' || connectionStatus === 'Ready'}
        sx={{ mr: 1 }}
      >
        Connect
      </Button>
      <Button
        variant="outlined"
        onClick={disconnectFromNode}
        disabled={!api || connectionStatus === 'Disconnected'}
      >
        Disconnect
      </Button>
      <Typography variant="body1" sx={{ mt: 2 }}>
        Status: {connectionStatus}
      </Typography>
      {chainInfo && (
        <Typography variant="body2" color="text.secondary">
          Chain: {chainInfo}
        </Typography>
      )}
    </Box>
  );
}

export default NodeConnection;
