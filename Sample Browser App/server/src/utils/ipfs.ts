import { create } from 'ipfs-http-client';

const ipfs = create({ host: 'localhost', port: 5001, protocol: 'http' });

export const uploadToIpfs = async (data: string | Buffer) => {
  const { cid } = await ipfs.add(data);
  return cid.toString();
};

export const downloadFromIpfs = async (cid: string): Promise<Buffer> => {
  const chunks: Uint8Array[] = [];
  for await (const chunk of ipfs.cat(cid)) {
    chunks.push(chunk);
  }
  return Buffer.concat(chunks);
};