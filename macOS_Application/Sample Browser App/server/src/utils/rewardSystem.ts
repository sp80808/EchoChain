/**
 * Reward system for EchoChain
 * Handles content rewards and network rewards distribution
 */

import { logger } from './logger';
import {
  distributeContentRewardsOnBlockchain,
  distributeNetworkRewardsOnBlockchain
} from './blockchain';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { cryptoWaitReady } from '@polkadot/util-crypto';

export interface ContentReward {
  creatorId: string;
  amount: number;
  reason: string;
}

export interface NetworkReward {
  contributorAddress: string;
  amount: number;
  bytesUploaded: number;
  bytesDownloaded: number;
}

class RewardSystem {
  private contentRewardInterval: NodeJS.Timeout | null = null;
  private networkRewardInterval: NodeJS.Timeout | null = null;
  
  // Configuration
  private readonly CONTENT_REWARD_INTERVAL = 24 * 60 * 60 * 1000; // 24 hours
  private readonly NETWORK_REWARD_INTERVAL = 7 * 24 * 60 * 60 * 1000; // 7 days
  private readonly MIN_APPROVED_SAMPLES = 5; // Minimum samples needed for content rewards
  private readonly CONTENT_REWARD_AMOUNT = 100; // ECHO tokens per eligible creator
  private readonly NETWORK_REWARD_POOL = 1000; // Total ECHO tokens for network rewards per period

  /**
   * Initialize the reward system with periodic distributions
   */
  public start(): void {
    logger.info('Starting EchoChain reward system...');
    
    // Start content rewards distribution
    this.contentRewardInterval = setInterval(
      () => this.distributeContentRewards(),
      this.CONTENT_REWARD_INTERVAL
    );
    
    // Start network rewards distribution
    this.networkRewardInterval = setInterval(
      () => this.distributeNetworkRewards(),
      this.NETWORK_REWARD_INTERVAL
    );
    
    // Run initial distribution (with delay to allow server to fully start)
    setTimeout(() => {
      this.distributeContentRewards();
      this.distributeNetworkRewards();
    }, 30000); // 30 seconds delay
    
    logger.info('Reward system started successfully');
  }

  /**
   * Stop the reward system
   */
  public stop(): void {
    if (this.contentRewardInterval) {
      clearInterval(this.contentRewardInterval);
      this.contentRewardInterval = null;
    }
    
    if (this.networkRewardInterval) {
      clearInterval(this.networkRewardInterval);
      this.networkRewardInterval = null;
    }
    
    logger.info('Reward system stopped');
  }

  /**
   * Distribute content rewards to eligible creators
   */
  private async distributeContentRewards(): Promise<void> {
    try {
      logger.info('Starting content rewards distribution...');
      
      // TODO: Implement logic to fetch eligible creators from database
      // For now, using placeholder logic
      const eligibleCreators = await this.getEligibleCreators();
      
      for (const creator of eligibleCreators) {
        const success = await distributeContentRewardsOnBlockchain(
          creator.id,
          this.CONTENT_REWARD_AMOUNT
        );
        
        if (success) {
          logger.info(`Content reward distributed to creator ${creator.id}: ${this.CONTENT_REWARD_AMOUNT} ECHO`);
        } else {
          logger.error(`Failed to distribute content reward to creator ${creator.id}`);
        }
      }
      
      logger.info(`Content rewards distribution completed. ${eligibleCreators.length} creators rewarded.`);
    } catch (error) {
      logger.error('Error during content rewards distribution:', error);
    }
  }

  /**
   * Distribute network rewards to contributors
   */
  private async distributeNetworkRewards(): Promise<void> {
    try {
      logger.info('Starting network rewards distribution...');
      
      // TODO: Implement logic to fetch network contributors from database
      // For now, using placeholder logic
      const contributors = await this.getNetworkContributors();
      
      if (contributors.length === 0) {
        logger.info('No network contributors found for this period');
        return;
      }
      
      // Calculate total contribution for proportional distribution
      const totalContribution = contributors.reduce(
        (sum, contributor) => sum + contributor.bytesUploaded + contributor.bytesDownloaded,
        0
      );
      
      for (const contributor of contributors) {
        const contribution = contributor.bytesUploaded + contributor.bytesDownloaded;
        const rewardAmount = Math.floor(
          (contribution / totalContribution) * this.NETWORK_REWARD_POOL
        );
        
        if (rewardAmount > 0) {
          const success = await distributeNetworkRewardsOnBlockchain(
            contributor.address,
            rewardAmount
          );
          
          if (success) {
            logger.info(`Network reward distributed to ${contributor.address}: ${rewardAmount} ECHO`);
          } else {
            logger.error(`Failed to distribute network reward to ${contributor.address}`);
          }
        }
      }
      
      logger.info(`Network rewards distribution completed. ${contributors.length} contributors rewarded.`);
    } catch (error) {
      logger.error('Error during network rewards distribution:', error);
    }
  }

  /**
   * Get eligible creators for content rewards
   * Fetches from blockchain to get real creator data
   */
  private async getEligibleCreators(): Promise<Array<{ id: string; approvedSamples: number }>> {
    try {
      const { getBlockchainApi } = await import('./blockchain');
      const api = await getBlockchainApi();
      
      // Query all creators from the sample registry
      const creators = await api.query.sampleRegistry.creators.entries();
      const eligibleCreators: Array<{ id: string; approvedSamples: number }> = [];
      
      for (const [key, value] of creators) {
        const creatorId = key.args[0].toString();
        const creatorData = value.toJSON() as any;
        
        // Count approved samples for this creator
        const creatorSamples = await api.query.sampleRegistry.samplesByCreator(creatorId);
        const approvedCount = (creatorSamples.toJSON() as any[]).filter(
          sample => sample.status === 'Approved'
        ).length;
        
        if (approvedCount >= this.MIN_APPROVED_SAMPLES) {
          eligibleCreators.push({
            id: creatorId,
            approvedSamples: approvedCount
          });
        }
      }
      
      logger.info(`Found ${eligibleCreators.length} eligible creators for content rewards`);
      return eligibleCreators;
    } catch (error) {
      logger.error('Error fetching eligible creators from blockchain:', error);
      
      // Fallback to placeholder data if blockchain is unavailable
      logger.warn('Using fallback creator data for content rewards');
      return [
        { id: 'creator1@example.com', approvedSamples: 10 },
        { id: 'creator2@example.com', approvedSamples: 7 },
        { id: 'creator3@example.com', approvedSamples: 15 }
      ].filter(creator => creator.approvedSamples >= this.MIN_APPROVED_SAMPLES);
    }
  }

  /**
   * Get network contributors for network rewards
   * TODO: Replace with actual database query
   */
  private async getNetworkContributors(): Promise<Array<{
    address: string;
    bytesUploaded: number;
    bytesDownloaded: number;
  }>> {
    // Placeholder implementation
    return [
      { address: 'ECHO_ADDRESS_1', bytesUploaded: 1000000, bytesDownloaded: 500000 },
      { address: 'ECHO_ADDRESS_2', bytesUploaded: 750000, bytesDownloaded: 800000 },
      { address: 'ECHO_ADDRESS_3', bytesUploaded: 2000000, bytesDownloaded: 300000 }
    ];
  }

  /**
   * Manually trigger content rewards distribution (for testing/admin)
   */
  public async triggerContentRewards(): Promise<void> {
    logger.info('Manually triggering content rewards distribution...');
    await this.distributeContentRewards();
  }

  /**
   * Manually trigger network rewards distribution (for testing/admin)
   */
  public async triggerNetworkRewards(): Promise<void> {
    logger.info('Manually triggering network rewards distribution...');
    await this.distributeNetworkRewards();
  }
}

// Create singleton instance
const rewardSystem = new RewardSystem();

/**
 * Setup and start the reward system
 */
export function setupRewardSystem(): void {
  rewardSystem.start();
}

/**
 * Stop the reward system
 */
export function stopRewardSystem(): void {
  rewardSystem.stop();
}

/**
 * Get access to the reward system instance for manual operations
 */
export function getRewardSystem(): RewardSystem {
  return rewardSystem;
}
