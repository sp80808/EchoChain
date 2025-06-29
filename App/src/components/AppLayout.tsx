import React from 'react';

interface AppLayoutProps {
  children: React.ReactNode;
  navigateTo: (page: string) => void;
}

const AppLayout: React.FC<AppLayoutProps> = ({ children, navigateTo }) => {
  return (
    <div className="flex h-screen bg-gray-900 text-white">
      {/* Sidebar */}
      <aside className="w-64 bg-gray-800 p-4 shadow-lg">
        <h1 className="text-3xl font-bold mb-6 text-blue-400">EchoChain</h1>
        <nav>
          <ul>
            <li className="mb-3">
              <button onClick={() => navigateTo('browse')} className="block py-2 px-4 rounded hover:bg-gray-700 transition-colors duration-200 w-full text-left">Explore Samples</button>
            </li>
            <li className="mb-3">
              <button onClick={() => navigateTo('upload')} className="block py-2 px-4 rounded hover:bg-gray-700 transition-colors duration-200 w-full text-left">Upload</button>
            </li>
            {/* Add other navigation items */}
          </ul>
        </nav>
      </aside>

      {/* Main Content */}
      <main className="flex-1 p-6 overflow-y-auto">
        {children}
      </main>
    </div>
  );
};

export default AppLayout;
