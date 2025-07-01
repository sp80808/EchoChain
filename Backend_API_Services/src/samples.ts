import { Request, Response } from 'express';
import { AppDataSource } from "./data-source";
import { Sample } from "./entity/Sample";
import { ApiError } from "./errors";
import logger from './logger';

const sampleRepository = AppDataSource.getRepository(Sample);

export const getSamples = async (req: Request, res: Response) => {
    try {
        const { search, category } = req.query;
        let samples: Sample[];

        if (search || category) {
            samples = await sampleRepository.find({
                where: [
                    search ? { title: new RegExp(search as string, 'i') } : {},
                    search ? { artist: new RegExp(search as string, 'i') } : {},
                    category ? { category: category as string } : {},
                ].filter(obj => Object.keys(obj).length > 0) // Filter out empty objects
            });
        } else {
            samples = await sampleRepository.find();
        }

        res.status(200).json(samples);
    } catch (error) {
        logger.error('Error fetching samples:', error);
        throw new ApiError('An unexpected error occurred while fetching samples.', 500);
    }
};

export const registerSample = async (req: Request, res: Response) => {
    const { id, title, artist, duration, category, p2pContentId, price, ownerAddress, blockchainHash } = req.body;

    if (!id || !title || !artist || !duration || !category || !p2pContentId || price == null || !ownerAddress || !blockchainHash) {
        throw new ApiError('All sample fields are required.', 400);
    }

    try {
        const newSample = sampleRepository.create({
            id,
            title,
            artist,
            duration,
            category,
            p2pContentId,
            price,
            ownerAddress,
            blockchainHash
        });
        await sampleRepository.save(newSample);
        res.status(201).json({ message: 'Sample registered successfully', sample: newSample });
    } catch (error) {
        logger.error('Error registering sample:', error);
        throw new ApiError('An unexpected error occurred during sample registration.', 500);
    }
};
