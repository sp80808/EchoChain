import React, { useState } from 'react';
import AppLayout from '../components/AppLayout';

const SampleUpload: React.FC = () => {
  const [file, setFile] = useState<File | null>(null);
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [category, setCategory] = useState('');
  const [tags, setTags] = useState('');
  const [bpm, setBpm] = useState('');
  const [key, setKey] = useState('');
  const [separateStems, setSeparateStems] = useState(false);
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      setFile(e.target.files[0]);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setMessage(null);
    setError(null);

    if (!file) {
      setError('Please select a file to upload.');
      setLoading(false);
      return;
    }

    const formData = new FormData();
    formData.append('sample', file);
    formData.append('title', title);
    formData.append('description', description);
    formData.append('category', category);
    formData.append('tags', tags);
    if (bpm) formData.append('bpm', bpm);
    if (key) formData.append('key', key);
    formData.append('separateStems', separateStems ? 'true' : 'false');

    try {
      const response = await fetch('http://localhost:3001/api/samples', {
        method: 'POST',
        headers: {
          'x-auth-token': localStorage.getItem('token') || '',
        },
        body: formData,
      });

      const data = await response.json();

      if (!response.ok) {
        throw new Error(data.msg || 'Upload failed');
      }

      setMessage('Sample uploaded successfully!');
      // Clear form
      setFile(null);
      setTitle('');
      setDescription('');
      setCategory('');
      setTags('');
      setBpm('');
      setKey('');
      setSeparateStems(false);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <AppLayout>
      <div className="container mx-auto">
        <h2 className="text-3xl font-bold mb-6">Upload New Sample</h2>

        {message && <p className="text-green-500 text-center mb-4">{message}</p>}
        {error && <p className="text-red-500 text-center mb-4">{error}</p>}

        <form onSubmit={handleSubmit} className="bg-gray-800 p-6 rounded-lg shadow-md">
          <div className="mb-4">
            <label htmlFor="file" className="block text-gray-300 text-sm font-bold mb-2">Audio File</label>
            <input
              type="file"
              id="file"
              accept="audio/*"
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline bg-gray-700 border-gray-600"
              onChange={handleFileChange}
              required
            />
          </div>

          <div className="mb-4">
            <label htmlFor="title" className="block text-gray-300 text-sm font-bold mb-2">Title</label>
            <input
              type="text"
              id="title"
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline bg-gray-700 border-gray-600"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              required
            />
          </div>

          <div className="mb-4">
            <label htmlFor="description" className="block text-gray-300 text-sm font-bold mb-2">Description</label>
            <textarea
              id="description"
              rows={3}
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline bg-gray-700 border-gray-600"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              required
            ></textarea>
          </div>

          <div className="mb-4">
            <label htmlFor="category" className="block text-gray-300 text-sm font-bold mb-2">Category</label>
            <select
              id="category"
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline bg-gray-700 border-gray-600"
              value={category}
              onChange={(e) => setCategory(e.target.value)}
              required
            >
              <option value="">Select Category</option>
              <option value="Drums">Drums</option>
              <option value="Vocals">Vocals</option>
              <option value="Synths">Synths</option>
              <option value="FX">FX</option>
              {/* Add more categories as needed */}
            </select>
          </div>

          <div className="mb-4">
            <label htmlFor="tags" className="block text-gray-300 text-sm font-bold mb-2">Tags (comma-separated)</label>
            <input
              type="text"
              id="tags"
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline bg-gray-700 border-gray-600"
              value={tags}
              onChange={(e) => setTags(e.target.value)}
              required
            />
          </div>

          <div className="mb-4">
            <label htmlFor="bpm" className="block text-gray-300 text-sm font-bold mb-2">BPM (Optional, auto-detected if empty)</label>
            <input
              type="number"
              id="bpm"
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline bg-gray-700 border-gray-600"
              value={bpm}
              onChange={(e) => setBpm(e.target.value)}
            />
          </div>

          <div className="mb-4">
            <label htmlFor="key" className="block text-gray-300 text-sm font-bold mb-2">Key (Optional, auto-detected if empty)</label>
            <input
              type="text"
              id="key"
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline bg-gray-700 border-gray-600"
              value={key}
              onChange={(e) => setKey(e.target.value)}
            />
          </div>

          <div className="mb-6 flex items-center">
            <input
              type="checkbox"
              id="separateStems"
              className="mr-2 leading-tight"
              checked={separateStems}
              onChange={(e) => setSeparateStems(e.target.checked)}
            />
            <label htmlFor="separateStems" className="text-gray-300 text-sm font-bold">Separate Stems (AI-powered)</label>
          </div>

          <div className="flex items-center justify-between">
            <button
              type="submit"
              className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline w-full"
              disabled={loading}
            >
              {loading ? 'Uploading...' : 'Upload Sample'}
            </button>
          </div>
        </form>
      </div>
    </AppLayout>
  );
};

export default SampleUpload;
