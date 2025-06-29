import React, { useState, useEffect } from 'react';
import AppLayout from '../components/AppLayout';

const Wallet: React.FC = () => {
  const [walletAddress, setWalletAddress] = useState<string | null>(null);
  const [balance, setBalance] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchWalletInfo = async () => {
      setLoading(true);
      setError(null);
      try {
        const token = localStorage.getItem('token');
        if (!token) {
          setError('User not authenticated.');
          setLoading(false);
          return;
        }

        // In a real application, this would fetch from a backend API
        // that queries the blockchain for wallet address and balance.
        // For now, using a placeholder.
        const dummyWalletAddress = "ECHO_WALLET_1234567890ABCDEF";
        const dummyBalance = "1000.00 ECHO";

        setWalletAddress(dummyWalletAddress);
        setBalance(dummyBalance);

      } catch (e: any) {
        setError(e.message);
      } finally {
        setLoading(false);
      }
    };

    fetchWalletInfo();
  }, []);

  return (
    <AppLayout>
      <div className="container mx-auto">
        <h2 className="text-3xl font-bold mb-6">My Wallet</h2>

        {loading && <p className="text-center text-blue-400">Loading wallet information...</p>}
        {error && <p className="text-center text-red-500">Error: {error}</p>}

        {!loading && !error && (
          <div className="bg-gray-800 p-6 rounded-lg shadow-md">
            <div className="mb-4">
              <p className="text-gray-400 text-sm">Wallet Address:</p>
              <p className="text-lg font-mono break-all">{walletAddress}</p>
            </div>
            <div>
              <p className="text-gray-400 text-sm">Current Balance:</p>
              <p className="text-2xl font-bold text-green-400">{balance}</p>
            </div>
            {/* Add more wallet details or transaction history here */}
          </div>
        )}
      </div>
    </AppLayout>
  );
};

export default Wallet;
