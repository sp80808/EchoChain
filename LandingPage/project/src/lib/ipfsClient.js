// src/lib/ipfsClient.js

import { create } from 'ipfs-http-client';
import { IPFS_API_URL } from './constants';

const ipfs = create({ url: IPFS_API_URL });

export const uploadFileToIPFS = async (file) => {
  try {
    const { cid } = await ipfs.add(file);
    console.log('Uploaded to IPFS. CID:', cid.toString());
    return cid.toString();
  } catch (error) {
    console.error('Error uploading to IPFS:', error);
    throw new Error('Failed to upload file to IPFS.');
  }
};

export const uploadJsonToIPFS = async (jsonObject) => {
  try {
    const jsonString = JSON.stringify(jsonObject);
    const { cid } = await ipfs.add(jsonString);
    console.log('Uploaded JSON to IPFS. CID:', cid.toString());
    return cid.toString();
  } catch (error) {
    console.error('Error uploading JSON to IPFS:', error);
    throw new Error('Failed to upload JSON to IPFS.');
  }
};
