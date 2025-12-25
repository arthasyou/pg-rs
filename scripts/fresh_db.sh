#!/bin/bash

# ============================================
#  SeaORM 一键执行脚本
#  执行内容：
#    1. migrate up
#    2. migrate refresh
#    3. generate entity
#  可按需扩展
# ============================================

# 获取脚本所在目录的父目录（项目根目录）
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# migration 目录
MIGRATION_DIR="$PROJECT_ROOT/crates/migration"
# entity 生成输出目录
ENTITY_DIR="$PROJECT_ROOT/crates/pg-tables/src/entity"

echo "============================================"
echo " SeaORM 一键执行工具"
echo " Project:    $PROJECT_ROOT"
echo " Migration:  $MIGRATION_DIR"
echo " Entity:     $ENTITY_DIR"
echo "============================================"

# 切换到项目根目录
cd "$PROJECT_ROOT" || exit 1

# # Step 1: migrate up
# echo "🚀 执行 migrate up ..."
# sea-orm-cli migrate up -d "$MIGRATION_DIR"
# if [ $? -ne 0 ]; then
#   echo "❌ migrate up 失败"
#   exit 1
# fi

# Step 2: migrate refresh（先 reset 再 up）
echo "🔄 执行 migrate refresh ..."
sea-orm-cli migrate refresh -d "$MIGRATION_DIR"
if [ $? -ne 0 ]; then
  echo "❌ migrate refresh 失败"
  exit 1
fi

# Step 3: generate entity
echo "📦 生成 entity ..."
sea-orm-cli generate entity \
  -o "$ENTITY_DIR" \
  --with-serde both \
  --date-time-crate time

if [ $? -ne 0 ]; then
  echo "❌ generate entity 失败"
  exit 1
fi

echo "============================================"
echo "🎉 SeaORM 全流程完成！"
# echo " migrate up ✓"
echo " migrate refresh ✓"
echo " generate entity ✓"
echo "============================================"
