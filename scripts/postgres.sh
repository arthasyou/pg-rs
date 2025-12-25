#!/usr/bin/env bash

CONTAINER_NAME="pg-tables"
DATA_DIR="$HOME/db/postgres-tables/"
POSTGRES_PASSWORD="123456"
PORT="15432"
IMAGE="postgres:latest"

echo "🔍 检查本地数据目录：$DATA_DIR"
if [ ! -d "$DATA_DIR" ]; then
  echo "📁 数据目录不存在，创建中..."
  mkdir -p "$DATA_DIR"
  chmod 700 "$DATA_DIR"
  echo "✔ 数据目录创建完成。"
fi

echo "🔍 检查是否已有同名容器：$CONTAINER_NAME"
if docker ps -a --format '{{.Names}}' | grep -w "$CONTAINER_NAME" > /dev/null; then
  echo "⚠️ 已存在容器：$CONTAINER_NAME"
  echo "❗ 停止并删除旧容器..."
  docker stop "$CONTAINER_NAME" > /dev/null 2>&1
  docker rm "$CONTAINER_NAME" > /dev/null 2>&1
  echo "✔ 旧容器已删除。"
fi

echo "🚀 启动 PostgreSQL 容器..."
docker run -d \
  --name "$CONTAINER_NAME" \
  --restart=always \
  -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
  -v "$DATA_DIR":/var/lib/postgresql \
  -p "$PORT":5432 \
  "$IMAGE"

if [ $? -eq 0 ]; then
  echo ""
  echo "🎉 PostgreSQL 启动成功！"
  echo "📦 容器名：$CONTAINER_NAME"
  echo "🔁 自动重启：已启用 (--restart=always)"
  echo "📁 数据持久化目录：$DATA_DIR"
  echo "🔐 数据库密码：$POSTGRES_PASSWORD"
else
  echo "❌ 启动失败，请检查 Docker 是否正常运行。"
fi
