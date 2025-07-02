export function getSidecarEndpoint() {
  const username = process.env.POLKADOT_NODE_USERNAME;
  const password = process.env.POLKADOT_NODE_PASSWORD;
  const sidecarAuth = process.env.POLKADOT_SIDECAR_ENDPOINT_AUTH;
  const sidecarPublic = process.env.POLKADOT_SIDECAR_ENDPOINT;
  if (username && password && sidecarAuth) {
    const url = new URL(sidecarAuth);
    url.username = username;
    url.password = password;
    return url.toString();
  }
  return sidecarPublic;
}

export function getHttpsEndpoint() {
  const username = process.env.POLKADOT_NODE_USERNAME;
  const password = process.env.POLKADOT_NODE_PASSWORD;
  const httpsAuth = process.env.POLKADOT_HTTPS_ENDPOINT_AUTH;
  const httpsPublic = process.env.POLKADOT_HTTPS_ENDPOINT;
  if (username && password && httpsAuth) {
    const url = new URL(httpsAuth);
    url.username = username;
    url.password = password;
    return url.toString();
  }
  return httpsPublic;
} 