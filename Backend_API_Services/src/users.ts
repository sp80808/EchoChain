import { Request, Response } from 'express';
import bcrypt from 'bcryptjs';
import jwt from 'jsonwebtoken';

// In-memory user database (replace with a real database in production)
interface User {
  id: number;
  username: string;
  password: string; // Hashed password
}

const users: User[] = [];

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

  // Check if user already exists
  if (users.find(user => user.username === username)) {
    res.status(409).json({ message: 'User already exists.' }); // Use 409 Conflict for existing resource
    return;
  }

  try {
    const hashedPassword = await hashPassword(password);
    const newUser = { id: users.length + 1, username, password: hashedPassword };
    users.push(newUser);
    res.status(201).json({ message: 'User registered successfully', user: { id: newUser.id, username: newUser.username } });
  } catch (error) {
    console.error('Registration error:', error); // Log the error for debugging
    res.status(500).json({ message: 'An unexpected error occurred during registration.' }); // Generic error message
  }
};

export const loginUser = async (req: Request, res: Response) => {
  const { username, password } = req.body;

  // Input Validation
  if (!username || !password) {
    res.status(400).json({ message: 'Username and password are required.' });
    return;
  }

  const user = users.find(u => u.username === username);
  if (!user) {
    res.status(401).json({ message: 'Invalid credentials.' }); // Use 401 Unauthorized for login failures
    return;
  }

  try {
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
  }
};