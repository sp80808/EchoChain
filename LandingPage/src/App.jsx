// src/App.jsx

import React, { useState } from 'react';
import Layout from './components/Layout';
import UploadSection from './sections/UploadSection';
import RoyaltySection from './sections/RoyaltySection';
import WalletSection from './sections/WalletSection';
import TransactionsSection from './sections/TransactionsSection';

const App = () => {
  const [currentView, setCurrentView] = useState('dashboard'); // Default view

  // Simple routing based on currentView state
  const renderView = () => {
    switch (currentView) {
      case 'upload':
        return <UploadSection />;
      case 'royalty':
        return <RoyaltySection />;
      case 'wallet':
        return <WalletSection />;
      case 'transactions':
        return <TransactionsSection />;
      case 'dashboard':
      default:
        return (
          <div className="text-white">
            <h2 className="text-2xl font-bold mb-4">Welcome to EchoChain Dashboard!</h2>
            <p>Use the sidebar to navigate through the features.</p>
          </div>
        );
    }
  };

  return (
    <Layout>
      {/* This is a simplified routing. In a real Vite/React app, you'd use react-router-dom */}
      {/* For now, the Layout component's navigation links will need to update `currentView` state */}
      {/* This means the <a> tags in Layout.jsx should be replaced with <button> or similar that calls setCurrentView */}
      {renderView()}
    </Layout>
  );
};

export default App;
