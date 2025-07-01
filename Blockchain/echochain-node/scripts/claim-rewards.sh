#!/bin/bash
# Usage: ./claim-rewards.sh <account-seed>
ACCOUNT_SEED="$1"
NODE_URL="ws://127.0.0.1:9944"

subxt tx proofOfContribution claim_rewards --url $NODE_URL --signer "$ACCOUNT_SEED" 