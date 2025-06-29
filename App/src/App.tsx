import React, { useState, useEffect } from 'react';
import Login from './components/Auth/Login';
import Register from './components/Auth/Register';
import SampleBrowser from './pages/SampleBrowser';
import SampleUpload from './pages/SampleUpload';
import MyLibrary from './pages/MyLibrary';
import Wallet from './pages/Wallet';
import Settings from './pages/Settings';
import About from './pages/About';
import Profile from './pages/Profile';
import Governance from './pages/Governance';
import NotFound from './pages/NotFound';
import AppLayout from './components/AppLayout';

interface BlockchainStats {
  totalSamples: number;
  totalCreators: number;
  totalNetworkStorage: string;
  monthlyRewardsDistributed: number;
}

const App: React.FC = () => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [showRegister, setShowRegister] = useState(false);
  const [currentPage, setCurrentPage] = useState('browse'); // 'browse', 'upload', 'my-library', 'wallet', 'settings', 'about', 'profile', 'governance', 'login', 'register'
  const [blockchainStats, setBlockchainStats] = useState<BlockchainStats | null>(null);

  // Check for token on initial load
  React.useEffect(() => {
    const token = localStorage.getItem('token');
    if (token) {
      setIsAuthenticated(true);
      setCurrentPage('browse');
    } else {
      setCurrentPage('login');
    }
  }, []);

  // Fetch blockchain stats
  useEffect(() => {
    const fetchStats = async () => {
      try {
        const response = await fetch('http://localhost:3001/api/samples/stats');
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data: BlockchainStats = await response.json();
        setBlockchainStats(data);
      } catch (error) {
        console.error('Failed to fetch blockchain stats:', error);
      }
    };
    fetchStats();
    // Refresh stats periodically
    const interval = setInterval(fetchStats, 60000); // Every minute
    return () => clearInterval(interval);
  }, []);

  const handleLoginSuccess = () => {
    setIsAuthenticated(true);
    setCurrentPage('browse');
  };

  const handleRegisterSuccess = () => {
    setShowRegister(false); // Go back to login after registration
    setCurrentPage('login');
  };

  const navigateTo = (page: string) => {
    if (page === 'login') {
      setIsAuthenticated(false);
      localStorage.removeItem('token');
    }
    setCurrentPage(page);
  };

  if (!isAuthenticated) {
    return showRegister ? (
      <Register onRegisterSuccess={handleRegisterSuccess} />
    ) : (
      <Login onLoginSuccess={handleLoginSuccess} onShowRegister={() => setShowRegister(true)} />
    );
  }

  return (
    <AppLayout navigateTo={navigateTo} blockchainStats={blockchainStats}>
      {currentPage === 'browse' && <SampleBrowser />}
      {currentPage === 'upload' && <SampleUpload />}
      {currentPage === 'my-library' && <MyLibrary />}
      {currentPage === 'wallet' && <Wallet />}
      {currentPage === 'settings' && <Settings />}
      {currentPage === 'about' && <About />}
      {currentPage === 'profile' && <Profile />}
      {currentPage === 'governance' && <Governance />}
      {/* Fallback for unknown pages */}
      {!['browse', 'upload', 'my-library', 'wallet', 'settings', 'about', 'profile', 'governance'].includes(currentPage) && <NotFound />}
    </AppLayout>
  );
};

export default App;
