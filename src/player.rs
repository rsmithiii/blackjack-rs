use std::io;
use std::io::Write;
use std::io::BufRead;
use crate::hand;
use crate::card;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlackjackAction
{
    Hit,
    Stay,
}

pub trait BlackjackPlayer
{
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn hit_or_stay(&self) -> BlackjackAction;
    fn add_card_to_hand(&mut self, card: card::Card);
    fn discard_hand(&mut self) -> Vec<card::Card>;
    fn get_point_value(&self) -> u32;
    fn get_num_cards(&self) -> u32;

    fn hand_under_21(&self) -> bool
    {
        self.get_point_value() <= 21
    }

    fn blackjack_hand(&self) -> bool
    {
        self.get_point_value() == 21 && self.get_num_cards() == 2
    }
}

pub struct HumanPlayer
{
    name: &'static str,
    pub hand: hand::Hand
}

impl HumanPlayer
{
    fn hit_or_stay_strategy<R: io::Read, W: io::Write>(&self, mut reader: io::BufReader<R>, writer: &mut io::BufWriter<W>) -> BlackjackAction
    {
        let mut tries_remaining = 3u8;
        let mut action: Option<BlackjackAction> = None;
        let mut input = String::new();

        let _ = write!(writer, "Hit or Stay? ");
        writer.flush().unwrap();
        while action == None && tries_remaining > 0
        {
            input.clear();
            let _ = reader.read_line(&mut input);
            action = match input.to_lowercase().as_str().trim()
            {
                "hit" => Some(BlackjackAction::Hit),
                "stay" => Some(BlackjackAction::Stay),
                _ => None
            };
            tries_remaining -= 1;

            let _ = match action
            {
                Some(BlackjackAction::Hit) => writeln!(writer, "Okay, you want to hit."),
                Some(BlackjackAction::Stay) => writeln!(writer, "Okay, you want to stay."),
                _ => writeln!(writer, "That didn't make any sense..."),
            };

            if action.is_none() && tries_remaining > 0
            {
                let _ = write!(writer, "Let's try again... Hit or Stay? ");
            }
            writer.flush().unwrap();
        }

        let fallback = || {
            let _ = writeln!(writer, "Let's just assume you want to stay.");
            writer.flush().unwrap();
            Some(BlackjackAction::Stay)
        };

        action.or_else(fallback).unwrap()
    }
}

impl BlackjackPlayer for HumanPlayer
{
    fn new(name: &'static str) -> HumanPlayer
    {
        HumanPlayer { name: name, hand: hand::Hand::new() }
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn hit_or_stay(&self) -> BlackjackAction
    {
        self.hit_or_stay_strategy(io::BufReader::new(io::stdin()), &mut io::BufWriter::new(io::stdout()))
    }

    fn add_card_to_hand(&mut self, card: card::Card)
    {
        self.hand.add_card_to_hand(card);
    }

    fn discard_hand(&mut self) -> Vec<card::Card>
    {
        self.hand.discard_hand()
    }

    fn get_point_value(&self) -> u32
    {
        self.hand.get_point_value()
    }

    fn get_num_cards(&self) -> u32
    {
        self.hand.get_num_cards()
    }

}

pub struct Dealer
{
    name: &'static str,
    pub hand: hand::Hand
}

impl BlackjackPlayer for Dealer
{
    fn new(_: &'static str) -> Dealer
    {
        Dealer { name: "Dealer", hand: hand::Hand::new() }
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn hit_or_stay(&self) -> BlackjackAction
    {
        if self.hand.get_point_value() >= 17
        {
            BlackjackAction::Stay
        }
        else
        {
            BlackjackAction::Hit
        }
    }

    fn add_card_to_hand(&mut self, card: card::Card)
    {
        self.hand.add_card_to_hand(card);
    }

    fn discard_hand(&mut self) -> Vec<card::Card>
    {
        self.hand.discard_hand()
    }

    fn get_point_value(&self) -> u32
    {
        self.hand.get_point_value()
    }

    fn get_num_cards(&self) -> u32
    {
        self.hand.get_num_cards()
    }

}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::rank;
    use crate::suit;
    use std::io::Cursor;
    use std::io::BufReader;
    use std::io::BufWriter;

    #[test]
    fn test_human_player_new_player_name()
    {
        let player1: HumanPlayer = BlackjackPlayer::new("Player 1");
        assert_eq!(player1.name(), "Player 1");
    }

    #[test]
    fn test_human_player_get_point_value()
    {
        let player1: HumanPlayer = BlackjackPlayer::new("Player 1");
        let mut player2: HumanPlayer = BlackjackPlayer::new("Player 2");
        let mut player3: HumanPlayer = BlackjackPlayer::new("Player 3");
        let mut player4: HumanPlayer = BlackjackPlayer::new("Player 4");
        player2.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player3.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player3.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds});

        assert_eq!(player1.get_point_value(), 0);
        assert_eq!(player2.get_point_value(), 11);
        assert_eq!(player3.get_point_value(), 21);
        assert_eq!(player4.get_point_value(), 13);
    }

    #[test]
    fn test_human_player_add_card_to_hand()
    {
        let player1: HumanPlayer = BlackjackPlayer::new("Player 1");
        let mut player2: HumanPlayer = BlackjackPlayer::new("Player 2");
        player2.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        let mut player3: HumanPlayer = BlackjackPlayer::new("Player 3");
        player3.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player3.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        let mut player4: HumanPlayer = BlackjackPlayer::new("Player 4");
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds});

        assert_eq!(player1.get_num_cards(), 0);
        assert_eq!(player2.get_num_cards(), 1);
        assert_eq!(player3.get_num_cards(), 2);
        assert_eq!(player4.get_num_cards(), 3);
    }

    #[test]
    fn test_human_player_get_num_cards()
    {
        let player1: HumanPlayer = BlackjackPlayer::new("Player 1");
        let mut player2: HumanPlayer = BlackjackPlayer::new("Player 2");
        player2.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        let mut player3: HumanPlayer = BlackjackPlayer::new("Player 3");
        player3.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player3.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        let mut player4: HumanPlayer = BlackjackPlayer::new("Player 4");
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds});

        assert_eq!(player1.get_num_cards(), 0);
        assert_eq!(player2.get_num_cards(), 1);
        assert_eq!(player3.get_num_cards(), 2);
        assert_eq!(player4.get_num_cards(), 3);
    }

    #[test]
    fn test_human_player_discard_hand()
    {
        let mut player4: HumanPlayer = BlackjackPlayer::new("Player 4");
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds});

        assert_eq!(player4.get_num_cards(), 3);

        let discarded_hand = player4.discard_hand();

        assert_eq!(discarded_hand, [card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades},
            card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs},
            card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds}]);

        assert_eq!(player4.get_num_cards(), 0);
    }

    #[test]
    fn test_human_player_hit_or_stay_strategy()
    {
        let cursor = Cursor::new(String::from("Hit").into_bytes());
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));

        let player: HumanPlayer = BlackjackPlayer::new("Player");
        assert_eq!(player.hit_or_stay_strategy(BufReader::new(cursor), &mut writer), BlackjackAction::Hit);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Hit or Stay?"));
        assert!(captured_output.contains("Okay, you want to hit."));

        let cursor = Cursor::new(String::from("Stay").into_bytes());
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));
        assert_eq!(player.hit_or_stay_strategy(BufReader::new(cursor), &mut writer), BlackjackAction::Stay);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Hit or Stay?"));
        assert!(captured_output.contains("Okay, you want to stay."));

        let cursor = Cursor::new(String::from("Anything Else").into_bytes());
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));
        assert_eq!(player.hit_or_stay_strategy(BufReader::new(cursor), &mut writer), BlackjackAction::Stay);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Hit or Stay?"));
        assert!(captured_output.contains("That didn't make any sense..."));
        assert!(captured_output.contains("Let's just assume you want to stay."));
        assert_eq!(captured_output.matches("That didn't make any sense...").collect::<Vec<_>>().len(), 3);

        let cursor = Cursor::new(String::from("asdf\njkl;\nHit\n").into_bytes());
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));
        assert_eq!(player.hit_or_stay_strategy(BufReader::new(cursor), &mut writer), BlackjackAction::Hit);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Hit or Stay?"));
        assert!(captured_output.contains("That didn't make any sense..."));
        assert!(captured_output.contains("Okay, you want to hit."));
        assert_eq!(captured_output.matches("That didn't make any sense...").collect::<Vec<_>>().len(), 2);

        let cursor = Cursor::new(String::from("asdf\njkl;\nStay\n").into_bytes());
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));
        assert_eq!(player.hit_or_stay_strategy(BufReader::new(cursor), &mut writer), BlackjackAction::Stay);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Hit or Stay?"));
        assert!(captured_output.contains("That didn't make any sense..."));
        assert!(captured_output.contains("Okay, you want to stay."));
        assert_eq!(captured_output.matches("That didn't make any sense...").collect::<Vec<_>>().len(), 2);

        let cursor = Cursor::new(Vec::new());
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));
        assert_eq!(player.hit_or_stay_strategy(BufReader::new(cursor), &mut writer), BlackjackAction::Stay);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Hit or Stay?"));
        assert!(captured_output.contains("That didn't make any sense..."));
        assert!(captured_output.contains("Let's just assume you want to stay."));
        assert_eq!(captured_output.matches("That didn't make any sense...").collect::<Vec<_>>().len(), 3);
    }

    #[test]
    fn test_dealer_new_player_name()
    {
        let player1: Dealer = BlackjackPlayer::new("Dealer 1");
        assert_eq!(player1.name(), "Dealer");
    }

    #[test]
    fn test_dealer_get_point_value()
    {
        let player1: Dealer = BlackjackPlayer::new("Dealer 1");
        let mut player2: Dealer = BlackjackPlayer::new("Dealer 2");
        let mut player3: Dealer = BlackjackPlayer::new("Dealer 3");
        let mut player4: Dealer = BlackjackPlayer::new("Dealer 4");
        player2.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player3.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player3.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds});

        assert_eq!(player1.get_point_value(), 0);
        assert_eq!(player2.get_point_value(), 11);
        assert_eq!(player3.get_point_value(), 21);
        assert_eq!(player4.get_point_value(), 13);
    }

    #[test]
    fn test_dealer_add_card_to_hand()
    {
        let player1: Dealer = BlackjackPlayer::new("Dealer 1");
        let mut player2: Dealer = BlackjackPlayer::new("Dealer 2");
        player2.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        let mut player3: Dealer = BlackjackPlayer::new("Dealer 3");
        player3.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player3.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        let mut player4: Dealer = BlackjackPlayer::new("Dealer 4");
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds});

        assert_eq!(player1.get_num_cards(), 0);
        assert_eq!(player2.get_num_cards(), 1);
        assert_eq!(player3.get_num_cards(), 2);
        assert_eq!(player4.get_num_cards(), 3);
    }

    #[test]
    fn test_dealer_get_num_cards()
    {
        let player1: Dealer = BlackjackPlayer::new("Dealer 1");
        let mut player2: Dealer = BlackjackPlayer::new("Dealer 2");
        player2.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        let mut player3: Dealer = BlackjackPlayer::new("Dealer 3");
        player3.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player3.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        let mut player4: Dealer = BlackjackPlayer::new("Dealer 4");
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds});

        assert_eq!(player1.get_num_cards(), 0);
        assert_eq!(player2.get_num_cards(), 1);
        assert_eq!(player3.get_num_cards(), 2);
        assert_eq!(player4.get_num_cards(), 3);
    }

    #[test]
    fn test_dealer_discard_hand()
    {
        let mut player4: Dealer = BlackjackPlayer::new("Dealer 4");
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        player4.add_card_to_hand(card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds});

        assert_eq!(player4.get_num_cards(), 3);

        let discarded_hand = player4.discard_hand();

        assert_eq!(discarded_hand, [card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades},
            card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs},
            card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds}]);

        assert_eq!(player4.get_num_cards(), 0);
    }

    #[test]
    fn test_dealer_hit_or_stay_strategy()
    {
        let mut player: Dealer = BlackjackPlayer::new("Dealer");
        assert_eq!(player.hit_or_stay(), BlackjackAction::Hit);

        player.add_card_to_hand(card::Card { rank: rank::Rank::Two, suit: suit::Suit::Diamonds});
        assert_eq!(player.hit_or_stay(), BlackjackAction::Hit);

        player.add_card_to_hand(card::Card { rank: rank::Rank::Jack, suit: suit::Suit::Clubs});
        assert_eq!(player.hit_or_stay(), BlackjackAction::Hit);

        player.add_card_to_hand(card::Card { rank: rank::Rank::Four, suit: suit::Suit::Hearts});
        assert_eq!(player.hit_or_stay(), BlackjackAction::Hit);

        player.add_card_to_hand(card::Card { rank: rank::Rank::Ace, suit: suit::Suit::Spades});
        assert_eq!(player.hit_or_stay(), BlackjackAction::Stay);

        player.add_card_to_hand(card::Card { rank: rank::Rank::Three, suit: suit::Suit::Clubs});
        assert_eq!(player.hit_or_stay(), BlackjackAction::Stay);
    }
}
