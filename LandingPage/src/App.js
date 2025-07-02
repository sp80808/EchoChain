import React from 'react';
import './index.css';

function App() {
  return (
    <div className="min-h-screen bg-gray-900 text-white flex flex-col items-center justify-center">
      {/* Hero Section */}
      <section className="text-center py-20 px-4">
        <h1 className="text-5xl md:text-6xl font-bold leading-tight mb-6">
          Create. Share. Earn. The Future of Sound is Yours.
        </h1>
        <p className="text-xl md:text-2xl mb-8">
          Discover, upload, and monetize your audio creations on EchoChain.
        </p>
        <div className="space-x-4">
          <button className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-8 rounded-full transition duration-300">
            Explore Samples
          </button>
          <button className="bg-gray-700 hover:bg-gray-600 text-white font-bold py-3 px-8 rounded-full transition duration-300">
            Join the Community
          </button>
        </div>
        {/* Placeholder for dynamic audio visualizer animation */}
        <div className="mt-12 w-full h-48 bg-gray-800 rounded-lg flex items-center justify-center text-gray-500">
          Audio Visualizer Placeholder
        </div>
      </section>

      {/* How It Works Section (Placeholder) */}
      <section className="py-20 px-4 text-center">
        <h2 className="text-4xl font-bold mb-12">How It Works</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          <div className="p-6 bg-gray-800 rounded-lg">
            <h3 className="text-2xl font-semibold mb-4">1. Upload Your Sounds & Contribute Resources</h3>
            <p className="text-gray-400">Share your unique audio creations and contribute to the network.</p>
          </div>
          <div className="p-6 bg-gray-800 rounded-lg">
            <h3 className="text-2xl font-semibold mb-4">2. Get Verified & Share</h3>
            <p className="text-gray-400">Verify your content and share it with the world.</p>
          </div>
          <div className="p-6 bg-gray-800 rounded-lg">
            <h3 className="text-2xl font-semibold mb-4">3. Earn Echo Tokens</h3>
            <p className="text-gray-400">Monetize your work and earn rewards in Echo Tokens.</p>
          </div>
        </div>
      </section>

      {/* Featured Samples Section (Placeholder) */}
      <section className="py-20 px-4 text-center">
        <h2 className="text-4xl font-bold mb-12">Featured Samples</h2>
        <div className="w-full h-64 bg-gray-800 rounded-lg flex items-center justify-center text-gray-500">
          Curated Samples List Placeholder
        </div>
      </section>

      {/* Tokenomics Overview Section (Placeholder) */}
      <section className="py-20 px-4 text-center">
        <h2 className="text-4xl font-bold mb-12">Tokenomics Overview</h2>
        <div className="w-full h-48 bg-gray-800 rounded-lg flex items-center justify-center text-gray-500">
          Tokenomics Explanation Placeholder
        </div>
      </section>

      {/* Community Section (Placeholder) */}
      <section className="py-20 px-4 text-center">
        <h2 className="text-4xl font-bold mb-12">Community Stats</h2>
        <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
          <div className="p-6 bg-gray-800 rounded-lg">
            <h3 className="text-3xl font-bold">100K+</h3>
            <p className="text-gray-400">Samples Shared</p>
          </div>
          <div className="p-6 bg-gray-800 rounded-lg">
            <h3 className="text-3xl font-bold">50K+</h3>
            <p className="text-gray-400">Creators</p>
          </div>
          <div className="p-6 bg-gray-800 rounded-lg">
            <h3 className="text-3xl font-bold">1PB+</h3>
            <p className="text-gray-400">Network Storage</p>
          </div>
          <div className="p-6 bg-gray-800 rounded-lg">
            <h3 className="text-3xl font-bold">$1M+</h3>
            <p className="text-gray-400">Monthly Rewards</p>
          </div>
        </div>
      </section>

      {/* FAQ Section (Placeholder) */}
      <section className="py-20 px-4 text-center">
        <h2 className="text-4xl font-bold mb-12">Frequently Asked Questions</h2>
        <div className="w-full h-48 bg-gray-800 rounded-lg flex items-center justify-center text-gray-500">
          FAQ Content Placeholder
        </div>
      </section>
    </div>
  );
}

export default App;