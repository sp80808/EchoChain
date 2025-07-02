import React from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import Home from './components/Home';
import Settings from './components/Settings';
import Network from './components/Network';
import Transfers from './components/Transfers';
import Tests from './components/Tests';
import Faucet from './components/Faucet';
import AccountStatus from './components/AccountStatus';
import SampleUpload from './components/SampleUpload';
import UserSamples from './components/UserSamples';
import CommissionPost from './components/CommissionPost';
import CommissionSubmissions from './components/CommissionSubmissions';
import CommissionSelection from './components/CommissionSelection';
import './App.css';
import { AccountProvider } from './contexts/AccountContext';
import { SnackbarProvider } from 'notistack';

function App() {
  return (
    <SnackbarProvider maxSnack={3} anchorOrigin={{ vertical: 'top', horizontal: 'right' }}>
      <AccountProvider>
        <Router>
          <div className="App">
            <header className="App-header">
              <h1>Echochain Blockchain Management GUI</h1>
              <AccountStatus />
              <nav>
                <ul>
                  <li><Link to="/">Home</Link></li>
                  <li><Link to="/settings">Settings</Link></li>
                  <li><Link to="/network">Network</Link></li>
                  <li><Link to="/transfers">Transfers</Link></li>
                  <li><Link to="/tests">Tests</Link></li>
                  <li><Link to="/faucet">Faucet</Link></li>
                  <li><Link to="/register-sample">Register Sample</Link></li>
                  <li><Link to="/my-samples">My Samples</Link></li>
                  <li><Link to="/post-commission">Post Commission</Link></li>
                  <li><Link to="/commission-submissions">Commission Submissions</Link></li>
                  <li><Link to="/commission-selection">Select Submission</Link></li>
                </ul>
              </nav>
            </header>
            <main>
              <Routes>
                <Route path="/" element={<Home />} />
                <Route path="/settings" element={<Settings />} />
                <Route path="/network" element={<Network />} />
                <Route path="/transfers" element={<Transfers />} />
                <Route path="/tests" element={<Tests />} />
                <Route path="/faucet" element={<Faucet />} />
                <Route path="/register-sample" element={<SampleUpload />} />
                <Route path="/my-samples" element={<UserSamples />} />
                <Route path="/post-commission" element={<CommissionPost />} />
                <Route path="/commission-submissions" element={<CommissionSubmissions />} />
                <Route path="/commission-selection" element={<CommissionSelection />} />
              </Routes>
            </main>
          </div>
        </Router>
      </AccountProvider>
    </SnackbarProvider>
  );
}

export default App;
