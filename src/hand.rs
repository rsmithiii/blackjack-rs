use std::fmt;
use std::vec;

use crate::rank;
use crate::card;

#[derive(Debug)]
pub struct Hand(Vec<card::Card>);

impl fmt::Display for Hand
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "[ ")?;
        for (index, card) in self.0.iter().enumerate()
        {
            if index != 0 { write!(f, ", ")?; }
            write!(f, "{}", card)?;
        }
        write!(f, " ]")
    }
}

impl Hand
{
    pub fn new() -> Hand
    {
        Hand(vec![])
    }

    pub fn get_point_value(&self) -> u32
    {
        let mut points: u32 = 0;
        let mut aces: u32 = 0;
        for card in self.0.iter()
        {
            points += card.get_point_value();
            if card.rank == rank::Rank::Ace
            {
                aces += 1;
            }
        }
        points += aces * 10;
        while points > 21 && aces > 0
        {
            points -= 10;
            aces -= 1;
        }
        points
    }

    pub fn get_num_cards(&self) -> u32
    {
        self.0.len() as u32
    }

    pub fn add_card_to_hand(&mut self, card: card::Card)
    {
        self.0.push(card);
    }

    pub fn discard_hand(&mut self) -> Vec<card::Card>
    {
        let discard: Vec<card::Card> = self.0.drain(..).collect();
        discard
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::suit;

    #[test]
    fn test_format_hand()
    {
        let empty_hand = Hand::new();
        let mut one_card_hand = Hand::new();
        one_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        let mut two_card_hand = Hand::new();
        two_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        two_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        let mut three_card_hand = Hand::new();
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds});

        let formatted_string = format!("{}", empty_hand);
        assert_eq!(formatted_string, "[  ]");
        let formatted_string = format!("{}", one_card_hand);
        assert_eq!(formatted_string, "[ Ace of Spades ]");
        let formatted_string = format!("{}", two_card_hand);
        assert_eq!(formatted_string, "[ Ace of Spades, Jack of Clubs ]");
        let formatted_string = format!("{}", three_card_hand);
        assert_eq!(formatted_string, "[ Ace of Spades, Jack of Clubs, 2 of Diamonds ]");
    }

    #[test]
    fn test_new_hand()
    {
        let empty_hand = Hand::new();

        assert_eq!(empty_hand.0, []);
    }

    #[test]
    fn test_hand_get_point_value()
    {
        let empty_hand = Hand::new();
        let mut one_card_hand = Hand::new();
        one_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        let mut two_card_hand = Hand::new();
        two_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        two_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        let mut three_card_hand = Hand::new();
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds});

        assert_eq!(empty_hand.get_point_value(), 0);
        assert_eq!(one_card_hand.get_point_value(), 11);
        assert_eq!(two_card_hand.get_point_value(), 21);
        assert_eq!(three_card_hand.get_point_value(), 13);
    }

    #[test]
    fn test_add_card_to_hand()
    {
        let empty_hand = Hand::new();
        let mut one_card_hand = Hand::new();
        one_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        let mut two_card_hand = Hand::new();
        two_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        two_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        let mut three_card_hand = Hand::new();
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds});

        assert!(!empty_hand.0.contains(&card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades}));
        assert!(!empty_hand.0.contains(&card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs}));
        assert!(!empty_hand.0.contains(&card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds}));
        assert_eq!(empty_hand.0.len(), 0);
        assert!(one_card_hand.0.contains(&card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades}));
        assert!(!one_card_hand.0.contains(&card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs}));
        assert!(!one_card_hand.0.contains(&card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds}));
        assert_eq!(one_card_hand.0.len(), 1);
        assert!(two_card_hand.0.contains(&card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades}));
        assert!(two_card_hand.0.contains(&card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs}));
        assert!(!two_card_hand.0.contains(&card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds}));
        assert_eq!(two_card_hand.0.len(), 2);
        assert!(three_card_hand.0.contains(&card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades}));
        assert!(three_card_hand.0.contains(&card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs}));
        assert!(three_card_hand.0.contains(&card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds}));
        assert_eq!(three_card_hand.0.len(), 3);
    }

    #[test]
    fn test_get_num_cards()
    {
        let empty_hand = Hand::new();
        let mut one_card_hand = Hand::new();
        one_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        let mut two_card_hand = Hand::new();
        two_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        two_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        let mut three_card_hand = Hand::new();
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds});

        assert_eq!(empty_hand.get_num_cards(), 0);
        assert_eq!(one_card_hand.get_num_cards(), 1);
        assert_eq!(two_card_hand.get_num_cards(), 2);
        assert_eq!(three_card_hand.get_num_cards(), 3);
    }

    #[test]
    fn test_discard_hand()
    {
        let mut three_card_hand = Hand::new();
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        three_card_hand.add_card_to_hand(card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds});

        assert_eq!(three_card_hand.0.len(), 3);

        let discarded_hand = three_card_hand.discard_hand();

        assert_eq!(discarded_hand, [card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades},
            card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs},
            card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds}]);

        assert_eq!(three_card_hand.0.len(), 0);
    }
}
