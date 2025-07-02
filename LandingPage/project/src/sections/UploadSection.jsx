// src/sections/UploadSection.jsx

import React, { useState } from 'react';
import Input from '../components/Input';
import Button from '../components/Button';
import { usePolkadot } from '../hooks/usePolkadot';
import { useIPFS } from '../hooks/useIPFS';

const UploadSection = () => {
  const { api, selectedAccount, signAndSend } = usePolkadot();
  const { isIpfsReady, ipfsError, uploadFileToIPFS, uploadJsonToIPFS } = useIPFS();

  const [audioFile, setAudioFile] = useState(null);
  const [title, setTitle] = useState('');
  const [artist, setArtist] = useState('');
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');
  const [error, setError] = useState('');

  const handleFileChange = (e) => {
    setAudioFile(e.target.files[0]);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    setMessage('');
    setError('');

    if (!api || !selectedAccount) {
      setError('Please connect wallet.');
      setLoading(false);
      return;
    }
    if (!isIpfsReady) {
      setError(ipfsError || 'IPFS daemon not ready.');
      setLoading(false);
      return;
    }
    if (!audioFile || !title || !artist) {
      setError('Please select an audio file and fill all fields.');
      setLoading(false);
      return;
    }

    try {
      // 1. Upload audio to IPFS
      setMessage('Uploading audio to IPFS...');
      const audioCid = await uploadFileToIPFS(audioFile);

      // 2. Upload metadata to IPFS
      setMessage('Uploading metadata to IPFS...');
      const metadata = { title, artist, audioCid, timestamp: Date.now() };
      const metadataCid = await uploadJsonToIPFS(metadata);

      // 3. Construct and send blockchain transaction
      setMessage('Registering sample on EchoChain...');
      const extrinsic = api.tx.sampleRegistry.registerSample(audioCid, metadataCid);
      await signAndSend(extrinsic);

      setMessage('Sample uploaded and registered successfully!');
      setAudioFile(null);
      setTitle('');
      setArtist('');
    } catch (err) {
      console.error('Upload error:', err);
      setError(`Upload failed: ${err.message}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="bg-gray-800 p-6 rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4 text-white">Upload New Sample</h2>
      <form onSubmit={handleSubmit}>
        <div className="mb-4">
          <label className="block text-gray-300 text-sm font-bold mb-2">Audio File:</label>
          <Input type="file" accept="audio/*" onChange={handleFileChange} />
          {audioFile && <p className="text-gray-400 text-sm mt-1">Selected: {audioFile.name}</p>}
        </div>
        <div className="mb-4">
          <label className="block text-gray-300 text-sm font-bold mb-2">Title:</label>
          <Input type="text" value={title} onChange={(e) => setTitle(e.target.value)} placeholder="Sample Title" />
        </div>
        <div className="mb-4">
          <label className="block text-gray-300 text-sm font-bold mb-2">Artist:</label>
          <Input type="text" value={artist} onChange={(e) => setArtist(e.target.value)} placeholder="Artist Name" />
        </div>
        <Button type="submit" disabled={loading || !selectedAccount || !isIpfsReady}>
          {loading ? 'Uploading...' : 'Upload Sample'}
        </Button>
        {message && <p className="text-green-500 mt-4">{message}</p>}
        {error && <p className="text-red-500 mt-4">{error}</p>}
        {!selectedAccount && <p className="text-yellow-500 mt-4">Please connect your wallet to upload samples.</p>}
        {!isIpfsReady && <p className="text-yellow-500 mt-4">IPFS daemon not connected. {ipfsError}</p>}
      </form>
    </div>
  );
};

export default UploadSection;
