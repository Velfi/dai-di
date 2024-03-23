mod constants;

use crate::{rank::Rank, suit::Suit};
pub use constants::STANDARD_DECK;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() < 2 {
            return Err(anyhow::anyhow!("`{s}` is not a valid card"));
        }

        let (rank, suit) = s.split_at(s.len() - 1);
        let rank = rank.parse()?;
        let suit = suit.parse()?;

        Ok(Card { rank, suit })
    }
}
