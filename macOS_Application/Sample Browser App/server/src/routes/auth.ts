import { Router } from 'express';
import bcrypt from 'bcryptjs';
import jwt from 'jsonwebtoken';
import User from '../auth/User';
import { getWalletBalanceFromBlockchain } from '../utils/blockchain';
import crypto from 'crypto'; // Import crypto for generating referral codes
import logger from '../utils/logger';

const router = Router();

// User Registration
router.post('/register', async (req, res) => {
  const { email, password } = req.body;

  try {
    let user = await User.findOne({ email });
    if (user) {
      return res.status(400).json({ msg: 'User already exists' });
    }

    const passwordHash = await bcrypt.hash(password, 10);
    
    // In a real application, a new wallet would be generated here.
    const walletAddress = `ECHO_${new Date().getTime()}`;

    user = new User({
      email,
      passwordHash,
      walletAddress,
    });

    await user.save();

    const payload = { userId: user.id };
    const token = jwt.sign(payload, 'your_jwt_secret', { expiresIn: '1h' });

    res.json({ token });
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

// User Login
router.post('/login', async (req, res) => {
  const { email, password } = req.body;

  try {
    const user = await User.findOne({ email });
    if (!user) {
      return res.status(400).json({ msg: 'Invalid credentials' });
    }

    const isMatch = await bcrypt.compare(password, user.passwordHash);
    if (!isMatch) {
      return res.status(400).json({ msg: 'Invalid credentials' });
    }

    const payload = { userId: user.id };
    const token = jwt.sign(payload, 'your_jwt_secret', { expiresIn: '1h' });

    res.json({ token });
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

// Get authenticated user's details
router.get('/me', auth, async (req, res) => {
  try {
    const user = await User.findById(req.user.id).select('-passwordHash');
    const balance = await getWalletBalanceFromBlockchain(user.walletAddress);
    res.json({ ...user.toObject(), balance, referralCode: user.referralCode });
  } catch (err) {
    logger.error(err.message);
    res.status(500).send('Server error');
  }
});

export default router;
