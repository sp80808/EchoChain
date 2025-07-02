import cron from 'node-cron';
import Sample from '../models/Sample';
import User from '../auth/User';

const BASE_REWARD = 50; // Base ECHO for eligible creators
const SAMPLE_USAGE_MULTIPLIER = 5; // ECHO per high-usage sample
const REFERRAL_MULTIPLIER = 10; // ECHO per referred user
const MIN_USAGE_COUNT = 10; // Minimum usage count for a sample to be considered 'high-usage'
import { distributeContentRewardsOnBlockchain, distributeNetworkRewardsOnBlockchain } from './blockchain';

export const setupRewardSystem = () => {
  // Schedule content rewards check for the first day of every month at midnight
  cron.schedule('0 0 1 * *', async () => {
    console.log('Running monthly content reward check...');
    try {
      const oneMonthAgo = new Date();
      oneMonthAgo.setMonth(oneMonthAgo.getMonth() - 1);

      // Find users who uploaded at least 5 approved samples in the last month
      // Find all approved samples created in the last month
      const approvedSamples = await Sample.find({
        status: 'approved',
        createdAt: { $gte: oneMonthAgo },
      }).populate('creator');

      // Group samples by creator and count high-usage samples
      const creatorData: { [key: string]: { creatorId: string; highUsageSamples: number; referredUsers: number } } = {};

      for (const sample of approvedSamples) {
        const creatorId = sample.creator._id.toString();
        if (!creatorData[creatorId]) {
          creatorData[creatorId] = { creatorId, highUsageSamples: 0, referredUsers: 0 };
        }
        if (sample.usageCount >= MIN_USAGE_COUNT) {
          creatorData[creatorId].highUsageSamples++;
        }
      }

      // Find referred users for each creator
      for (const creatorId in creatorData) {
        const referredUsers = await User.countDocuments({ referrerId: creatorId });
        creatorData[creatorId].referredUsers = referredUsers;
      }

      for (const creatorId in creatorData) {
        const { highUsageSamples, referredUsers } = creatorData[creatorId];
        let rewardAmount = BASE_REWARD;
        rewardAmount += highUsageSamples * SAMPLE_USAGE_MULTIPLIER;
        rewardAmount += referredUsers * REFERRAL_MULTIPLIER;

        console.log(`Creator ${creatorId} is eligible for content rewards: ${rewardAmount} ECHO.`);
        await distributeContentRewardsOnBlockchain(
          creatorId,
          rewardAmount
        );
        console.log(`Content reward reported for creator ${creatorId}`);
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

export const calculateUserFaucetAmount = async (userId: string): Promise<number> => {
  try {
    // Calculate high-usage samples for the specific user
    const highUsageSamplesCount = await Sample.countDocuments({
      creator: userId,
      status: 'approved',
      usageCount: { $gte: MIN_USAGE_COUNT },
    });

    // Calculate referred users for the specific user
    const referredUsersCount = await User.countDocuments({ referrerId: userId });

    let faucetAmount = BASE_REWARD;
    faucetAmount += highUsageSamplesCount * SAMPLE_USAGE_MULTIPLIER;
    faucetAmount += referredUsersCount * REFERRAL_MULTIPLIER;

    return faucetAmount;
  } catch (error) {
    console.error('Error calculating user faucet amount:', error);
    return 0; // Return 0 on error
  }
};
