#!/bin/bash
set -e

echo "Cleaning Blockchain Node..."
cd ../polkadot-sdk/substrate/node-template && cargo clean

cd ../../../Backend_API_Services && npm run clean || rm -rf dist node_modules

cd ../LandingPage/project && npm run clean || rm -rf dist node_modules

cd ../../P2P_File_Sharing_System/audio_analysis_service && find . -type d -name __pycache__ -exec rm -rf {} +

echo "Cleaned all build artifacts and caches!" 