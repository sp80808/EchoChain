import { Router, Request, Response, NextFunction } from 'express';
import multer from 'multer';
import auth from '../middleware/auth';
import Sample from '../models/Sample';
import { uploadToIpfs } from '../utils/ipfs';
import { checkOriginality } from '../utils/audioFingerprinting';
import { registerSampleOnBlockchain, getSampleMetadataFromBlockchain } from '../utils/blockchain';
import { analyzeAudioFile } from '../utils/audioAnalysis';
import { separateStems } from '../utils/stemSeparation';
import fs from 'fs';
import path from 'path';

const router = Router();
const upload = multer({ dest: 'uploads/' });

// Upload Sample
router.post('/', [auth, upload.single('sample')], async (req, res) => {
  const { title, description, category, tags, separateStems: shouldSeparateStems } = req.body;
  let { bpm, key } = req.body; // Allow overriding auto-detected values

  try {
    if (!req.file) {
      return res.status(400).json({ msg: 'No file uploaded' });
    }

    const filePath = req.file.path;
    const fileContent = fs.readFileSync(filePath);

    // Auto-detect BPM and Key if not provided
    if (!bpm || !key) {
      const analysisResult = await analyzeAudioFile(filePath);
      if (analysisResult.error) {
        console.warn(`Audio analysis failed: ${analysisResult.error}`);
      } else {
        bpm = bpm || analysisResult.bpm;
        key = key || analysisResult.key;
      }
    }

    // 1. Upload the original file to IPFS and get the CID.
    const ipfsCid = await uploadToIpfs(fileContent);

    // 2. Create a metadata JSON file and upload it to IPFS.
    const metadata = {
      title,
      description,
      category,
      tags: tags.split(',').map((tag: string) => tag.trim()),
      bpm: bpm ? parseInt(bpm) : undefined,
      key,
      creator: req.user.id,
      fileIpfsCid: ipfsCid,
    };
    const metadataIpfsCid = await uploadToIpfs(JSON.stringify(metadata));

    // 3. Call the audio fingerprinting API
    const isOriginal = await checkOriginality(filePath);
    if (!isOriginal) {
      fs.unlinkSync(filePath); // Clean up uploaded file
      return res.status(400).json({ msg: 'Sample is not original' });
    }

    // 4. Register the original sample on the blockchain
    const blockchainRegistered = await registerSampleOnBlockchain(ipfsCid, metadataIpfsCid, req.user.id);
    if (!blockchainRegistered) {
      fs.unlinkSync(filePath); // Clean up uploaded file
      return res.status(500).json({ msg: 'Failed to register sample on blockchain' });
    }

    const newSample = new Sample({
      title,
      description,
      category,
      tags: tags.split(',').map((tag: string) => tag.trim()),
      bpm: bpm ? parseInt(bpm) : undefined,
      key,
      creator: req.user.id,
      ipfsCid,
      metadataIpfsCid,
      status: 'pending', // Set to pending until blockchain confirmation
    });

    const sample = await newSample.save();

    // 5. Handle Stem Separation if requested
    if (shouldSeparateStems === 'true') {
      const outputDir = path.join(__dirname, '..', '..', 'uploads', 'stems', sample._id.toString());
      fs.mkdirSync(outputDir, { recursive: true });

      const separationResult = await separateStems(filePath, outputDir);

      if (separationResult.success) {
        const stemFiles = fs.readdirSync(outputDir);
        for (const stemFile of stemFiles) {
          const stemFilePath = path.join(outputDir, stemFile);
          const stemFileContent = fs.readFileSync(stemFilePath);
          const stemIpfsCid = await uploadToIpfs(stemFileContent);

          const stemMetadata = {
            title: `${title} - ${stemFile.replace('.wav', '')}`,
            description: `Stem from ${title}: ${stemFile.replace('.wav', '')}`,
            category: category,
            tags: [...tags.split(',').map((tag: string) => tag.trim()), 'stem', stemFile.replace('.wav', '')],
            bpm: bpm ? parseInt(bpm) : undefined,
            key,
            creator: req.user.id,
            fileIpfsCid: stemIpfsCid,
            parentSampleId: sample._id, // Link to the original sample
          };
          const stemMetadataIpfsCid = await uploadToIpfs(JSON.stringify(stemMetadata));

          // Register stem on blockchain (placeholder)
          await registerSampleOnBlockchain(stemIpfsCid, stemMetadataIpfsCid, req.user.id);

          const newStemSample = new Sample({
            title: stemMetadata.title,
            description: stemMetadata.description,
            category: stemMetadata.category,
            tags: stemMetadata.tags,
            bpm: stemMetadata.bpm,
            key: stemMetadata.key,
            creator: req.user.id,
            ipfsCid: stemIpfsCid,
            metadataIpfsCid: stemMetadataIpfsCid,
            status: 'pending',
          });
          await newStemSample.save();
        }
        fs.rmSync(outputDir, { recursive: true, force: true }); // Clean up stem files
      } else {
        logger.error(`Stem separation failed: ${separationResult.error}`);
      }
    }

    fs.unlinkSync(filePath); // Clean up original uploaded file

    res.json(sample);
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

// Get all approved samples
router.get('/', async (req: Request, res: Response, next: NextFunction) => {
  try {
    const { category, tags, bpm, key, type, search, sortBy, order } = req.query;
    const query: any = { status: 'approved' };

    if (category) {
      query.category = category;
    }

    if (tags) {
      // Allow multiple tags separated by comma
      const tagArray = (tags as string).split(',').map(tag => tag.trim());
      query.tags = { $in: tagArray };
    }

    if (bpm) {
      query.bpm = parseInt(bpm as string);
    }

    if (key) {
      query.key = key;
    }

    // New: Filter by type (one-shot, loop, fx) using tags
    if (type) {
      const typeTags = (type as string).split(',').map(t => t.trim().toLowerCase());
      query.tags = query.tags || {};
      if (query.tags.$in) {
        query.tags.$in = [...query.tags.$in, ...typeTags];
      } else {
        query.tags.$in = typeTags;
      }
    }

    if (search) {
      query.$or = [
        { title: { $regex: search, $options: 'i' } },
        { description: { $regex: search, $options: 'i' } },
        { tags: { $regex: search, $options: 'i' } },
      ];
    }

    let sortOptions: { [key: string]: 1 | -1 } = { createdAt: -1 }; // Default sort by newest

    if (sortBy) {
      switch (sortBy) {
        case 'downloads':
          sortOptions = { usageCount: order === 'asc' ? 1 : -1 };
          break;
        case 'uploadDate':
          sortOptions = { createdAt: order === 'asc' ? 1 : -1 };
          break;
        // Add other sort options as needed (e.g., 'title', 'artist')
        case 'title':
          sortOptions = { title: order === 'asc' ? 1 : -1 };
          break;
        case 'artist':
          sortOptions = { 'creator.email': order === 'asc' ? 1 : -1 }; // Assuming creator.email is the artist name
          break;
        default:
          // Fallback to default sort if sortBy is not recognized
          break;
      }
    }

    // Fetch samples from MongoDB based on query and sort options
    const samplesFromDb = await Sample.find(query).populate('creator', 'email walletAddress').sort(sortOptions);

    // For approved samples, simulate fetching metadata from blockchain
    const finalSamples = await Promise.all(samplesFromDb.map(async (sample) => {
      if (sample.status === 'approved') {
        const blockchainMetadata = await getSampleMetadataFromBlockchain((sample._id as any).toString());
        if (blockchainMetadata) {
          return { ...sample.toObject(), ...blockchainMetadata };
        } else {
          // If blockchain metadata not found, return original from DB
          return sample.toObject();
        }
      } else {
        return sample.toObject();
      }
    }));

    res.json(finalSamples);
  } catch (err) {
    console.error((err as Error).message);
    res.status(500).send('Server error');
  }
});

// Download Sample
router.get('/my', auth, async (req, res) => {
  try {
    // Fetch samples from MongoDB based on creator
    const samplesFromDb = await Sample.find({ creator: req.user.id }).populate('creator', 'email walletAddress');

    // For approved samples, simulate fetching metadata from blockchain
    const finalSamples = await Promise.all(samplesFromDb.map(async (sample) => {
      if (sample.status === 'approved') {
        const blockchainMetadata = await getSampleMetadataFromBlockchain(sample._id.toString());
        if (blockchainMetadata) {
          return { ...sample.toObject(), ...blockchainMetadata };
        } else {
          // If blockchain metadata not found, return original from DB
          return sample.toObject();
        }
      } else {
        return sample.toObject();
      }
    }));

    res.json(finalSamples);
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

router.get('/:id/download', auth, async (req, res) => {
  try {
    const sample = await Sample.findById(req.params.id);
    if (!sample) {
      return res.status(404).json({ msg: 'Sample not found' });
    }

    const fileContent = await downloadFromIpfs(sample.ipfsCid);

    // Set appropriate headers for file download
    res.set({
      'Content-Type': 'audio/mpeg', // Assuming MP3 for now, adjust based on actual file type
      'Content-Disposition': `attachment; filename="${sample.title}.mp3"`,
    });
    res.send(fileContent);

    // Increment usageCount with debounce
    const userId = req.user.id;
    const sampleId = sample._id.toString();
    const now = Date.now();

    if (!downloadDebounce[userId]) {
      downloadDebounce[userId] = {};
    }

    if (!downloadDebounce[userId][sampleId] || (now - downloadDebounce[userId][sampleId] > DEBOUNCE_INTERVAL_MS)) {
      sample.usageCount = (sample.usageCount || 0) + 1;
      await sample.save();
      downloadDebounce[userId][sampleId] = now;
      console.log(`Sample ${sample.title} usageCount incremented to ${sample.usageCount}`);
    } else {
      console.log(`Sample ${sample.title} download debounced for user ${userId}`);
    }

  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

// Report Network Contribution
router.post('/contributions/network', auth, async (req, res) => {
  const { storageBytes, bandwidthBytes } = req.body;

  try {
    const reported = await reportNetworkContributionToBlockchain(
      req.user.walletAddress,
      storageBytes,
      bandwidthBytes
    );

    if (reported) {
      res.json({ msg: 'Network contribution reported successfully' });
    } else {
      res.status(500).json({ msg: 'Failed to report network contribution' });
    }
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

// Admin route to approve a sample (simulated blockchain interaction)
router.post('/:id/approve', auth, async (req, res) => {
  try {
    // In a real application, this route would be protected by an admin role
    const sample = await Sample.findById(req.params.id);
    if (!sample) {
      return res.status(404).json({ msg: 'Sample not found' });
    }

    const approvedOnBlockchain = await approveSampleOnBlockchain(sample._id.toString());

    if (approvedOnBlockchain) {
      sample.status = 'approved';
      await sample.save();
      res.json({ msg: 'Sample approved successfully', sample });
    } else {
      res.status(500).json({ msg: 'Failed to approve sample on blockchain' });
    }
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

// Tip a creator
router.post('/:id/tip', auth, async (req, res) => {
  const { amount } = req.body;

  try {
    const sample = await Sample.findById(req.params.id);
    if (!sample) {
      return res.status(404).json({ msg: 'Sample not found' });
    }

    // Placeholder for blockchain tipping transaction
    // In a real scenario, this would involve a blockchain transaction
    // to transfer tokens from the tipper to the creator.
    const tipSuccessful = await transferTokensOnBlockchain(
      req.user.walletAddress, // Assuming req.user has walletAddress from auth middleware
      sample.creator.walletAddress, // Assuming sample.creator has walletAddress
      amount
    );

    if (tipSuccessful) {
      res.json({ msg: `Successfully tipped ${amount} ECHO to ${sample.creator.email}` });
    } else {
      res.status(500).json({ msg: 'Failed to process tip transaction' });
    }
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

// Get blockchain statistics
router.get('/stats', async (req, res) => {
  try {
    const stats = await getBlockchainStats();
    res.json(stats);
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

// Submit a new proposal
router.post('/governance/proposals', auth, async (req, res) => {
  const { title, description } = req.body;

  try {
    const submitted = await submitProposalToBlockchain(
      req.user.walletAddress,
      title,
      description
    );

    if (submitted) {
      res.json({ msg: 'Proposal submitted successfully' });
    } else {
      res.status(500).json({ msg: 'Failed to submit proposal' });
    }
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

// Vote on a proposal
router.post('/governance/proposals/:id/vote', auth, async (req, res) => {
  const { vote } = req.body; // 'aye' or 'nay'

  try {
    const voted = await voteOnProposalOnBlockchain(
      req.user.walletAddress,
      req.params.id,
      vote
    );

    if (voted) {
      res.json({ msg: `Vote cast successfully for proposal ${req.params.id}` });
    } else {
      res.status(500).json({ msg: 'Failed to cast vote' });
    }
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

// Trigger stem separation for an existing sample
router.post('/:id/separate-stems', auth, async (req, res) => {
  try {
    const sample = await Sample.findById(req.params.id);
    if (!sample) {
      return res.status(404).json({ msg: 'Sample not found' });
    }

    // For now, we'll simulate the file path. In a real scenario, you'd need to retrieve the actual file.
    // This would likely involve downloading from IPFS first.
    const dummyFilePath = `/tmp/${sample._id}.wav`; // Placeholder
    // Create a dummy file for the Python script to process
    fs.writeFileSync(dummyFilePath, "dummy audio data");

    const outputDir = path.join(__dirname, '..', '..', 'uploads', 'stems', sample._id.toString());
    fs.mkdirSync(outputDir, { recursive: true });

    const separationResult = await separateStems(dummyFilePath, outputDir);

    if (separationResult.success) {
      const stemFiles = fs.readdirSync(outputDir);
      for (const stemFile of stemFiles) {
        const stemFilePath = path.join(outputDir, stemFile);
        const stemFileContent = fs.readFileSync(stemFilePath);
        const stemIpfsCid = await uploadToIpfs(stemFileContent);

        const stemMetadata = {
          title: `${sample.title} - ${stemFile.replace('.wav', '')}`,
          description: `Stem from ${sample.title}: ${stemFile.replace('.wav', '')}`,
          category: sample.category,
          tags: [...sample.tags, 'stem', stemFile.replace('.wav', '')],
          bpm: sample.bpm,
          key: sample.key,
          creator: sample.creator,
          fileIpfsCid: stemIpfsCid,
          parentSampleId: sample._id, // Link to the original sample
        };
        const stemMetadataIpfsCid = await uploadToIpfs(JSON.stringify(stemMetadata));

        // Register stem on blockchain (placeholder)
        await registerSampleOnBlockchain(stemIpfsCid, stemMetadataIpfsCid, sample.creator.toString());

        const newStemSample = new Sample({
          title: stemMetadata.title,
          description: stemMetadata.description,
          category: stemMetadata.category,
          tags: stemMetadata.tags,
          bpm: stemMetadata.bpm,
          key: stemMetadata.key,
          creator: stemMetadata.creator,
          ipfsCid: stemIpfsCid,
          metadataIpfsCid: stemMetadataIpfsCid,
          status: 'pending',
        });
        await newStemSample.save();
      }
      fs.rmSync(outputDir, { recursive: true, force: true }); // Clean up stem files
      fs.unlinkSync(dummyFilePath); // Clean up dummy file
      res.json({ msg: 'Stem separation initiated successfully' });
    } else {
      fs.unlinkSync(dummyFilePath); // Clean up dummy file
      res.status(500).json({ msg: `Stem separation failed: ${separationResult.error}` });
    }
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

export default router;

// Get all proposals
router.get('/governance/proposals', async (req, res) => {
  try {
    const proposals = await getProposalsFromBlockchain();
    res.json(proposals);
  } catch (err) {
    console.error(err.message);
    res.status(500).send('Server error');
  }
});

export default router;
