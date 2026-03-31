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

    /// 花色大小: 黑桃 > 红桃 > 梅花 > 方块 (索引越小越大)
    pub fn compare(&self, other: &Suit) -> Option<std::cmp::Ordering> {
        let self_idx = self.to_index();
        let other_idx = other.to_index();
        if self_idx == other_idx {
            Some(std::cmp::Ordering::Equal)
        } else if self_idx < other_idx {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
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
    pub suit: Option<Suit>,    // 花色，大小王时为 None
    pub rank: Option<Rank>,    // 牌点，大小王时为 None
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
    Waiting,  // 等待开始
    Dealing,  // 发牌
    Bidding,  // 叫分
    Playing,  // 打牌
    Scoring,  // 结算
}

/// 玩家
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub cards: Vec<Card>,
    pub is_host: bool,    // 是否为庄家
    pub is_robot: bool,   // 是否为机器人
    pub score: i32,        // 已获得分数
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
}

/// 游戏状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub players: Vec<Player>,
    pub current_player: usize,  // 当前出牌玩家
    pub bottom_cards: Vec<Card>, // 底牌
    pub current_bid: u32,      // 当前叫分
    pub bidder: Option<usize>,  // 庄家索引
    pub trump_suit: Option<Suit>, // 主牌花色
    pub scores: [i32; 2],       // [庄家得分, 闲家得分]
    pub round_cards: Vec<Option<Card>>, // 当前轮次的牌
    pub round_winner: Option<usize>, // 当前轮次获胜者
    pub phase: GamePhase,
    pub round_number: u32,     // 回合数
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
