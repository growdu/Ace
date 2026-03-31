use super::types::{Card, GameState, GamePhase, Player, Suit};
use super::card::{create_deck, card_cmp};
use rand::seq::SliceRandom;

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
        let mut rng = rand::thread_rng();
        deck.shuffle(&mut rng);

        // 每人发12张牌
        for (i, player) in self.state.players.iter_mut().enumerate() {
            let start = i * 12;
            let end = start + 12;
            player.cards = deck[start..end].to_vec();
            // 排序手牌
            player.cards.sort_by(|a, b| {
                let cmp = card_cmp(a, b, &self.state.trump_suit);
                if cmp > 0 {
                    std::cmp::Ordering::Greater
                } else if cmp < 0 {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            });
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

    /// 确认庄家并开始游戏
    pub fn confirm_bidder(&mut self) {
        if let Some(bidder_id) = self.state.bidder {
            // 设置庄家
            self.state.players[bidder_id].is_host = true;

            // 庄家拿底牌
            let player = &mut self.state.players[bidder_id];
            player.cards.extend(self.state.bottom_cards.clone());

            // 排序并保留12张
            player.cards.sort_by(|a, b| {
                let cmp = card_cmp(a, b, &self.state.trump_suit);
                if cmp > 0 {
                    std::cmp::Ordering::Greater
                } else if cmp < 0 {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            player.cards.truncate(12);

            // 设置主牌花色（简化：使用黑桃）
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
            self.state.round_number += 1;
        }

        // 移动到下一个玩家
        self.state.current_player = (self.state.current_player + 1) % 4;

        Ok(card)
    }

    /// 确定回合获胜者
    fn determine_round_winner(&mut self) -> usize {
        let mut winner = 0;
        let mut max_card: Option<&Card> = None;

        for (i, card_opt) in self.state.round_cards.iter().enumerate() {
            if let Some(card) = card_opt {
                if let Some(max) = max_card {
                    if card_cmp(card, max, &self.state.trump_suit) > 0 {
                        winner = i;
                        max_card = Some(card);
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

        // 重置轮次牌
        self.state.round_cards = vec![None; 4];

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
