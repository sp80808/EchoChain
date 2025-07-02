// src/components/Layout.jsx

import React from 'react';
import WalletConnect from './WalletConnect';

const Layout = ({ children }) => {
  return (
    <div className="min-h-screen bg-gray-900 text-white flex flex-col">
      <header className="bg-gray-800 p-4 shadow-md">
        <div className="container mx-auto flex justify-between items-center">
          <h1 className="text-2xl font-bold">EchoChain Dashboard</h1>
          <nav>
            {/* Navigation links will go here */}
          </nav>
        </div>
      </header>

      <main className="container mx-auto flex-grow p-4 flex">
        <aside className="w-1/4 p-4 bg-gray-800 rounded-lg shadow-md mr-4">
          <WalletConnect />
          {/* Sidebar navigation will go here */}
          <nav className="mt-8">
            <ul>
              <li className="mb-2"><a href="/dashboard" className="text-blue-400 hover:text-blue-600">Dashboard Home</a></li>
              <li className="mb-2"><a href="/dashboard/upload" className="text-blue-400 hover:text-blue-600">Upload Sample</a></li>
              <li className="mb-2"><a href="/dashboard/royalty" className="text-blue-400 hover:text-blue-600">Royalty Management</a></li>
              <li className="mb-2"><a href="/dashboard/wallet" className="text-blue-400 hover:text-blue-600">Wallet Overview</a></li>
              <li className="mb-2"><a href="/dashboard/transactions" className="text-blue-400 hover:text-blue-600">Transactions</a></li>
            </ul>
          </nav>
        </aside>
        <section className="flex-grow p-4 bg-gray-800 rounded-lg shadow-md">
          {children}
        </section>
      </main>

      <footer className="bg-gray-800 p-4 text-center text-gray-500 text-sm">
        © 2025 EchoChain. All rights reserved.
      </footer>
    </div>
  );
};

export default Layout;
