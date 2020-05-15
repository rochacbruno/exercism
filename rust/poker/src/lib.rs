use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug)]
enum Value {
    Num(u8),
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Eq, PartialEq, Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Eq, PartialEq, Debug)]
struct Card {
    value: Value,
    suit: Suit,
}

impl Card {
    fn new(s: &str) -> Self {
        let s_value = s.chars().nth(0).unwrap();
        let s_suit = s.chars().last().unwrap();

        let value = match s_value {
            '1' => Value::Num(10u8),
            '2'..='9' => Value::Num(s_value.to_digit(10).unwrap() as u8),
            'J' => Value::Jack,
            'Q' => Value::Queen,
            'K' => Value::King,
            'A' => Value::Ace,
            _ => panic!("wrong value for card."),
        };

        let suit = match s_suit {
            'H' => Suit::Hearts,
            'D' => Suit::Diamonds,
            'S' => Suit::Spades,
            'C' => Suit::Clubs,
            _ => panic!("wrong suit option."),
        };

        Self { value, suit }
    }

    fn get_rank(&self) -> u8 {
        match self.value {
            Value::Num(n) => n,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13,
            Value::Ace => 14,
        }
    }
}

struct Hand<'a> {
    cards: Vec<Card>,
    s: &'a str,
}

// impl<'a> PartialOrd for Hand<'a> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

//     }
// }

impl<'a> Hand<'a> {
    fn new(s: &'a str) -> Self {
        let mut cards = Vec::new();
        for s_card in s.split_whitespace() {
            cards.push(Card::new(s_card));
        }

        // cards.sort_by(|a, b| a.get_rank().partial_cmp(b));
        Self { cards, s }
    }

    fn is_flush(&self) -> bool {
        self.cards.iter().all(|c| c.suit == self.cards[0].suit)
    }

    fn is_straight(&self) -> bool {

        let mut hand_ranks = [0;5];
        
        for (index, card) in self.cards.iter().enumerate() {
            hand_ranks[index] = if index == 0 && card.value == Value::Ace {
                1
            } else {
                card.get_rank()
            }
        }
        (1..=14).collect::<Vec<_>>().windows(5).any(|w| w == hand_ranks)
    }

    fn evaluate() -> u32 {
        

    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
// pub fn winning_hands<'a>(s_hands: &[&'a str]) -> Option<Vec<&'a str>> {
    // hands: &["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"]
    // let mut hands = s_hands.iter().map(Hand::new).collect::<Vec<Hand>>();

    // hands.sort_by(|a, b| b.partial_cmp(a).unwrap());
    // Some(Vec::new(&s_hands))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_card() {
        let card = Card::new("10H");
        assert_eq!(card.value, Value::Num(10));
        assert_eq!(card.suit, Suit::Hearts);
    }

    #[test]
    fn test_hand_is_straight() {
        let hand = Hand::new("4S 5S 6S 7S 8S");
        assert_eq!(hand.is_straight(), true);
    }


    #[test]
    fn test_hand_is_not_straight() {
        let hand = Hand::new("JS 2S 3S 4S 5S");
        assert_eq!(hand.is_straight(), false);
    }

    #[test]
    fn test_hand_is_flush() {
        let hand = Hand::new("4S 5S 6S 7S 8S");
        assert_eq!(hand.is_flush(), true);
    }
}
