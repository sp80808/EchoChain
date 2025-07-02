import 'dotenv/config';
import express from 'express';
import cors from 'cors';
import connectDB from './db';
import authRoutes from './routes/auth';
import sampleRoutes from './routes/samples';
import userRoutes from './routes/users';
import { setupRewardSystem } from './utils/rewardSystem';

const app = express();
const port = process.env.PORT || 3001;

// Connect to database
connectDB();

app.use(cors());
app.use(express.json());

app.use('/api/auth', authRoutes);
app.use('/api/samples', sampleRoutes);
app.use('/api/users', userRoutes);

app.get('/', (req, res) => {
  res.send('EchoChain Server is running!');
});

app.listen(port, () => {
  logger.info(`Server is running on http://localhost:${port}`);
  setupRewardSystem(); // Initialize reward system
});
