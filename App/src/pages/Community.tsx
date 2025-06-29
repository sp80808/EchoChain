import React, { useState, useEffect } from 'react';
import AppLayout from '../components/AppLayout';
import LoadingSpinner from '../components/LoadingSpinner';

interface BlockchainStats {
  totalSamples: number;
  totalCreators: number;
  totalNetworkStorage: string;
  monthlyRewardsDistributed: number;
}

const Community: React.FC = () => {
  const [stats, setStats] = useState<BlockchainStats | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchStats = async () => {
      setLoading(true);
      setError(null);
      try {
        const response = await fetch('http://localhost:3001/api/samples/stats');
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data: BlockchainStats = await response.json();
        setStats(data);
      } catch (e: any) {
        setError(e.message);
      } finally {
        setLoading(false);
      }
    };

    fetchStats();
    const interval = setInterval(fetchStats, 60000); // Refresh every minute
    return () => clearInterval(interval);
  }, []);

  return (
    <AppLayout>
      <div className="container mx-auto">
        <h2 className="text-3xl font-bold mb-6">Community Statistics</h2>

        {loading && <LoadingSpinner />}
        {error && <p className="text-center text-red-500">Error: {error}</p>}

        {!loading && !error && stats && (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 bg-gray-800 p-6 rounded-lg shadow-md border border-gray-700">
            <div className="text-center p-4 bg-gray-700 rounded-lg">
              <p className="text-4xl font-bold text-blue-400">{stats.totalSamples}</p>
              <p className="text-gray-300">Samples Shared</p>
            </div>
            <div className="text-center p-4 bg-gray-700 rounded-lg">
              <p className="text-4xl font-bold text-green-400">{stats.totalCreators}</p>
              <p className="text-gray-300">Creators</p>
            </div>
            <div className="text-center p-4 bg-gray-700 rounded-lg">
              <p className="text-4xl font-bold text-purple-400">{stats.totalNetworkStorage}</p>
              <p className="text-gray-300">Network Storage</p>
            </div>
            <div className="text-center p-4 bg-gray-700 rounded-lg">
              <p className="text-4xl font-bold text-yellow-400">{stats.monthlyRewardsDistributed} ECHO</p>
              <p className="text-gray-300">Monthly Rewards</p>
            </div>
          </div>
        )}
      </div>
    </AppLayout>
  );
};

export default Community;
