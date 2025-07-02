import { Router, Request, Response } from 'express';
import auth from '../middleware/auth';
import User from '../auth/User';
import Sample from '../models/Sample';

const router = Router();

// @route   GET /api/users/:id
// @desc    Get user profile and their samples
// @access  Private (can be public for viewing profiles, but for now, let's keep it private)
router.get('/:id', auth, async (req: Request, res: Response) => {
  try {
    const user = await User.findById(req.params.id).select('-passwordHash'); // Exclude password hash
    if (!user) {
      return res.status(404).json({ msg: 'User not found' });
    }

    const userSamples = await Sample.find({ creator: req.params.id, status: 'approved' });

    res.json({ user, samples: userSamples });
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

// @route   GET /api/users/:id/referred-count
// @desc    Get the count of users referred by a specific user
// @access  Private (or public if referral counts are public)
router.get('/:id/referred-count', auth, async (req: Request, res: Response) => {
  try {
    const referredUsersCount = await User.countDocuments({ referrerId: req.params.id });
    res.json({ count: referredUsersCount });
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

// @route   GET /api/users/:id/faucet-amount
// @desc    Get the calculated faucet amount for a user
// @access  Private
router.get('/:id/faucet-amount', auth, async (req: Request, res: Response) => {
  try {
    // In a real scenario, this would involve complex logic to calculate the faucet amount
    // based on samples used, referrals, etc. For now, we'll return a simulated value.
    const faucetAmount = await calculateUserFaucetAmount(req.params.id);
    res.json({ amount: faucetAmount });
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

export default router;
