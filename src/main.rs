#![allow(dead_code)]
use std::io;
use std::io::Write;
use std::io::BufRead;

mod suit;
mod rank;
mod card;
mod hand;
mod player;
mod deck;

use crate::player::BlackjackPlayer;

enum BlackjackPlayChoice
{
    Play,
    DontPlay,
}

fn ask_play_blackjack(first_game: bool) -> bool
{
    ask_play_blackjack_get_response(first_game, io::BufReader::new(io::stdin()), &mut io::BufWriter::new(io::stdout()))
}

fn ask_play_blackjack_get_response<R: io::Read, W: io::Write>(first_game: bool, mut reader: io::BufReader<R>, writer: &mut io::BufWriter<W>) -> bool
{
    let mut tries_remaining = 3u8;
    let mut choice: Option<bool> = None;
    let mut input = String::new();

    let _ = match first_game
    {
        true => write!(writer, "Do you want to play Blackjack? (yes/no) "),
        false => write!(writer, "Keep playing Blackjack? (yes/no) "),
    };
    writer.flush().unwrap();
    while choice == None && tries_remaining > 0
    {
        input.clear();
        let _ = reader.read_line(&mut input);
        choice = match input.to_lowercase().as_str().trim()
        {
            "yes" => Some(true),
            "no" => Some(false),
            _ => None
        };
        tries_remaining -= 1;

        let _ = match choice
        {
            Some(true) => writeln!(writer, "Alright! Let's play!"),
            Some(false) => writeln!(writer, "Okay. Maybe another time."),
            None => writeln!(writer, "I didn't understand that."),
        };
        writer.flush().unwrap();
    }

    let fallback = || {
        let _ = writeln!(writer, "There seems to be a failure to communicate between us. Perhaps we'll play another time.");
        writer.flush().unwrap();
        Some(false)
    };

    choice.or_else(fallback).unwrap()
}

fn main()
{
    let mut deck = deck::Deck::new();
    // Conditionally print deck
    let mut play_blackjack = ask_play_blackjack(true);
    // println!("Play Blackjack choice: {}", play_blackjack);

    while play_blackjack
    {
        let mut dealer: player::Dealer = player::Dealer::new("Dealer");
        let mut player: player::HumanPlayer = player::HumanPlayer::new("Player 1");

        println!("Shuffling the deck");
        deck.shuffle();
        // Conditionally print deck

        println!("Dealing cards");
        player.add_card_to_hand(deck.deal_card());
        dealer.add_card_to_hand(deck.deal_card());
        player.add_card_to_hand(deck.deal_card());
        dealer.add_card_to_hand(deck.deal_card());

        // Conditionally print deck

        let mut continue_playing = true;
        let mut dealer_plays = true;

        while continue_playing
        {
            if player.hand_under_21()
            {
                if player.blackjack_hand()
                {
                    if dealer.blackjack_hand()
                    {
                        println!("PUSH!");
                    }
                    else
                    {
                        println!("{0} got BLACKJACK!!! {1} WINS!!!", player.name(), player.name().to_uppercase());
                    }
                    continue_playing = false;
                    dealer_plays = false;
                }
                else if dealer.blackjack_hand()
                {
                    println!("{0} got BLACKJACK!!! {0} WINS!!!", dealer.name());
                    continue_playing = false;
                    dealer_plays = false;
                }
                else
                {
                    println!("{}'s hand: ", player.name());
                    println!("{}", player.hand);
                    let choice = player.hit_or_stay();
                    let _ = match choice
                    {
                        player::BlackjackAction::Hit => player.add_card_to_hand(deck.deal_card()),
                        player::BlackjackAction::Stay => {
                            println!("{} Total: {}", player.name(), player.get_point_value());
                            continue_playing = false;
                        }
                    };
                }
            }
            else
            {
                println!("{} Total: {}", player.name(), player.get_point_value());
                println!("BUST! You lost this round.");
                continue_playing = false;
                dealer_plays = false;
            }
        }

        while dealer_plays
        {
            if dealer.hand_under_21()
            {
                if dealer.blackjack_hand()
                {
                    println!("{0} got BLACKJACK!!! {0} WINS!!!???", dealer.name());
                    dealer_plays = false;
                }
                else
                {
                    let choice = dealer.hit_or_stay();
                    let _ = match choice
                    {
                        player::BlackjackAction::Hit => dealer.add_card_to_hand(deck.deal_card()),
                        player::BlackjackAction::Stay => {
                            println!("{} Total: {}", dealer.name(), dealer.get_point_value());
                            dealer_plays = false;

                            let mut winner: Option<&str> = None;
                            if dealer.get_point_value() > player.get_point_value()
                            {
                                winner = Some(dealer.name());
                            }
                            else if dealer.get_point_value() < player.get_point_value()
                            {
                                winner = Some(player.name());
                            }

                            match winner
                            {
                                Some(x) => println!("{} WINS!!!", x),
                                None => println!("PUSH!"),
                            };
                        }
                    };
                }
            }
            else
            {
                println!("{} Total: {}", dealer.name(), dealer.get_point_value());
                println!("{} BUSTS! {} wins this round!", dealer.name(), player.name());
                dealer_plays = false;
            }
        }

        println!("{}'s hand: ", player.name());
        println!("{}", player.hand);
        println!("{}'s hand: ", dealer.name());
        println!("{}", dealer.hand);

        deck.collect_played_cards(player.discard_hand());
        deck.collect_played_cards(dealer.discard_hand());

        // Conditionally print deck

        play_blackjack = ask_play_blackjack(false);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use std::io::Cursor;
    use std::io::BufReader;
    use std::io::BufWriter;

    #[test]
    fn test_play_blackjack_get_response_first_time()
    {
        // Attempt

        let reader = BufReader::new(Cursor::new(String::from("yes").into_bytes()));
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));

        let response = ask_play_blackjack_get_response(true, reader, &mut writer);
        assert_eq!(response, true);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Do you want to play Blackjack? (yes/no)"));
        assert!(captured_output.contains("Alright! Let's play!"));

        // Attempt

        let reader = BufReader::new(Cursor::new(String::from("no").into_bytes()));
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));

        let response = ask_play_blackjack_get_response(true, reader, &mut writer);
        assert_eq!(response, false);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Do you want to play Blackjack? (yes/no)"));
        assert!(captured_output.contains("Okay. Maybe another time."));

        // Attempt

        let reader = BufReader::new(Cursor::new(String::from("Anything Else").into_bytes()));
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));

        let response = ask_play_blackjack_get_response(true, reader, &mut writer);
        assert_eq!(response, false);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Do you want to play Blackjack? (yes/no)"));
        assert!(captured_output.contains("I didn't understand that."));
        assert!(captured_output.contains("There seems to be a failure to communicate between us. Perhaps we'll play another time."));
        assert_eq!(captured_output.matches("I didn't understand that.").collect::<Vec<_>>().len(), 3);

        // Attempt

        let reader = BufReader::new(Cursor::new(String::from("asdf\njkl;\nyes").into_bytes()));
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));

        let response = ask_play_blackjack_get_response(true, reader, &mut writer);
        assert_eq!(response, true);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Do you want to play Blackjack? (yes/no)"));
        assert!(captured_output.contains("I didn't understand that."));
        assert!(captured_output.contains("Alright! Let's play!"));
        assert_eq!(captured_output.matches("I didn't understand that.").collect::<Vec<_>>().len(), 2);

        // Attempt

        let reader = BufReader::new(Cursor::new(String::from("asdf\njkl;\nno").into_bytes()));
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));

        let response = ask_play_blackjack_get_response(true, reader, &mut writer);
        assert_eq!(response, false);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Do you want to play Blackjack? (yes/no)"));
        assert!(captured_output.contains("I didn't understand that."));
        assert!(captured_output.contains("Okay. Maybe another time."));
        assert_eq!(captured_output.matches("I didn't understand that.").collect::<Vec<_>>().len(), 2);

        // Attempt

        let reader = BufReader::new(Cursor::new(Vec::new()));
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));

        let response = ask_play_blackjack_get_response(true, reader, &mut writer);
        assert_eq!(response, false);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Do you want to play Blackjack? (yes/no)"));
        assert!(captured_output.contains("I didn't understand that."));
        assert!(captured_output.contains("There seems to be a failure to communicate between us. Perhaps we'll play another time."));
        assert_eq!(captured_output.matches("I didn't understand that.").collect::<Vec<_>>().len(), 3);
    }

    #[test]
    fn test_play_blackjack_get_response_keep_playing()
    {
        // Attempt

        let reader = BufReader::new(Cursor::new(String::from("yes").into_bytes()));
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));

        let response = ask_play_blackjack_get_response(false, reader, &mut writer);
        assert_eq!(response, true);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Keep playing Blackjack? (yes/no)"));
        assert!(captured_output.contains("Alright! Let's play!"));

        // Attempt

        let reader = BufReader::new(Cursor::new(String::from("no").into_bytes()));
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));

        let response = ask_play_blackjack_get_response(false, reader, &mut writer);
        assert_eq!(response, false);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Keep playing Blackjack? (yes/no)"));
        assert!(captured_output.contains("Okay. Maybe another time."));

        // Attempt

        let reader = BufReader::new(Cursor::new(String::from("Anything Else").into_bytes()));
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));

        let response = ask_play_blackjack_get_response(false, reader, &mut writer);
        assert_eq!(response, false);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Keep playing Blackjack? (yes/no)"));
        assert!(captured_output.contains("I didn't understand that."));
        assert!(captured_output.contains("There seems to be a failure to communicate between us. Perhaps we'll play another time."));
        assert_eq!(captured_output.matches("I didn't understand that.").collect::<Vec<_>>().len(), 3);

        // Attempt

        let reader = BufReader::new(Cursor::new(String::from("asdf\njkl;\nyes").into_bytes()));
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));

        let response = ask_play_blackjack_get_response(false, reader, &mut writer);
        assert_eq!(response, true);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Keep playing Blackjack? (yes/no)"));
        assert!(captured_output.contains("I didn't understand that."));
        assert!(captured_output.contains("Alright! Let's play!"));
        assert_eq!(captured_output.matches("I didn't understand that.").collect::<Vec<_>>().len(), 2);

        // Attempt

        let reader = BufReader::new(Cursor::new(String::from("asdf\njkl;\nno").into_bytes()));
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));

        let response = ask_play_blackjack_get_response(false, reader, &mut writer);
        assert_eq!(response, false);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Keep playing Blackjack? (yes/no)"));
        assert!(captured_output.contains("I didn't understand that."));
        assert!(captured_output.contains("Okay. Maybe another time."));
        assert_eq!(captured_output.matches("I didn't understand that.").collect::<Vec<_>>().len(), 2);

        // Attempt

        let reader = BufReader::new(Cursor::new(Vec::new()));
        let mut writer = BufWriter::new(Cursor::new(vec![0; 512]));

        let response = ask_play_blackjack_get_response(false, reader, &mut writer);
        assert_eq!(response, false);
        let (recovered_writer, _buffered_data) = writer.into_parts();
        let captured_output = String::from_utf8(recovered_writer.into_inner()).unwrap_or(String::from("buffer write failed!"));
        assert_ne!(captured_output.len(), 0);
        assert!(captured_output.contains("Keep playing Blackjack? (yes/no)"));
        assert!(captured_output.contains("I didn't understand that."));
        assert!(captured_output.contains("There seems to be a failure to communicate between us. Perhaps we'll play another time."));
        assert_eq!(captured_output.matches("I didn't understand that.").collect::<Vec<_>>().len(), 3);
    }
}
