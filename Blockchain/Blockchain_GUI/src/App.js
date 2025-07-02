import React, { useState } from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
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
import NodeConnection from './components/NodeConnection'; // Import NodeConnection
import './App.css';
import { AccountProvider } from './contexts/AccountContext';
import { SnackbarProvider } from 'notistack';
import { NotificationProvider } from './contexts/NotificationContext';

function App() {
  const [api, setApi] = useState(null); // State to hold the Polkadot API instance

  const theme = createTheme({
    palette: {
      primary: {
        main: '#1976d2',
      },
      secondary: {
        main: '#dc004e',
      },
    },
    typography: {
      fontFamily: 'Roboto, -apple-system, BlinkMacSystemFont, "Segoe UI", Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif',
    },
  });

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <SnackbarProvider maxSnack={3} anchorOrigin={{ vertical: 'top', horizontal: 'right' }}>
        <NotificationProvider>
          <AccountProvider>
            <Router>
              <div className="App">
              <header className="App-header">
                <h1>Echochain Blockchain Management GUI</h1>
                <AccountStatus />
                <NodeConnection onConnect={setApi} /> {/* Add NodeConnection component */}
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
                  <Route path="/" element={<Home api={api} />} /> {/* Pass api to Home */}
                  <Route path="/settings" element={<Settings api={api} />} />
                  <Route path="/network" element={<Network api={api} />} />
                  <Route path="/transfers" element={<Transfers api={api} />} />
                  <Route path="/tests" element={<Tests api={api} />} />
                  <Route path="/faucet" element={<Faucet api={api} />} />
                  <Route path="/register-sample" element={<SampleUpload api={api} />} />
                  <Route path="/my-samples" element={<UserSamples api={api} />} />
                  <Route path="/post-commission" element={<CommissionPost api={api} />} />
                  <Route path="/commission-submissions" element={<CommissionSubmissions api={api} />} />
                  <Route path="/commission-selection" element={<CommissionSelection api={api} />} />
                </Routes>
              </main>
            </div>
          </Router>
        </AccountProvider>
      </NotificationProvider>
    </SnackbarProvider>
    </ThemeProvider>
  );
}

export default App;
