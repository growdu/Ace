use crate::game::{types::Card, types::Suit, card::card_cmp};

/// 机器人难度等级
#[derive(Debug, Clone, Copy)]
pub enum BotLevel {
    Easy,    // 简单
    Normal,  // 普通
    Hard,    // 困难
}

/// 机器人
#[derive(Debug)]
pub struct Bot {
    level: BotLevel,
}

impl Bot {
    pub fn new(level: BotLevel) -> Self {
        Bot { level }
    }

    /// 选择出牌
    pub fn choose_card(
        &self,
        hand: &[Card],
        current_lead: &Option<Suit>,
        trump_suit: &Option<Suit>,
        round_cards: &[Option<Card>],
    ) -> Option<usize> {
        if hand.is_empty() {
            return None;
        }

        match self.level {
            BotLevel::Easy => self.easy_strategy(hand),
            BotLevel::Normal => self.normal_strategy(hand, current_lead, trump_suit),
            BotLevel::Hard => self.hard_strategy(hand, current_lead, trump_suit, round_cards),
        }
    }

    /// 简单策略：随机出牌
    fn easy_strategy(&self, hand: &[Card]) -> Option<usize> {
        use std::time::SystemTime;
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize;
        Some(seed % hand.len())
    }

    /// 普通策略：优先出小牌，保留分数牌
    fn normal_strategy(
        &self,
        hand: &[Card],
        current_lead: &Option<Suit>,
        trump_suit: &Option<Suit>,
    ) -> Option<usize> {
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
    fn hard_strategy(
        &self,
        hand: &[Card],
        current_lead: &Option<Suit>,
        trump_suit: &Option<Suit>,
        _round_cards: &[Option<Card>],
    ) -> Option<usize> {
        // 检查是否有首家花色
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
        self.normal_strategy(hand, current_lead, trump_suit)
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
    use crate::game::types::Rank;

    #[test]
    fn test_bot_creation() {
        let bot = Bot::new(BotLevel::Normal);
        assert!(matches!(bot.level, BotLevel::Normal));
    }

    #[test]
    fn test_bot_decide_bid() {
        let bot = Bot::new(BotLevel::Normal);
        let hand = vec![
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Hearts, Rank::King),
        ];
        let bid = bot.decide_bid(&hand, &Some(Suit::Spades));
        assert!(bid >= 75);
    }
}
