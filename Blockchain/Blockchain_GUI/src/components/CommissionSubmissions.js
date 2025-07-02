import React, { useState, useEffect } from 'react';
import { fetchCommissions, submitForCommission, fetchSubmissions } from '../services/commissionService';
import { useAccount } from '../contexts/AccountContext';

function CommissionSubmissions() {
  const [commissions, setCommissions] = useState([]);
  const [selectedCommission, setSelectedCommission] = useState(null);
  const [submissions, setSubmissions] = useState([]);
  const [submissionForm, setSubmissionForm] = useState({
    id: '',
    contentHash: ''
  });
  const [status, setStatus] = useState('idle'); // idle, loading, submitting, success, error
  const [error, setError] = useState(null);
  const { account, jwt } = useAccount();

  useEffect(() => {
    loadCommissions();
  }, []);

  useEffect(() => {
    if (selectedCommission) {
      loadSubmissions(selectedCommission.id);
    }
  }, [selectedCommission]);

  const loadCommissions = async () => {
    setStatus('loading');
    try {
      const data = await fetchCommissions({ status: 'Open' });
      setCommissions(data);
      setStatus('idle');
    } catch (err) {
      setError(err.response?.data?.message || 'Failed to load commissions');
      setStatus('error');
    }
  };

  const loadSubmissions = async (commissionId) => {
    setStatus('loading');
    try {
      const data = await fetchSubmissions(commissionId);
      setSubmissions(data);
      setStatus('idle');
    } catch (err) {
      setError(err.response?.data?.message || 'Failed to load submissions');
      setStatus('error');
    }
  };

  const handleCommissionSelect = (commission) => {
    setSelectedCommission(commission);
  };

  const handleSubmissionChange = (e) => {
    setSubmissionForm({ ...submissionForm, [e.target.name]: e.target.value });
  };

  const handleSubmissionSubmit = async (e) => {
    e.preventDefault();
    if (!selectedCommission) return;
    setStatus('submitting');
    setError(null);
    try {
      await submitForCommission(selectedCommission.id, {
        ...submissionForm
      }, jwt, account);
      setStatus('success');
      setSubmissionForm({
        id: '',
        contentHash: ''
      });
      loadSubmissions(selectedCommission.id);
    } catch (err) {
      setError(err.response?.data?.message || 'Submission failed');
      setStatus('error');
    }
  };

  return (
    <div style={{ margin: '2em 0' }}>
      <h2>Commission Submissions</h2>
      {status === 'loading' && <div>Loading...</div>}
      {status === 'error' && <div style={{ color: 'red' }}>{error}</div>}
      {status === 'success' && <div style={{ color: 'green' }}>Submission successful!</div>}
      
      <div style={{ display: 'flex', flexDirection: 'row', gap: '20px' }}>
        <div style={{ flex: 1, borderRight: '1px solid #ccc', paddingRight: '20px' }}>
          <h3>Open Commissions</h3>
          {commissions.length === 0 ? (
            <p>No open commissions available.</p>
          ) : (
            <ul style={{ listStyleType: 'none', padding: 0 }}>
              {commissions.map(commission => (
                <li key={commission.id} onClick={() => handleCommissionSelect(commission)} style={{ cursor: 'pointer', padding: '10px', border: selectedCommission?.id === commission.id ? '2px solid blue' : '1px solid #ddd', marginBottom: '5px' }}>
                  <strong>ID: {commission.id}</strong> - Bounty: {commission.bounty} ECHO
                  <p>{commission.description}</p>
                </li>
              ))}
            </ul>
          )}
        </div>
        
        {selectedCommission && (
          <div style={{ flex: 1 }}>
            <h3>Submit for Commission ID: {selectedCommission.id}</h3>
            <form onSubmit={handleSubmissionSubmit}>
              <input name="id" placeholder="Submission ID" value={submissionForm.id} onChange={handleSubmissionChange} required /> <br />
              <input name="contentHash" placeholder="Content Hash" value={submissionForm.contentHash} onChange={handleSubmissionChange} required /> <br />
              <button type="submit" disabled={status === 'submitting' || !account}>Submit</button>
            </form>
            {!account && <div style={{ color: 'red' }}>Connect your wallet to submit for a commission.</div>}
            
            <h4>Submissions for this Commission</h4>
            {submissions.length === 0 ? (
              <p>No submissions yet.</p>
            ) : (
              <ul style={{ listStyleType: 'none', padding: 0 }}>
                {submissions.map(submission => (
                  <li key={submission.id} style={{ padding: '5px', border: '1px solid #ddd', marginBottom: '5px' }}>
                    <strong>ID: {submission.id}</strong> - Submitter: {submission.submitter}
                    <p>Content Hash: {submission.contentHash}</p>
                  </li>
                ))}
              </ul>
            )}
          </div>
        )}
      </div>
    </div>
  );
}

export default CommissionSubmissions;
