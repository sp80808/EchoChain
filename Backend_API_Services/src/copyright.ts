import { Request, Response } from 'express';
import axios from 'axios'; // For making HTTP requests

// Mock external copyright check service endpoint
const MOCK_COPYRIGHT_SERVICE_URL = 'https://api.mockcopyright.com/check'; // Placeholder

// Helper function for retries with exponential backoff
async function retry<T>(fn: () => Promise<T>, retries: number = 3, delay: number = 1000): Promise<T> {
  try {
    return await fn();
  } catch (error) {
    if (retries > 0) {
      console.warn(`Retrying after error: ${(error as Error).message}. Retries left: ${retries}`);
      await new Promise(res => setTimeout(res, delay));
      return retry(fn, retries - 1, delay * 2); // Exponential backoff
    }
    throw error;
  }
}

export const checkCopyright = async (req: Request, res: Response) => {
  const { sampleId, userId, contentHash } = req.body; // Added contentHash for more realistic check

  // Input Validation
  if (!sampleId || !userId || !contentHash) {
    return res.status(400).json({ message: 'Sample ID, User ID, and Content Hash are required for copyright check.' });
  }

  // Basic validation for contentHash format (e.g., SHA256 hash)
  if (typeof contentHash !== 'string' || !/^[a-f0-9]{64}$/i.test(contentHash)) {
    return res.status(400).json({ message: 'Invalid content hash format. Must be a SHA256 hash.' });
  }

  try {
    // Simulate an HTTP request to an external copyright check service
    // In a real application, you would configure axios with proper base URLs, timeouts, etc.
    const externalServiceCall = async () => {
      console.log(`Attempting copyright check for sampleId: ${sampleId}, userId: ${userId}, contentHash: ${contentHash}`);
      // Simulate network delay and potential failures
      await new Promise(resolve => setTimeout(resolve, Math.random() * 500 + 100)); // 100-600ms delay

      if (Math.random() < 0.1) { // 10% chance of network error
        throw new Error('Simulated network error during copyright check.');
      }

      // Mock response from the external service
      const mockResponse = {
        sampleId: sampleId,
        contentHash: contentHash,
        isCopyrightInfringement: Math.random() > 0.8, // 20% chance of infringement
        details: 'Mock copyright check completed. This is a simulated response.',
        source: 'MockCopyrightService',
        timestamp: new Date().toISOString(),
      };
      return mockResponse;
    };

    const result = await retry(externalServiceCall, 3, 500); // Retry up to 3 times with initial 500ms delay

    // Data mapping and response handling
    if (result.isCopyrightInfringement) {
      return res.status(403).json({
        message: 'Copyright infringement detected.',
        data: {
          sampleId: result.sampleId,
          infringement: true,
          details: result.details,
          checkedAt: result.timestamp,
        }
      });
    } else {
      return res.status(200).json({
        message: 'No copyright infringement detected.',
        data: {
          sampleId: result.sampleId,
          infringement: false,
          details: result.details,
          checkedAt: result.timestamp,
        }
      });
    }

  } catch (error: unknown) { // Explicitly type error as unknown
    console.error('Error during copyright check:', error);
    // Differentiate between external service errors and internal errors if possible
    if (axios.isAxiosError(error)) {
      console.error('Axios error details:', error.response?.data || error.message);
      return res.status(502).json({ message: 'External copyright service unavailable or returned an error.' });
    } else if (error instanceof Error) {
      console.error('General error details:', error.message);
    }
    return res.status(500).json({ message: 'An unexpected server error occurred during copyright check.' });
  }
};