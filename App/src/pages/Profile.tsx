import React, { useState, useEffect } from 'react';
import AppLayout from '../components/AppLayout';
import LoadingSpinner from '../components/LoadingSpinner';

interface UserProfile {
  email: string;
  walletAddress: string;
}

const Profile: React.FC = () => {
  const [userProfile, setUserProfile] = useState<UserProfile | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchUserProfile = async () => {
      setLoading(true);
      setError(null);
      try {
        const token = localStorage.getItem('token');
        if (!token) {
          setError('User not authenticated.');
          setLoading(false);
          return;
        }

        const response = await fetch(`http://localhost:3001/api/auth/me`, {
          headers: {
            'x-auth-token': token,
          },
        });

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data: UserProfile = await response.json();
        setUserProfile(data);
      } catch (e: any) {
        setError(e.message);
      } finally {
        setLoading(false);
      }
    };

    fetchUserProfile();
  }, []);

  return (
    <AppLayout>
      <div className="container mx-auto">
        <h2 className="text-3xl font-bold mb-6">My Profile</h2>

        {loading && <LoadingSpinner />}
        {error && <p className="text-center text-red-500">Error: {error}</p>}

        {!loading && !error && userProfile && (
          <div className="bg-gray-800 p-6 rounded-lg shadow-md border border-gray-700">
            <div className="mb-4">
              <p className="text-gray-400 text-sm">Email:</p>
              <p className="text-lg font-mono break-all">{userProfile.email}</p>
            </div>
            <div>
              <p className="text-gray-400 text-sm">Wallet Address:</p>
              <p className="text-lg font-mono break-all">{userProfile.walletAddress}</p>
            </div>
            {/* Add more profile details here */}
          </div>
        )}
      </div>
    </AppLayout>
  );
};

export default Profile;
