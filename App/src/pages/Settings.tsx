import React from 'react';
import AppLayout from '../components/AppLayout';

const Settings: React.FC = () => {
  return (
    <AppLayout>
      <div className="container mx-auto">
        <h2 className="text-3xl font-bold mb-6">Settings</h2>
        <div className="bg-gray-800 p-6 rounded-lg shadow-md">
          <p className="text-gray-300">This is the settings page. More options will be added here.</p>
        </div>
      </div>
    </AppLayout>
  );
};

export default Settings;
