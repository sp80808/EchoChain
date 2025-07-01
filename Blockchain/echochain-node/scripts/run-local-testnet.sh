#!/usr/bin/env bash
set -e

# EchoChain Local Testnet Launcher
# Launches Alice, Bob, and Charlie nodes for local development/testing

NODES=(Alice Bob Charlie)
BASE_PORT=30333
RPC_PORT=9944
WS_PORT=9945
CHAIN=local-testnet
BIN=../node/target/release/echochain-node

function build_node() {
  if [ ! -f "$BIN" ]; then
    echo "*** Building EchoChain node binary..."
    (cd ../node && cargo build --release)
  fi
}

function cleanup() {
  echo "*** Stopping all nodes and cleaning up..."
  pkill -f echochain-node || true
  for NODE in "${NODES[@]}"; do
    rm -rf /tmp/echochain-$NODE
  done
}

function launch_nodes() {
  for i in "${!NODES[@]}"; do
    NODE=${NODES[$i]}
    PORT=$((BASE_PORT + i))
    RPC=$((RPC_PORT + i))
    WS=$((WS_PORT + i))
    echo "*** Launching $NODE on p2p:$PORT rpc:$RPC ws:$WS..."
    $BIN \
      --chain $CHAIN \
      --base-path /tmp/echochain-$NODE \
      --name $NODE \
      --port $PORT \
      --rpc-port $RPC \
      --ws-port $WS \
      --validator \
      --alice --bob --charlie \
      --rpc-cors=all \
      --rpc-methods=Unsafe \
      --ws-external \
      --rpc-external \
      > /tmp/echochain-$NODE.log 2>&1 &
  done
  echo "*** All nodes launched. Logs: /tmp/echochain-<Node>.log"
}

function deploy_contracts_and_fund_accounts() {
  echo "*** Installing JavaScript dependencies for deployment script..."
  (cd .. && npm install @polkadot/api)

  echo "*** Waiting for nodes to be ready (checking Alice's RPC)..."
  until curl -s -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method":"system_health", "params":[]}' http://127.0.0.1:9944 > /dev/null; do
    sleep 1
  done
  echo "*** Alice node is ready."

  echo "*** Executing deployment and funding script..."
  node deploy-and-fund.js
}

function usage() {
  echo "Usage: $0 [start|stop|restart]"
  echo "  start   - Build and launch local testnet nodes (default)"
  echo "  stop    - Stop all nodes and clean up"
  echo "  restart - Stop, clean up, and relaunch nodes"
}

case "$1" in
  stop)
    cleanup
    ;;
  restart)
    cleanup
    build_node
    launch_nodes
    deploy_contracts_and_fund_accounts
    ;;
  start|"")
    build_node
    launch_nodes
    deploy_contracts_and_fund_accounts
    ;;
  *)
    usage
    ;;
esac 