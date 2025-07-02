// src/sections/RoyaltySection.jsx

import React, { useState, useEffect } from 'react';
import Button from '../components/Button';
import { usePolkadot } from '../hooks/usePolkadot';
import { formatBalance } from '../lib/utils';

const RoyaltySection = () => {
  const { api, selectedAccount, signAndSend } = usePolkadot();
  const [contentRewards, setContentRewards] = useState(null);
  const [networkRewards, setNetworkRewards] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [message, setMessage] = useState('');

  useEffect(() => {
    const fetchRewards = async () => {
      if (api && selectedAccount) {
        setLoading(true);
        setError('');
        try {
          // Fetch content rewards (conceptual - assuming a query for unclaimed rewards)
          // This might require a custom RPC or a more complex query depending on pallet implementation
          // For now, mock data
          const mockContentRewards = '123456789000000000'; // 0.123456789 ECHO
          setContentRewards(mockContentRewards);

          // Fetch network rewards (conceptual)
          const mockNetworkRewards = '987654321000000000'; // 0.987654321 ECHO
          setNetworkRewards(mockNetworkRewards);

        } catch (err) {
          console.error('Failed to fetch rewards:', err);
          setError('Failed to load royalty information.');
        } finally {
          setLoading(false);
        }
      }
    };
    fetchRewards();
  }, [api, selectedAccount]);

  const handleClaimContentRewards = async () => {
    if (!api || !selectedAccount) {
      setError('Wallet not connected.');
      return;
    }
    setLoading(true);
    setMessage('');
    setError('');
    try {
      // Assuming a `claim_rewards` extrinsic exists in contentRewards pallet
      const extrinsic = api.tx.contentRewards.claimRewards(); // This extrinsic needs to be implemented in the pallet
      await signAndSend(extrinsic);
      setMessage('Content rewards claim initiated!');
      // Refresh rewards after transaction
      // In a real app, you'd subscribe to events or refetch after a delay
    } catch (err) {
      console.error('Claim content rewards error:', err);
      setError(`Failed to claim content rewards: ${err.message}`);
    } finally {
      setLoading(false);
    }
  };

  const handleClaimNetworkRewards = async () => {
    if (!api || !selectedAccount) {
      setError('Wallet not connected.');
      return;
    }
    setLoading(true);
    setMessage('');
    setError('');
    try {
      // Assuming a `claim_rewards` extrinsic exists in networkRewards pallet
      const extrinsic = api.tx.networkRewards.claimRewards(); // This extrinsic needs to be implemented in the pallet
      await signAndSend(extrinsic);
      setMessage('Network rewards claim initiated!');
      // Refresh rewards after transaction
      // In a real app, you'd subscribe to events or refetch after a delay
    } catch (err) {
      console.error('Claim network rewards error:', err);
      setError(`Failed to claim network rewards: ${err.message}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="bg-gray-800 p-6 rounded-lg shadow-md text-white">
      <h2 className="text-2xl font-bold mb-4">Royalty Management</h2>
      {loading ? (
        <p>Loading royalty data...</p>
      ) : error ? (
        <p className="text-red-500">{error}</p>
      ) : (
        <div>
          <div className="mb-4">
            <h3 className="text-xl font-semibold">Content Rewards</h3>
            <p>Unclaimed: {formatBalance(contentRewards)}</p>
            <Button onClick={handleClaimContentRewards} disabled={loading || !selectedAccount || !contentRewards || contentRewards === '0'}>
              Claim Content Rewards
            </Button>
          </div>
          <div>
            <h3 className="text-xl font-semibold">Network Rewards</h3>
            <p>Unclaimed: {formatBalance(networkRewards)}</p>
            <Button onClick={handleClaimNetworkRewards} disabled={loading || !selectedAccount || !networkRewards || networkRewards === '0'}>
              Claim Network Rewards
            </Button>
          </div>
          {message && <p className="text-green-500 mt-4">{message}</p>}
        </div>
      )}
    </div>
  );
};

export default RoyaltySection;
