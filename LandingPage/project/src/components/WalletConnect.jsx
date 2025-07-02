// src/components/WalletConnect.jsx

import React from 'react';
import { usePolkadot } from '../hooks/usePolkadot';
import Button from './Button';
import { formatAddress, formatBalance } from '../lib/utils';

const WalletConnect = () => {
  const { accounts, selectedAccount, balance, loading, error, handleAccountChange } = usePolkadot();

  if (loading) return <div className="text-center text-gray-500">Connecting to blockchain...</div>;
  if (error) return <div className="text-center text-red-500">Error: {error}</div>;

  return (
    <div className="bg-gray-800 p-4 rounded-lg shadow-md text-white">
      <h3 className="text-lg font-semibold mb-2">Wallet Status</h3>
      {selectedAccount ? (
        <div>
          <p>Connected Account:</p>
          <select
            onChange={(e) => handleAccountChange(accounts.find(acc => acc.address === e.target.value))}
            value={selectedAccount.address}
            className="w-full p-2 rounded bg-gray-700 text-white border border-gray-600"
          >
            {accounts.map((account) => (
              <option key={account.address} value={account.address}>
                {account.meta.name || formatAddress(account.address)}
              </option>
            ))}
          </select>
          <p className="mt-2">Balance: {formatBalance(balance)}</p>
        </div>
      ) : (
        <div>
          <p className="text-gray-400">No account connected.</p>
          {accounts.length > 0 && (
            <Button onClick={() => handleAccountChange(accounts[0])} className="mt-2">
              Connect Wallet
            </Button>
          )}
          {accounts.length === 0 && (
            <p className="text-sm text-gray-400 mt-2">Please install Polkadot.js extension and create an account.</p>
          )}
        </div>
      )}
    </div>
  );
};

export default WalletConnect;
