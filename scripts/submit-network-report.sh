#!/bin/bash
# Usage: ./submit-network-report.sh <account-seed> <uploaded-bytes> <downloaded-bytes>
ACCOUNT_SEED="$1"
UPLOADED="$2"
DOWNLOADED="$3"
NODE_URL="ws://127.0.0.1:9944"

subxt tx networkRewards submit_report $UPLOADED $DOWNLOADED --url $NODE_URL --signer "$ACCOUNT_SEED" 