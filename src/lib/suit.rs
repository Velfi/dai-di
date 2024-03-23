use std::{fmt, str::FromStr};

// Different games have different rules for the suits so we don't derive PartialOrd/Ord
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit = match self {
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        };
        write!(f, "{suit}")
    }
}

impl FromStr for Suit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("d") || s.eq_ignore_ascii_case("diamonds") || s == "♦" || s == "♢"
        {
            Ok(Suit::Diamonds)
        } else if s.eq_ignore_ascii_case("c")
            || s.eq_ignore_ascii_case("clubs")
            || s == "♣"
            || s == "♧"
        {
            Ok(Suit::Clubs)
        } else if s.eq_ignore_ascii_case("h")
            || s.eq_ignore_ascii_case("hearts")
            || s == "♥"
            || s == "♡"
        {
            Ok(Suit::Hearts)
        } else if s.eq_ignore_ascii_case("s")
            || s.eq_ignore_ascii_case("spades")
            || s == "♠"
            || s == "♤"
        {
            Ok(Suit::Spades)
        } else {
            Err(anyhow::anyhow!("`{s}` is not a suit"))
        }
    }
}
