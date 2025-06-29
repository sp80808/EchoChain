#!/bin/bash
# Usage: ./distribute-network-rewards.sh <sudo-seed>
SUDO_SEED="$1"
NODE_URL="ws://127.0.0.1:9944"

subxt tx sudo sudo --call "networkRewards.distribute_network_rewards()" --url $NODE_URL --signer "$SUDO_SEED" 