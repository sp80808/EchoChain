import React, { useState, useEffect } from 'react';
import { getUserCommissions, fetchSubmissions, selectSubmission } from '../services/commissionService';
import { useAccount } from '../contexts/AccountContext';

function CommissionSelection() {
  const [commissions, setCommissions] = useState([]);
  const [selectedCommission, setSelectedCommission] = useState(null);
  const [submissions, setSubmissions] = useState([]);
  const [selectedSubmission, setSelectedSubmission] = useState(null);
  const [status, setStatus] = useState('idle'); // idle, loading, selecting, success, error
  const [error, setError] = useState(null);
  const { account, jwt } = useAccount();

  useEffect(() => {
    if (account) {
      loadUserCommissions(account.address);
    }
  }, [account]);

  useEffect(() => {
    if (selectedCommission) {
      loadSubmissions(selectedCommission.id);
    }
  }, [selectedCommission]);

  const loadUserCommissions = async (address) => {
    setStatus('loading');
    try {
      const data = await getUserCommissions(address);
      setCommissions(data);
      setStatus('idle');
    } catch (err) {
      setError(err.response?.data?.message || 'Failed to load your commissions');
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
    setSelectedSubmission(null);
  };

  const handleSubmissionSelect = (submission) => {
    setSelectedSubmission(submission);
  };

  const handleSelectSubmission = async () => {
    if (!selectedCommission || !selectedSubmission) return;
    setStatus('selecting');
    setError(null);
    try {
      await selectSubmission(selectedCommission.id, selectedSubmission.id, jwt, account);
      setStatus('success');
      // Refresh commissions to reflect the closed status
      if (account) {
        loadUserCommissions(account.address);
      }
      setSelectedCommission(null);
      setSelectedSubmission(null);
      setSubmissions([]);
    } catch (err) {
      setError(err.response?.data?.message || 'Failed to select submission');
      setStatus('error');
    }
  };

  return (
    <div style={{ margin: '2em 0' }}>
      <h2>Select Commission Submission</h2>
      {status === 'loading' && <div>Loading...</div>}
      {status === 'error' && <div style={{ color: 'red' }}>{error}</div>}
      {status === 'success' && <div style={{ color: 'green' }}>Submission selected and bounty awarded successfully!</div>}
      
      {!account ? (
        <div style={{ color: 'red' }}>Connect your wallet to view and select submissions for your commissions.</div>
      ) : (
        <div style={{ display: 'flex', flexDirection: 'row', gap: '20px' }}>
          <div style={{ flex: 1, borderRight: '1px solid #ccc', paddingRight: '20px' }}>
            <h3>Your Open Commissions</h3>
            {commissions.length === 0 ? (
              <p>You have no open commissions.</p>
            ) : (
              <ul style={{ listStyleType: 'none', padding: 0 }}>
                {commissions.filter(c => c.status === 'Open').map(commission => (
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
              <h3>Submissions for Commission ID: {selectedCommission.id}</h3>
              {submissions.length === 0 ? (
                <p>No submissions yet for this commission.</p>
              ) : (
                <ul style={{ listStyleType: 'none', padding: 0 }}>
                  {submissions.map(submission => (
                    <li key={submission.id} onClick={() => handleSubmissionSelect(submission)} style={{ cursor: 'pointer', padding: '5px', border: selectedSubmission?.id === submission.id ? '2px solid green' : '1px solid #ddd', marginBottom: '5px' }}>
                      <strong>ID: {submission.id}</strong> - Submitter: {submission.submitter}
                      <p>Content Hash: {submission.contentHash}</p>
                    </li>
                  ))}
                </ul>
              )}
              {selectedSubmission && (
                <button onClick={handleSelectSubmission} disabled={status === 'selecting'}>Award Bounty to Submission ID: {selectedSubmission.id}</button>
              )}
            </div>
          )}
        </div>
      )}
    </div>
  );
}

export default CommissionSelection;
