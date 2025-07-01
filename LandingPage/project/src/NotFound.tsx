import React from 'react';
import AppLayout from '../components/AppLayout';

const NotFound: React.FC = () => {
  return (
    <AppLayout>
      <div className="flex flex-col items-center justify-center h-full text-white">
        <h1 className="text-6xl font-bold text-blue-400 mb-4">404</h1>
        <p className="text-2xl mb-8">Page Not Found</p>
        <p className="text-gray-400">The page you are looking for does not exist.</p>
      </div>
    </AppLayout>
  );
};

export default NotFound;
