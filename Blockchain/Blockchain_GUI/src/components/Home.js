import React, { useState, useEffect } from 'react';
import { useAccount } from '../contexts/AccountContext';
import { useNotification } from '../contexts/NotificationContext';
import { getBlockchainStats } from '../services/blockchainService';

function Home() {
  const { account } = useAccount();
  const { showSuccess, showError, showInfo } = useNotification();
  const [stats, setStats] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadStats();
  }, []);

  const loadStats = async () => {
    try {
      setLoading(true);
      showInfo('Fetching latest blockchain statistics...');
      
      // Use real blockchain stats instead of mock data
      const blockchainStats = await getBlockchainStats();
      
      // Add additional network status info
      const enhancedStats = {
        ...blockchainStats,
        currentBlockNumber: Math.floor(Math.random() * 200000) + 100000, // This would come from API
        activeValidators: 12, // This would come from API
        networkStatus: 'Healthy' // This would come from API
      };
      
      setStats(enhancedStats);
      showSuccess('Blockchain statistics updated successfully');
    } catch (error) {
      console.error('Failed to load blockchain stats:', error);
      showError(`Failed to fetch blockchain statistics: ${error.message}`);
      // Fallback to show some basic info even if blockchain is not available
      setStats({
        totalSamples: 0,
        totalCreators: 0,
        totalNetworkStorage: '0.00 TB',
        monthlyRewardsDistributed: 0,
        currentBlockNumber: 0,
        activeValidators: 0,
        networkStatus: 'Disconnected'
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ padding: '20px' }}>
      <h2>Welcome to Echochain Blockchain Management</h2>
      
      {account ? (
        <div style={{ marginBottom: '30px', padding: '15px', backgroundColor: '#e8f5e8', borderRadius: '8px' }}>
          <h3>Connected Account</h3>
          <p><strong>Address:</strong> {account.address}</p>
          <p><strong>Name:</strong> {account.meta?.name || 'Unnamed Account'}</p>
        </div>
      ) : (
        <div style={{ marginBottom: '30px', padding: '15px', backgroundColor: '#fff3cd', borderRadius: '8px' }}>
          <p>⚠️ No account connected. Use the Account Status section to connect your Polkadot extension.</p>
        </div>
      )}

      <h3>Blockchain Overview</h3>
      
      {loading ? (
        <p>Loading blockchain statistics...</p>
      ) : stats ? (
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))', gap: '20px', marginBottom: '30px' }}>
          <div style={{ padding: '20px', border: '1px solid #ddd', borderRadius: '8px' }}>
            <h4>Network Status</h4>
            <p style={{ color: stats.networkStatus === 'Healthy' ? 'green' : 'red', fontWeight: 'bold' }}>
              {stats.networkStatus}
            </p>
            <p><strong>Block:</strong> #{stats.currentBlockNumber}</p>
            <p><strong>Validators:</strong> {stats.activeValidators}</p>
          </div>
          
          <div style={{ padding: '20px', border: '1px solid #ddd', borderRadius: '8px' }}>
            <h4>Content Statistics</h4>
            <p><strong>Total Samples:</strong> {stats.totalSamples}</p>
            <p><strong>Total Creators:</strong> {stats.totalCreators}</p>
            <p><strong>Avg Samples/Creator:</strong> {Math.round(stats.totalSamples / stats.totalCreators)}</p>
          </div>
          
          <div style={{ padding: '20px', border: '1px solid #ddd', borderRadius: '8px' }}>
            <h4>Network Storage</h4>
            <p><strong>Total Storage:</strong> {stats.totalNetworkStorage}</p>
            <p><strong>Monthly Rewards:</strong> {stats.monthlyRewardsDistributed} ECHO</p>
          </div>
        </div>
      ) : (
        <p>Failed to load blockchain statistics.</p>
      )}

      <h3>Quick Actions</h3>
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '15px' }}>
        <div style={{ padding: '15px', border: '1px solid #ddd', borderRadius: '8px' }}>
          <h4>📂 Sample Management</h4>
          <p>Upload and manage your music samples on the blockchain.</p>
          <p><a href="/register-sample">Register Sample</a> | <a href="/my-samples">My Samples</a></p>
        </div>
        
        <div style={{ padding: '15px', border: '1px solid #ddd', borderRadius: '8px' }}>
          <h4>💼 Marketplace</h4>
          <p>Post commissions and submit work for bounties.</p>
          <p><a href="/post-commission">Post Commission</a> | <a href="/commission-submissions">Browse Commissions</a></p>
        </div>
        
        <div style={{ padding: '15px', border: '1px solid #ddd', borderRadius: '8px' }}>
          <h4>💰 Transfers</h4>
          <p>Send and receive ECHO tokens.</p>
          <p><a href="/transfers">Transfer Tokens</a> | <a href="/faucet">Get Test Tokens</a></p>
        </div>
        
        <div style={{ padding: '15px', border: '1px solid #ddd', borderRadius: '8px' }}>
          <h4>🔧 Network</h4>
          <p>Monitor and manage network settings.</p>
          <p><a href="/network">Network Status</a> | <a href="/settings">Settings</a></p>
        </div>
      </div>

      <div style={{ marginTop: '40px', padding: '20px', backgroundColor: '#f8f9fa', borderRadius: '8px' }}>
        <h3>About EchoChain</h3>
        <p>
          EchoChain is a gas-free blockchain designed for decentralized music sample sharing,
          contribution rewards, and seamless integration with P2P and backend services.
        </p>
        <ul>
          <li><strong>Gas-Free:</strong> No transaction fees for core operations</li>
          <li><strong>Sample Registry:</strong> Register and approve music samples on-chain</li>
          <li><strong>Reward System:</strong> Automatic content and network rewards</li>
          <li><strong>Marketplace:</strong> Commission-based audio work platform</li>
          <li><strong>P2P Integration:</strong> Decentralized file sharing with rewards</li>
        </ul>
      </div>
    </div>
  );
}

export default Home;
