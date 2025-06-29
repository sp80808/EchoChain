import mongoose, { Document, Schema } from 'mongoose';

export interface ISample extends Document {
  title: string;
  description: string;
  category: string;
  tags: string[];
  bpm: number;
  key: string;
  creator: mongoose.Schema.Types.ObjectId;
  ipfsCid: string;
  metadataIpfsCid: string;
  status: 'pending' | 'approved' | 'rejected';
}

const SampleSchema: Schema = new Schema({
  title: { type: String, required: true },
  description: { type: String, required: true },
  category: { type: String, required: true },
  tags: { type: [String], required: true },
  bpm: { type: Number },
  key: { type: String },
  creator: { type: mongoose.Schema.Types.ObjectId, ref: 'User', required: true },
  ipfsCid: { type: String, required: true },
  metadataIpfsCid: { type: String, required: true },
  status: { type: String, enum: ['pending', 'approved', 'rejected'], default: 'pending' },
});

export default mongoose.model<ISample>('Sample', SampleSchema);
