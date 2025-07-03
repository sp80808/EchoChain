import { Request, Response } from 'express';
import jwt from 'jsonwebtoken';
import { ApiError } from './errors';
import logger from './logger';
import { ethers } from 'ethers';

const JWT_SECRET = process.env.JWT_SECRET;

export const ethLogin = async (req: Request, res: Response) => {
  const { address, signature, nonce } = req.body;
  if (!address || !signature || !nonce) {
    throw new ApiError('Address, signature, and nonce are required.', 400);
  }
  try {
    const recovered = ethers.verifyMessage(nonce, signature);
    if (recovered.toLowerCase() !== address.toLowerCase()) {
      throw new ApiError('Signature verification failed.', 401);
    }
    const token = jwt.sign({ address }, JWT_SECRET, { expiresIn: '1h' });
    res.status(200).json({ token });
  } catch (error) {
    logger.error('Ethereum login error:', error);
    throw new ApiError('Ethereum login failed.', 500);
  }
}; 