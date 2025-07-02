import React, { useState } from 'react';
import { registerSample } from '../services/sampleService';
import { useAccount } from '../contexts/AccountContext';
import LoadingSpinner from './LoadingSpinner';

function SampleUpload() {
  const [form, setForm] = useState({
    id: '',
    title: '',
    artist: '',
    duration: '',
    category: '',
    p2pContentId: '',
    price: '',
    blockchainHash: ''
  });
  const [status, setStatus] = useState('idle'); // idle, submitting, success, error
  const [error, setError] = useState(null);
  const { account, jwt } = useAccount();

  const handleChange = e => {
    setForm({ ...form, [e.target.name]: e.target.value });
  };

  const handleSubmit = async e => {
    e.preventDefault();
    setStatus('submitting');
    setError(null);
    try {
      await registerSample({
        ...form,
        price: Number(form.price),
        ownerAddress: account ? account.address : '',
      }, jwt, account);
      setStatus('success');
      setForm({
        id: '',
        title: '',
        artist: '',
        duration: '',
        category: '',
        p2pContentId: '',
        price: '',
        blockchainHash: ''
      });
    } catch (err) {
      setError(err.response?.data?.message || err.message || 'Registration failed. Please check your connection and try again.');
      setStatus('error');
    }
  };

  return (
    <div style={{ margin: '2em 0' }}>
      <h2>Register Sample</h2>
      <form onSubmit={handleSubmit}>
        <input name="id" placeholder="ID" value={form.id} onChange={handleChange} required /> <br />
        <input name="title" placeholder="Title" value={form.title} onChange={handleChange} required /> <br />
        <input name="artist" placeholder="Artist" value={form.artist} onChange={handleChange} required /> <br />
        <input name="duration" placeholder="Duration (seconds)" value={form.duration} onChange={handleChange} required /> <br />
        <input name="category" placeholder="Category" value={form.category} onChange={handleChange} required /> <br />
        <input name="p2pContentId" placeholder="P2P Content ID" value={form.p2pContentId} onChange={handleChange} required /> <br />
        <input name="price" placeholder="Price" type="number" value={form.price} onChange={handleChange} required /> <br />
        <input name="blockchainHash" placeholder="Blockchain Hash" value={form.blockchainHash} onChange={handleChange} required /> <br />
        <input name="ownerAddress" value={account ? account.address : ''} readOnly disabled style={{ background: '#eee' }} /> <br />
        <button type="submit" disabled={status === 'submitting' || !account}>Register</button>
      </form>
      {!account && <div style={{ color: 'red' }}>Connect your wallet to register a sample.</div>}
      {status === 'submitting' && <><LoadingSpinner /><div>Registering sample on-chain. Please wait, this may take a few moments...</div></>}
      {status === 'success' && <div style={{ color: 'green' }}>Sample registered successfully! (Backend & Blockchain confirmed)</div>}
      {status === 'error' && <div style={{ color: 'red' }}>{error}</div>}
    </div>
  );
}

export default SampleUpload;
