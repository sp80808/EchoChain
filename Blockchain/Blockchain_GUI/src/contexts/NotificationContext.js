import React, { createContext, useContext } from 'react';
import { useSnackbar } from 'notistack';

const NotificationContext = createContext();

export const useNotification = () => {
  const context = useContext(NotificationContext);
  if (!context) {
    throw new Error('useNotification must be used within a NotificationProvider');
  }
  return context;
};

export const NotificationProvider = ({ children }) => {
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();

  const showSuccess = (message, options = {}) => {
    return enqueueSnackbar(message, {
      variant: 'success',
      autoHideDuration: 5000,
      anchorOrigin: { vertical: 'top', horizontal: 'right' },
      ...options
    });
  };

  const showError = (message, options = {}) => {
    return enqueueSnackbar(message, {
      variant: 'error',
      autoHideDuration: 8000,
      anchorOrigin: { vertical: 'top', horizontal: 'right' },
      ...options
    });
  };

  const showWarning = (message, options = {}) => {
    return enqueueSnackbar(message, {
      variant: 'warning',
      autoHideDuration: 6000,
      anchorOrigin: { vertical: 'top', horizontal: 'right' },
      ...options
    });
  };

  const showInfo = (message, options = {}) => {
    return enqueueSnackbar(message, {
      variant: 'info',
      autoHideDuration: 5000,
      anchorOrigin: { vertical: 'top', horizontal: 'right' },
      ...options
    });
  };

  const showBlockchainTransaction = (message, txHash, options = {}) => {
    return enqueueSnackbar(`${message} (Tx: ${txHash?.slice(0, 8)}...)`, {
      variant: 'success',
      autoHideDuration: 10000,
      anchorOrigin: { vertical: 'top', horizontal: 'right' },
      action: txHash ? (
        <button 
          onClick={() => window.open(`https://polkadot.js.org/apps/?rpc=${encodeURIComponent('ws://127.0.0.1:9944')}#/explorer/query/${txHash}`, '_blank')}
          style={{ color: 'white', textDecoration: 'underline', background: 'none', border: 'none', cursor: 'pointer' }}
        >
          View
        </button>
      ) : null,
      ...options
    });
  };

  const value = {
    showSuccess,
    showError,
    showWarning,
    showInfo,
    showBlockchainTransaction,
    closeSnackbar
  };

  return (
    <NotificationContext.Provider value={value}>
      {children}
    </NotificationContext.Provider>
  );
};