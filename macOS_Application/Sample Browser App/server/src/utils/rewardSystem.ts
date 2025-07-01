import cron from 'node-cron';
import Sample from '../models/Sample';
import { distributeContentRewardsOnBlockchain, distributeNetworkRewardsOnBlockchain } from './blockchain';

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
        await distributeContentRewardsOnBlockchain(
          creator._id.toString(),
          100 // Assuming 100 ECHO per eligible creator
        );
        console.log(`Content reward reported for creator ${creator._id}`);
      }
    } catch (error) {
      console.error('Error during monthly content reward check:', error);
    }
  });

  // Schedule network rewards check (e.g., daily for testing)
  cron.schedule('0 0 * * *', async () => {
    console.log('Running daily network reward check...');
    try {
      // In a real scenario, this would involve fetching network contribution data
      // from a decentralized source or client reports.
      // For now, we'll simulate a single contributor receiving a reward.
      const simulatedContributorAddress = "ECHO_CONTRIBUTOR_SIMULATED";
      const simulatedRewardAmount = 50; // Assuming 50 ECHO for network contribution

      await distributeNetworkRewardsOnBlockchain(
        simulatedContributorAddress,
        simulatedRewardAmount
      );
      console.log(`Network reward reported for ${simulatedContributorAddress}`);
    } catch (error) {
      console.error('Error during daily network reward check:', error);
    }
  });

  console.log('Reward system scheduled.');
};
