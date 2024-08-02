use std::fmt;

use crate::suit;
use crate::rank;

#[derive(Debug, Eq, PartialEq)]
pub struct Card
{
    pub rank: rank::Rank,
    pub suit: suit::Suit,
}

impl fmt::Display for Card
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

impl Card
{
    pub fn get_point_value(&self) -> u32
    {
        match &self.rank
        {
            rank::Rank::Jack => 10,
            rank::Rank::Queen => 10,
            rank::Rank::King => 10,
            _ => self.rank as u32
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_format_card()
    {
        let ace_spades: Card = Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades};
        let jack_clubs: Card = Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs};
        let two_diamonds: Card = Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds};
        let formatted_string = format!("{}", ace_spades);
        assert_eq!(formatted_string, "Ace of Spades");
        let formatted_string = format!("{}", jack_clubs);
        assert_eq!(formatted_string, "Jack of Clubs");
        let formatted_string = format!("{}", two_diamonds);
        assert_eq!(formatted_string, "2 of Diamonds");
    }

    #[test]
    fn test_get_point_value()
    {
        let ace_spades: Card = Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades};
        let two_diamonds: Card = Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds};
        let three_diamonds: Card = Card { rank: rank::Rank::Three, suit: suit::Suit::Diamonds};
        let four_diamonds: Card = Card { rank: rank::Rank::Four, suit: suit::Suit::Diamonds};
        let five_diamonds: Card = Card { rank: rank::Rank::Five, suit: suit::Suit::Diamonds};
        let six_diamonds: Card = Card { rank: rank::Rank::Six, suit: suit::Suit::Diamonds};
        let seven_diamonds: Card = Card { rank: rank::Rank::Seven, suit: suit::Suit::Diamonds};
        let eight_diamonds: Card = Card { rank: rank::Rank::Eight, suit: suit::Suit::Diamonds};
        let nine_diamonds: Card = Card { rank: rank::Rank::Nine, suit: suit::Suit::Diamonds};
        let ten_diamonds: Card = Card { rank: rank::Rank::Ten, suit: suit::Suit::Diamonds};
        let jack_clubs: Card = Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs};
        let queen_hearts: Card = Card { rank: rank::Rank::Queen, suit: suit::Suit::Hearts};
        let king_hearts: Card = Card { rank: rank::Rank::King, suit: suit::Suit::Hearts};
        assert_eq!(ace_spades.get_point_value(), 1);
        assert_eq!(two_diamonds.get_point_value(), 2);
        assert_eq!(three_diamonds.get_point_value(), 3);
        assert_eq!(four_diamonds.get_point_value(), 4);
        assert_eq!(five_diamonds.get_point_value(), 5);
        assert_eq!(six_diamonds.get_point_value(), 6);
        assert_eq!(seven_diamonds.get_point_value(), 7);
        assert_eq!(eight_diamonds.get_point_value(), 8);
        assert_eq!(nine_diamonds.get_point_value(), 9);
        assert_eq!(ten_diamonds.get_point_value(), 10);
        assert_eq!(jack_clubs.get_point_value(), 10);
        assert_eq!(queen_hearts.get_point_value(), 10);
        assert_eq!(king_hearts.get_point_value(), 10);
    }
}
