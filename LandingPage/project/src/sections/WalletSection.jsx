// src/sections/WalletSection.jsx

import React from 'react';
import { usePolkadot } from '../hooks/usePolkadot';
import { formatAddress, formatBalance } from '../lib/utils';

const WalletSection = () => {
  const { selectedAccount, balance, loading, error } = usePolkadot();

  if (loading) return <p>Loading wallet data...</p>;
  if (error) return <p className="text-red-500">Error: {error}</p>;
  if (!selectedAccount) return <p>Please connect your wallet.</p>;

  return (
    <div className="bg-gray-800 p-6 rounded-lg shadow-md text-white">
      <h2 className="text-2xl font-bold mb-4">Wallet Overview</h2>
      <div className="mb-4">
        <p className="text-lg"><strong>Address:</strong> {formatAddress(selectedAccount.address)}</p>
        <p className="text-lg"><strong>Full Address:</strong> {selectedAccount.address}</p>
        <p className="text-lg"><strong>Balance:</strong> {formatBalance(balance)}</p>
      </div>
      {/* Add more wallet functionalities here, e.g., send tokens */}
    </div>
  );
};

export default WalletSection;
