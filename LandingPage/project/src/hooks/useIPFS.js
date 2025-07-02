// src/hooks/useIPFS.js

import { useState, useEffect } from 'react';
import { uploadFileToIPFS, uploadJsonToIPFS } from '../lib/ipfsClient';

export const useIPFS = () => {
  const [isIpfsReady, setIsIpfsReady] = useState(false);
  const [ipfsError, setIpfsError] = useState(null);

  useEffect(() => {
    // Basic check if IPFS daemon is accessible
    const checkIpfs = async () => {
      try {
        // Attempt a simple IPFS operation, e.g., getting ID
        // The ipfs-http-client `create` function doesn't expose a direct `isReady` or `ping`
        // so we'll rely on the first operation to fail or succeed.
        // For a more robust check, you might need a dedicated /api/v0/id call.
        await uploadJsonToIPFS({ test: 'connection' }); // Try a dummy upload
        setIsIpfsReady(true);
      } catch (err) {
        console.error('IPFS connection check failed:', err);
        setIpfsError('Failed to connect to IPFS daemon. Make sure it's running.');
        setIsIpfsReady(false);
      }
    };
    checkIpfs();
  }, []);

  return { isIpfsReady, ipfsError, uploadFileToIPFS, uploadJsonToIPFS };
};
