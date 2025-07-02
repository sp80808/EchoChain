import React from 'react';
import AppLayout from '../components/AppLayout';

const About: React.FC = () => {
  return (
    <AppLayout>
      <div className="container mx-auto">
        <h2 className="text-3xl font-bold mb-6">About EchoChain</h2>
        <div className="bg-gray-800 p-6 rounded-lg shadow-md border border-gray-700">
          <p className="text-gray-300 mb-4">
            EchoChain is a decentralized, community-driven music sample sharing platform. Our mission is to empower music producers by providing a platform where they can discover, share, and download high-quality, royalty-free audio samples.
          </p>
          <p className="text-gray-300 mb-4">
            The entire ecosystem is powered by a novel, purpose-built blockchain and its native cryptocurrency, the "Echo Token" (ECHO). We believe in rewarding creators directly for their contributions and minimizing centralized infrastructure costs through a peer-to-peer file-sharing model.
          </p>
          <p className="text-gray-300">
            Join our community and be a part of the future of sound!
          </p>
        </div>
      </div>
    </AppLayout>
  );
};

export default About;
