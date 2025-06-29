#!/bin/bash
set -e

echo "Testing Blockchain Node..."
cd ../polkadot-sdk/substrate/node-template && cargo test

cd ../../../Backend_API_Services && npm test

cd ../LandingPage/project && npm test

cd ../../P2P_File_Sharing_System/audio_analysis_service && source ../venv/bin/activate && pytest

echo "All tests completed!" 