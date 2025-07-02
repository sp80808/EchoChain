import React, { useState } from 'react';

interface TipModalProps {
  isOpen: boolean;
  onClose: () => void;
  sampleId: string;
  creatorEmail: string;
  onTipSuccess: () => void;
}

const TipModal: React.FC<TipModalProps> = ({ isOpen, onClose, sampleId, creatorEmail, onTipSuccess }) => {
  const [amount, setAmount] = useState<string>('1');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    const tipAmount = parseFloat(amount);
    if (isNaN(tipAmount) || tipAmount <= 0) {
      setError('Please enter a valid tip amount.');
      setLoading(false);
      return;
    }

    try {
      const token = localStorage.getItem('token');
      const response = await fetch(`http://localhost:3001/api/samples/${sampleId}/tip`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-auth-token': token || '',
        },
        body: JSON.stringify({ amount: tipAmount }),
      });

      const data = await response.json();

      if (!response.ok) {
        throw new Error(data.msg || 'Tipping failed');
      }

      onTipSuccess();
      onClose();
      setAmount('1'); // Reset amount
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-gray-900 bg-opacity-75 flex items-center justify-center z-50">
      <div className="bg-gray-800 p-8 rounded-lg shadow-lg w-full max-w-md border border-gray-700">
        <h2 className="text-2xl font-bold text-white mb-6 text-center">Tip Creator</h2>
        <p className="text-gray-300 mb-4 text-center">Send ECHO tokens to {creatorEmail}</p>
        {error && <p className="text-red-500 text-center mb-4 text-sm">{error}</p>}
        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label htmlFor="tipAmount" className="block text-gray-300 text-sm font-bold mb-2">Amount (ECHO)</label>
            <input
              type="number"
              id="tipAmount"
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline bg-gray-700 border-gray-600 placeholder-gray-500"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              min="0.01"
              step="0.01"
              required
            />
          </div>
          <div className="flex justify-end space-x-4">
            <button
              type="button"
              onClick={onClose}
              className="bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition-colors duration-200"
            >
              Cancel
            </button>
            <button
              type="submit"
              className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition-colors duration-200"
              disabled={loading}
            >
              {loading ? 'Sending...' : 'Send Tip'}
            </button>
          </div>
        </form>
      </div>
    </div>
