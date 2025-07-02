import React, { useState } from 'react';
import { registerSample } from '../services/sampleService';
import { useAccount } from '../contexts/AccountContext';
import { useNotification } from '../contexts/NotificationContext';
import LoadingSpinner from './LoadingSpinner';
import { TextField, Button, Typography, Box } from '@mui/material';

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
  const { account, jwt } = useAccount();
  const { showSuccess, showError, showInfo, showBlockchainTransaction } = useNotification();

  const handleChange = e => {
    setForm({ ...form, [e.target.name]: e.target.value });
  };

  const handleSubmit = async e => {
    e.preventDefault();
    setStatus('submitting');
    
    showInfo('Starting sample registration...', { persist: true });
    
    try {
      const result = await registerSample({
        ...form,
        price: Number(form.price),
        ownerAddress: account ? account.address : '',
      }, jwt, account);
      
      setStatus('success');
      
      if (result && result.blockchainHash) {
        showBlockchainTransaction(
          'Sample registered successfully on blockchain!',
          result.blockchainHash
        );
      } else {
        showSuccess('Sample registered successfully!');
      }
      
      // Reset form
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
      const errorMessage = err.response?.data?.message || err.message || 'Registration failed. Please check your connection and try again.';
      setStatus('error');
      showError(`Registration failed: ${errorMessage}`);
    }
  };

  return (
    <Box sx={{ mt: 4 }}>
      <Typography variant="h5" gutterBottom>Register Sample</Typography>
      <form onSubmit={handleSubmit}>
        <TextField
          name="id"
          label="ID"
          value={form.id}
          onChange={handleChange}
          required
          fullWidth
          margin="normal"
        />
        <TextField
          name="title"
          label="Title"
          value={form.title}
          onChange={handleChange}
          required
          fullWidth
          margin="normal"
        />
        <TextField
          name="artist"
          label="Artist"
          value={form.artist}
          onChange={handleChange}
          required
          fullWidth
          margin="normal"
        />
        <TextField
          name="duration"
          label="Duration (seconds)"
          value={form.duration}
          onChange={handleChange}
          required
          fullWidth
          margin="normal"
        />
        <TextField
          name="category"
          label="Category"
          value={form.category}
          onChange={handleChange}
          required
          fullWidth
          margin="normal"
        />
        <TextField
          name="p2pContentId"
          label="P2P Content ID"
          value={form.p2pContentId}
          onChange={handleChange}
          required
          fullWidth
          margin="normal"
        />
        <TextField
          name="price"
          label="Price"
          type="number"
          value={form.price}
          onChange={handleChange}
          required
          fullWidth
          margin="normal"
        />
        <TextField
          name="blockchainHash"
          label="Blockchain Hash"
          value={form.blockchainHash}
          onChange={handleChange}
          required
          fullWidth
          margin="normal"
        />
        <TextField
          name="ownerAddress"
          label="Owner Address"
          value={account ? account.address : ''}
          InputProps={{
            readOnly: true,
          }}
          disabled
          fullWidth
          margin="normal"
          sx={{ backgroundColor: '#f5f5f5' }}
        />
        <Button
          type="submit"
          variant="contained"
          color="primary"
          disabled={status === 'submitting' || !account}
          sx={{ mt: 2 }}
        >
          Register
        </Button>
      </form>
      {!account && (
        <Typography variant="body2" color="error" sx={{ mt: 2 }}>
          Connect your wallet to register a sample.
        </Typography>
      )}
      {status === 'submitting' && (
        <Box sx={{ display: 'flex', alignItems: 'center', mt: 2 }}>
          <LoadingSpinner />
          <Typography variant="body1" sx={{ ml: 2 }}>
            Registering sample on-chain. Please wait, this may take a few moments...
          </Typography>
        </Box>
      )}
    </Box>
  );
}

export default SampleUpload;
