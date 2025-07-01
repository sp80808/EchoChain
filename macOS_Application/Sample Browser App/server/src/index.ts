import express from 'express';
import cors from 'cors';
import connectDB from './db';
import authRoutes from './routes/auth';
import sampleRoutes from './routes/samples';
import { setupRewardSystem } from './utils/rewardSystem';

const app = express();
const port = process.env.PORT || 3001;

// Connect to database
connectDB();

app.use(cors());
app.use(express.json());

app.use('/api/auth', authRoutes);
app.use('/api/samples', sampleRoutes);

app.get('/', (req, res) => {
  res.send('EchoChain Server is running!');
});

app.listen(port, () => {
  console.log(`Server is running on http://localhost:${port}`);
  setupRewardSystem(); // Initialize reward system
});
