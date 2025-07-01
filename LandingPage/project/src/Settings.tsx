import React from 'react';
import AppLayout from '../components/AppLayout';

const Settings: React.FC = () => {
  return (
    <AppLayout>
      <div className="container mx-auto">
        <h2 className="text-3xl font-bold mb-6">Settings</h2>
        <div className="bg-gray-800 p-6 rounded-lg shadow-md">
          <h3 className="text-xl font-semibold mb-4">General Settings</h3>
          <div className="mb-4">
            <label htmlFor="theme" className="block text-gray-300 text-sm font-bold mb-2">Theme</label>
            <select
              id="theme"
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline bg-gray-700 border-gray-600"
            >
              <option value="dark">Dark Mode</option>
              {/* Add other themes as needed */}
            </select>
          </div>
          <h3 className="text-xl font-semibold mb-4 mt-6">Account Settings</h3>
          <button
            className="bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            onClick={() => alert('Change Password functionality coming soon!')}
          >
            Change Password
          </button>
        </div>
      </div>
    </AppLayout>
  );
};

export default Settings;
