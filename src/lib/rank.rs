use std::{fmt, str::FromStr};

// Different games have different rules for the ranks so we don't derive PartialOrd/Ord
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
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
    Ace,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank = match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        write!(f, "{rank}")
    }
}

impl FromStr for Rank {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "2" || s.eq_ignore_ascii_case("two") || s.eq_ignore_ascii_case("deuce") {
            Ok(Rank::Two)
        } else if s == "3" || s.eq_ignore_ascii_case("three") {
            Ok(Rank::Three)
        } else if s == "4" || s.eq_ignore_ascii_case("four") {
            Ok(Rank::Four)
        } else if s == "5" || s.eq_ignore_ascii_case("five") {
            Ok(Rank::Five)
        } else if s == "6" || s.eq_ignore_ascii_case("six") {
            Ok(Rank::Six)
        } else if s == "7" || s.eq_ignore_ascii_case("seven") {
            Ok(Rank::Seven)
        } else if s == "8" || s.eq_ignore_ascii_case("eight") {
            Ok(Rank::Eight)
        } else if s == "9" || s.eq_ignore_ascii_case("nine") {
            Ok(Rank::Nine)
        } else if s == "10" || s.eq_ignore_ascii_case("ten") {
            Ok(Rank::Ten)
        } else if s == "11" || s.eq_ignore_ascii_case("j") || s.eq_ignore_ascii_case("jack") {
            Ok(Rank::Jack)
        } else if s == "12" || s.eq_ignore_ascii_case("q") || s.eq_ignore_ascii_case("queen") {
            Ok(Rank::Queen)
        } else if s == "13" || s.eq_ignore_ascii_case("k") || s.eq_ignore_ascii_case("king") {
            Ok(Rank::King)
        } else if s == "1" || s.eq_ignore_ascii_case("a") || s.eq_ignore_ascii_case("ace") {
            Ok(Rank::Ace)
        } else {
            Err(anyhow::anyhow!("`{s}` is not a rank"))
        }
    }
}
