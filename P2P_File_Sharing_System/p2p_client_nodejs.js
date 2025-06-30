// Node.js P2P Client for EchoChain Python P2P Node
const net = require('net');
const fs = require('fs');

class P2PClient {
  constructor(host = '127.0.0.1', port = 8002) {
    this.host = host;
    this.port = port;
  }

  _sendCommand(commandType, payload) {
    return new Promise((resolve, reject) => {
      const client = net.createConnection({ host: this.host, port: this.port }, () => {
        const message = JSON.stringify({ type: commandType, payload });
        client.write(message);
      });
      client.on('data', (data) => {
        try {
          const response = JSON.parse(data.toString());
          client.end();
          resolve(response);
        } catch (e) {
          client.end();
          reject(e);
        }
      });
      client.on('error', reject);
    });
  }

  async addFileAndAnnounce(filepath) {
    const addResp = await this._sendCommand('local_add_file', { filepath });
    if (addResp.status !== 'success') throw new Error(addResp.message);
    const fileHash = addResp.file_hash;
    const announceResp = await this._sendCommand('local_announce_content', { content_hash: fileHash });
    if (announceResp.status !== 'success') throw new Error(announceResp.message);
    return fileHash;
  }

  async discoverContentPeers(contentHash) {
    const resp = await this._sendCommand('local_request_content_info', { content_hash: contentHash });
    if (resp.status !== 'success') throw new Error(resp.message);
    return resp.peers;
  }

  async requestFileDownload(contentHash) {
    const resp = await this._sendCommand('local_request_file', { content_hash: contentHash });
    if (resp.status !== 'success') throw new Error(resp.message);
    return true;
  }
}

// Example usage:
(async () => {
  const client = new P2PClient('127.0.0.1', 8002);
  try {
    const fileHash = await client.addFileAndAnnounce('path/to/your/file.wav');
    console.log('File added and announced with hash:', fileHash);

    const peers = await client.discoverContentPeers(fileHash);
    console.log('Peers for content:', peers);

    const downloadStarted = await client.requestFileDownload(fileHash);
    console.log('Download started:', downloadStarted);
  } catch (err) {
    console.error('P2P error:', err);
  }
})(); 