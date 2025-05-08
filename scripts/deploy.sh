#!/bin/bash

echo "🚀 Starting NewsWave deployment..."

# 构建前端
echo "Building frontend..."
cd frontend
npm run build

# 构建后端
echo "Building backend..."
cd ../backend
pip install -r requirements.txt

# 部署智能合约
echo "Deploying smart contracts..."
cd ../contracts
cargo build --release

echo "✅ Deployment completed successfully!"