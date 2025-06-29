import React from 'react';

interface AppLayoutProps {
  children: React.ReactNode;
  navigateTo: (page: string) => void;
  blockchainStats: {
    totalSamples: number;
    totalCreators: number;
    totalNetworkStorage: string;
    monthlyRewardsDistributed: number;
  } | null;
}

const AppLayout: React.FC<AppLayoutProps> = ({ children, navigateTo, blockchainStats }) => {
  return (
    <div className="flex h-screen bg-gray-900 text-white">
      {/* Sidebar */}
      <aside className="w-64 bg-gray-800 p-4 shadow-lg flex flex-col justify-between">
        <div>
          <h1 className="text-3xl font-bold mb-6 text-blue-400">EchoChain</h1>
          <nav>
            <ul>
              <li className="mb-3">
                <button onClick={() => navigateTo('browse')} className="block py-2 px-4 rounded hover:bg-gray-700 transition-colors duration-200 w-full text-left">Explore Samples</button>
              </li>
              <li className="mb-3">
                <button onClick={() => navigateTo('my-library')} className="block py-2 px-4 rounded hover:bg-gray-700 transition-colors duration-200 w-full text-left">My Library</button>
              </li>
              <li className="mb-3">
                <button onClick={() => navigateTo('upload')} className="block py-2 px-4 rounded hover:bg-gray-700 transition-colors duration-200 w-full text-left">Upload</button>
              </li>
              <li className="mb-3">
                <button onClick={() => navigateTo('wallet')} className="block py-2 px-4 rounded hover:bg-gray-700 transition-colors duration-200 w-full text-left">Wallet</button>
              </li>
              <li className="mb-3">
                <button onClick={() => navigateTo('settings')} className="block py-2 px-4 rounded hover:bg-gray-700 transition-colors duration-200 w-full text-left">Settings</button>
              </li>
              <li className="mb-3">
                <button onClick={() => navigateTo('profile')} className="block py-2 px-4 rounded hover:bg-gray-700 transition-colors duration-200 w-full text-left">Profile</button>
              </li>
              <li className="mb-3">
                <button onClick={() => navigateTo('governance')} className="block py-2 px-4 rounded hover:bg-gray-700 transition-colors duration-200 w-full text-left">Governance</button>
              </li>
              <li className="mb-3">
                <button onClick={() => navigateTo('community')} className="block py-2 px-4 rounded hover:bg-gray-700 transition-colors duration-200 w-full text-left">Community</button>
              </li>
              <li className="mb-3">
                <button onClick={() => navigateTo('about')} className="block py-2 px-4 rounded hover:bg-gray-700 transition-colors duration-200 w-full text-left">About</button>
              </li>
            </ul>
          </nav>
        </div>
        <div>
          <button onClick={() => {
            localStorage.removeItem('token');
            navigateTo('login');
          }} className="block py-2 px-4 rounded hover:bg-gray-700 transition-colors duration-200 w-full text-left text-red-400">Logout</button>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 flex flex-col">
        <header className="bg-gray-800 p-4 shadow-lg flex items-center justify-between">
          <h2 className="text-2xl font-bold">Welcome to EchoChain!</h2>
          {blockchainStats && (
            <div className="flex space-x-4 text-sm text-gray-400">
              <span>Samples: <span className="font-bold text-white">{blockchainStats.totalSamples}</span></span>
              <span>Creators: <span className="font-bold text-white">{blockchainStats.totalCreators}</span></span>
              <span>Storage: <span className="font-bold text-white">{blockchainStats.totalNetworkStorage}</span></span>
              <span>Rewards: <span className="font-bold text-white">{blockchainStats.monthlyRewardsDistributed} ECHO</span></span>
            </div>
          )}
        </header>
        <div className="flex-1 p-6 overflow-y-auto">
          {children}
        </div>
        <footer className="bg-gray-800 p-4 shadow-lg text-center text-gray-500 text-sm">
          &copy; {new Date().getFullYear()} EchoChain. All rights reserved.
        </footer>
      </main>
    </div>
  );
};

export default AppLayout;
