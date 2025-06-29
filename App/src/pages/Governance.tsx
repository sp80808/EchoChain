import React, { useState, useEffect } from 'react';
import AppLayout from '../components/AppLayout';
import LoadingSpinner from '../components/LoadingSpinner';

interface Proposal {
  _id: string;
  title: string;
  description: string;
  proposer: { email: string; walletAddress: string };
  status: 'open' | 'approved' | 'rejected';
  votesAye: number;
  votesNay: number;
}

const Governance: React.FC = () => {
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [newProposalTitle, setNewProposalTitle] = useState('');
  const [newProposalDescription, setNewProposalDescription] = useState('');
  const [submittingProposal, setSubmittingProposal] = useState(false);

  useEffect(() => {
    const fetchProposals = async () => {
      setLoading(true);
      setError(null);
      try {
        const response = await fetch('http://localhost:3001/api/samples/governance/proposals');
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data: Proposal[] = await response.json();
        setProposals(data);
      } catch (e: any) {
        setError(e.message);
      } finally {
        setLoading(false);
      }
    };

    fetchProposals();
  }, []);

  const handleSubmitProposal = async (e: React.FormEvent) => {
    e.preventDefault();
    setSubmittingProposal(true);
    setError(null);
    try {
      const token = localStorage.getItem('token');
      const response = await fetch('http://localhost:3001/api/samples/governance/proposals', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-auth-token': token || '',
        },
        body: JSON.stringify({ title: newProposalTitle, description: newProposalDescription }),
      });

      const data = await response.json();

      if (!response.ok) {
        throw new Error(data.msg || 'Failed to submit proposal');
      }

      alert('Proposal submitted successfully!');
      setNewProposalTitle('');
      setNewProposalDescription('');
      // Refresh proposals
      // fetchProposals(); // Would re-fetch from backend
    } catch (err: any) {
      setError(err.message);
    } finally {
      setSubmittingProposal(false);
    }
  };

  const handleVote = async (proposalId: string, vote: 'aye' | 'nay') => {
    try {
      const token = localStorage.getItem('token');
      const response = await fetch(`http://localhost:3001/api/samples/governance/proposals/${proposalId}/vote`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-auth-token': token || '',
        },
        body: JSON.stringify({ vote }),
      });

      const data = await response.json();

      if (!response.ok) {
        throw new Error(data.msg || 'Failed to cast vote');
      }

      alert(`Vote ${vote} cast successfully for proposal ${proposalId}!`);
      // Refresh proposals
      // fetchProposals(); // Would re-fetch from backend
    } catch (err: any) {
      setError(err.message);
    }
  };

  return (
    <AppLayout>
      <div className="container mx-auto">
        <h2 className="text-3xl font-bold mb-6">Governance Proposals</h2>

        {/* Submit New Proposal */}
        <div className="bg-gray-800 p-6 rounded-lg shadow-md border border-gray-700 mb-8">
          <h3 className="text-xl font-semibold mb-4">Submit New Proposal</h3>
          {error && <p className="text-red-500 text-center mb-4 text-sm">{error}</p>}
          <form onSubmit={handleSubmitProposal}>
            <div className="mb-4">
              <label htmlFor="proposalTitle" className="block text-gray-300 text-sm font-bold mb-2">Title</label>
              <input
                type="text"
                id="proposalTitle"
                className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline bg-gray-700 border-gray-600 placeholder-gray-500"
                value={newProposalTitle}
                onChange={(e) => setNewProposalTitle(e.target.value)}
                required
              />
            </div>
            <div className="mb-6">
              <label htmlFor="proposalDescription" className="block text-gray-300 text-sm font-bold mb-2">Description</label>
              <textarea
                id="proposalDescription"
                rows={4}
                className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline bg-gray-700 border-gray-600 placeholder-gray-500"
                value={newProposalDescription}
                onChange={(e) => setNewProposalDescription(e.target.value)}
                required
              ></textarea>
            </div>
            <button
              type="submit"
              className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition-colors duration-200"
              disabled={submittingProposal}
            >
              {submittingProposal ? <LoadingSpinner /> : 'Submit Proposal'}
            </button>
          </form>
        </div>

        {/* Active Proposals */}
        <h3 className="text-xl font-bold mb-4">Active Proposals</h3>
        {loading && <LoadingSpinner />}
        {error && <p className="text-center text-red-500">Error: {error}</p>}

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {!loading && !error && proposals.length === 0 && (
            <p className="col-span-full text-center text-gray-500">No active proposals.</p>
          )}
          {proposals.map((proposal) => (
            <div key={proposal._id} className="bg-gray-800 rounded-lg shadow-md p-4 border border-gray-700">
              <h4 className="text-lg font-semibold mb-2">{proposal.title}</h4>
              <p className="text-gray-400 text-sm mb-3">{proposal.description}</p>
              <p className="text-gray-500 text-xs mb-2">Proposed by: {proposal.proposer.email}</p>
              <div className="flex justify-between items-center text-sm mb-4">
                <span>Votes Aye: <span className="font-bold text-green-400">{proposal.votesAye}</span></span>
                <span>Votes Nay: <span className="font-bold text-red-400">{proposal.votesNay}</span></span>
              </div>
              <div className="flex space-x-2">
                <button
                  className="flex-1 bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded transition-colors duration-200"
                  onClick={() => handleVote(proposal._id, 'aye')}
                >
                  Vote Aye
                </button>
                <button
                  className="flex-1 bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded transition-colors duration-200"
                  onClick={() => handleVote(proposal._id, 'nay')}
                >
                  Vote Nay
                </button>
              </div>
            </div>
          ))}
        </div>
      </div>
    </AppLayout>
  );
};

export default Governance;
