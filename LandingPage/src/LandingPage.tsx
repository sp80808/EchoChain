import React, { useState, useEffect } from 'react';
import HowItWorks from '../components/HowItWorks';
import LoadingSpinner from '../components/LoadingSpinner';

interface BlockchainStats {
  totalSamples: number;
  totalCreators: number;
  totalNetworkStorage: string;
  monthlyRewardsDistributed: number;
}

interface LandingPageProps {
  onExploreSamples: () => void;
  onJoinCommunity: () => void;
}

const LandingPage: React.FC<LandingPageProps> = ({ onExploreSamples, onJoinCommunity }) => {
  const [blockchainStats, setBlockchainStats] = useState<BlockchainStats | null>(null);
  const [loadingStats, setLoadingStats] = useState(true);
  const [statsError, setStatsError] = useState<string | null>(null);

  useEffect(() => {
    const fetchStats = async () => {
      setLoadingStats(true);
      setStatsError(null);
      try {
        const response = await fetch('http://localhost:3001/api/samples/stats');
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data: BlockchainStats = await response.json();
        setBlockchainStats(data);
      } catch (error: any) {
        setStatsError(error.message);
      } finally {
        setLoadingStats(false);
      }
    };
    fetchStats();
    const interval = setInterval(fetchStats, 60000); // Refresh every minute
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="min-h-screen bg-gray-900 text-white flex flex-col">
      {/* Hero Section */}
      <section className="relative h-screen flex items-center justify-center text-center overflow-hidden">
        {/* Dynamic Audio Visualizer Animation (Placeholder) */}
        <div className="absolute inset-0 bg-gradient-to-br from-blue-900 via-purple-900 to-gray-900 opacity-70 z-0"></div>
        <div className="relative z-10 p-4">
          <h1 className="text-5xl md:text-7xl font-extrabold leading-tight mb-6 animate-fade-in-up">
            Create. Share. Earn. <br /> The Future of Sound is Yours.
          </h1>
          <div className="space-x-4 animate-fade-in-up animation-delay-500">
            <button
              onClick={onExploreSamples}
              className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-8 rounded-full text-lg transition-all duration-300 transform hover:scale-105"
            >
              Explore Samples
            </button>
            <button
              onClick={onJoinCommunity}
              className="bg-transparent border-2 border-white hover:border-blue-400 text-white font-bold py-3 px-8 rounded-full text-lg transition-all duration-300 transform hover:scale-105"
            >
              Join the Community
            </button>
          </div>
        </div>
      </section>

      {/* How It Works Section */}
      <HowItWorks />

      {/* Tokenomics Overview (Placeholder) */}
      <section className="py-16 bg-gray-800 text-white text-center">
        <div className="container mx-auto">
          <h2 className="text-3xl font-bold mb-8">Echo Token (ECHO) Overview</h2>
          <p className="text-gray-300 max-w-3xl mx-auto mb-8">
            The Echo Token (ECHO) powers the entire EchoChain ecosystem. It's designed to reward creators and network contributors directly, fostering a vibrant and sustainable community.
          </p>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            <div className="p-6 bg-gray-700 rounded-lg shadow-md border border-gray-600">
              <h3 className="text-xl font-semibold mb-2">Direct Creator Rewards</h3>
              <p className="text-gray-300">Earn ECHO for uploading high-quality, original samples.</p>
            </div>
            <div className="p-6 bg-gray-700 rounded-lg shadow-md border border-gray-600">
              <h3 className="text-xl font-semibold mb-2">Network Contribution Incentives</h3>
              <p className="text-gray-300">Get rewarded for contributing storage and bandwidth to the P2P network.</p>
            </div>
            <div className="p-6 bg-gray-700 rounded-lg shadow-md border border-gray-600">
              <h3 className="text-xl font-semibold mb-2">Future Governance & Access</h3>
              <p className="text-gray-300">ECHO holders will participate in governance and unlock exclusive content.</p>
            </div>
          </div>
        </div>
      </section>

      {/* Community Section */}
      <section className="py-16 bg-gray-900 text-white text-center">
        <div className="container mx-auto">
          <h2 className="text-3xl font-bold mb-8">Community Stats</h2>
          {loadingStats && <LoadingSpinner />}
          {statsError && <p className="text-center text-red-500">Error loading stats: {statsError}</p>}
          {!loadingStats && !statsError && blockchainStats && (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 bg-gray-800 p-6 rounded-lg shadow-md border border-gray-700">
              <div className="text-center p-4 bg-gray-700 rounded-lg">
                <p className="text-4xl font-bold text-blue-400">{blockchainStats.totalSamples}</p>
                <p className="text-gray-300">Samples Shared</p>
              </div>
              <div className="text-center p-4 bg-gray-700 rounded-lg">
                <p className="text-4xl font-bold text-green-400">{blockchainStats.totalCreators}</p>
                <p className="text-gray-300">Creators</p>
              </div>
              <div className="text-center p-4 bg-gray-700 rounded-lg">
                <p className="text-4xl font-bold text-purple-400">{blockchainStats.totalNetworkStorage}</p>
                <p className="text-gray-300">Network Storage</p>
              </div>
              <div className="text-center p-4 bg-gray-700 rounded-lg">
                <p className="text-4xl font-bold text-yellow-400">{blockchainStats.monthlyRewardsDistributed} ECHO</p>
                <p className="text-gray-300">Monthly Rewards</p>
              </div>
            </div>
          )}
        </div>
      </section>

      {/* FAQ Section (Placeholder) */}
      <section className="py-16 bg-gray-800 text-white">
        <div className="container mx-auto">
          <h2 className="text-3xl font-bold mb-8 text-center">Frequently Asked Questions</h2>
          <div className="max-w-3xl mx-auto space-y-4">
            <div className="bg-gray-700 p-4 rounded-lg border border-gray-600">
              <h3 className="font-semibold text-lg mb-2">What about copyright?</h3>
              <p className="text-gray-300">All samples on EchoChain undergo an automated originality check. Only royalty-free and original content is approved for sharing.</p>
            </div>
            <div className="bg-gray-700 p-4 rounded-lg border border-gray-600">
              <h3 className="font-semibold text-lg mb-2">How does the blockchain work?</h3>
              <p className="text-gray-300">EchoChain uses a custom Substrate-based blockchain to manage token transactions, sample metadata, and creator rewards, ensuring transparency and decentralization.</p>
            </div>
            <div className="bg-gray-700 p-4 rounded-lg border border-gray-600">
              <h3 className="font-semibold text-lg mb-2">How do I earn Echo Tokens?</h3>
              <p className="text-gray-300">You earn ECHO by uploading approved, original samples (Content Rewards) and by contributing storage and bandwidth to the peer-to-peer network (Network Rewards).</p>
            </div>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-gray-900 py-8 text-gray-500 text-center text-sm">
        &copy; {new Date().getFullYear()} EchoChain. All rights reserved.
      </footer>
    </div>
  );
};

export default LandingPage;
