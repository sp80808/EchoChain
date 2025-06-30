import React from 'react';
import { UploadCloud, CheckCircle, DollarSign } from 'lucide-react';

const HowItWorks: React.FC = () => {
  return (
    <div className="py-16 bg-gray-900 text-white">
      <div className="container mx-auto text-center">
        <h2 className="text-3xl font-bold mb-12">How It Works</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          <div className="flex flex-col items-center p-6 bg-gray-800 rounded-lg shadow-md border border-gray-700">
            <div className="p-4 bg-blue-600 rounded-full mb-4">
              <UploadCloud size={48} />
            </div>
            <h3 className="text-xl font-semibold mb-2">1. Upload Your Sounds & Contribute Resources</h3>
            <p className="text-gray-300">Share your high-quality audio samples and contribute to the decentralized network by seeding files.</p>
          </div>
          <div className="flex flex-col items-center p-6 bg-gray-800 rounded-lg shadow-md border border-gray-700">
            <div className="p-4 bg-green-600 rounded-full mb-4">
              <CheckCircle size={48} />
            </div>
            <h3 className="text-xl font-semibold mb-2">2. Get Verified & Share</h3>
            <p className="text-gray-300">Your samples undergo an originality check. Once approved, they are shared with the community.</p>
          </div>
          <div className="flex flex-col items-center p-6 bg-gray-800 rounded-lg shadow-md border border-gray-700">
            <div className="p-4 bg-purple-600 rounded-full mb-4">
              <DollarSign size={48} />
            </div>
            <h3 className="text-xl font-semibold mb-2">3. Earn Echo Tokens</h3>
            <p className="text-gray-300">Get rewarded with Echo Tokens (ECHO) for your content and network contributions.</p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default HowItWorks;
