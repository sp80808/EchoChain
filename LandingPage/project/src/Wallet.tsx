import React, { useState, useEffect } from 'react';
import AppLayout from '../components/AppLayout';
import LoadingSpinner from '../components/LoadingSpinner';

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

        const response = await fetch(`http://localhost:3001/api/auth/me`, {
          headers: {
            'x-auth-token': token,
          },
        });

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const userData = await response.json();
        setWalletAddress(userData.walletAddress);
        setBalance(`${userData.balance.toFixed(2)} ECHO`);

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

        {loading && <LoadingSpinner />}
        {error && <p className="text-center text-red-500">Error: {error}</p>}

        {!loading && !error && (
          <div className="bg-gray-800 p-6 rounded-lg shadow-md border border-gray-700">
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
