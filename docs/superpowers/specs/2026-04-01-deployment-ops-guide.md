# 腾冲百分游戏 - 运维与部署手册

**项目名称**：ACE - 腾冲百分游戏平台

**版本**：1.0

**日期**：2026-04-01

---

## 目录

1. [系统架构概览](#1-系统架构概览)
2. [本地开发部署](#2-本地开发部署)
3. [Docker 部署](#3-docker-部署)
4. [云服务器部署](#4-云服务器部署)
5. [监控与告警](#5-监控与告警)
6. [备份与恢复](#6-备份与恢复)
7. [自动部署脚本](#7-自动部署脚本)
8. [故障排查](#8-故障排查)

---

## 1. 系统架构概览

```
┌─────────────────────────────────────────────────────────────────┐
│                         用户终端                                │
│    ┌─────────┐    ┌─────────┐    ┌─────────┐                 │
│    │ 桌面端   │    │ Web端   │    │ 移动端   │                 │
│    │ (Tauri)  │    │(浏览器)  │    │         │                 │
│    └────┬────┘    └────┬────┘    └─────────┘                 │
│         │              │                                      │
└─────────┼──────────────┼──────────────────────────────────────┘
          │              │
    ┌─────▼──────────────▼──────┐
    │      Nginx 反向代理         │
    │     (端口 80/443)          │
    └──────────┬─────────────────┘
               │
    ┌──────────▼─────────────────┐
    │      游戏服务器             │
    │   Rust + Axum + Tokio     │
    │     (端口 8080)            │
    └───────────────────────────┘
```

### 组件说明

| 组件 | 技术 | 端口 | 说明 |
|------|------|------|------|
| 前端 | React + Vite | 1420 | 开发模式，生产构建后由 Nginx 服务 |
| 后端 | Rust + Axum | 8080 | REST API + WebSocket |
| Nginx | - | 80/443 | 反向代理、静态文件服务 |

---

## 2. 本地开发部署

### 2.1 环境要求

- **操作系统**：Linux / macOS / Windows (WSL)
- **Node.js**：>= 18.0
- **Rust**：>= 1.70
- **Cargo**：>= 1.70

### 2.2 安装步骤

```bash
# 1. 克隆项目
git clone <repository-url>
cd ace

# 2. 安装前端依赖
cd client
npm install

# 3. 构建前端（可选，开发模式可跳过）
npm run build

# 4. 返回项目根目录，安装 Rust 依赖
cd ../server
cargo build --release
```

### 2.3 启动服务

**终端 1 - 启动后端：**

```bash
cd server
cargo run --release
# 输出：Server running on http://0.0.0.0:8080
```

**终端 2 - 启动前端（开发模式）：**

```bash
cd client
npm run dev
# 输出：Local: http://localhost:1420/
```

**或使用生产构建：**

```bash
# 先构建
npm run build
# 启动静态服务器
npx serve dist -l 1420
```

### 2.4 验证

```bash
# 测试后端健康检查
curl http://localhost:8080/health

# 测试用户注册
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"123456"}'

# 测试 WebSocket 连接
wscat -c ws://localhost:8080/ws/game/test-room
```

---

## 3. Docker 部署

### 3.1 前提条件

- Docker >= 20.10
- Docker Compose >= 2.0

### 3.2 项目结构

```
deploy/
├── docker-compose.yml
├── nginx/
│   └── nginx.conf
├── Dockerfile.server
├── Dockerfile.client
└── .env
```

### 3.3 Docker Compose 配置

```yaml
version: '3.8'

services:
  # 游戏后端服务
  server:
    build:
      context: ..
      dockerfile: deploy/Dockerfile.server
    ports:
      - "8080:8080"
    environment:
      - RUST_LOG=info
      - PORT=8080
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  # 前端构建与 Nginx 服务
  client:
    build:
      context: ..
      dockerfile: deploy/Dockerfile.client
    ports:
      - "80:80"
    depends_on:
      - server
    restart: unless-stopped

networks:
  default:
    name: ace-network
```

### 3.4 Nginx 配置

```nginx
events {
    worker_connections 1024;
}

http {
    include /etc/nginx/mime.types;
    default_type application/octet-stream;

    upstream backend {
        server server:8080;
    }

    server {
        listen 80;
        server_name localhost;

        # 前端静态文件
        location / {
            root /usr/share/nginx/html;
            index index.html;
            try_files $uri $uri/ /index.html;
        }

        # API 代理
        location /api/ {
            proxy_pass http://backend;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection 'upgrade';
            proxy_set_header Host $host;
            proxy_cache_bypass $http_upgrade;
        }

        # WebSocket 代理
        location /ws/ {
            proxy_pass http://backend;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_set_header Host $host;
        }
    }
}
```

### 3.5 构建与启动

```bash
# 进入部署目录
cd deploy

# 构建并启动所有服务
docker-compose up -d --build

# 查看服务状态
docker-compose ps

# 查看日志
docker-compose logs -f

# 停止服务
docker-compose down
```

### 3.6 生产镜像优化

```dockerfile
# Dockerfile.server
FROM rust:1.75 as builder

WORKDIR /build
COPY ../server/Cargo.toml ../server/src .
RUN cargo build --release --lib -p ace-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/ace-server /usr/local/bin/
EXPOSE 8080
CMD ["ace-server"]
```

---

## 4. 云服务器部署

### 4.1 服务器要求

| 配置 | 最低要求 | 推荐配置 |
|------|----------|----------|
| CPU | 2 核 | 2 核 |
| 内存 | 2 GB | 4 GB |
| 磁盘 | 20 GB | 50 GB |
| 带宽 | 5 Mbps | 10 Mbps |
| 系统 | Ubuntu 20.04 | Ubuntu 22.04 |

### 4.2 系统初始化

```bash
# 1. 更新系统
sudo apt update && sudo apt upgrade -y

# 2. 创建应用用户
sudo adduser --system --group ace
sudo mkdir -p /opt/ace
sudo chown -R ace:ace /opt/ace

# 3. 安装必要工具
sudo apt install -y curl git nginx certbot python3-certbot-nginx
```

### 4.3 部署步骤

```bash
# 1. 克隆项目
cd /opt/ace
sudo git clone <repository-url> .
sudo chown -R ace:ace /opt/ace

# 2. 准备前端构建
cd client
npm install
npm run build
cd ..

# 3. 构建后端
cd server
cargo build --release
cd ..

# 4. 配置 Nginx
sudo cp deploy/nginx/nginx.prod.conf /etc/nginx/sites-available/ace
sudo ln -s /etc/nginx/sites-available/ace /etc/nginx/sites-enabled/
sudo nginx -t

# 5. 启动服务（使用 systemd）
sudo cp deploy/ace-server.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable ace-server
sudo systemctl start ace-server
sudo systemctl enable nginx
sudo systemctl reload nginx
```

### 4.4 Systemd 服务配置

```ini
# /etc/systemd/system/ace-server.service
[Unit]
Description=ACE Tengchong Baifen Game Server
After=network.target

[Service]
Type=simple
User=ace
WorkingDirectory=/opt/ace/server
Environment=RUST_LOG=info
ExecStart=/opt/ace/server/target/release/ace-server
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

### 4.5 HTTPS 配置

```bash
# 使用 Let's Encrypt 免费证书
sudo certbot --nginx -d your-domain.com

# 自动续期测试
sudo certbot renew --dry-run
```

### 4.6 防火墙配置

```bash
# 开放必要端口
sudo ufw allow 22    # SSH
sudo ufw allow 80    # HTTP
sudo ufw allow 443   # HTTPS
sudo ufw enable
sudo ufw status
```

---

## 5. 监控与告警

### 5.1 服务健康检查

```bash
# 创建监控脚本
cat > /opt/ace/monitor.sh << 'EOF'
#!/bin/bash

# 检查后端服务
response=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/health)
if [ "$response" != "200" ]; then
    echo "后端服务异常，HTTP码: $response"
    # 发送告警（可集成钉钉/企业微信等）
    curl -X POST "https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=YOUR_KEY" \
         -H "Content-Type: application/json" \
         -d '{"msgtype":"text","text":{"content":"ACE服务异常"}}'
fi

# 检查 WebSocket 连接数
ws_conn=$(ss -tn | grep :8080 | wc -l)
echo "WebSocket连接数: $ws_conn"
EOF

chmod +x /opt/ace/monitor.sh

# 添加定时任务
crontab -e
# 每5分钟执行一次
*/5 * * * * /opt/ace/monitor.sh >> /var/log/ace-monitor.log 2>&1
```

### 5.2 资源监控

```bash
# 安装监控工具
sudo apt install -y htop

# 创建资源监控脚本
cat > /opt/ace/resource-monitor.sh << 'EOF'
#!/bin/bash

# CPU 使用率
cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1)
echo "$(date): CPU使用率: ${cpu_usage}%"

# 内存使用
mem_used=$(free -m | awk '/Mem:/ {print $3}')
mem_total=$(free -m | awk '/Mem:/ {print $2}')
mem_pct=$((mem_used * 100 / mem_total))
echo "$(date): 内存使用: ${mem_used}MB/${mem_total}MB (${mem_pct}%)"

# 磁盘使用
disk_pct=$(df -h / | awk 'NR==2 {print $5}' | cut -d'%' -f1)
echo "$(date): 磁盘使用: ${disk_pct}%"
EOF
```

### 5.3 日志管理

```bash
# 配置日志轮转
sudo cat > /etc/logrotate.d/ace << 'EOF'
/var/log/ace-*.log {
    daily
    rotate 7
    compress
    delaycompress
    missingok
    notifempty
    create 0640 ace ace
}
EOF

# 查看日志
journalctl -u ace-server -f
tail -f /var/log/nginx/access.log
```

---

## 6. 备份与恢复

### 6.1 备份脚本

```bash
#!/bin/bash
# /opt/ace/backup.sh

BACKUP_DIR="/opt/ace/backups"
DATE=$(date +%Y%m%d_%H%M%S)
mkdir -p $BACKUP_DIR

# 备份用户数据
if [ -f /opt/ace/server/data/users.json ]; then
    cp /opt/ace/server/data/users.json $BACKUP_DIR/users_$DATE.json
    echo "用户数据已备份"
fi

# 备份配置文件
cp -r /opt/ace/deploy/config $BACKUP_DIR/config_$DATE
echo "配置文件已备份"

# 备份 Nginx 配置
cp /etc/nginx/sites-available/ace $BACKUP_DIR/nginx_$DATE.conf

# 清理 7 天前的备份
find $BACKUP_DIR -type f -mtime +7 -delete

echo "备份完成: $DATE"
```

### 6.2 恢复脚本

```bash
#!/bin/bash
# /opt/ace/restore.sh

if [ -z "$1" ]; then
    echo "用法: $0 <备份日期>"
    echo "示例: $0 20260401_143000"
    exit 1
fi

BACKUP_DIR="/opt/ace/backups"
DATE=$1

# 恢复用户数据
if [ -f $BACKUP_DIR/users_$DATE.json ]; then
    cp $BACKUP_DIR/users_$DATE.json /opt/ace/server/data/users.json
    echo "用户数据已恢复"
fi

# 重启服务
systemctl restart ace-server
echo "服务已重启"
```

### 6.3 自动备份

```bash
# 添加定时任务
crontab -e
# 每天凌晨 3 点执行备份
0 3 * * * /opt/ace/backup.sh >> /var/log/ace-backup.log 2>&1
```

---

## 7. 自动部署脚本

### 7.1 部署脚本

```bash
#!/bin/bash
# /opt/ace/deploy.sh

set -e

VERSION=${1:-"latest"}
DEPLOY_DIR="/opt/ace"
LOG_FILE="/var/log/ace-deploy.log"

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a $LOG_FILE
}

log "开始部署版本: $VERSION"

# 拉取最新代码
cd $DEPLOY_DIR
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
sudo systemctl restart ace-server

# 检查服务状态
sleep 3
if systemctl is-active --quiet ace-server; then
    log "部署成功!"
else
    log "部署失败，服务启动异常"
    journalctl -u ace-server -n 50
    exit 1
fi
```

### 7.2 回滚脚本

```bash
#!/bin/bash
# /opt/ace/rollback.sh

COMMIT=${1:-"HEAD~1"}

log "回滚到版本: $COMMIT"

git checkout $COMMIT

# 重新构建
cd client && npm run build
cd ../server && cargo build --release

# 重启服务
systemctl restart ace-server

log "回滚完成"
```

---

## 8. 故障排查

### 8.1 常见问题

| 问题 | 可能原因 | 解决方案 |
|------|----------|----------|
| 后端无法启动 | 端口被占用 | `lsof -i:8080` 查看占用进程 |
| 前端无法连接后端 | 跨域问题 | 检查 Nginx 代理配置 |
| WebSocket 连接失败 | 代理未配置 | 确认 Nginx 支持 WebSocket |
| 用户无法登录 | 数据文件权限 | `chmod 666 data/users.json` |

### 8.2 调试命令

```bash
# 查看后端日志
journalctl -u ace-server -f

# 查看 Nginx 错误日志
tail -f /var/log/nginx/error.log

# 检查端口监听
ss -tlnp | grep -E '8080|80'

# 检查进程状态
ps aux | grep ace-server

# 检查资源使用
htop
```

### 8.3 服务重启

```bash
# 重启后端
sudo systemctl restart ace-server

# 重启 Nginx
sudo systemctl reload nginx

# 重启所有服务
sudo systemctl restart ace-server nginx
```

---

## 附录

### A. 端口说明

| 端口 | 服务 | 说明 |
|------|------|------|
| 22 | SSH | 远程管理 |
| 80 | HTTP | Web 访问 |
| 443 | HTTPS | 安全访问 |
| 8080 | API | 后端服务 |

### B. 目录结构

```
/opt/ace/
├── client/          # 前端代码
├── server/          # 后端代码
├── deploy/          # 部署配置
│   ├── docker-compose.yml
│   ├── nginx/
│   └── ace-server.service
├── backups/         # 备份文件
└── logs/            # 日志目录
```

---

**文档完成** ✅