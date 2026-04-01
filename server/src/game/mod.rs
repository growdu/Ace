use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 花色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
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
            _ => Suit::Spades,
        }
    }
}

/// 牌点
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Rank {
    Three = 3, Four = 4, Five = 5, Six = 6, Seven = 7,
    Eight = 8, Nine = 9, Ten = 10, Jack = 11, Queen = 12,
    King = 13, Ace = 14, Two = 15,
}

impl Rank {
    pub fn from_index(index: u8) -> Self {
        match index {
            0 => Rank::Three, 1 => Rank::Four, 2 => Rank::Five, 3 => Rank::Six,
            4 => Rank::Seven, 5 => Rank::Eight, 6 => Rank::Nine, 7 => Rank::Ten,
            8 => Rank::Jack, 9 => Rank::Queen, 10 => Rank::King, 11 => Rank::Ace, 12 => Rank::Two,
            _ => Rank::Three,
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            Rank::Three => 3, Rank::Four => 4, Rank::Five => 5, Rank::Six => 6,
            Rank::Seven => 7, Rank::Eight => 8, Rank::Nine => 9, Rank::Ten => 10,
            Rank::Jack => 11, Rank::Queen => 12, Rank::King => 13, Rank::Ace => 14, Rank::Two => 15,
        }
    }
}

/// 大小王
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Joker {
    Small,
    Large,
}

/// 扑克牌
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    pub suit: Option<Suit>,
    pub rank: Option<Rank>,
    pub joker: Option<Joker>,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // 大小王最大
        if self.is_joker() && !other.is_joker() { return std::cmp::Ordering::Greater; }
        if !self.is_joker() && other.is_joker() { return std::cmp::Ordering::Less; }
        if self.is_joker() && other.is_joker() {
            return match (self.joker, other.joker) {
                (Some(Joker::Large), Some(Joker::Small)) => std::cmp::Ordering::Greater,
                (Some(Joker::Small), Some(Joker::Large)) => std::cmp::Ordering::Less,
                _ => std::cmp::Ordering::Equal,
            };
        }

        let self_suit = self.suit.unwrap_or(Suit::Spades);
        let other_suit = other.suit.unwrap_or(Suit::Spades);
        let self_rank = self.rank.unwrap_or(Rank::Three);
        let other_rank = other.rank.unwrap_or(Rank::Three);

        // 比较花色
        let self_suit_val = match self_suit { Suit::Spades => 0, Suit::Hearts => 1, Suit::Clubs => 2, Suit::Diamonds => 3 };
        let other_suit_val = match other_suit { Suit::Spades => 0, Suit::Hearts => 1, Suit::Clubs => 2, Suit::Diamonds => 3 };
        if self_suit_val != other_suit_val {
            return other_suit_val.cmp(&self_suit_val); // 黑桃最大
        }

        // 比较点数
        other_rank.value().cmp(&self_rank.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit: Some(suit), rank: Some(rank), joker: None }
    }

    pub fn small_joker() -> Self {
        Card { suit: None, rank: None, joker: Some(Joker::Small) }
    }

    pub fn large_joker() -> Self {
        Card { suit: None, rank: None, joker: Some(Joker::Large) }
    }

    pub fn is_joker(&self) -> bool {
        self.joker.is_some()
    }

    pub fn is_score(&self) -> bool {
        if self.is_joker() { return false; }
        matches!(self.rank, Some(Rank::Ace) | Some(Rank::Ten) | Some(Rank::Five))
    }

    pub fn score(&self) -> i32 {
        if self.is_joker() { return 0; }
        match self.rank {
            Some(Rank::Ace) | Some(Rank::Ten) => 10,
            Some(Rank::Five) => 5,
            _ => 0,
        }
    }
}

/// 游戏阶段
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GamePhase {
    Waiting,  // 等待
    Bidding,  // 叫分
    Playing,  // 打牌
    Scoring,  // 结算
}

/// 游戏状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRoom {
    pub id: String,
    pub players: Vec<GamePlayer>,
    pub phase: GamePhase,
    pub current_player: usize,
    pub current_bid: u32,
    pub bidder: Option<usize>,
    pub bids: Vec<Option<u32>>,
    pub trump_suit: Option<Suit>,
    pub bottom_cards: Vec<Card>,
    pub round_cards: Vec<Option<Card>>,
    pub scores: [i32; 2],
    pub round_number: u32,
    pub lead_suit: Option<Suit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamePlayer {
    pub user_id: String,
    pub username: String,
    pub cards: Vec<Card>,
    pub is_robot: bool,
    pub is_ready: bool,
}

impl GameRoom {
    pub fn new(room_id: String, players: Vec<GamePlayer>) -> Self {
        GameRoom {
            id: room_id,
            players,
            phase: GamePhase::Waiting,
            current_player: 0,
            current_bid: 0,
            bidder: None,
            bids: vec![None; 4],
            trump_suit: None,
            bottom_cards: Vec::new(),
            round_cards: vec![None; 4],
            scores: [0, 0],
            round_number: 0,
            lead_suit: None,
        }
    }

    pub fn deal_cards(&mut self) {
        let mut deck = create_deck();
        shuffle(&mut deck);

        for (i, player) in self.players.iter_mut().enumerate() {
            let start = i * 12;
            player.cards = deck[start..start+12].to_vec();
            sort_cards(&mut player.cards, &None);
        }

        self.bottom_cards = deck[48..].to_vec();
        self.phase = GamePhase::Bidding;
        self.current_player = 0;
    }

    pub fn bid(&mut self, player_id: usize, bid: u32) -> bool {
        if self.phase != GamePhase::Bidding { return false; }
        if player_id != self.current_player { return false; }
        if bid > self.current_bid && bid >= 75 {
            self.current_bid = bid;
            self.bids[player_id] = Some(bid);
            self.bidder = Some(player_id);
            return true;
        }
        false
    }

    pub fn pass_bid(&mut self, player_id: usize) -> bool {
        if self.phase != GamePhase::Bidding { return false; }
        if player_id != self.current_player { return false; }

        self.bids[player_id] = Some(0);

        if self.bids.iter().all(|b| b.is_some()) {
            self.finish_bidding();
        } else {
            self.current_player = (self.current_player + 1) % 4;
        }
        true
    }

    fn finish_bidding(&mut self) {
        if let Some(bidder_idx) = self.bidder {
            let player = &mut self.players[bidder_idx];
            player.is_ready = true;
            player.cards.extend(self.bottom_cards.clone());
            sort_cards(&mut player.cards, &self.trump_suit);
            let keep = player.cards.len() - 6;
            player.cards.truncate(keep);
            self.trump_suit = Some(Suit::Spades);
            self.phase = GamePhase::Playing;
            self.current_player = bidder_idx;
            self.round_number = 1;
            self.round_cards = vec![None; 4];
        }
    }

    pub fn can_play_card(&self, player_id: usize, card: &Card) -> bool {
        if self.phase != GamePhase::Playing { return false; }

        if self.round_cards.iter().all(|c| c.is_none()) {
            return true;
        }

        if let Some(lead) = self.lead_suit {
            if card.is_joker() || card.suit == Some(self.trump_suit.unwrap_or(Suit::Spades)) {
                return true;
            }
            if card.suit == Some(lead) {
                return true;
            }
            let has_lead_suit = self.players[player_id].cards.iter()
                .any(|c| c.suit == Some(lead) && !c.is_joker());
            return !has_lead_suit;
        }

        true
    }

    pub fn play_card(&mut self, player_id: usize, card_idx: usize) -> Option<Card> {
        if self.phase != GamePhase::Playing { return None; }
        if player_id != self.current_player { return None; }

        let player = &mut self.players[player_id];
        if card_idx >= player.cards.len() { return None; }

        let card = player.cards.remove(card_idx);
        self.round_cards[player_id] = Some(card);

        if self.lead_suit.is_none() {
            self.lead_suit = card.suit;
        }

        self.current_player = (self.current_player + 1) % 4;

        if self.round_cards.iter().all(|c| c.is_some()) {
            self.finish_round();
        }

        Some(card)
    }

    fn finish_round(&mut self) {
        let mut winner = 0;
        let mut max_card: Option<&Card> = None;

        for (i, card_opt) in self.round_cards.iter().enumerate() {
            if let Some(card) = card_opt {
                if let Some(max) = max_card {
                    if card_greater(card, max, &self.trump_suit) {
                        winner = i;
                        max_card = Some(card);
                    }
                } else {
                    winner = i;
                    max_card = Some(card);
                }
            }
        }

        let round_score: i32 = self.round_cards.iter()
            .filter_map(|c| c.as_ref())
            .map(|c| c.score())
            .sum();

        let winner_team = winner % 2;
        self.scores[winner_team] += round_score;

        self.round_cards = vec![None; 4];
        self.lead_suit = None;
        self.current_player = winner;
        self.round_number += 1;
    }

    pub fn is_game_over(&self) -> bool {
        self.players.iter().all(|p| p.cards.is_empty())
    }

    pub fn get_result(&self) -> Option<(bool, i32, u32)> {
        if !self.is_game_over() { return None; }
        let bidder_idx = self.bidder?;
        let bidder_team = bidder_idx % 2;
        let bidder_score = self.scores[bidder_team];
        let success = bidder_score >= self.current_bid as i32;
        Some((success, bidder_score, self.current_bid))
    }
}

fn create_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(54);
    for suit_idx in 0..4 {
        let suit = Suit::from_index(suit_idx);
        for rank_idx in 0..13 {
            let rank = Rank::from_index(rank_idx);
            deck.push(Card::new(suit, rank));
        }
    }
    deck.push(Card::small_joker());
    deck.push(Card::large_joker());
    deck
}

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

fn sort_cards(cards: &mut Vec<Card>, _trump_suit: &Option<Suit>) {
    cards.sort_by(|a, b| b.cmp(a)); // 从大到小排序
}

fn card_cmp(a: &Card, b: &Card, trump_suit: &Option<Suit>) -> i32 {
    if a.is_joker() && !b.is_joker() { return 1; }
    if !a.is_joker() && b.is_joker() { return -1; }
    if a.is_joker() && b.is_joker() {
        return match (a.joker, b.joker) {
            (Some(Joker::Large), Some(Joker::Small)) => 1,
            (Some(Joker::Small), Some(Joker::Large)) => -1,
            _ => 0,
        };
    }

    let a_suit = a.suit.unwrap_or(Suit::Spades);
    let b_suit = b.suit.unwrap_or(Suit::Spades);
    let a_rank = a.rank.unwrap_or(Rank::Three);
    let b_rank = b.rank.unwrap_or(Rank::Three);

    if let Some(trump) = trump_suit {
        if a_suit == *trump && b_suit != *trump { return 1; }
        if b_suit == *trump && a_suit != *trump { return -1; }
    }

    let a_suit_val = match a_suit { Suit::Spades => 0, Suit::Hearts => 1, Suit::Clubs => 2, Suit::Diamonds => 3 };
    let b_suit_val = match b_suit { Suit::Spades => 0, Suit::Hearts => 1, Suit::Clubs => 2, Suit::Diamonds => 3 };
    if a_suit_val != b_suit_val { return b_suit_val.cmp(&a_suit_val) as i32; }

    b_rank.value().cmp(&a_rank.value()) as i32
}

fn card_greater(a: &Card, b: &Card, trump_suit: &Option<Suit>) -> bool {
    card_cmp(a, b, trump_suit) > 0
}