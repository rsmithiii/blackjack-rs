use std::fmt;
use std::convert;
use std::error;

#[derive(Debug)]
pub struct InvalidIntToRank;

impl fmt::Display for InvalidIntToRank
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "invalid integer; does not map to rank")
    }
}

impl error::Error for InvalidIntToRank {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Rank
{
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl fmt::Display for Rank
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match &self
        {
            Rank::Ace => write!(f, "Ace"),
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "Jack"),
            Rank::Queen => write!(f, "Queen"),
            Rank::King => write!(f, "King"),
        }
    }
}

impl convert::TryFrom<u32> for Rank
{
    type Error = InvalidIntToRank;

    fn try_from(value: u32) -> Result<Self, Self::Error>
    {
        let number_to_try = Option::Some(value);
        match number_to_try
        {
            Some(x) if x == Rank::Ace as u32 => Ok(Rank::Ace),
            Some(x) if x == Rank::Two as u32 => Ok(Rank::Two),
            Some(x) if x == Rank::Three as u32 => Ok(Rank::Three),
            Some(x) if x == Rank::Four as u32 => Ok(Rank::Four),
            Some(x) if x == Rank::Five as u32 => Ok(Rank::Five),
            Some(x) if x == Rank::Six as u32 => Ok(Rank::Six),
            Some(x) if x == Rank::Seven as u32 => Ok(Rank::Seven),
            Some(x) if x == Rank::Eight as u32 => Ok(Rank::Eight),
            Some(x) if x == Rank::Nine as u32 => Ok(Rank::Nine),
            Some(x) if x == Rank::Ten as u32 => Ok(Rank::Ten),
            Some(x) if x == Rank::Jack as u32 => Ok(Rank::Jack),
            Some(x) if x == Rank::Queen as u32 => Ok(Rank::Queen),
            Some(x) if x == Rank::King as u32 => Ok(Rank::King),
            _ => Err(InvalidIntToRank)
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_format_rank()
    {
        let formatted_string = format!("{}", Rank::Ace);
        assert_eq!(formatted_string, "Ace");
        let formatted_string = format!("{}", Rank::Two);
        assert_eq!(formatted_string, "2");
        let formatted_string = format!("{}", Rank::Three);
        assert_eq!(formatted_string, "3");
        let formatted_string = format!("{}", Rank::Four);
        assert_eq!(formatted_string, "4");
        let formatted_string = format!("{}", Rank::Five);
        assert_eq!(formatted_string, "5");
        let formatted_string = format!("{}", Rank::Six);
        assert_eq!(formatted_string, "6");
        let formatted_string = format!("{}", Rank::Seven);
        assert_eq!(formatted_string, "7");
        let formatted_string = format!("{}", Rank::Eight);
        assert_eq!(formatted_string, "8");
        let formatted_string = format!("{}", Rank::Nine);
        assert_eq!(formatted_string, "9");
        let formatted_string = format!("{}", Rank::Ten);
        assert_eq!(formatted_string, "10");
        let formatted_string = format!("{}", Rank::Jack);
        assert_eq!(formatted_string, "Jack");
        let formatted_string = format!("{}", Rank::Queen);
        assert_eq!(formatted_string, "Queen");
        let formatted_string = format!("{}", Rank::King);
        assert_eq!(formatted_string, "King");
    }

    #[test]
    fn test_try_from_u32()
    {
        for n in 0..20
        {
            let try_rank_from_u32 = Rank::try_from(n);
            let expected_okay = match Some(n)
            {
                Some(x) if x >= Rank::Ace as u32 && x <= Rank::King as u32 => true,
                _ => false,
            };
            assert_eq!(try_rank_from_u32.is_ok(), expected_okay);
        }
    }
}
