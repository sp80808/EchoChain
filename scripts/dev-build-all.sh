#!/bin/bash
set -e

echo "Building Blockchain Node..."
cd ../polkadot-sdk/substrate/node-template && cargo build --release

cd ../../../Backend_API_Services && npm install && npm run build

cd ../LandingPage/project && npm install && npm run build

cd ../../P2P_File_Sharing_System/audio_analysis_service && source ../venv/bin/activate && pip install -r requirements.txt

echo "All components built successfully!" 