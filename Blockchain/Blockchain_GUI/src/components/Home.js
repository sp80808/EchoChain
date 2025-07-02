import React, { useEffect, useState } from 'react';
import { Typography, Box } from '@mui/material';

function Home({ api }) {
  const [nodeInfo, setNodeInfo] = useState('Not connected');

  useEffect(() => {
    const fetchNodeInfo = async () => {
      if (api && api.isReady) {
        try {
          const [chain, nodeName, nodeVersion] = await Promise.all([
            api.rpc.system.chain(),
            api.rpc.system.name(),
            api.rpc.system.version()
          ]);
          setNodeInfo(`${chain} (Node: ${nodeName} v${nodeVersion})`);
        } catch (error) {
          console.error('Error fetching node info in Home:', error);
          setNodeInfo('Error fetching node info');
        }
      } else {
        setNodeInfo('Not connected');
      }
    };

    fetchNodeInfo();

    // Optional: Listen for API ready/disconnected events if needed for dynamic updates
    // This might be redundant if NodeConnection component already handles it well
    // but can be useful for components that directly depend on API state.
    if (api) {
      api.on('ready', fetchNodeInfo);
      api.on('disconnected', fetchNodeInfo);
    }

    return () => {
      if (api) {
        api.off('ready', fetchNodeInfo);
        api.off('disconnected', fetchNodeInfo);
      }
    };
  }, [api]);

  return (
    <Box sx={{ p: 2 }}>
      <Typography variant="h4" component="h1" gutterBottom>
        Welcome to Echochain GUI
      </Typography>
      <Typography variant="body1">
        Node Connection Status: {nodeInfo}
      </Typography>
      {api && api.isReady && (
        <Typography variant="body2" color="text.secondary">
          API is ready. You can now interact with the blockchain.
        </Typography>
      )}
      {!api || !api.isReady && (
        <Typography variant="body2" color="error.main">
          Please connect to a Polkadot node using the connection panel above.
        </Typography>
      )}
    </Box>
  );
}

export default Home;
