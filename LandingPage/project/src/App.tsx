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

function App() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 via-white to-purple-50">
      {/* Header */}
      <header className="bg-white shadow-lg border-b border-gray-100">
        <div className="max-w-7xl mx-auto px-6 py-8">
          <div className="flex items-center space-x-4 mb-4">
            <div className="p-3 bg-gradient-to-r from-blue-500 to-purple-600 rounded-xl text-white">
              <Music size={32} />
            </div>
            <div>
              <h1 className="text-4xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                EchoChain
              </h1>
              <p className="text-xl text-gray-600">Decentralized Music Sample Marketplace</p>
            </div>
          </div>
          <div className="bg-gradient-to-r from-blue-100 to-purple-100 rounded-lg p-4">
            <p className="text-gray-700 font-medium">
              Technical Specification v1.0 • A comprehensive architecture document for building 
              a decentralized music sample marketplace with P2P file sharing and blockchain integration.
            </p>
          </div>
        </div>
      </header>

      <div className="max-w-7xl mx-auto px-6 py-12 space-y-8">
        
        {/* System Architecture */}
        <CollapsibleSection 
          title="System Architecture" 
          icon={<Database size={24} />}
        >
          <div className="space-y-8">
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
              <FeatureCard
                icon={<Smartphone size={20} />}
                title="Frontend Layer"
                description="Modern React application with TypeScript, providing an intuitive user interface for sample discovery and trading."
                technologies={["React 18", "TypeScript", "Tailwind CSS", "Vite", "Web3.js"]}
              />
              <FeatureCard
                icon={<Server size={20} />}
                title="Backend Services"
                description="Scalable Node.js microservices architecture handling authentication, metadata, and blockchain interactions."
                technologies={["Node.js", "Express", "PostgreSQL", "Redis", "Docker"]}
              />
              <FeatureCard
                icon={<Globe size={20} />}
                title="P2P Network"
                description="Decentralized file sharing using WebTorrent protocol for efficient sample distribution."
                technologies={["WebTorrent", "WebRTC", "IPFS", "DHT"]}
              />
            </div>

            <div className="bg-gray-50 rounded-xl p-6">
              <h3 className="text-xl font-bold text-gray-800 mb-4 flex items-center">
                <GitBranch size={20} className="mr-2" />
                Architecture Overview
              </h3>
              <CodeBlock language="mermaid">
{`graph TB
    A[Web App] --> B[API Gateway]
    B --> C[Auth Service]
    B --> D[Sample Service]
    B --> E[Blockchain Service]
    
    C --> F[(User DB)]
    D --> G[(Sample Metadata)]
    E --> H[Smart Contracts]
    
    A --> I[WebTorrent Client]
    I --> J[P2P Network]
    
    H --> K[Ethereum Network]
    K --> L[Token Contract]
    K --> M[Marketplace Contract]`}
              </CodeBlock>
            </div>

            <div className="grid md:grid-cols-2 gap-6">
              <div className="bg-white rounded-lg p-6 border border-gray-200">
                <h4 className="text-lg font-bold text-gray-800 mb-3">Frontend Components</h4>
                <ul className="space-y-2 text-gray-600">
                  <li className="flex items-center"><Check size={16} className="mr-2 text-green-500" />Sample Browser & Player</li>
                  <li className="flex items-center"><Check size={16} className="mr-2 text-green-500" />Wallet Integration</li>
                  <li className="flex items-center"><Check size={16} className="mr-2 text-green-500" />Upload Interface</li>
                  <li className="flex items-center"><Check size={16} className="mr-2 text-green-500" />Profile Management</li>
                  <li className="flex items-center"><Check size={16} className="mr-2 text-green-500" />Analytics Dashboard</li>
                </ul>
              </div>
              <div className="bg-white rounded-lg p-6 border border-gray-200">
                <h4 className="text-lg font-bold text-gray-800 mb-3">Backend Services</h4>
                <ul className="space-y-2 text-gray-600">
                  <li className="flex items-center"><Check size={16} className="mr-2 text-green-500" />Authentication API</li>
                  <li className="flex items-center"><Check size={16} className="mr-2 text-green-500" />File Processing Service</li>
                  <li className="flex items-center"><Check size={16} className="mr-2 text-green-500" />Copyright Validation</li>
                  <li className="flex items-center"><Check size={16} className="mr-2 text-green-500" />Blockchain Interface</li>
                  <li className="flex items-center"><Check size={16} className="mr-2 text-green-500" />Analytics Engine</li>
                </ul>
              </div>
            </div>

            <div className="bg-blue-50 rounded-xl p-6">
              <h4 className="text-lg font-bold text-gray-800 mb-3">Database Schema</h4>
              <CodeBlock language="sql">
{`-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,
    wallet_address VARCHAR(42),
    profile_data JSONB,
    reputation_score INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Samples table
CREATE TABLE samples (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    creator_id UUID REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    genre VARCHAR(50),
    bpm INTEGER,
    key_signature VARCHAR(10),
    duration_ms INTEGER,
    file_hash VARCHAR(64) UNIQUE,
    price DECIMAL(18,8),
    license_type VARCHAR(50),
    copyright_cleared BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Transactions table
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sample_id UUID REFERENCES samples(id),
    buyer_id UUID REFERENCES users(id),
    seller_id UUID REFERENCES users(id),
    amount DECIMAL(18,8),
    tx_hash VARCHAR(66),
    status VARCHAR(20),
    created_at TIMESTAMP DEFAULT NOW()
);`}
              </CodeBlock>
            </div>
          </div>
        </CollapsibleSection>

        {/* Core Features */}
        <CollapsibleSection 
          title="Core Features" 
          icon={<Zap size={24} />}
        >
          <div className="space-y-8">
            <div className="grid md:grid-cols-2 gap-6">
              <div className="bg-gradient-to-r from-green-50 to-blue-50 rounded-xl p-6">
                <h3 className="text-xl font-bold text-gray-800 mb-4">Authentication Flow</h3>
                <div className="space-y-3">
                  <div className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-green-500 rounded-full flex items-center justify-center text-white text-sm font-bold">1</div>
                    <span className="text-gray-700">Email/Password Registration</span>
                  </div>
                  <div className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-green-500 rounded-full flex items-center justify-center text-white text-sm font-bold">2</div>
                    <span className="text-gray-700">Wallet Connection (MetaMask/WalletConnect)</span>
                  </div>
                  <div className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-green-500 rounded-full flex items-center justify-center text-white text-sm font-bold">3</div>
                    <span className="text-gray-700">Profile Verification</span>
                  </div>
                  <div className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-green-500 rounded-full flex items-center justify-center text-white text-sm font-bold">4</div>
                    <span className="text-gray-700">Two-Factor Authentication</span>
                  </div>
                </div>
              </div>

              <div className="bg-gradient-to-r from-purple-50 to-pink-50 rounded-xl p-6">
                <h3 className="text-xl font-bold text-gray-800 mb-4">Sample Upload Process</h3>
                <div className="space-y-3">
                  <div className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-purple-500 rounded-full flex items-center justify-center text-white text-sm font-bold">1</div>
                    <span className="text-gray-700">File Upload & Validation</span>
                  </div>
                  <div className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-purple-500 rounded-full flex items-center justify-center text-white text-sm font-bold">2</div>
                    <span className="text-gray-700">Audio Analysis & Metadata Extraction</span>
                  </div>
                  <div className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-purple-500 rounded-full flex items-center justify-center text-white text-sm font-bold">3</div>
                    <span className="text-gray-700">Copyright Verification</span>
                  </div>
                  <div className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-purple-500 rounded-full flex items-center justify-center text-white text-sm font-bold">4</div>
                    <span className="text-gray-700">P2P Distribution</span>
                  </div>
                </div>
              </div>
            </div>

            <div className="bg-gray-50 rounded-xl p-6">
              <h3 className="text-xl font-bold text-gray-800 mb-4">API Endpoints</h3>
              <CodeBlock language="javascript">
{`// Authentication endpoints
POST /api/auth/register
POST /api/auth/login
POST /api/auth/wallet-connect
GET  /api/auth/profile

// Sample management
GET    /api/samples
POST   /api/samples
GET    /api/samples/:id
PUT    /api/samples/:id
DELETE /api/samples/:id

// Search and discovery
GET  /api/search?q={query}&genre={genre}&bpm={range}
GET  /api/recommendations/:userId
GET  /api/trending

// Transactions
POST /api/purchase
GET  /api/transactions/:userId
POST /api/tip/:creatorId

// P2P endpoints
GET  /api/torrent/:sampleId
POST /api/seed/:sampleId
GET  /api/peers/:sampleId`}
              </CodeBlock>
            </div>

            <div className="grid md:grid-cols-3 gap-6">
              <div className="bg-white rounded-lg p-6 border border-gray-200">
                <h4 className="text-lg font-bold text-gray-800 mb-3 flex items-center">
                  <FileText size={20} className="mr-2" />
                  Copyright System
                </h4>
                <ul className="space-y-2 text-gray-600">
                  <li>• Audio fingerprinting</li>
                  <li>• Database matching</li>
                  <li>• Manual review process</li>
                  <li>• Dispute resolution</li>
                </ul>
              </div>
              <div className="bg-white rounded-lg p-6 border border-gray-200">
                <h4 className="text-lg font-bold text-gray-800 mb-3 flex items-center">
                  <Cloud size={20} className="mr-2" />
                  P2P Sharing
                </h4>
                <ul className="space-y-2 text-gray-600">
                  <li>• WebTorrent protocol</li>
                  <li>• Automatic seeding</li>
                  <li>• Bandwidth optimization</li>
                  <li>• Offline availability</li>
                </ul>
              </div>
              <div className="bg-white rounded-lg p-6 border border-gray-200">
                <h4 className="text-lg font-bold text-gray-800 mb-3 flex items-center">
                  <Users size={20} className="mr-2" />
                  Community Features
                </h4>
                <ul className="space-y-2 text-gray-600">
                  <li>• Creator profiles</li>
                  <li>• Rating system</li>
                  <li>• Comments & feedback</li>
                  <li>• Collaboration tools</li>
                </ul>
              </div>
            </div>
          </div>
        </CollapsibleSection>

        {/* Smart Contract Specifications */}
        <CollapsibleSection 
          title="Smart Contract Specifications" 
          icon={<Code size={24} />}
        >
          <div className="space-y-8">
            <div className="grid md:grid-cols-2 gap-6">
              <div className="bg-white rounded-lg p-6 border border-gray-200">
                <h3 className="text-xl font-bold text-gray-800 mb-4">ECHO Token Contract</h3>
                <CodeBlock language="solidity">
{`pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract EchoToken is ERC20, Ownable {
    uint256 public constant MAX_SUPPLY = 1000000000 * 10**18;
    
    mapping(address => bool) public minters;
    
    constructor() ERC20("EchoChain", "ECHO") {}
    
    function mint(address to, uint256 amount) 
        external onlyMinter {
        require(totalSupply() + amount <= MAX_SUPPLY);
        _mint(to, amount);
    }
    
    modifier onlyMinter() {
        require(minters[msg.sender] || msg.sender == owner());
        _;
    }
}`}
                </CodeBlock>
              </div>

              <div className="bg-white rounded-lg p-6 border border-gray-200">
                <h3 className="text-xl font-bold text-gray-800 mb-4">Marketplace Contract</h3>
                <CodeBlock language="solidity">
{`pragma solidity ^0.8.19;

contract EchoMarketplace {
    struct Sample {
        address creator;
        uint256 price;
        string metadataHash;
        bool active;
        uint256 totalSales;
    }
    
    mapping(uint256 => Sample) public samples;
    mapping(address => uint256[]) public creatorSamples;
    
    event SampleListed(uint256 indexed sampleId, address creator);
    event SamplePurchased(uint256 indexed sampleId, address buyer);
    
    function listSample(
        uint256 sampleId,
        uint256 price,
        string memory metadataHash
    ) external {
        samples[sampleId] = Sample({
            creator: msg.sender,
            price: price,
            metadataHash: metadataHash,
            active: true,
            totalSales: 0
        });
        
        creatorSamples[msg.sender].push(sampleId);
        emit SampleListed(sampleId, msg.sender);
    }
}`}
                </CodeBlock>
              </div>
            </div>

            <div className="bg-gradient-to-r from-blue-50 to-purple-50 rounded-xl p-6">
              <h3 className="text-xl font-bold text-gray-800 mb-4">Revenue Distribution</h3>
              <div className="grid md:grid-cols-4 gap-4">
                <div className="text-center">
                  <div className="text-3xl font-bold text-blue-600">70%</div>
                  <div className="text-gray-600">Creator</div>
                </div>
                <div className="text-center">
                  <div className="text-3xl font-bold text-purple-600">15%</div>
                  <div className="text-gray-600">Platform</div>
                </div>
                <div className="text-center">
                  <div className="text-3xl font-bold text-green-600">10%</div>
                  <div className="text-gray-600">Validators</div>
                </div>
                <div className="text-center">
                  <div className="text-3xl font-bold text-orange-600">5%</div>
                  <div className="text-gray-600">DAO Treasury</div>
                </div>
              </div>
            </div>

            <div className="bg-gray-50 rounded-xl p-6">
              <h3 className="text-xl font-bold text-gray-800 mb-4">Governance & Tipping</h3>
              <CodeBlock language="solidity">
{`contract EchoGovernance {
    struct Proposal {
        string description;
        uint256 forVotes;
        uint256 againstVotes;
        uint256 endTime;
        bool executed;
    }
    
    function createProposal(string memory description) external {
        require(echoToken.balanceOf(msg.sender) >= MIN_PROPOSAL_THRESHOLD);
        // Implementation
    }
    
    function vote(uint256 proposalId, bool support, uint256 amount) external {
        // Voting implementation
    }
}

contract EchoTipping {
    function tip(address creator, uint256 amount) external {
        require(echoToken.transferFrom(msg.sender, creator, amount));
        emit TipSent(msg.sender, creator, amount);
    }
}`}
              </CodeBlock>
            </div>
          </div>
        </CollapsibleSection>

        {/* Security Considerations */}
        <CollapsibleSection 
          title="Security Considerations" 
          icon={<Shield size={24} />}
        >
          <div className="space-y-8">
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
              <div className="bg-red-50 rounded-lg p-6 border border-red-200">
                <h4 className="text-lg font-bold text-red-800 mb-3 flex items-center">
                  <Lock size={20} className="mr-2" />
                  Authentication
                </h4>
                <ul className="space-y-2 text-red-700">
                  <li>• JWT token validation</li>
                  <li>• Rate limiting</li>
                  <li>• Session management</li>
                  <li>• Password hashing (bcrypt)</li>
                </ul>
              </div>
              <div className="bg-orange-50 rounded-lg p-6 border border-orange-200">
                <h4 className="text-lg font-bold text-orange-800 mb-3 flex items-center">
                  <FileText size={20} className="mr-2" />
                  File Security
                </h4>
                <ul className="space-y-2 text-orange-700">
                  <li>• File type validation</li>
                  <li>• Virus scanning</li>
                  <li>• Size limitations</li>
                  <li>• Content filtering</li>
                </ul>
              </div>
              <div className="bg-yellow-50 rounded-lg p-6 border border-yellow-200">
                <h4 className="text-lg font-bold text-yellow-800 mb-3 flex items-center">
                  <Code size={20} className="mr-2" />
                  Smart Contracts
                </h4>
                <ul className="space-y-2 text-yellow-700">
                  <li>• Reentrancy protection</li>
                  <li>• Access control</li>
                  <li>• Overflow protection</li>
                  <li>• Audit requirements</li>
                </ul>
              </div>
            </div>

            <div className="bg-white rounded-xl p-6 border border-gray-200">
              <h3 className="text-xl font-bold text-gray-800 mb-4">Security Implementation</h3>
              <CodeBlock language="javascript">
{`// Input validation middleware
const validateSampleUpload = async (req, res, next) => {
    const { file } = req;
    
    // File type validation
    const allowedTypes = ['audio/mpeg', 'audio/wav', 'audio/flac'];
    if (!allowedTypes.includes(file.mimetype)) {
        return res.status(400).json({ error: 'Invalid file type' });
    }
    
    // Size validation (max 50MB)
    if (file.size > 50 * 1024 * 1024) {
        return res.status(400).json({ error: 'File too large' });
    }
    
    // Virus scanning
    const scanResult = await virusScanner.scan(file.buffer);
    if (scanResult.infected) {
        return res.status(400).json({ error: 'File failed security scan' });
    }
    
    next();
};

// Rate limiting
const rateLimiter = rateLimit({
    windowMs: 15 * 60 * 1000, // 15 minutes
    max: 100, // Limit each IP to 100 requests per windowMs
    message: 'Too many requests from this IP'
});`}
              </CodeBlock>
            </div>

            <div className="grid md:grid-cols-2 gap-6">
              <div className="bg-green-50 rounded-lg p-6 border border-green-200">
                <h4 className="text-lg font-bold text-green-800 mb-3">P2P Network Security</h4>
                <ul className="space-y-2 text-green-700">
                  <li>• Peer verification</li>
                  <li>• Content hash validation</li>
                  <li>• Bandwidth throttling</li>
                  <li>• Malicious peer detection</li>
                  <li>• Encrypted connections</li>
                </ul>
              </div>
              <div className="bg-blue-50 rounded-lg p-6 border border-blue-200">
                <h4 className="text-lg font-bold text-blue-800 mb-3">Data Protection</h4>
                <ul className="space-y-2 text-blue-700">
                  <li>• GDPR compliance</li>
                  <li>• Data encryption at rest</li>
                  <li>• Secure API endpoints</li>
                  <li>• User privacy controls</li>
                  <li>• Audit logging</li>
                </ul>
              </div>
            </div>
          </div>
        </CollapsibleSection>

        {/* Performance Requirements */}
        <CollapsibleSection 
          title="Performance Requirements" 
          icon={<Cpu size={24} />}
        >
          <div className="space-y-8">
            <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
              <div className="bg-gradient-to-br from-blue-500 to-blue-600 text-white rounded-xl p-6">
                <div className="text-3xl font-bold mb-2">&lt;200ms</div>
                <div className="text-blue-100">API Response Time</div>
              </div>
              <div className="bg-gradient-to-br from-green-500 to-green-600 text-white rounded-xl p-6">
                <div className="text-3xl font-bold mb-2">99.9%</div>
                <div className="text-green-100">Uptime SLA</div>
              </div>
              <div className="bg-gradient-to-br from-purple-500 to-purple-600 text-white rounded-xl p-6">
                <div className="text-3xl font-bold mb-2">1M+</div>
                <div className="text-purple-100">Concurrent Users</div>
              </div>
              <div className="bg-gradient-to-br from-orange-500 to-orange-600 text-white rounded-xl p-6">
                <div className="text-3xl font-bold mb-2">&lt;3s</div>
                <div className="text-orange-100">File Upload Time</div>
              </div>
            </div>

            <div className="bg-gray-50 rounded-xl p-6">
              <h3 className="text-xl font-bold text-gray-800 mb-4">Optimization Strategies</h3>
              <div className="grid md:grid-cols-2 gap-6">
                <div>
                  <h4 className="text-lg font-semibold text-gray-800 mb-3">Frontend Optimization</h4>
                  <ul className="space-y-2 text-gray-600">
                    <li>• Code splitting and lazy loading</li>
                    <li>• Service worker caching</li>
                    <li>• Image optimization</li>
                    <li>• Bundle size optimization</li>
                    <li>• CDN integration</li>
                  </ul>
                </div>
                <div>
                  <h4 className="text-lg font-semibold text-gray-800 mb-3">Backend Optimization</h4>
                  <ul className="space-y-2 text-gray-600">
                    <li>• Database indexing</li>
                    <li>• Redis caching</li>
                    <li>• Connection pooling</li>
                    <li>• Load balancing</li>
                    <li>• Microservice scaling</li>
                  </ul>
                </div>
              </div>
            </div>

            <div className="bg-white rounded-xl p-6 border border-gray-200">
              <h3 className="text-xl font-bold text-gray-800 mb-4">Scalability Metrics</h3>
              <CodeBlock language="yaml">
{`# Performance targets
api_response_time: <200ms (95th percentile)
database_query_time: <50ms (average)
file_upload_speed: >10MB/s
download_speed: >5MB/s
concurrent_connections: 100,000+
transactions_per_second: 1,000+

# Monitoring thresholds
cpu_utilization: <70%
memory_utilization: <80%
disk_io: <80%
network_bandwidth: <70%

# Auto-scaling triggers
cpu_threshold: 70%
memory_threshold: 80%
response_time_threshold: 500ms
error_rate_threshold: 1%`}
              </CodeBlock>
            </div>

            <div className="grid md:grid-cols-3 gap-6">
              <div className="bg-white rounded-lg p-6 border border-gray-200">
                <h4 className="text-lg font-bold text-gray-800 mb-3">Testing Strategy</h4>
                <ul className="space-y-2 text-gray-600">
                  <li>• Unit testing (90% coverage)</li>
                  <li>• Integration testing</li>
                  <li>• Load testing</li>
                  <li>• Security testing</li>
                  <li>• E2E testing</li>
                </ul>
              </div>
              <div className="bg-white rounded-lg p-6 border border-gray-200">
                <h4 className="text-lg font-bold text-gray-800 mb-3">Deployment</h4>
                <ul className="space-y-2 text-gray-600">
                  <li>• Kubernetes orchestration</li>
                  <li>• Blue-green deployment</li>
                  <li>• Automated rollbacks</li>
                  <li>• Health checks</li>
                  <li>• Zero-downtime updates</li>
                </ul>
              </div>
              <div className="bg-white rounded-lg p-6 border border-gray-200">
                <h4 className="text-lg font-bold text-gray-800 mb-3">Monitoring</h4>
                <ul className="space-y-2 text-gray-600">
                  <li>• Real-time analytics</li>
                  <li>• Error tracking</li>
                  <li>• Performance metrics</li>
                  <li>• User behavior analysis</li>
                  <li>• Alert systems</li>
                </ul>
              </div>
            </div>
          </div>
        </CollapsibleSection>

        {/* Technology Stack */}
        <CollapsibleSection 
          title="Technology Stack & Implementation" 
          icon={<GitBranch size={24} />}
          defaultOpen={false}
        >
          <div className="space-y-8">
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
              <div className="bg-white rounded-lg p-6 border border-gray-200">
                <h4 className="text-lg font-bold text-gray-800 mb-3">Frontend Stack</h4>
                <div className="space-y-3">
                  <div className="flex justify-between">
                    <span className="text-gray-600">Framework</span>
                    <span className="font-medium">React 18</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Language</span>
                    <span className="font-medium">TypeScript</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Styling</span>
                    <span className="font-medium">Tailwind CSS</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Build Tool</span>
                    <span className="font-medium">Vite</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Web3</span>
                    <span className="font-medium">ethers.js</span>
                  </div>
                </div>
              </div>

              <div className="bg-white rounded-lg p-6 border border-gray-200">
                <h4 className="text-lg font-bold text-gray-800 mb-3">Backend Stack</h4>
                <div className="space-y-3">
                  <div className="flex justify-between">
                    <span className="text-gray-600">Runtime</span>
                    <span className="font-medium">Node.js</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Framework</span>
                    <span className="font-medium">Express.js</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Database</span>
                    <span className="font-medium">PostgreSQL</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Cache</span>
                    <span className="font-medium">Redis</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Queue</span>
                    <span className="font-medium">Bull/BullMQ</span>
                  </div>
                </div>
              </div>

              <div className="bg-white rounded-lg p-6 border border-gray-200">
                <h4 className="text-lg font-bold text-gray-800 mb-3">Infrastructure</h4>
                <div className="space-y-3">
                  <div className="flex justify-between">
                    <span className="text-gray-600">Containers</span>
                    <span className="font-medium">Docker</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Orchestration</span>
                    <span className="font-medium">Kubernetes</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Cloud</span>
                    <span className="font-medium">AWS/GCP</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">CDN</span>
                    <span className="font-medium">CloudFlare</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Monitoring</span>
                    <span className="font-medium">Datadog</span>
                  </div>
                </div>
              </div>
            </div>

            <div className="bg-gradient-to-r from-gray-50 to-blue-50 rounded-xl p-6">
              <h3 className="text-xl font-bold text-gray-800 mb-4">Development Workflow</h3>
              <CodeBlock language="yaml">
{`# CI/CD Pipeline
stages:
  - test
  - build
  - security-scan
  - deploy-staging
  - integration-tests
  - deploy-production

test:
  script:
    - npm run test:unit
    - npm run test:integration
    - npm run lint
    - npm run type-check
  coverage: 90%

build:
  script:
    - docker build -t echochain:$CI_COMMIT_SHA .
    - docker push registry/echochain:$CI_COMMIT_SHA

security-scan:
  script:
    - snyk test
    - docker scan echochain:$CI_COMMIT_SHA
    - npm audit

deploy-production:
  script:
    - kubectl apply -f k8s/
    - kubectl set image deployment/api api=echochain:$CI_COMMIT_SHA
  only:
    - main`}
              </CodeBlock>
            </div>
          </div>
        </CollapsibleSection>
      </div>

      {/* Footer */}
      <footer className="bg-gray-800 text-white mt-16">
        <div className="max-w-7xl mx-auto px-6 py-12">
          <div className="grid md:grid-cols-4 gap-8">
            <div>
              <div className="flex items-center space-x-3 mb-4">
                <div className="p-2 bg-gradient-to-r from-blue-500 to-purple-600 rounded-lg">
                  <Music size={24} />
                </div>
                <span className="text-xl font-bold">EchoChain</span>
              </div>
              <p className="text-gray-400">
                Decentralized music sample marketplace enabling creators to monetize their work
                through blockchain technology and P2P distribution.
              </p>
            </div>
            <div>
              <h4 className="font-bold mb-4">Technology</h4>
              <ul className="space-y-2 text-gray-400">
                <li>React & TypeScript</li>
                <li>Node.js Backend</li>
                <li>Smart Contracts</li>
                <li>WebTorrent P2P</li>
              </ul>
            </div>
            <div>
              <h4 className="font-bold mb-4">Features</h4>
              <ul className="space-y-2 text-gray-400">
                <li>Sample Marketplace</li>
                <li>Creator Rewards</li>
                <li>Copyright Protection</li>
                <li>Community Governance</li>
              </ul>
            </div>
            <div>
              <h4 className="font-bold mb-4">Documentation</h4>
              <ul className="space-y-2 text-gray-400">
                <li>API Reference</li>
                <li>Smart Contract Docs</li>
                <li>Integration Guide</li>
                <li>Security Audit</li>
              </ul>
            </div>
          </div>
          <div className="border-t border-gray-700 mt-8 pt-8 text-center text-gray-400">
            <p>&copy; 2024 EchoChain. Technical Specification v1.0 - Built for decentralized music innovation.</p>
          </div>
        </div>
      </footer>
    </div>
  );
}

export default App;