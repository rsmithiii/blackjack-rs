use std::fmt;
use std::collections::VecDeque;
use rand::prelude::*;

use crate::card;
use crate::suit;
use crate::rank;

#[derive(Debug)]
pub struct Deck(VecDeque<card::Card>);

impl Deck
{
    pub fn new() -> Deck
    {
        let mut new_deck: Deck = Deck(VecDeque::new());
        for s in suit::Suit::Diamonds as u32..=suit::Suit::Spades as u32
        {
            for r in rank::Rank::Ace as u32..=rank::Rank::King as u32
            {
                let new_suit = suit::Suit::try_from(s);
                let new_rank = rank::Rank::try_from(r);

                match (new_rank, new_suit)
                {
                    (Ok(rr), Ok(ss)) => new_deck.0.push_back(card::Card { rank: rr , suit: ss } ),
                    _ => ()
                };
            }
        }

        new_deck
    }

    pub fn shuffle(&mut self)
    {
        let mut rng = rand::thread_rng();
        for _ in 1..5
        {
            self.0.make_contiguous().shuffle(&mut rng);
        }
    }

    pub fn collect_played_cards(&mut self, played_cards: Vec<card::Card>)
    {
        let mut collected_cards = VecDeque::from(played_cards);
        self.0.append(&mut collected_cards);
    }

    pub fn deal_card(&mut self) -> card::Card
    {
        self.0.pop_front().unwrap()
    }
}

impl fmt::Display for Deck
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

#[cfg(test)]
mod tests
{
    use super::*;
    use std::vec;

    #[test]
    fn test_format_suit()
    {
        let new_deck = Deck::new();

        let formatted_string = format!("{}", new_deck);
        assert!(formatted_string.contains("[ "));
        assert!(formatted_string.contains("Ace of Diamonds, "));
        assert!(formatted_string.contains("Queen of Spades, "));
        assert!(formatted_string.contains("King of Spades "));
        assert!(formatted_string.contains(" ]"));
    }

    #[test]
    fn test_new_deck_card_count()
    {
        let new_deck = Deck::new();

        assert_eq!(new_deck.0.len(), 52);
    }

    #[test]
    fn test_shuffled_deck()
    {
        let mut new_deck = Deck::new();

        assert_eq!(new_deck.0.len(), 52);

        let first_card_in_deck_before_shuffle: card::Card = card::Card { rank: new_deck.0[0].rank, suit: new_deck.0[0].suit };
        let seventh_card_in_deck_before_shuffle: card::Card = card::Card { rank: new_deck.0[6].rank, suit: new_deck.0[6].suit };
        let last_card_in_deck_before_shuffle: card::Card = card::Card { rank: new_deck.0[51].rank, suit: new_deck.0[51].suit };
        new_deck.shuffle();
        let first_card_in_deck_after_shuffle: card::Card = card::Card { rank: new_deck.0[0].rank, suit: new_deck.0[0].suit };
        let seventh_card_in_deck_after_shuffle: card::Card = card::Card { rank: new_deck.0[6].rank, suit: new_deck.0[6].suit };
        let last_card_in_deck_after_shuffle: card::Card = card::Card { rank: new_deck.0[51].rank, suit: new_deck.0[51].suit };
        assert_ne!(first_card_in_deck_before_shuffle, first_card_in_deck_after_shuffle);
        assert_ne!(seventh_card_in_deck_before_shuffle, seventh_card_in_deck_after_shuffle);
        assert_ne!(last_card_in_deck_before_shuffle, last_card_in_deck_after_shuffle);
        assert_eq!(new_deck.0.len(), 52);
    }

    #[test]
    fn test_collect_played_cards()
    {
        let mut new_deck = Deck::new();
        let mut collected_cards: Vec<card::Card> = vec![];

        assert_eq!(new_deck.0.len(), 52);

        collected_cards.push(new_deck.0.pop_front().unwrap());
        collected_cards.push(new_deck.0.pop_front().unwrap());
        collected_cards.push(new_deck.0.pop_front().unwrap());

        assert_eq!(new_deck.0.len(), 49);
        assert_eq!(collected_cards.len(), 3);

        new_deck.collect_played_cards(collected_cards);

        assert_eq!(new_deck.0.len(), 52);
    }

    #[test]
    fn test_deal_card()
    {
        let mut new_deck = Deck::new();

        assert_eq!(new_deck.0.len(), 52);

        let dealt_card = new_deck.deal_card();

        assert_eq!(new_deck.0.len(), 51);
        assert_eq!(dealt_card, card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Diamonds });
    }
}
