import React, { useState, useEffect } from 'react';
import AppLayout from '../components/AppLayout';

interface Sample {
  _id: string;
  title: string;
  description: string;
  category: string;
  tags: string[];
  bpm?: number;
  key?: string;
  creator: { email: string; walletAddress: string };
  ipfsCid: string;
}

const MyLibrary: React.FC = () => {
  const [samples, setSamples] = useState<Sample[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchMySamples = async () => {
      setLoading(true);
      setError(null);
      try {
        const token = localStorage.getItem('token');
        if (!token) {
          setError('User not authenticated.');
          setLoading(false);
          return;
        }

        const response = await fetch(`http://localhost:3001/api/samples/my`, {
          headers: {
            'x-auth-token': token,
          },
        });

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data: Sample[] = await response.json();
        setSamples(data);
      } catch (e: any) {
        setError(e.message);
      } finally {
        setLoading(false);
      }
    };

    fetchMySamples();
  }, []);

  return (
    <AppLayout>
      <div className="container mx-auto">
        <h2 className="text-3xl font-bold mb-6">My Library</h2>

        {loading && <p className="text-center text-blue-400">Loading your samples...</p>}
        {error && <p className="text-center text-red-500">Error: {error}</p>}

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {!loading && !error && samples.length === 0 && (
            <p className="col-span-full text-center text-gray-500">You haven't uploaded any samples yet.</p>
          )}
          {samples.map((sample) => (
            <div key={sample._id} className="bg-gray-800 rounded-lg shadow-md p-4">
              <h3 className="text-xl font-semibold mb-2">{sample.title}</h3>
              <p className="text-gray-400 text-sm mb-2">By: {sample.creator.email}</p>
              <p className="text-gray-300 text-sm mb-3">{sample.description}</p>
              <div className="flex flex-wrap gap-2 mb-3">
                <span className="bg-blue-600 text-white text-xs px-2 py-1 rounded-full">{sample.category}</span>
                {sample.tags.map((tag, index) => (
                  <span key={index} className="bg-gray-700 text-gray-300 text-xs px-2 py-1 rounded-full">{tag}</span>
                ))}
                {sample.bpm && <span className="bg-purple-600 text-white text-xs px-2 py-1 rounded-full">BPM: {sample.bpm}</span>}
                {sample.key && <span className="bg-green-600 text-white text-xs px-2 py-1 rounded-full">Key: {sample.key}</span>}
              </div>
              {/* Placeholder for audio waveform and play button */}
              <div className="bg-gray-700 h-16 rounded-lg mb-3 flex items-center justify-center">
                <p className="text-gray-500">Audio Waveform Preview</p>
              </div>
              <button
                className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded w-full"
                onClick={() => alert(`Downloading ${sample.title}`)}
              >
                Download
              </button>
            </div>
          ))}
        </div>
      </div>
    </AppLayout>
  );
};

export default MyLibrary;
