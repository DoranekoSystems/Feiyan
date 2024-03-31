#!/bin/bash

ARGS="$@"

echo "Building frontend..."
cd frontend
./build.sh $ARGS
cd ..

echo "Building backend..."
cd backend
./build.sh $ARGS
cd ..

echo "Build process completed."

sudo ./target/aarch64-apple-darwin/release/memory-server