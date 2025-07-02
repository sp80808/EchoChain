import React, { useState } from 'react';
import { postCommission } from '../services/commissionService';
import { useAccount } from '../contexts/AccountContext';

function CommissionPost() {
  const [form, setForm] = useState({
    id: '',
    bounty: '',
    description: '',
    durationBlocks: ''
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
      await postCommission({
        ...form,
        bounty: Number(form.bounty),
        durationBlocks: form.durationBlocks ? Number(form.durationBlocks) : '',
      }, jwt, account);
      setStatus('success');
      setForm({
        id: '',
        bounty: '',
        description: '',
        durationBlocks: ''
      });
    } catch (err) {
      setError(err.response?.data?.message || 'Commission posting failed');
      setStatus('error');
    }
  };

  return (
    <div style={{ margin: '2em 0' }}>
      <h2>Post Commission</h2>
      <form onSubmit={handleSubmit}>
        <input name="id" placeholder="Commission ID" value={form.id} onChange={handleChange} required /> <br />
        <input name="bounty" placeholder="Bounty (in ECHO tokens)" type="number" value={form.bounty} onChange={handleChange} required /> <br />
        <textarea name="description" placeholder="Description of the audio piece needed" value={form.description} onChange={handleChange} required /> <br />
        <input name="durationBlocks" placeholder="Duration Blocks (optional)" type="number" value={form.durationBlocks} onChange={handleChange} /> <br />
        <button type="submit" disabled={status === 'submitting' || !account}>Post Commission</button>
      </form>
      {!account && <div style={{ color: 'red' }}>Connect your wallet to post a commission.</div>}
      {status === 'success' && <div style={{ color: 'green' }}>Commission posted successfully!</div>}
      {status === 'error' && <div style={{ color: 'red' }}>{error}</div>}
    </div>
  );
}

export default CommissionPost;
