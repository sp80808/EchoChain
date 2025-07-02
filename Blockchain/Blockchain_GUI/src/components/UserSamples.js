import React, { useState, useEffect, useContext } from 'react';
import { AccountContext } from '../contexts/AccountContext';
import { getUserSamples } from '../services/sampleService';

const UserSamples = () => {
  const { account } = useContext(AccountContext);
  const [samples, setSamples] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchSamples = async () => {
      if (!account) {
        setLoading(false);
        return;
      }

      try {
        setLoading(true);
        const userSamples = await getUserSamples(account.address);
        setSamples(userSamples);
        setError(null);
      } catch (err) {
        setError('Failed to load samples. Please try again later.');
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    fetchSamples();
  }, [account]);

  if (!account) {
    return <div>Please connect your account to view your samples.</div>;
  }

  if (loading) {
    return <div>Loading samples...</div>;
  }

  if (error) {
    return <div style={{ color: 'red' }}>{error}</div>;
  }

  return (
    <div>
      <h2>My Samples</h2>
      {samples.length === 0 ? (
        <p>No samples registered yet.</p>
      ) : (
        <ul style={{ listStyleType: 'none', padding: 0 }}>
          {samples.map((sample, index) => (
            <li key={index} style={{ marginBottom: '10px', border: '1px solid #ccc', padding: '10px', borderRadius: '5px' }}>
              <strong>Sample ID:</strong> {sample.id}<br />
              <strong>Description:</strong> {sample.description}<br />
              <strong>Hash:</strong> {sample.hash}<br />
              <strong>Registered On:</strong> {new Date(sample.registrationDate).toLocaleString()}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default UserSamples;
