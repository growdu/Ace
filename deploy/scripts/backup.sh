#!/bin/bash
# 备份脚本 - /opt/ace/backup.sh

set -e

BACKUP_DIR="/opt/ace/backups"
DATE=$(date +%Y%m%d_%H%M%S)
LOG_FILE="/var/log/ace-backup.log"

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a $LOG_FILE
}

mkdir -p $BACKUP_DIR

log "开始备份..."

# 备份用户数据
if [ -f /opt/ace/server/data/users.json ]; then
    cp /opt/ace/server/data/users.json $BACKUP_DIR/users_$DATE.json
    log "用户数据已备份: users_$DATE.json"
fi

# 备份配置文件
if [ -d /opt/ace/deploy ]; then
    tar -czf $BACKUP_DIR/config_$DATE.tar.gz -C /opt/ace deploy/
    log "配置文件已备份: config_$DATE.tar.gz"
fi

# 备份 Nginx 配置
if [ -f /etc/nginx/sites-available/ace ]; then
    cp /etc/nginx/sites-available/ace $BACKUP_DIR/nginx_$DATE.conf
    log "Nginx配置已备份"
fi

# 备份 Systemd 服务配置
if [ -f /etc/systemd/system/ace-server.service ]; then
    cp /etc/systemd/system/ace-server.service $BACKUP_DIR/ace-server.service_$DATE
    log "Systemd服务配置已备份"
fi

# 清理 7 天前的备份
find $BACKUP_DIR -type f -mtime +7 -delete
log "清理完成"

log "备份完成: $DATE"