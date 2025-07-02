import { Request, Response } from 'express';
import bcrypt from 'bcryptjs';
import jwt from 'jsonwebtoken';
import { AppDataSource } from "./data-source";
import { User } from "./entity/User";
import { ApiError } from "./errors";
import logger from './logger';

const userRepository = AppDataSource.getRepository(User);

// JWT Secret Key (should be stored securely in environment variables in production)
const JWT_SECRET = process.env.JWT_SECRET;

if (!JWT_SECRET) {
  logger.error("JWT_SECRET is not defined in the environment variables. Please set it for security.");
  process.exit(1);
}

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
    throw new ApiError('Username and password are required.', 400);
  }

  if (username.length < 3) {
    throw new ApiError('Username must be at least 3 characters long.', 400);
  }

  if (password.length < 6) {
    throw new ApiError('Password must be at least 6 characters long.', 400);
  }

  try {
    // Check if user already exists
    const existingUser = await userRepository.findOneBy({ username });
    if (existingUser) {
      throw new ApiError('User already exists.', 409);
    }

    const hashedPassword = await hashPassword(password);
    const newUser = userRepository.create({ username, password: hashedPassword });
    await userRepository.save(newUser);
    res.status(201).json({ message: 'User registered successfully', user: { id: newUser.id, username: newUser.username } });
  } catch (error) {
    logger.error('Registration error:', error); // Log the error for debugging
    if (error instanceof ApiError) {
        res.status(error.statusCode).json({ message: error.message });
    } else {
        res.status(500).json({ message: 'An unexpected error occurred during registration.' }); // Generic error message
    }
  }
};

export const loginUser = async (req: Request, res: Response) => {
  const { username, password } = req.body;

  // Input Validation
  if (!username || !password) {
    throw new ApiError('Username and password are required.', 400);
  }

  try {
    const user = await userRepository.findOneBy({ username });
    if (!user) {
      throw new ApiError('Invalid credentials.', 401);
    }

    const isMatch = await comparePassword(password, user.password);
    if (!isMatch) {
      throw new ApiError('Invalid credentials.', 401);
    }

    // Generate JWT token
    const token = jwt.sign({ id: user.id, username: user.username }, JWT_SECRET, { expiresIn: '1h' });

    res.status(200).json({ message: 'Logged in successfully', token });
  } catch (error) {
    logger.error('Login error:', error); // Log the error for debugging
    if (error instanceof ApiError) {
        res.status(error.statusCode).json({ message: error.message });
    } else {
        res.status(500).json({ message: 'An unexpected error occurred during login.' }); // Generic error message
    }
  }
};