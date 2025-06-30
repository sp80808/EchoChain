import express, { Request, Response, NextFunction, RequestHandler } from 'express';
import { registerUser, loginUser, getUserProfile } from './users';
import { checkCopyright } from './copyright';
import { createSchema } from './db';
import jwt from 'jsonwebtoken';
import rateLimit from 'express-rate-limit';
import helmet from 'helmet';
import cors from 'cors';
import morgan from 'morgan'; // For request logging
import dotenv from 'dotenv'; // For environment variables

dotenv.config(); // Load environment variables from .env file

const app = express();
const PORT = process.env.PORT || 3000;
const JWT_SECRET = process.env.JWT_SECRET || 'supersecretjwtkey'; // Fallback for development, use .env in production

// Extend the Request interface to include the user property
declare global {
  namespace Express {
    interface Request {
      user?: { id: number; username: string };
    }
  }
}

// Utility to wrap async route handlers and catch errors
const asyncHandler = (fn: (req: Request, res: Response, next: NextFunction) => Promise<any>) =>
  (req: Request, res: Response, next: NextFunction) =>
    Promise.resolve(fn(req, res, next)).catch(next);

// Security Middleware
app.use(helmet()); // Sets various HTTP headers for security
app.use(cors());   // Enables CORS with default options

// Request Logging
app.use(morgan('combined')); // Combined log format

// Rate Limiting
const authRateLimiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100, // Limit each IP to 100 requests per windowMs
  message: 'Too many authentication requests from this IP, please try again after 15 minutes',
});

const apiRateLimiter = rateLimit({
  windowMs: 60 * 60 * 1000, // 1 hour
  max: 1000, // Limit each IP to 1000 requests per windowMs
  message: 'Too many API requests from this IP, please try again after an hour',
});

// JWT Authentication Middleware
const authenticateToken: RequestHandler = (req, res, next) => {
  const authHeader = req.headers['authorization'];
  const token = authHeader && authHeader.split(' ')[1];

  if (token == null) {
    res.status(401).json({ message: 'Authentication token required.' });
    return; // Explicitly return void
  }

  jwt.verify(token, JWT_SECRET, (err, user) => {
    if (err) {
      console.error('JWT verification error:', err);
      res.status(403).json({ message: 'Invalid or expired token.' });
      return; // Explicitly return void
    }
    // Ensure user is of the expected type
    req.user = user as { id: number; username: string };
    next();
  });
};

app.use(express.json()); // For parsing application/json

app.get('/', (req, res) => {
  res.send('EchoChain Backend API Services are running!');
});

// Public routes
app.post('/api/register', authRateLimiter, asyncHandler(registerUser));
app.post('/api/login', authRateLimiter, asyncHandler(loginUser));

// Protected routes - apply authenticateToken middleware
app.post('/api/copyright-check', apiRateLimiter, authenticateToken, asyncHandler(checkCopyright));
app.get('/api/profile', apiRateLimiter, authenticateToken, asyncHandler(getUserProfile));


// Global error handling middleware
app.use((err: Error, req: Request, res: Response, next: NextFunction) => {
  console.error('Global error handler:', err.stack);
  res.status(500).json({ message: 'An unexpected server error occurred.' });
});

// Initialize database and start server
createSchema()
  .then(() => {
    app.listen(PORT, () => {
      console.log(`Server running on port ${PORT}`);
    });
  })
  .catch(err => {
    console.error('Failed to initialize database:', err);
    process.exit(1);
  });