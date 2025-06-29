import React, { useState, useEffect, useRef } from 'react';
import AppLayout from '../components/AppLayout';
import LoadingSpinner from '../components/LoadingSpinner';
import TipModal from '../components/TipModal';
import WaveSurfer from 'wavesurfer.js';

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

const SampleBrowser: React.FC = () => {
  const [samples, setSamples] = useState<Sample[]>([]);
  const [featuredSamples, setFeaturedSamples] = useState<Sample[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [categoryFilter, setCategoryFilter] = useState<string>('');
  const [tagFilter, setTagFilter] = useState<string>('');
  const [isTipModalOpen, setIsTipModalOpen] = useState(false);
  const [selectedCreatorEmail, setSelectedCreatorEmail] = useState('');
  const [selectedSampleId, setSelectedSampleId] = useState('');
  const wavesurferInstances = useRef<{ [key: string]: WaveSurfer }>({});

  useEffect(() => {
    const fetchSamples = async () => {
      setLoading(true);
      setError(null);
      try {
        const params = new URLSearchParams();
        if (searchQuery) params.append('search', searchQuery);
        if (categoryFilter) params.append('category', categoryFilter);
        if (tagFilter) params.append('tags', tagFilter);

        const response = await fetch(`http://localhost:3001/api/samples?${params.toString()}`, {
          headers: {
            'x-auth-token': localStorage.getItem('token') || '',
          },
        });

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data: Sample[] = await response.json();
        setSamples(data);

        // Simulate featured samples (e.g., first 3 samples)
        setFeaturedSamples(data.slice(0, 3));

      } catch (e: any) {
        setError(e.message);
      } finally {
        setLoading(false);
      }
    };

    fetchSamples();
  }, [searchQuery, categoryFilter, tagFilter]);

  useEffect(() => {
    samples.forEach(sample => {
      if (!wavesurferInstances.current[sample._id]) {
        const wavesurfer = WaveSurfer.create({
          container: `#waveform-${sample._id}`,
          waveColor: '#4F46E5',
          progressColor: '#3B82F6',
          cursorColor: '#fff',
          barWidth: 2,
          height: 60,
          responsive: true,
          hideScrollbar: true,
        });

        // Assuming the backend provides a direct URL to the audio file for preview
        // For now, we'll use a placeholder or a direct IPFS gateway link if available
        // In a real scenario, you might have a dedicated endpoint to stream audio.
        wavesurfer.load(`http://localhost:3001/api/samples/${sample._id}/download`);

        wavesurferInstances.current[sample._id] = wavesurfer;

        // Add play/pause functionality
        const playButton = document.getElementById(`play-button-${sample._id}`);
        if (playButton) {
          playButton.onclick = () => {
            wavesurfer.playPause();
          };
        }
      }
    });

    return () => {
      // Destroy wavesurfer instances on unmount
      Object.values(wavesurferInstances.current).forEach(ws => ws.destroy());
      wavesurferInstances.current = {};
    };
  }, [samples]);

  const handleTip = (amount: number) => {
    console.log(`Tipping ${amount} ECHO to ${selectedCreatorEmail}`);
    // TODO: Integrate with backend API for tipping (blockchain interaction)
    alert(`Successfully tipped ${amount} ECHO to ${selectedCreatorEmail}! (Simulated)`);
  };

  return (
    <AppLayout>
      <div className="container mx-auto">
        <h2 className="text-3xl font-bold mb-6">Explore Samples</h2>

        {/* Featured Samples */}
        <h3 className="text-2xl font-bold mb-4">Featured Samples</h3>
        <div className="flex overflow-x-auto space-x-4 pb-4 mb-8">
          {featuredSamples.map((sample) => (
            <div key={sample._id} className="flex-none w-64 bg-gray-800 rounded-lg shadow-md p-4 border border-gray-700">
              <h4 className="text-lg font-semibold mb-1">{sample.title}</h4>
              <p className="text-gray-400 text-sm mb-2">By: {sample.creator.email}</p>
              {/* Add more details or a smaller waveform here if desired */}
              <button
                className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-1 px-3 rounded text-sm w-full"
                onClick={() => alert(`Playing featured sample: ${sample.title}`)}
              >
                Listen
              </button>
            </div>
          ))}
        </div>

        {/* Search and Filter */}
        <div className="mb-6 flex space-x-4">
          <input
            type="text"
            placeholder="Search samples..."
            className="flex-1 p-3 rounded-lg bg-gray-800 border border-gray-700 text-white focus:outline-none focus:border-blue-500 placeholder-gray-500"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
          <select
            className="p-3 rounded-lg bg-gray-800 border border-gray-700 text-white focus:outline-none focus:border-blue-500"
            value={categoryFilter}
            onChange={(e) => setCategoryFilter(e.target.value)}
          >
            <option value="">All Categories</option>
            <option value="Drums">Drums</option>
            <option value="Vocals">Vocals</option>
            <option value="Synths">Synths</option>
            <option value="FX">FX</option>
            {/* Add more categories as needed */}
          </select>
          <input
            type="text"
            placeholder="Filter by tags (comma-separated)"
            className="flex-1 p-3 rounded-lg bg-gray-800 border border-gray-700 text-white focus:outline-none focus:border-blue-500 placeholder-gray-500"
            value={tagFilter}
            onChange={(e) => setTagFilter(e.target.value)}
          />
        </div>

        {loading && <LoadingSpinner />}
        {error && <p className="text-center text-red-500">Error: {error}</p>}

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {!loading && !error && samples.length === 0 && (
            <p className="col-span-full text-center text-gray-500">No samples found.</p>
          )}
          {samples.map((sample) => (
            <div key={sample._id} className="bg-gray-800 rounded-lg shadow-md p-4 border border-gray-700">
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
              {/* Audio Waveform and Play Button */}
              <div className="flex items-center justify-between mt-3">
                <div id={`waveform-${sample._id}`} className="w-3/4 h-16 bg-gray-700 rounded-lg"></div>
                <button
                  id={`play-button-${sample._id}`}
                  className="ml-2 bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
                  onClick={() => wavesurferInstances.current[sample._id]?.playPause()}
                >
                  Play
                </button>
              </div>
              <button
                className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded w-full mt-3 transition-colors duration-200"
                onClick={async () => {
                  try {
                    const token = localStorage.getItem('token');
                    const response = await fetch(`http://localhost:3001/api/samples/${sample._id}/download`, {
                      headers: {
                        'x-auth-token': token || '',
                      },
                    });

                    if (!response.ok) {
                      throw new Error(`HTTP error! status: ${response.status}`);
                    }

                    const blob = await response.blob();
                    const url = window.URL.createObjectURL(blob);
                    const a = document.createElement('a');
                    a.href = url;
                    a.download = `${sample.title}.mp3`; // Assuming MP3, adjust as needed
                    document.body.appendChild(a);
                    a.click();
                    a.remove();
                    window.URL.revokeObjectURL(url);
                  } catch (err: any) {
                    alert(`Failed to download sample: ${err.message}`);
                  }
                }}
              >
                Download
              </button>
              <button
                className="bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded w-full mt-2 transition-colors duration-200"
                onClick={() => {
                  setSelectedCreatorEmail(sample.creator.email);
                  setSelectedSampleId(sample._id);
                  setIsTipModalOpen(true);
                }}
              >
                Tip Creator
              </button>
            </div>
          ))}
        </div>
      </div>
      <TipModal
        isOpen={isTipModalOpen}
        onClose={() => setIsTipModalOpen(false)}
        sampleId={selectedSampleId}
        creatorEmail={selectedCreatorEmail}
        onTipSuccess={() => alert('Tip sent successfully!')}
      />
    </AppLayout>
  );
};

export default SampleBrowser;
