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
        return match (a.joker, b.joker) {
            (Some(Joker::Large), Some(Joker::Small)) => 1,
            (Some(Joker::Small), Some(Joker::Large)) => -1,
            _ => 0,
        };
    }

    // 普通牌比较
    let a_suit = a.suit.unwrap();
    let b_suit = b.suit.unwrap();
    let a_rank = a.rank.unwrap();
    let b_rank = b.rank.unwrap();

    // 先比较花色
    let suit_cmp_result = suit_cmp(&a_suit, &b_suit, trump_suit);
    if suit_cmp_result != 0 {
        return suit_cmp_result;
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
        if a == trump && b != trump {
            return 1;
        }
        if b == trump && a != trump {
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
        1 // 索引越小，花色越大
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
