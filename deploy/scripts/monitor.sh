#!/bin/bash
# 监控脚本 - /opt/ace/monitor.sh

set -e

LOG_FILE="/var/log/ace-monitor.log"
WEBHOOK_URL=""  # 替换为实际 webhook 地址

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a $LOG_FILE
}

# 检查后端服务
check_backend() {
    response=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/health 2>/dev/null || echo "000")
    if [ "$response" != "200" ]; then
        log "ERROR: 后端服务异常，HTTP码: $response"
        send_alert "后端服务异常，HTTP码: $response"
        return 1
    fi
    log "OK: 后端服务正常"
    return 0
}

# 检查 WebSocket 连接数
check_websocket() {
    ws_conn=$(ss -tn | grep :8080 | grep ESTAB | wc -l 2>/dev/null || echo "0")
    log "WebSocket连接数: $ws_conn"
    if [ "$ws_conn" -gt 1000 ]; then
        log "WARNING: WebSocket连接数过高: $ws_conn"
    fi
}

# 检查资源使用
check_resources() {
    # CPU 使用率
    cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1 | head -1)
    log "CPU使用率: ${cpu_usage}%"

    # 内存使用
    mem_used=$(free -m | awk '/Mem:/ {print $3}')
    mem_total=$(free -m | awk '/Mem:/ {print $2}')
    log "内存使用: ${mem_used}MB/${mem_total}MB"

    # 磁盘使用
    disk_pct=$(df -h / | awk 'NR==2 {print $5}' | cut -d'%' -f1)
    log "磁盘使用: ${disk_pct}%"
}

# 发送告警
send_alert() {
    local message="$1"
    if [ -n "$WEBHOOK_URL" ]; then
        curl -s -X POST "$WEBHOOK_URL" \
            -H "Content-Type: application/json" \
            -d "{\"msgtype\":\"text\",\"text\":{\"content\":\"[ACE监控] $message\"}}" \
            > /dev/null 2>&1 || true
    fi
}

# 主流程
log "========== 开始监控 =========="
check_backend
check_websocket
check_resources
log "========== 监控完成 =========="