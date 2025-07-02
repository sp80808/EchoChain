// src/lib/utils.js

export const formatAddress = (address) => {
  if (!address) return '';
  return `${address.substring(0, 6)}...${address.slice(-4)}`;
};

export const formatBalance = (balance, decimals = 12, symbol = 'ECHO') => {
  if (!balance) return `0 ${symbol}`;
  const num = parseFloat(balance) / Math.pow(10, decimals);
  return `${num.toFixed(4)} ${symbol}`;
};
