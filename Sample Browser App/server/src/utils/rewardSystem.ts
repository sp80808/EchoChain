import cron from 'node-cron';
import Sample from '../models/Sample';
import { registerSampleOnBlockchain } from './blockchain'; // Re-using for reward reporting

export const setupRewardSystem = () => {
  // Schedule content rewards check for the first day of every month at midnight
  cron.schedule('0 0 1 * *', async () => {
    console.log('Running monthly content reward check...');
    try {
      const oneMonthAgo = new Date();
      oneMonthAgo.setMonth(oneMonthAgo.getMonth() - 1);

      // Find users who uploaded at least 5 approved samples in the last month
      const eligibleCreators = await Sample.aggregate([
        {
          $match: {
            status: 'approved',
            createdAt: { $gte: oneMonthAgo },
          },
        },
        {
          $group: {
            _id: '$creator',
            count: { $sum: 1 },
          },
        },
        {
          $match: {
            count: { $gte: 5 },
          },
        },
      ]);

      for (const creator of eligibleCreators) {
        console.log(`Creator ${creator._id} is eligible for content rewards.`);
        // Placeholder for reporting to blockchain
        // In a real scenario, this would call a specific blockchain extrinsic
        // to distribute content rewards.
        await registerSampleOnBlockchain(
          'content_reward',
          `reward_for_${creator._id}_${new Date().toISOString()}`,
          creator._id.toString()
        );
        console.log(`Content reward reported for creator ${creator._id}`);
      }
    } catch (error) {
      console.error('Error during monthly content reward check:', error);
    }
  });

  console.log('Reward system scheduled.');
};
