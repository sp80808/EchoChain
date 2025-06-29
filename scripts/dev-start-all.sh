#!/bin/bash
set -e

echo "Starting Blockchain Node..."
cd ../polkadot-sdk/substrate/node-template && ./target/release/node-template --dev &
NODE_PID=$!

cd ../../../Backend_API_Services && npm start &
BACKEND_PID=$!

cd ../LandingPage/project && npm start &
FRONTEND_PID=$!

cd ../../P2P_File_Sharing_System/audio_analysis_service && source ../venv/bin/activate && uvicorn main:app --reload &
P2P_PID=$!

trap "kill $NODE_PID $BACKEND_PID $FRONTEND_PID $P2P_PID" EXIT
wait 