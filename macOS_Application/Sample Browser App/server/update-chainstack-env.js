const fs = require('fs');
const fetch = require('node-fetch');

const CHAINSTACK_API_TOKEN = process.env.CHAINSTACK_API_TOKEN;
const NODE_ID = process.env.CHAINSTACK_NODE_ID;

if (!CHAINSTACK_API_TOKEN || !NODE_ID) {
  console.error('CHAINSTACK_API_TOKEN and CHAINSTACK_NODE_ID must be set as environment variables.');
  process.exit(1);
}

async function updateEnvWithEndpoints() {
  const res = await fetch(`https://api.chainstack.com/v1/nodes/${NODE_ID}`, {
    headers: { 'Authorization': `Bearer ${CHAINSTACK_API_TOKEN}` }
  });
  if (!res.ok) {
    console.error('Failed to fetch node details:', await res.text());
    process.exit(1);
  }
  const data = await res.json();

  if (!data.endpoints) {
    console.error('No endpoints found in API response:', JSON.stringify(data, null, 2));
    process.exit(1);
  }

  // Extract endpoints
  const ws = data.endpoints.find(e => e.type === 'wss');
  const https = data.endpoints.find(e => e.type === 'https');
  const sidecar = data.endpoints.find(e => e.type === 'sidecar');

  // Read and update .env
  let env = fs.readFileSync('.env', 'utf8');
  if (ws) env = env.replace(/POLKADOT_WS_ENDPOINT=.*/g, `POLKADOT_WS_ENDPOINT=${ws.url}`);
  if (https) env = env.replace(/POLKADOT_HTTPS_ENDPOINT=.*/g, `POLKADOT_HTTPS_ENDPOINT=${https.url}`);
  if (sidecar) env = env.replace(/POLKADOT_SIDECAR_ENDPOINT=.*/g, `POLKADOT_SIDECAR_ENDPOINT=${sidecar.url}`);
  fs.writeFileSync('.env', env);

  console.log('Updated .env with latest Chainstack endpoints!');
}

updateEnvWithEndpoints(); 