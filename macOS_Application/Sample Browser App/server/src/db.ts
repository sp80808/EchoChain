/**
 * Database connection for EchoChain server
 * Handles MongoDB connection and configuration
 */

import mongoose from 'mongoose';
import { logger } from './utils/logger';

const connectDB = async (): Promise<void> => {
  try {
    const mongoURI = process.env.MONGODB_URI || 'mongodb://localhost:27017/echochain';
    
    logger.info('Connecting to MongoDB...');
    
    const conn = await mongoose.connect(mongoURI, {
      // Remove deprecated options, use only supported ones
      maxPoolSize: 10,
      serverSelectionTimeoutMS: 5000,
      socketTimeoutMS: 45000,
    });

    logger.info(`MongoDB Connected: ${conn.connection.host}`);
    
    // Handle connection events
    mongoose.connection.on('error', (err) => {
      logger.error('MongoDB connection error:', err);
    });

    mongoose.connection.on('disconnected', () => {
      logger.warn('MongoDB disconnected');
    });

    process.on('SIGINT', async () => {
      await mongoose.connection.close();
      logger.info('MongoDB connection closed due to app termination');
      process.exit(0);
    });

  } catch (error) {
    logger.error('Error connecting to MongoDB:', error);
    
    // In development, continue without database
    if (process.env.NODE_ENV === 'development') {
      logger.warn('Running in development mode without database connection');
    } else {
      process.exit(1);
    }
  }
};

export default connectDB;
