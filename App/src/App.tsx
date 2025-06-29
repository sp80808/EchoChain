import React, { useState } from 'react';
import { 
  Music, 
  Users, 
  Shield, 
  Zap, 
  Database, 
  Cloud, 
  Lock, 
  Cpu, 
  Globe,
  Code,
  FileText,
  ArrowRight,
  Check,
  GitBranch,
  Server,
  Smartphone,
  ChevronDown,
  ChevronUp
} from 'lucide-react';

interface SectionProps {
  title: string;
  icon: React.ReactNode;
  children: React.ReactNode;
  defaultOpen?: boolean;
}

function CollapsibleSection({ title, icon, children, defaultOpen = true }: SectionProps) {
  const [isOpen, setIsOpen] = useState(defaultOpen);
  
  return (
    <div className="bg-white rounded-xl shadow-lg border border-gray-100 overflow-hidden">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-full px-8 py-6 bg-gradient-to-r from-blue-50 to-purple-50 border-b border-gray-100 flex items-center justify-between hover:from-blue-100 hover:to-purple-100 transition-all duration-200"
      >
        <div className="flex items-center space-x-4">
          <div className="p-2 bg-gradient-to-r from-blue-500 to-purple-600 rounded-lg text-white">
            {icon}
          </div>
          <h2 className="text-2xl font-bold text-gray-800">{title}</h2>
        </div>
        {isOpen ? <ChevronUp size={24} /> : <ChevronDown size={24} />}
      </button>
      {isOpen && (
        <div className="p-8">
          {children}
        </div>
      )}
    </div>
  );
}

interface CodeBlockProps {
  language: string;
  children: string;
}

function CodeBlock({ language, children }: CodeBlockProps) {
  return (
    <div className="bg-gray-900 rounded-lg overflow-hidden my-4">
      <div className="bg-gray-800 px-4 py-2 text-gray-300 text-sm font-medium">
        {language}
      </div>
      <pre className="p-4 text-green-400 text-sm overflow-x-auto">
        <code>{children}</code>
      </pre>
    </div>
  );
}

interface FeatureCardProps {
  icon: React.ReactNode;
  title: string;
  description: string;
  technologies: string[];
}

function FeatureCard({ icon, title, description, technologies }: FeatureCardProps) {
  return (
    <div className="bg-gradient-to-br from-white to-gray-50 rounded-xl p-6 border border-gray-200 hover:shadow-lg transition-all duration-300 hover:scale-105">
      <div className="flex items-center space-x-3 mb-4">
        <div className="p-2 bg-gradient-to-r from-blue-500 to-purple-600 rounded-lg text-white">
          {icon}
        </div>
        <h3 className="text-lg font-bold text-gray-800">{title}</h3>
      </div>
      <p className="text-gray-600 mb-4">{description}</p>
      <div className="flex flex-wrap gap-2">
        {technologies.map((tech, index) => (
          <span
            key={index}
            className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-xs font-medium"
          >
            {tech}
          </span>
        ))}
      </div>
    </div>
  );
}

import React, { useState } from 'react';
import Login from './components/Auth/Login';
import Register from './components/Auth/Register';
import SampleBrowser from './pages/SampleBrowser';
import SampleUpload from './pages/SampleUpload';
import AppLayout from './components/AppLayout';

const App: React.FC = () => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [showRegister, setShowRegister] = useState(false);
  const [currentPage, setCurrentPage] = useState('browse'); // 'browse', 'upload', 'login', 'register'

  // Check for token on initial load
  React.useEffect(() => {
    const token = localStorage.getItem('token');
    if (token) {
      setIsAuthenticated(true);
      setCurrentPage('browse');
    } else {
      setCurrentPage('login');
    }
  }, []);

  const handleLoginSuccess = () => {
    setIsAuthenticated(true);
    setCurrentPage('browse');
  };

  const handleRegisterSuccess = () => {
    setShowRegister(false); // Go back to login after registration
    setCurrentPage('login');
  };

  const navigateTo = (page: string) => {
    setCurrentPage(page);
  };

  if (!isAuthenticated) {
    return showRegister ? (
      <Register onRegisterSuccess={handleRegisterSuccess} />
    ) : (
      <Login onLoginSuccess={handleLoginSuccess} onShowRegister={() => setShowRegister(true)} />
    );
  }

  return (
    <AppLayout navigateTo={navigateTo}>
      {currentPage === 'browse' && <SampleBrowser />}
      {currentPage === 'upload' && <SampleUpload />}
      {/* Add other pages here */}
    </AppLayout>
  );
};

export default App;


export default App;