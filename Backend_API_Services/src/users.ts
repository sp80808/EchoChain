import { Request, Response } from 'express';
import bcrypt from 'bcryptjs';
import jwt from 'jsonwebtoken';
import { openDb } from './db';

// JWT Secret Key (should be stored securely in environment variables in production)
const JWT_SECRET = process.env.JWT_SECRET || 'supersecretjwtkey';

// Mock password hashing (for demonstration purposes)
const hashPassword = async (password: string): Promise<string> => {
  const salt = await bcrypt.genSalt(10);
  return bcrypt.hash(password, salt);
};

const comparePassword = async (password: string, hashedPassword: string): Promise<boolean> => {
  return bcrypt.compare(password, hashedPassword);
};

export const registerUser = async (req: Request, res: Response) => {
  const { username, password } = req.body;

  // Input Validation
  if (!username || !password) {
    res.status(400).json({ message: 'Username and password are required.' });
    return;
  }

  if (username.length < 3) {
    res.status(400).json({ message: 'Username must be at least 3 characters long.' });
    return;
  }

  if (password.length < 6) {
    res.status(400).json({ message: 'Password must be at least 6 characters long.' });
    return;
  }

  const db = await openDb();

  try {
    // Check if user already exists
    const existingUser = await db.get('SELECT * FROM users WHERE username = ?', username);
    if (existingUser) {
      res.status(409).json({ message: 'User already exists.' }); // Use 409 Conflict for existing resource
      return;
    }

    const hashedPassword = await hashPassword(password);
    const result = await db.run('INSERT INTO users (username, password) VALUES (?, ?)', username, hashedPassword);
    const newUser = { id: result.lastID, username };
    res.status(201).json({ message: 'User registered successfully', user: newUser });
  } catch (error) {
    console.error('Registration error:', error); // Log the error for debugging
    res.status(500).json({ message: 'An unexpected error occurred during registration.' }); // Generic error message
  } finally {
    await db.close();
  }
};

export const loginUser = async (req: Request, res: Response) => {
  const { username, password } = req.body;

  // Input Validation
  if (!username || !password) {
    res.status(400).json({ message: 'Username and password are required.' });
    return;
  }

  const db = await openDb();

  try {
    const user = await db.get('SELECT * FROM users WHERE username = ?', username);
    if (!user) {
      res.status(401).json({ message: 'Invalid credentials.' }); // Use 401 Unauthorized for login failures
      return;
    }

    const isMatch = await comparePassword(password, user.password);
    if (!isMatch) {
      res.status(401).json({ message: 'Invalid credentials.' }); // Use 401 Unauthorized for login failures
      return;
    }

    // Generate JWT token
    const token = jwt.sign({ id: user.id, username: user.username }, JWT_SECRET, { expiresIn: '1h' });

    res.status(200).json({ message: 'Logged in successfully', token });
  } catch (error) {
    console.error('Login error:', error); // Log the error for debugging
    res.status(500).json({ message: 'An unexpected error occurred during login.' }); // Generic error message
  } finally {
    await db.close();
  }
};

// Example of a protected route that could be added
export const getUserProfile = async (req: Request, res: Response) => {
  // The user object is attached to the request by the authentication middleware
  const user = req.user;
  if (!user) {
    return res.status(404).json({ message: 'User not found.' });
  }
  
  const db = await openDb();
  try {
    const userProfile = await db.get('SELECT id, username FROM users WHERE id = ?', user.id);
    if (!userProfile) {
      return res.status(404).json({ message: 'User not found.' });
    }
    res.status(200).json(userProfile);
  } catch (error) {
    console.error('Get user profile error:', error);
    res.status(500).json({ message: 'An unexpected error occurred while fetching user profile.' });
  } finally {
    await db.close();
  }
};