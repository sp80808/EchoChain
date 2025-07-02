// src/sections/TransactionsSection.jsx

import React, { useState, useEffect } from 'react';
import Input from '../components/Input';
import Button from '../components/Button';
import { usePolkadot } from '../hooks/usePolkadot';

const TransactionsSection = () => {
  const { api, selectedAccount, signAndSend } = usePolkadot();
  const [recentTransactions, setRecentTransactions] = useState([]);
  const [callModule, setCallModule] = useState('');
  const [callMethod, setCallMethod] = useState('');
  const [callArgs, setCallArgs] = useState(''); // JSON string for arguments
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');
  const [error, setError] = useState('');

  useEffect(() => {
    const fetchRecentTransactions = async () => {
      if (api && selectedAccount) {
        // This is complex to implement purely client-side without a custom RPC endpoint
        // that indexes transactions by account. For a production app, this would
        // typically come from a backend API service that has indexed the chain.
        // For now, we'll mock it or show a placeholder.
        setRecentTransactions([
          { hash: '0xabc123...', method: 'sampleRegistry.registerSample', status: 'Finalized' },
          { hash: '0xdef456...', method: 'balances.transfer', status: 'InBlock' },
        ]);
      }
    };
    fetchRecentTransactions();
  }, [api, selectedAccount]);

  const handleCustomTransaction = async (e) => {
    e.preventDefault();
    setLoading(true);
    setMessage('');
    setError('');

    if (!api || !selectedAccount) {
      setError('Wallet not connected.');
      setLoading(false);
      return;
    }
    if (!callModule || !callMethod) {
      setError('Module and Method are required.');
      setLoading(false);
      return;
    }

    try {
      let args = [];
      if (callArgs) {
        try {
          args = JSON.parse(callArgs);
        } catch {
          throw new Error('Invalid JSON for arguments.');
        }
      }

      // Construct the extrinsic
      const extrinsic = api.tx[callModule][callMethod](...args);
      await signAndSend(extrinsic);
      setMessage('Custom transaction sent!');
      setCallModule('');
      setCallMethod('');
      setCallArgs('');
    } catch (err) {
      console.error('Custom transaction error:', err);
      setError(`Custom transaction failed: ${err.message}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="bg-gray-800 p-6 rounded-lg shadow-md text-white">
      <h2 className="text-2xl font-bold mb-4">Transactions</h2>

      <div className="mb-6">
        <h3 className="text-xl font-semibold mb-3">Recent Transactions (Mock)</h3>
        {recentTransactions.length === 0 ? (
          <p className="text-gray-400">No recent transactions found.</p>
        ) : (
          <ul className="list-disc list-inside">
            {recentTransactions.map((tx, index) => (
              <li key={index} className="mb-1">
                <span className="font-mono text-sm">{tx.hash.substring(0, 10)}...</span> - {tx.method} ({tx.status})
              </li>
            ))}
          </ul>
        )}
      </div>

      <div>
        <h3 className="text-xl font-semibold mb-3">Execute Custom Transaction</h3>
        <form onSubmit={handleCustomTransaction}>
          <div className="mb-4">
            <label className="block text-gray-300 text-sm font-bold mb-2">Module:</label>
            <Input type="text" value={callModule} onChange={(e) => setCallModule(e.target.value)} placeholder="e.g., sampleRegistry" />
          </div>
          <div className="mb-4">
            <label className="block text-gray-300 text-sm font-bold mb-2">Method:</label>
            <Input type="text" value={callMethod} onChange={(e) => setCallMethod(e.target.value)} placeholder="e.g., registerSample" />
          </div>
          <div className="mb-4">
            <label className="block text-gray-300 text-sm font-bold mb-2">Arguments (JSON Array):</label>
            <textarea
              value={callArgs}
              onChange={(e) => setCallArgs(e.target.value)}
              placeholder='e.g., ["QmAudioCid", "QmMetadataCid"]
'
              className="w-full p-2 rounded bg-gray-700 text-white border border-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
              rows="3"
            ></textarea>
          </div>
          <Button type="submit" disabled={loading || !selectedAccount}>
            {loading ? 'Sending...' : 'Send Transaction'}
          </Button>
          {message && <p className="text-green-500 mt-4">{message}</p>}
          {error && <p className="text-red-500 mt-4">{error}</p>}
          {!selectedAccount && <p className="text-yellow-500 mt-4">Please connect your wallet to send transactions.</p>}
        </form>
      </div>
    </div>
  );
};

export default TransactionsSection;
