import React, { createContext, useContext, useState, useEffect } from 'react';

const AccountContext = createContext();

export function AccountProvider({ children }) {
  const [account, setAccount] = useState(null); // { address, meta }
  const [jwt, setJwt] = useState(() => window.localStorage.getItem('jwtToken') || '');

  useEffect(() => {
    if (jwt) {
      window.localStorage.setItem('jwtToken', jwt);
    }
  }, [jwt]);

  return (
    <AccountContext.Provider value={{ account, setAccount, jwt, setJwt }}>
      {children}
    </AccountContext.Provider>
  );
}

export function useAccount() {
  return useContext(AccountContext);
} 