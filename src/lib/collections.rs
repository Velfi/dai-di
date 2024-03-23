use crate::card::Card;
use anyhow::bail;
use itertools::Itertools;
use std::{cmp::Ordering, fmt, marker::PhantomData, str::FromStr};

/// A collection of cards.
pub struct Cards<G> {
    inner: Vec<Card>,
    _game: PhantomData<G>,
}

impl<G> fmt::Debug for Cards<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl<G> Clone for Cards<G> {
    fn clone(&self) -> Self {
        Cards {
            inner: self.inner.clone(),
            _game: PhantomData,
        }
    }
}

impl<G> PartialEq for Cards<G> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<G> Eq for Cards<G> {}

impl<G> Cards<G> {
    pub fn into_inner(self) -> Vec<Card> {
        self.inner
    }

    pub fn iter(&self) -> impl Iterator<Item = &Card> {
        self.inner.iter()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn contains(&self, card: &Card) -> bool {
        self.inner.contains(card)
    }

    pub fn first(&self) -> Option<&Card> {
        self.inner.first()
    }

    pub fn sort_by(&mut self, f: impl FnMut(&Card, &Card) -> Ordering) {
        self.inner.sort_by(f);
    }

    pub fn are_all_of_same_rank(&self) -> anyhow::Result<bool> {
        match self.inner.first() {
            Some(first) => {
                let rank = first.rank();
                Ok(self.inner.iter().all(|it| it.rank() == rank))
            }
            None => bail!("this hand is empty"),
        }
    }

    pub fn retain(&mut self, f: impl FnMut(&Card) -> bool) {
        self.inner.retain(f);
    }

    pub fn is_a_pair(&self) -> bool {
        match self.inner.as_slice() {
            [c1, c2] => c1.rank() == c2.rank(),
            _ => false,
        }
    }

    pub fn is_a_triplet(&self) -> bool {
        match self.inner.as_slice() {
            [c1, c2, c3] => c1.rank() == c2.rank() && c2.rank() == c3.rank(),
            _ => false,
        }
    }

    /// If this hand is a four of a kind plus one, return true.
    ///
    /// A four of a kind plus one is a hand with four cards of one rank and one card of another rank.
    pub fn is_four_of_a_kind_plus_one(&self) -> bool {
        match self.inner.as_slice() {
            [c1, c2, c3, c4, c5] => is_four_of_a_kind_plus_one([c1, c2, c3, c4, c5]),
            _ => false,
        }
    }

    /// If this hand is a full house, return true.
    ///
    /// A full house is a hand with three cards of one rank and two cards of another rank.
    pub fn is_a_full_house(&self) -> bool {
        match self.inner.as_slice() {
            [c1, c2, c3, c4, c5] => is_a_full_house([c1, c2, c3, c4, c5]),
            _ => false,
        }
    }

    /// If this hand is a flush, return true.
    ///
    /// A flush is a hand with five cards of the same suit.
    pub fn is_a_flush(&self) -> bool {
        match self.inner.as_slice() {
            [c1, c2, c3, c4, c5] => is_flush([c1, c2, c3, c4, c5]),
            _ => false,
        }
    }

    pub(crate) fn permutations(&self, k: usize) -> impl Iterator<Item = Cards<G>> + '_ {
        self.inner.iter().permutations(k).map(|it| Cards {
            inner: it.into_iter().cloned().collect(),
            _game: PhantomData,
        })
    }
}

pub fn is_a_pair(cards: [&Card; 2]) -> bool {
    cards[0].rank() == cards[1].rank()
}

pub fn is_a_triplet(cards: [&Card; 3]) -> bool {
    cards[0].rank() == cards[1].rank() && cards[1].rank() == cards[2].rank()
}

pub fn is_four_of_a_kind_plus_one(cards: [&Card; 5]) -> bool {
    let mut a = (None, 0);
    let mut b = (None, 0);

    for card in cards.iter() {
        if a.0.is_none() || a.0 == Some(card.rank()) {
            a = (Some(card.rank()), a.1 + 1);
        } else if b.0.is_none() || b.0 == Some(card.rank()) {
            b = (Some(card.rank()), b.1 + 1);
        } else {
            return false;
        }
    }

    (a.1 == 4 && b.1 == 1) || (a.1 == 1 && b.1 == 4)
}

pub fn is_a_full_house(cards: [&Card; 5]) -> bool {
    let mut a = (None, 0);
    let mut b = (None, 0);

    for card in cards.iter() {
        if a.0.is_none() || a.0 == Some(card.rank()) {
            a = (Some(card.rank()), a.1 + 1);
        } else if b.0.is_none() || b.0 == Some(card.rank()) {
            b = (Some(card.rank()), b.1 + 1);
        } else {
            return false;
        }
    }

    (a.1 == 3 && b.1 == 2) || (a.1 == 2 && b.1 == 3)
}

pub fn is_flush(cards: [&Card; 5]) -> bool {
    let suit = cards[0].suit();
    cards.iter().all(|it| it.suit() == suit)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortCardsBy {
    Rank,
    Suit,
}

impl fmt::Display for SortCardsBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortCardsBy::Rank => write!(f, "rank"),
            SortCardsBy::Suit => write!(f, "suit"),
        }
    }
}

impl<G> IntoIterator for Cards<G> {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<G> fmt::Display for Cards<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, card) in self.inner.iter().enumerate() {
            if i == self.inner.len() - 1 {
                write!(f, "{card}")?;
            } else {
                write!(f, "{card}, ")?;
            }
        }
        Ok(())
    }
}

impl<G> From<Vec<Card>> for Cards<G> {
    fn from(inner: Vec<Card>) -> Self {
        Cards {
            inner,
            _game: PhantomData,
        }
    }
}

impl<G> TryFrom<Vec<&str>> for Cards<G> {
    type Error = anyhow::Error;

    fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
        let inner = value
            .iter()
            .map(|it| it.parse::<Card>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Cards {
            inner,
            _game: PhantomData,
        })
    }
}

impl<G> From<Card> for Cards<G> {
    fn from(card: Card) -> Self {
        Cards {
            inner: vec![card],
            _game: PhantomData,
        }
    }
}

impl<G> FromStr for Cards<G> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .split_whitespace()
            .map(|it| it.trim_end_matches(','))
            .map(str::parse::<Card>)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Cards {
            inner: cards,
            _game: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_a_pair() {
        // Valid pairs
        assert!(Cards::<()>::try_from(vec!["3S", "3D"]).unwrap().is_a_pair());

        // Invalid pairs
        assert!(!Cards::<()>::try_from(vec!["3S", "4D"]).unwrap().is_a_pair());
        assert!(!Cards::<()>::try_from(vec!["3S", "3D", "4S"])
            .unwrap()
            .is_a_pair());
    }

    #[test]
    fn test_is_a_triplet() {
        // Valid triplets
        assert!(Cards::<()>::try_from(vec!["3S", "3D", "3H"])
            .unwrap()
            .is_a_triplet());

        // Invalid triplets
        assert!(!Cards::<()>::try_from(vec!["3S", "3D"])
            .unwrap()
            .is_a_triplet());
        assert!(!Cards::<()>::try_from(vec!["3S", "3D", "4S"])
            .unwrap()
            .is_a_triplet());
        assert!(!Cards::<()>::try_from(vec!["3S", "3D", "3H", "4S"])
            .unwrap()
            .is_a_triplet());
    }

    #[test]
    fn test_is_four_of_a_kind_plus_one() {
        // Valid four of a kind plus one
        assert!(Cards::<()>::try_from(vec!["3S", "3D", "3H", "3C", "4S"])
            .unwrap()
            .is_four_of_a_kind_plus_one());
        assert!(Cards::<()>::try_from(vec!["3S", "3D", "3H", "3C", "4S"])
            .unwrap()
            .is_four_of_a_kind_plus_one());

        // Invalid four of a kind plus one
        assert!(!Cards::<()>::try_from(vec!["3S", "3D", "3H", "4S", "4D"])
            .unwrap()
            .is_four_of_a_kind_plus_one());
        assert!(!Cards::<()>::try_from(vec!["3S", "3D", "3H", "3C"])
            .unwrap()
            .is_four_of_a_kind_plus_one());
    }

    #[test]
    fn is_valid_full_house() {
        // Valid full house
        assert!(Cards::<()>::try_from(vec!["6S", "6D", "6H", "JS", "JD"])
            .unwrap()
            .is_a_full_house());
        assert!(Cards::<()>::try_from(vec!["9S", "9D", "9S", "3D", "3H"])
            .unwrap()
            .is_a_full_house());

        // Invalid full house
        assert!(!Cards::<()>::try_from(vec!["3S", "3D", "3H", "3C", "4S"])
            .unwrap()
            .is_a_full_house());
        assert!(
            !Cards::<()>::try_from(vec!["3S", "3D", "3H", "4S", "4D", "4H"])
                .unwrap()
                .is_a_full_house()
        );
    }

    #[test]
    fn is_valid_flush() {
        // Valid flush
        assert!(Cards::<()>::try_from(vec!["3S", "4S", "5S", "6S", "7S"])
            .unwrap()
            .is_a_flush());
        assert!(Cards::<()>::try_from(vec!["3D", "4D", "5D", "6D", "7D"])
            .unwrap()
            .is_a_flush());

        // Invalid flush
        assert!(!Cards::<()>::try_from(vec!["3S", "4S", "5S", "6S", "7D"])
            .unwrap()
            .is_a_flush());
        assert!(!Cards::<()>::try_from(vec!["3S", "4S", "5S", "6S"])
            .unwrap()
            .is_a_flush());
    }
}
