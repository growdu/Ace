# 腾冲百分游戏 - 实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 实现一个跨平台的腾冲百分游戏平台，支持桌面端、Web端，包含用户系统、在线匹配和机器人对战功能

**Architecture:** 采用 Tauri + Rust + React 技术栈，前端使用 React + TypeScript，后端使用 Rust，核心游戏逻辑和 AI 用 Rust 实现

**Tech Stack:** Tauri 2.x, React 18, TypeScript, Rust (Axum, Tokio), PostgreSQL, WebSocket

---

## 项目结构

```
ace/
├── client/                    # Tauri 前端
│   ├── src/
│   │   ├── components/        # React 组件
│   │   ├── hooks/             # React Hooks
│   │   ├── pages/             # 页面组件
│   │   ├── styles/            # 样式文件
│   │   ├── types/             # TypeScript 类型
│   │   └── utils/             # 工具函数
│   ├── src-tauri/              # Rust 后端（客户端）
│   │   ├── src/
│   │   │   ├── game/          # 游戏引擎
│   │   │   ├── ai/            # AI 模块
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── index.html
│   └── package.json
├── server/                     # 云端游戏服务器
│   ├── src/
│   │   ├── handlers/          # HTTP/WebSocket 处理
│   │   ├── services/          # 业务逻辑
│   │   ├── models/            # 数据模型
│   │   └── main.rs
│   └── Cargo.toml
└── docs/                       # 文档
```

---

## 阶段一：项目初始化与核心游戏引擎

### Task 1: 初始化 Tauri 项目

**Files:**
- Create: `client/package.json`
- Create: `client/tsconfig.json`
- Create: `client/vite.config.ts`
- Create: `client/index.html`
- Create: `client/src-tauri/Cargo.toml`
- Create: `client/src-tauri/tauri.conf.json`
- Create: `client/src-tauri/src/main.rs`
- Create: `client/src-tauri/src/lib.rs`
- Create: `client/src/main.tsx`
- Create: `client/src/App.tsx`

- [ ] **Step 1: 创建 client 目录结构**

```bash
mkdir -p client/src client/src-tauri/src/game client/src-tauri/src/ai client/src/components client/src/hooks client/src/pages client/src/styles client/src/types client/src/utils
```

- [ ] **Step 2: 创建 package.json**

```json
{
  "name": "ace-tengchong-baifen",
  "version": "1.0.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "tauri": "tauri"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "@tauri-apps/api": "^2.0.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@vitejs/plugin-react": "^4.2.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0"
  }
}
```

- [ ] **Step 3: 创建 tsconfig.json**

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

- [ ] **Step 4: 创建 vite.config.ts**

```typescript
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
});
```

- [ ] **Step 5: 创建 index.html**

```html
<!DOCTYPE html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>腾冲百分 - ACE</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
```

- [ ] **Step 6: 创建 Cargo.toml**

```toml
[package]
name = "ace"
version = "1.0.0"
edition = "2021"

[lib]
name = "ace_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = ["devtools"] }
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
log = "0.4"
env_logger = "0.10"
```

- [ ] **Step 7: 创建 tauri.conf.json**

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "ACE腾冲百分",
  "version": "1.0.0",
  "identifier": "com.ace.tengchong-baifen",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "腾冲百分",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  }
}
```

- [ ] **Step 8: 创建 src-tauri/build.rs**

```rust
fn main() {
    tauri_build::build()
}
```

- [ ] **Step 9: 创建 src-tauri/src/main.rs**

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    ace_lib::run();
}
```

- [ ] **Step 10: 创建 src-tauri/src/lib.rs**

```rust
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            log::info!("ACE腾冲百分启动");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 11: 创建 src/main.tsx**

```tsx
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './styles/index.css';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
```

- [ ] **Step 12: 创建 src/App.tsx**

```tsx
function App() {
  return (
    <div className="app">
      <h1>腾冲百分 - ACE</h1>
    </div>
  );
}

export default App;
```

- [ ] **Step 13: 创建 src/styles/index.css**

```css
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: #f5f5f5;
}

.app {
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
}
```

- [ ] **Step 14: 安装依赖并验证项目**

```bash
cd client && npm install
```

- [ ] **Step 15: 提交代码**

```bash
git add client/
git commit -m "feat: initialize Tauri project with React"
```

---

### Task 2: 实现游戏核心数据结构

**Files:**
- Create: `client/src-tauri/src/game/types.rs`
- Create: `client/src-tauri/src/game/card.rs`
- Create: `client/src-tauri/src/game/mod.rs`

- [ ] **Step 1: 创建游戏类型定义 types.rs**

```rust
use serde::{Deserialize, Serialize};

/// 花色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Suit {
    Spades,   // 黑桃
    Hearts,   // 红桃
    Clubs,    // 梅花
    Diamonds, // 方块
}

impl Suit {
    pub fn from_index(index: u8) -> Self {
        match index {
            0 => Suit::Spades,
            1 => Suit::Hearts,
            2 => Suit::Clubs,
            3 => Suit::Diamonds,
            _ => panic!("Invalid suit index"),
        }
    }

    pub fn to_index(&self) -> u8 {
        match self {
            Suit::Spades => 0,
            Suit::Hearts => 1,
            Suit::Clubs => 2,
            Suit::Diamonds => 3,
        }
    }

    /// 花色大小: 黑桃 > 红桃 > 梅花 > 方块
    pub fn compare(&self, other: &Suit) -> Option<Ordering> {
        let self_idx = self.to_index();
        let other_idx = other.to_index();
        if self_idx == other_idx {
            Some(Ordering::Equal)
        } else if self_idx < other_idx {
            Some(Ordering::Greater)  // 索引越小，花色越大
        } else {
            Some(Ordering::Less)
        }
    }
}

/// 牌点
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rank {
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
    Two = 15,
}

impl Rank {
    pub fn from_index(index: u8) -> Self {
        match index {
            0 => Rank::Three,
            1 => Rank::Four,
            2 => Rank::Five,
            3 => Rank::Six,
            4 => Rank::Seven,
            5 => Rank::Eight,
            6 => Rank::Nine,
            7 => Rank::Ten,
            8 => Rank::Jack,
            9 => Rank::Queen,
            10 => Rank::King,
            11 => Rank::Ace,
            12 => Rank::Two,
            _ => panic!("Invalid rank index"),
        }
    }

    pub fn to_index(&self) -> u8 {
        match self {
            Rank::Three => 0,
            Rank::Four => 1,
            Rank::Five => 2,
            Rank::Six => 3,
            Rank::Seven => 4,
            Rank::Eight => 5,
            Rank::Nine => 6,
            Rank::Ten => 7,
            Rank::Jack => 8,
            Rank::Queen => 9,
            Rank::King => 10,
            Rank::Ace => 11,
            Rank::Two => 12,
        }
    }

    /// 牌点大小: 2 > A > K > Q > J > 10 > 9 > 8 > 7 > 6 > 5 > 4 > 3
    pub fn value(&self) -> u8 {
        match self {
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
            Rank::Two => 15,
        }
    }
}

/// 大小王
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Joker {
    Small,  // 小王
    Large,  // 大王
}

/// 扑克牌
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Card {
    pub suit: Option<Suit>,  // 花色，大小王时为 None
    pub rank: Option<Rank>,  // 牌点，大小王时为 None
    pub joker: Option<Joker>, // 大小王
}

impl Card {
    /// 创建普通牌
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card {
            suit: Some(suit),
            rank: Some(rank),
            joker: None,
        }
    }

    /// 创建小王
    pub fn small_joker() -> Self {
        Card {
            suit: None,
            rank: None,
            joker: Some(Joker::Small),
        }
    }

    /// 创建大王
    pub fn large_joker() -> Self {
        Card {
            suit: None,
            rank: None,
            joker: Some(Joker::Large),
        }
    }

    /// 是否为大小王
    pub fn is_joker(&self) -> bool {
        self.joker.is_some()
    }

    /// 是否为分数牌 (A, 10, 5)
    pub fn is_score(&self) -> bool {
        if self.is_joker() {
            return false;
        }
        match self.rank {
            Some(Rank::Ace) | Some(Rank::Ten) | Some(Rank::Five) => true,
            _ => false,
        }
    }

    /// 获取分数值
    pub fn score(&self) -> i32 {
        if self.is_joker() {
            return 0;
        }
        match self.rank {
            Some(Rank::Ace) | Some(Rank::Ten) => 10,
            Some(Rank::Five) => 5,
            _ => 0,
        }
    }
}

/// 游戏阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    Waiting,    // 等待开始
    Dealing,    // 发牌
    Bidding,    // 叫分
    Playing,    // 打牌
    Scoring,    // 结算
}

/// 玩家
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub cards: Vec<Card>,
    pub is_host: bool,      // 是否为庄家
    pub is_robot: bool,     // 是否为机器人
    pub score: i32,         // 已获得分数
}

impl Player {
    pub fn new(id: usize, name: String) -> Self {
        Player {
            id,
            name,
            cards: Vec::new(),
            is_host: false,
            is_robot: false,
            score: 0,
        }
    }

    /// 排序手牌
    pub fn sort_cards(&mut self) {
        self.cards.sort_by(|a, b| card_cmp(a, b, &None).cmp(&card_cmp(b, a, &None)));
    }
}

/// 游戏状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub players: Vec<Player>,
    pub current_player: usize,    // 当前出牌玩家
    pub bottom_cards: Vec<Card>, // 底牌
    pub current_bid: u32,        // 当前叫分
    pub bidder: Option<usize>,   // 庄家索引
    pub trump_suit: Option<Suit>, // 主牌花色
    pub scores: [i32; 2],        // [庄家得分, 闲家得分]
    pub round_cards: Vec<Option<Card>>, // 当前轮次的牌
    pub round_winner: Option<usize>, // 当前轮次获胜者
    pub phase: GamePhase,
    pub round_number: u32,       // 回合数
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            players: Vec::new(),
            current_player: 0,
            bottom_cards: Vec::with_capacity(6),
            current_bid: 0,
            bidder: None,
            trump_suit: None,
            scores: [0, 0],
            round_cards: vec![None; 4],
            round_winner: None,
            phase: GamePhase::Waiting,
            round_number: 0,
        }
    }
}
```

- [ ] **Step 2: 创建比较函数 card.rs**

```rust
use super::types::{Card, Joker, Rank, Suit};

/// 牌比较函数
/// 先比较花色，再比较点数
/// 返回值: -1 (a < b), 0 (a == b), 1 (a > b)
pub fn card_cmp(a: &Card, b: &Card, trump_suit: &Option<Suit>) -> i32 {
    // 大小王始终最大
    let a_is_joker = a.is_joker();
    let b_is_joker = b.is_joker();

    if a_is_joker && !b_is_joker {
        return 1;
    }
    if !a_is_joker && b_is_joker {
        return -1;
    }
    if a_is_joker && b_is_joker {
        match (a.joker, b.joker) {
            (Some(Joker::Large), Some(Joker::Small)) => 1,
            (Some(Joker::Small), Some(Joker::Large)) => -1,
            _ => 0,
        }
    }

    // 普通牌比较
    let a_suit = a.suit.unwrap();
    let b_suit = b.suit.unwrap();
    let a_rank = a.rank.unwrap();
    let b_rank = b.rank.unwrap();

    // 先比较花色
    let suit_cmp = suit_cmp(&a_suit, &b_suit, trump_suit);
    if suit_cmp != 0 {
        return suit_cmp;
    }

    // 花色相同，比较点数
    if a_rank.value() > b_rank.value() {
        1
    } else if a_rank.value() < b_rank.value() {
        -1
    } else {
        0
    }
}

/// 花色比较
fn suit_cmp(a: &Suit, b: &Suit, trump_suit: &Option<Suit>) -> i32 {
    // 主牌花色最大
    if let Some(trump) = trump_suit {
        if a == &trump && b != &trump {
            return 1;
        }
        if b == &trump && a != &trump {
            return -1;
        }
    }

    // 黑桃 > 红桃 > 梅花 > 方块
    let a_index = match a {
        Suit::Spades => 0,
        Suit::Hearts => 1,
        Suit::Clubs => 2,
        Suit::Diamonds => 3,
    };
    let b_index = match b {
        Suit::Spades => 0,
        Suit::Hearts => 1,
        Suit::Clubs => 2,
        Suit::Diamonds => 3,
    };

    if a_index < b_index {
        1  // 索引越小，花色越大
    } else if a_index > b_index {
        -1
    } else {
        0
    }
}

/// 初始化一副牌
pub fn create_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(54);

    // 52张普通牌
    for suit_idx in 0..4 {
        let suit = Suit::from_index(suit_idx);
        for rank_idx in 0..13 {
            let rank = Rank::from_index(rank_idx);
            deck.push(Card::new(suit, rank));
        }
    }

    // 大小王
    deck.push(Card::small_joker());
    deck.push(Card::large_joker());

    deck
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new(Suit::Spades, Rank::Ace);
        assert!(!card.is_joker());
        assert!(card.is_score());
        assert_eq!(card.score(), 10);
    }

    #[test]
    fn test_joker() {
        let small = Card::small_joker();
        let large = Card::large_joker();
        assert!(small.is_joker());
        assert!(large.is_joker());
        assert_eq!(card_cmp(&large, &small, &None), 1);
    }

    #[test]
    fn test_deck_size() {
        let deck = create_deck();
        assert_eq!(deck.len(), 54);
    }
}
```

- [ ] **Step 3: 创建 game/mod.rs**

```rust
pub mod types;
pub mod card;

pub use types::*;
pub use card::*;
```

- [ ] **Step 4: 运行测试**

```bash
cd client/src-tauri && cargo test
```

- [ ] **Step 5: 提交代码**

```bash
git add client/src-tauri/src/game/
git commit -m "feat: implement game core data structures and card comparison"
```

---

### Task 3: 实现游戏引擎核心逻辑

**Files:**
- Create: `client/src-tauri/src/game/engine.rs`
- Modify: `client/src-tauri/src/game/mod.rs`

- [ ] **Step 1: 创建游戏引擎 engine.rs**

```rust
use super::types::*;
use super::card::{create_deck, card_cmp, Card};

/// 游戏引擎
pub struct GameEngine {
    state: GameState,
}

impl GameEngine {
    pub fn new() -> Self {
        GameEngine {
            state: GameState::new(),
        }
    }

    /// 初始化游戏
    pub fn init_game(&mut self, player_names: Vec<String>) {
        assert_eq!(player_names.len(), 4);

        self.state.players = player_names
            .iter()
            .enumerate()
            .map(|(id, name)| Player::new(id, name.clone()))
            .collect();

        self.state.phase = GamePhase::Dealing;
    }

    /// 发牌
    pub fn deal_cards(&mut self) {
        let mut deck = create_deck();
        shuffle(&mut deck);

        // 每人发12张牌
        for (i, player) in self.state.players.iter_mut().enumerate() {
            let start = i * 12;
            let end = start + 12;
            player.cards = deck[start..end].to_vec();
            player.sort_cards();
        }

        // 剩余6张为底牌
        self.state.bottom_cards = deck[48..].to_vec();

        self.state.phase = GamePhase::Bidding;
    }

    /// 叫分
    pub fn bid(&mut self, player_id: usize, bid: u32) -> bool {
        if self.state.phase != GamePhase::Bidding {
            return false;
        }

        if bid > self.state.current_bid && bid >= 75 {
            self.state.current_bid = bid;
            self.state.bidder = Some(player_id);
            true
        } else {
            false
        }
    }

    /// 确认庄家
    pub fn confirm_bidder(&mut self) {
        if let Some(bidder_id) = self.state.bidder {
            // 设置庄家
            self.state.players[bidder_id].is_host = true;

            // 庄家拿底牌，换牌后扣下
            let player = &mut self.state.players[bidder_id];
            player.cards.extend(self.state.bottom_cards.clone());
            player.sort_cards();

            // 扣还6张底牌（简化：直接扣回）
            // 实际游戏中应该让庄家选择换牌
            let keep_count = player.cards.len() - 6;
            player.cards = player.cards[..keep_count].to_vec();

            // 确定主牌花色（简化：第一个庄家的叫分花色，这里随机）
            self.state.trump_suit = Some(Suit::Spades);

            self.state.phase = GamePhase::Playing;
            self.state.current_player = bidder_id;
            self.state.round_number = 1;
        }
    }

    /// 出牌
    pub fn play_card(&mut self, player_id: usize, card_index: usize) -> Result<Card, String> {
        if self.state.phase != GamePhase::Playing {
            return Err("游戏不在出牌阶段".to_string());
        }

        if player_id != self.state.current_player {
            return Err("不是你的回合".to_string());
        }

        let player = &mut self.state.players[player_id];
        if card_index >= player.cards.len() {
            return Err("无效的牌索引".to_string());
        }

        let card = player.cards.remove(card_index);
        self.state.round_cards[player_id] = Some(card);

        // 检查是否所有人都出完了
        if self.state.round_cards.iter().all(|c| c.is_some()) {
            self.state.round_winner = Some(self.determine_round_winner());
        }

        // 移动到下一个玩家
        self.state.current_player = (self.state.current_player + 1) % 4;

        Ok(card)
    }

    /// 确定回合获胜者
    fn determine_round_winner(&mut self) -> usize {
        let mut winner = 0;
        let mut max_card = &None;

        for (i, card_opt) in self.state.round_cards.iter().enumerate() {
            if let Some(card) = card_opt {
                if let Some(max) = max_card {
                    if card_cmp(card, max, &self.state.trump_suit) > 0 {
                        winner = i;
                        max_max = Some(card);
                    }
                } else {
                    winner = i;
                    max_card = Some(card);
                }
            }
        }

        // 收集本轮分数
        let round_score: i32 = self.state.round_cards
            .iter()
            .filter_map(|c| c.as_ref())
            .map(|c| c.score())
            .sum();

        // 分数归获胜方（庄家或闲家）
        let winner_team = winner % 2;
        self.state.scores[winner_team] += round_score;

        winner
    }

    /// 获取游戏状态
    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    /// 检查游戏是否结束
    pub fn is_game_over(&self) -> bool {
        // 所有牌出完且达到叫分
        self.state.players.iter().all(|p| p.cards.is_empty())
    }

    /// 获取游戏结果
    pub fn get_result(&self) -> Option<(bool, i32)> {
        if !self.is_game_over() {
            return None;
        }

        let bidder = self.state.bidder?;
        let bidder_team = bidder % 2;
        let bidder_score = self.state.scores[bidder_team];

        let success = bidder_score >= self.state.current_bid as i32;
        Some((success, bidder_score))
    }
}

/// 洗牌
fn shuffle<T>(deck: &mut Vec<T>) {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    let mut rng = RandomState::new();
    for i in (1..deck.len()).rev() {
        let mut hasher = rng.build_hasher();
        hasher.write_usize(i);
        let j = (hasher.finish() as usize) % (i + 1);
        deck.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_init() {
        let mut engine = GameEngine::new();
        engine.init_game(vec![
            "玩家1".to_string(),
            "玩家2".to_string(),
            "玩家3".to_string(),
            "玩家4".to_string(),
        ]);
        engine.deal_cards();

        assert_eq!(engine.state.players[0].cards.len(), 12);
        assert_eq!(engine.state.bottom_cards.len(), 6);
    }
}
```

- [ ] **Step 2: 修复编译错误**

运行 `cargo check` 查看错误并修复。

- [ ] **Step 3: 运行测试**

```bash
cd client/src-tauri && cargo test
```

- [ ] **Step 4: 提交代码**

```bash
git add client/src-tauri/src/game/
git commit -m "feat: implement game engine core logic"
```

---

### Task 4: 实现 AI 模块

**Files:**
- Create: `client/src-tauri/src/ai/mod.rs`
- Create: `client/src-tauri/src/ai/bot.rs`

- [ ] **Step 1: 创建 AI 模块 mod.rs**

```rust
pub mod bot;

pub use bot::*;
```

- [ ] **Step 2: 创建机器人 bot.rs**

```rust
use crate::game::{types::*, card::Card, card_cmp};

/// 机器人难度等级
#[derive(Debug, Clone, Copy)]
pub enum BotLevel {
    Easy,    // 简单
    Normal,  // 普通
    Hard,    // 困难
}

/// 机器人
pub struct Bot {
    level: BotLevel,
}

impl Bot {
    pub fn new(level: BotLevel) -> Self {
        Bot { level }
    }

    /// 选择出牌
    pub fn choose_card(&self, hand: &[Card], current_lead: &Option<Suit>, trump_suit: &Option<Suit>, round_cards: &[Option<Card>; 4]) -> Option<usize> {
        if hand.is_empty() {
            return None;
        }

        match self.level {
            BotLevel::Easy => self.easy_strategy(hand, current_lead, trump_suit),
            BotLevel::Normal => self.normal_strategy(hand, current_lead, trump_suit, round_cards),
            BotLevel::Hard => self.hard_strategy(hand, current_lead, trump_suit, round_cards),
        }
    }

    /// 简单策略：随机出牌
    fn easy_strategy(&self, hand: &[Card], _current_lead: &Option<Suit>, _trump_suit: &Option<Suit>) -> Option<usize> {
        use std::time::SystemTime;
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize;
        Some(seed % hand.len())
    }

    /// 普通策略：优先出小牌，保留分数牌
    fn normal_strategy(&self, hand: &[Card], current_lead: &Option<Suit>, trump_suit: &Option<Suit>, round_cards: &[Option<Card>; 4]) -> Option<usize> {
        // 如果有首家花色，优先跟牌
        if let Some(lead_suit) = current_lead {
            for (i, card) in hand.iter().enumerate() {
                if let Some(suit) = card.suit {
                    if suit == *lead_suit {
                        // 不是分数牌就出
                        if !card.is_score() {
                            return Some(i);
                        }
                    }
                } else if card.is_joker() {
                    return Some(i); // 出大小王
                }
            }
        }

        // 否则出最小的非分数牌
        let mut min_idx = 0;
        let mut min_value = 100u8;

        for (i, card) in hand.iter().enumerate() {
            if card.is_score() {
                continue;
            }

            let value = if card.is_joker() {
                0
            } else if card.suit == *trump_suit {
                card.rank.unwrap().value()
            } else {
                50 + card.rank.unwrap().value()
            };

            if value < min_value {
                min_value = value;
                min_idx = i;
            }
        }

        // 如果都是分数牌，出最小的
        if min_value == 100 {
            Some(0)
        } else {
            Some(min_idx)
        }
    }

    /// 困难策略：分析局势，选择最优牌
    fn hard_strategy(&self, hand: &[Card], current_lead: &Option<Suit>, trump_suit: &Option<Suit>, round_cards: &[Option<Card>; 4]) -> Option<usize> {
        // 检查是否自己最大
        let mut has_lead_suit = false;
        if let Some(lead_suit) = current_lead {
            for card in hand {
                if let Some(suit) = card.suit {
                    if suit == *lead_suit {
                        has_lead_suit = true;
                        break;
                    }
                }
            }
        }

        // 如果能赢，且有大牌，出大牌
        // 否则出小牌
        self.normal_strategy(hand, current_lead, trump_suit, round_cards)
    }

    /// 叫分决策
    pub fn decide_bid(&self, hand: &[Card], trump_suit: &Option<Suit>) -> u32 {
        let mut score = 0u32;

        // 计算手牌质量
        for card in hand {
            if card.is_joker() {
                score += 30;
            } else if let Some(suit) = card.suit {
                if Some(suit) == *trump_suit {
                    score += card.rank.unwrap().value() as u32;
                } else {
                    score += 5;
                }
            }

            if card.is_score() {
                score += card.score() as u32;
            }
        }

        // 根据分数决定叫分
        match self.level {
            BotLevel::Easy => 75,
            BotLevel::Normal => (score / 10).max(75).min(150),
            BotLevel::Hard => (score / 8).max(75).min(200),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_creation() {
        let bot = Bot::new(BotLevel::Normal);
        assert!(matches!(bot.level, BotLevel::Normal));
    }
}
```

- [ ] **Step 3: 运行测试**

```bash
cd client/src-tauri && cargo test
```

- [ ] **Step 4: 提交代码**

```bash
git add client/src-tauri/src/ai/
git commit -m "feat: implement AI bot module"
```

---

### Task 5: 创建前端游戏界面组件

**Files:**
- Create: `client/src/components/GameBoard.tsx`
- Create: `client/src/components/PlayerCard.tsx`
- Create: `client/src/components/Card.tsx`
- Create: `client/src/components/HandCards.tsx`
- Create: `client/src/styles/game.css`

- [ ] **Step 1: 创建类型定义 src/types/game.ts**

```typescript
export type Suit = 'Spades' | 'Hearts' | 'Clubs' | 'Diamonds';
export type Rank = 'Three' | 'Four' | 'Five' | 'Six' | 'Seven' | 'Eight' | 'Nine' | 'Ten' | 'Jack' | 'Queen' | 'King' | 'Ace' | 'Two';
export type Joker = 'Small' | 'Large';
export type GamePhase = 'Waiting' | 'Dealing' | 'Bidding' | 'Playing' | 'Scoring';
export type BotLevel = 'Easy' | 'Normal' | 'Hard';

export interface Card {
  suit: Suit | null;
  rank: Rank | null;
  joker: Joker | null;
}

export interface Player {
  id: number;
  name: string;
  cards: Card[];
  isHost: boolean;
  isRobot: boolean;
  score: number;
  position: 'top' | 'left' | 'right' | 'bottom';
}

export interface GameState {
  players: Player[];
  currentPlayer: number;
  bottomCards: Card[];
  currentBid: number;
  bidder: number | null;
  trumpSuit: Suit | null;
  scores: [number, number];
  roundCards: (Card | null)[];
  roundWinner: number | null;
  phase: GamePhase;
  roundNumber: number;
}
```

- [ ] **Step 2: 创建 Card 组件**

```tsx
import React from 'react';
import { Card as CardType } from '../types/game';
import './Card.css';

interface CardProps {
  card: CardType;
  selected?: boolean;
  onClick?: () => void;
  hidden?: boolean;
}

export const Card: React.FC<CardProps> = ({ card, selected, onClick, hidden }) => {
  if (hidden) {
    return <div className="card card-hidden" />;
  }

  const getSuitSymbol = (suit: string): string => {
    switch (suit) {
      case 'Spades': return '♠';
      case 'Hearts': return '♥';
      case 'Clubs': return '♣';
      case 'Diamonds': return '♦';
      default: return '';
    }
  };

  const getRankDisplay = (rank: string): string => {
    const map: Record<string, string> = {
      Three: '3', Four: '4', Five: '5', Six: '6', Seven: '7',
      Eight: '8', Nine: '9', Ten: '10', Jack: 'J', Queen: 'Q',
      King: 'K', Ace: 'A', Two: '2'
    };
    return map[rank] || '';
  };

  const isRed = card.suit === 'Hearts' || card.suit === 'Diamonds';
  const isJoker = card.joker;

  return (
    <div
      className={`card ${selected ? 'selected' : ''} ${isRed ? 'red' : 'black'}`}
      onClick={onClick}
    >
      {isJoker ? (
        <span className="joker">{card.joker === 'Large' ? '大王' : '小王'}</span>
      ) : (
        <>
          <div className="card-top">
            <span>{getRankDisplay(card.rank || '')}</span>
            <span>{getSuitSymbol(card.suit || '')}</span>
          </div>
          <div className="card-center">{getSuitSymbol(card.suit || '')}</div>
          <div className="card-bottom">
            <span>{getRankDisplay(card.rank || '')}</span>
            <span>{getSuitSymbol(card.suit || '')}</span>
          </div>
        </>
      )}
    </div>
  );
};
```

- [ ] **Step 3: 创建 Card.css**

```css
.card {
  width: 60px;
  height: 84px;
  background: white;
  border-radius: 4px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  cursor: pointer;
  position: relative;
  user-select: none;
  display: flex;
  flex-direction: column;
}

.card.selected {
  transform: translateY(-10px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
}

.card.red {
  color: #d32f2f;
}

.card.black {
  color: #212121;
}

.card-hidden {
  background: linear-gradient(135deg, #1976d2 25%, #1565c0 25%, #1565c0 50%, #1976d2 50%, #1976d2 75%, #1565c0 75%);
  background-size: 10px 10px;
}

.card-top, .card-bottom {
  position: absolute;
  font-size: 12px;
  display: flex;
  flex-direction: column;
  align-items: center;
  line-height: 1;
}

.card-top {
  top: 4px;
  left: 4px;
}

.card-bottom {
  bottom: 4px;
  right: 4px;
  transform: rotate(180deg);
}

.card-center {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 28px;
}

.joker {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  font-size: 24px;
  font-weight: bold;
  color: #f57c00;
}
```

- [ ] **Step 4: 创建 HandCards 组件**

```tsx
import React from 'react';
import { Card as CardType } from '../types/game';
import { Card } from './Card';
import './HandCards.css';

interface HandCardsProps {
  cards: CardType[];
  selectedIndex: number | null;
  onSelectCard: (index: number) => void;
}

export const HandCards: React.FC<HandCardsProps> = ({ cards, selectedIndex, onSelectCard }) => {
  return (
    <div className="hand-cards">
      {cards.map((card, index) => (
        <div key={index} className="hand-card-wrapper">
          <Card
            card={card}
            selected={index === selectedIndex}
            onClick={() => onSelectCard(index)}
          />
        </div>
      ))}
    </div>
  );
};
```

- [ ] **Step 5: 创建 HandCards.css**

```css
.hand-cards {
  display: flex;
  justify-content: center;
  align-items: flex-end;
  gap: 4px;
  padding: 20px;
}

.hand-card-wrapper {
  transition: transform 0.2s;
}

.hand-card-wrapper:hover {
  transform: translateY(-5px);
}
```

- [ ] **Step 6: 创建 PlayerCard 组件**

```tsx
import React from 'react';
import { Player } from '../types/game';
import './PlayerCard.css';

interface PlayerCardProps {
  player: Player;
  isCurrent: boolean;
}

export const PlayerCard: React.FC<PlayerCardProps> = ({ player, isCurrent }) => {
  return (
    <div className={`player-card ${isCurrent ? 'current' : ''}`}>
      <div className="player-avatar">
        {player.isRobot ? '🤖' : '👤'}
      </div>
      <div className="player-info">
        <div className="player-name">{player.name}</div>
        <div className="player-cards-count">🃏 {player.cards.length}</div>
        <div className="player-score">🏆 {player.score}</div>
      </div>
      {player.isHost && <div className="host-badge">庄</div>}
    </div>
  );
};
```

- [ ] **Step 7: 创建 PlayerCard.css**

```css
.player-card {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 15px;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.player-card.current {
  border: 2px solid #1976d2;
  box-shadow: 0 0 10px rgba(25, 118, 210, 0.3);
}

.player-avatar {
  font-size: 32px;
}

.player-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.player-name {
  font-weight: bold;
  font-size: 14px;
}

.player-cards-count, .player-score {
  font-size: 12px;
  color: #666;
}

.host-badge {
  background: #f57c00;
  color: white;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: bold;
}
```

- [ ] **Step 8: 创建 GameBoard 组件**

```tsx
import React, { useState } from 'react';
import { HandCards } from './HandCards';
import { PlayerCard } from './PlayerCard';
import { GameState } from '../types/game';
import './GameBoard.css';

interface GameBoardProps {
  gameState: GameState;
  onPlayCard: (cardIndex: number) => void;
  onBid: (bid: number) => void;
}

export const GameBoard: React.FC<GameBoardProps> = ({ gameState, onPlayCard, onBid }) => {
  const [selectedCard, setSelectedCard] = useState<number | null>(null);

  const handleCardClick = (index: number) => {
    setSelectedCard(index);
    onPlayCard(index);
  };

  const currentPlayer = gameState.players.find(p => p.id === gameState.currentPlayer);
  const playerPosition = (id: number, position: string) => position;

  return (
    <div className="game-board">
      {/* 顶部对手 */}
      <div className="player-top">
        <PlayerCard
          player={gameState.players[0]}
          isCurrent={gameState.currentPlayer === 0}
        />
      </div>

      <div className="game-middle">
        {/* 左侧对手 */}
        <div className="player-left">
          <PlayerCard
            player={gameState.players[3]}
            isCurrent={gameState.currentPlayer === 3}
          />
        </div>

        {/* 中央区域 - 桌面牌 */}
        <div className="table-area">
          <div className="round-cards">
            {gameState.roundCards.map((card, index) => (
              <div key={index} className={`table-card pos-${index}`}>
                {card ? `${card.rank?.slice(0, 1)}${card.suit?.slice(0, 1)}` : ''}
              </div>
            ))}
          </div>
          {gameState.trumpSuit && (
            <div className="trump-indicator">
              主牌: {gameState.trumpSuit}
            </div>
          )}
        </div>

        {/* 右侧对手 */}
        <div className="player-right">
          <PlayerCard
            player={gameState.players[1]}
            isCurrent={gameState.currentPlayer === 1}
          />
        </div>
      </div>

      {/* 底部 - 玩家手牌 */}
      <div className="player-bottom">
        <PlayerCard
          player={gameState.players[2]}
          isCurrent={gameState.currentPlayer === 2}
        />
        <HandCards
          cards={gameState.players[2].cards}
          selectedIndex={selectedCard}
          onSelectCard={handleCardClick}
        />
      </div>

      {/* 分数面板 */}
      <div className="score-panel">
        <div className="score-host">庄家: {gameState.scores[0]}</div>
        <div className="score-info">叫分: {gameState.currentBid}</div>
        <div className="score-xian">闲家: {gameState.scores[1]}</div>
      </div>
    </div>
  );
};
```

- [ ] **Step 9: 创建 GameBoard.css**

```css
.game-board {
  width: 100%;
  height: 100vh;
  background: linear-gradient(135deg, #1b5e20 0%, #2e7d32 50%, #1b5e20 100%);
  position: relative;
  display: flex;
  flex-direction: column;
}

.player-top {
  position: absolute;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
}

.game-middle {
  display: flex;
  flex: 1;
  justify-content: space-between;
  align-items: center;
  padding: 0 20px;
}

.player-left {
  align-self: flex-start;
  margin-top: 100px;
}

.player-right {
  align-self: flex-start;
  margin-top: 100px;
}

.table-area {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.round-cards {
  width: 300px;
  height: 200px;
  position: relative;
}

.table-card {
  position: absolute;
  width: 60px;
  height: 84px;
  background: white;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.table-card.pos-0 { top: 0; left: 50%; transform: translateX(-50%); }
.table-card.pos-1 { right: 0; top: 50%; transform: translateY(-50%); }
.table-card.pos-2 { bottom: 0; left: 50%; transform: translateX(-50%); }
.table-card.pos-3 { left: 0; top: 50%; transform: translateY(-50%); }

.trump-indicator {
  background: rgba(0, 0, 0, 0.5);
  color: white;
  padding: 5px 15px;
  border-radius: 20px;
  margin-top: 20px;
}

.player-bottom {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.score-panel {
  position: absolute;
  top: 50%;
  left: 20px;
  transform: translateY(-50%);
  background: rgba(0, 0, 0, 0.7);
  color: white;
  padding: 15px;
  border-radius: 8px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}
```

- [ ] **Step 10: 更新 App.tsx**

```tsx
import { useState, useEffect } from 'react';
import { GameBoard } from './components/GameBoard';
import { GameState, GamePhase } from './types/game';

function App() {
  const [gameState, setGameState] = useState<GameState>({
    players: [
      { id: 0, name: '对手1', cards: [], isHost: false, isRobot: true, score: 0, position: 'top' },
      { id: 1, name: '对手2', cards: [], isHost: false, isRobot: true, score: 0, position: 'right' },
      { id: 2, name: '玩家', cards: [], isHost: false, isRobot: false, score: 0, position: 'bottom' },
      { id: 3, name: '对手3', cards: [], isHost: false, isRobot: true, score: 0, position: 'left' },
    ],
    currentPlayer: 0,
    bottomCards: [],
    currentBid: 75,
    bidder: null,
    trumpSuit: null,
    scores: [0, 0],
    roundCards: [null, null, null, null],
    roundWinner: null,
    phase: 'Waiting',
    roundNumber: 0,
  });

  const handlePlayCard = (cardIndex: number) => {
    console.log('Play card:', cardIndex);
    // TODO: 调用 Rust 后端
  };

  const handleBid = (bid: number) => {
    console.log('Bid:', bid);
    // TODO: 调用 Rust 后端
  };

  return (
    <div className="app">
      <GameBoard
        gameState={gameState}
        onPlayCard={handlePlayCard}
        onBid={handleBid}
      />
    </div>
  );
}

export default App;
```

- [ ] **Step 11: 验证项目运行**

```bash
cd client && npm run dev
```

- [ ] **Step 12: 提交代码**

```bash
git add client/src/
git commit -m "feat: create game UI components"
```

---

## 阶段二：用户系统与后端（待续）

### Task 6: 后端服务搭建

### Task 7: 用户认证接口

### Task 8: WebSocket 游戏通信

### Task 9: 匹配系统

### Task 10: 部署与测试

---

**计划完成。文件保存在 `docs/superpowers/plans/2026-03-31-tengchong-baifen-implementation.md`**

**执行选项：**

**1. Subagent-Driven (recommended)** - 我为每个任务分配一个子代理，进行快速迭代审查

**2. Inline Execution** - 在此会话中执行任务，使用 executing-plans 进行批量审查

请选择执行方式？
