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
import './App.css';
import { AccountProvider } from './contexts/AccountContext';

function App() {
  return (
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
            </Routes>
          </main>
        </div>
      </Router>
    </AccountProvider>
  );
}

export default App;
