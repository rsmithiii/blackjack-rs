use std::fmt;
use std::convert;
use std::error;

#[derive(Debug)]
pub struct InvalidIntToSuit;

impl fmt::Display for InvalidIntToSuit
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "invalid integer; does not map to suit")
    }
}

impl error::Error for InvalidIntToSuit {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Suit
{
    Diamonds,
    Clubs,
    Hearts,
    Spades
}

impl fmt::Display for Suit
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match &self
        {
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Clubs => write!(f, "Clubs"),
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Spades => write!(f, "Spades"),
        }
    }
}

impl convert::TryFrom<u32> for Suit
{
    type Error = InvalidIntToSuit;

    fn try_from(value: u32) -> Result<Self, Self::Error>
    {
        let number_to_try = Option::Some(value);
        match number_to_try
        {
            // {Suit::Diamonds as u32} => Ok(Suit::Diamonds),
            // Suit::Clubs as u32 => Ok(Suit::Clubs),
            // Suit::Hearts as u32 => Ok(Suit::Hearts),
            // Suit::Spades as u32 => Ok(Suit::Spades),
            Some(x) if x == Suit::Diamonds as u32 => Ok(Suit::Diamonds),
            Some(x) if x == Suit::Clubs as u32 => Ok(Suit::Clubs),
            Some(x) if x == Suit::Hearts as u32 => Ok(Suit::Hearts),
            Some(x) if x == Suit::Spades as u32 => Ok(Suit::Spades),
            _ => Err(InvalidIntToSuit)
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_format_suit()
    {
        let formatted_string = format!("{}", Suit::Diamonds);
        assert_eq!(formatted_string, "Diamonds");
        let formatted_string = format!("{}", Suit::Clubs);
        assert_eq!(formatted_string, "Clubs");
        let formatted_string = format!("{}", Suit::Hearts);
        assert_eq!(formatted_string, "Hearts");
        let formatted_string = format!("{}", Suit::Spades);
        assert_eq!(formatted_string, "Spades");
    }

    #[test]
    fn test_try_from_u32()
    {
        for n in 0..10
        {
            let try_suit_from_u32 = Suit::try_from(n);
            let expected_okay = match Some(n)
            {
                Some(x) if x >= Suit::Diamonds as u32 && x <= Suit::Spades as u32 => true,
                _ => false,
            };
            assert_eq!(try_suit_from_u32.is_ok(), expected_okay);
        }
    }
}
