#!/bin/bash

# 确保脚本可执行
chmod +x scripts/*.sh

# 启动开发环境
echo "🚀 Starting development environment..."

# 启动 MongoDB
docker-compose up -d mongodb

# 启动后端服务
cd backend
python -m uvicorn src.main:app --reload &

# 启动前端开发服务器
cd ../frontend
npm run dev