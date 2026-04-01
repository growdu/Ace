# 腾冲百分游戏 - 后端系统设计文档

**项目名称**：ACE - 腾冲百分游戏平台

**版本**：1.0

**日期**：2026-03-31

---

## 1. 系统概述

### 1.1 目标

实现腾冲百分游戏的后端服务系统，支持用户注册登录、在线对战、房间匹配等功能。

### 1.2 技术选型

| 组件 | 技术方案 |
|------|----------|
| Web框架 | Rust + Axum |
| 异步运行时 | Tokio |
| WebSocket | tokio-tungstenite |
| 数据存储 | JSON 文件（用户数据）+ 内存（游戏状态） |
| 认证 | JWT |

---

## 2. 模块设计

### 2.1 认证模块 (auth)

**功能**：
- 用户注册（用户名 + 密码）
- 用户登录（返回 JWT Token）
- Token 验证

**接口**：

| 接口 | 方法 | 描述 |
|------|------|------|
| /api/auth/register | POST | 用户注册 |
| /api/auth/login | POST | 用户登录 |
| /api/auth/verify | POST | 验证 Token |

**数据结构**：

```rust
struct User {
    id: String,
    username: String,
    password_hash: String,
    score: i32,       // 积分
    wins: i32,       // 胜场
    losses: i32,      // 负场
    created_at: i64,
}

struct AuthResponse {
    token: String,
    user: UserInfo,
}
```

### 2.2 用户模块 (user)

**功能**：
- 获取用户信息
- 更新用户资料
- 查询战绩

**接口**：

| 接口 | 方法 | 描述 |
|------|------|------|
| /api/user/profile | GET | 获取用户信息 |
| /api/user/profile | PUT | 更新用户信息 |
| /api/user/stats | GET | 获取战绩统计 |

### 2.3 房间模块 (room)

**功能**：
- 创建房间（房主）
- 加入房间
- 离开房间
- 房间状态同步

**接口**：

| 接口 | 方法 | 描述 |
|------|------|------|
| /api/room/create | POST | 创建房间 |
| /api/room/join | POST | 加入房间 |
| /api/room/leave | POST | 离开房间 |
| /api/room/{id} | GET | 获取房间信息 |

**房间状态**：

```rust
struct Room {
    id: String,
    owner_id: String,
    players: Vec<RoomPlayer>,
    status: RoomStatus,
    created_at: i64,
}

enum RoomStatus {
    Waiting,   // 等待玩家
    Full,     // 房间已满
    Playing,  //游戏中
    Closed,   // 已关闭
}
```

### 2.4 游戏模块 (game)

**功能**：
- 游戏状态管理
- 出牌验证
- 回合结算
- 机器人托管

**WebSocket 消息**：

| 消息类型 | 方向 | 描述 |
|----------|------|------|
| RoomState | 服务端→客户端 | 房间状态推送 |
| GameStart | 服务端→客户端 | 游戏开始 |
| DealCards | 服务端→客户端 | 发牌 |
| YourTurn | 服务端→客户端 | 轮到某玩家 |
| PlayCard | 客户端→服务端 | 出牌 |
| RoundEnd | 服务端→客户端 | 回合结束 |
| GameEnd | 服务端→客户端 | 游戏结束 |

### 2.5 匹配模块 (match)

**功能**：
- 自动匹配玩家
- 分配机器人
- 平衡队伍

**匹配模式**：
- 单人匹配：随机匹配3名玩家
- 双人匹配：匹配队友 + 2名对手
- 人机匹配：玩家 + 3机器人

---

## 3. 数据流

### 3.1 用户登录流程

```
客户端                    服务端
   |                        |
   |--- POST /login ------->|
   |    {username, password} |
   |                        |
   |<-- {token, user} ------|
   |                        |
```

### 3.2 创建房间流程

```
客户端                    服务端
   |                        |
   |--- POST /room/create -->|
   |    {token}             |
   |                        |
   |<-- {room_id} ----------|
   |                        |
   |                        |
   |--- WS /ws/game/{id} -->|
   |                        |
   |<-- RoomState ----------|
```

### 3.3 游戏流程

```
1. 4人进入房间 → 房主点击开始
2. 服务端发牌 → 客户端显示手牌
3. 叫分阶段 → 玩家依次叫分
4. 出牌阶段 → 玩家轮流出牌
5. 回合结算 → 分数归获胜方
6. 游戏结束 → 结算积分
```

---

## 4. 安全设计

### 4.1 认证

- JWT Token 有效期 24 小时
- 密码使用 bcrypt 哈希存储
- Token 签名使用 HS256

### 4.2 游戏公平性

- 所有游戏逻辑在服务端验证
- 出牌必须符合规则
- 防止客户端作弊

---

## 5. 部署

### 5.1 开发环境

```bash
cd server
cargo run
```

服务监听：http://localhost:8080

### 5.2 目录结构

```
server/
├── src/
│   ├── main.rs           # 入口
│   ├── lib.rs            # 库导出
│   ├── auth/             # 认证模块
│   │   ├── mod.rs
│   │   ├── handler.rs
│   │   └── service.rs
│   ├── user/             # 用户模块
│   │   ├── mod.rs
│   │   ├── handler.rs
│   │   └── service.rs
│   ├── room/             # 房间模块
│   │   ├── mod.rs
│   │   ├── handler.rs
│   │   └── service.rs
│   ├── game/             # 游戏模块
│   │   ├── mod.rs
│   │   ├── handler.rs
│   │   └── service.rs
│   └── ws/               # WebSocket
│       ├── mod.rs
│       └── handler.rs
├── Cargo.toml
└── data/                 # 数据存储
    └── users.json
```

---

## 6. 待实现

1. 完整的 JWT 认证
2. 房间创建/加入/离开
3. WebSocket 游戏通信
4. 游戏逻辑验证
5. 机器人托管
6. 积分系统

---

**文档状态**：已完成 ✅