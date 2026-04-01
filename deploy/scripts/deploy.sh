#!/bin/bash
# 自动部署脚本 - /opt/ace/deploy.sh

set -e

VERSION=${1:-"latest"}
DEPLOY_DIR="/opt/ace"
LOG_FILE="/var/log/ace-deploy.log"

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a $LOG_FILE
}

log "========== 开始部署版本: $VERSION =========="

# 检查是否以 root 运行
if [ "$EUID" -ne 0 ]; then
    log "请使用 root 用户运行此脚本"
    exit 1
fi

# 备份当前版本
log "备份当前版本..."
if [ -d "$DEPLOY_DIR/server/target/release" ]; then
    tar -czf "$DEPLOY_DIR/backups/server_$(date +%Y%m%d_%H%M%S).tar.gz" \
        -C "$DEPLOY_DIR/server" target/release/ace-server || true
fi

# 进入部署目录
cd $DEPLOY_DIR

# 拉取最新代码
log "拉取最新代码..."
git fetch origin
git checkout main
git pull origin main

# 构建前端
log "构建前端..."
cd $DEPLOY_DIR/client
npm install
npm run build

# 构建后端
log "构建后端..."
cd $DEPLOY_DIR/server
cargo build --release

# 重启服务
log "重启服务..."
systemctl restart ace-server

# 检查服务状态
sleep 3
if systemctl is-active --quiet ace-server; then
    log "========== 部署成功! =========="
else
    log "========== 部署失败! =========="
    journalctl -u ace-server -n 50
    # 回滚
    log "尝试回滚..."
    latest_backup=$(ls -t $DEPLOY_DIR/backups/server_*.tar.gz 2>/dev/null | head -1)
    if [ -n "$latest_backup" ]; then
        tar -xzf "$latest_backup" -C $DEPLOY_DIR/server/
        systemctl restart ace-server
        log "已回滚"
    fi
    exit 1
fi