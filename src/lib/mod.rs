#![allow(clippy::new_without_default)]

pub mod card;
pub mod cho_dai_di;
pub mod collections;
pub mod player;
pub mod rank;
pub mod suit;

use card::{Card, STANDARD_DECK};
use once_cell::sync::Lazy;
use rand::{prelude::*, rngs::SmallRng, SeedableRng};
use std::{marker::PhantomData, sync::Mutex};

static RNG: Lazy<Mutex<SmallRng>> = Lazy::new(|| Mutex::new(SmallRng::from_entropy()));

pub fn shuffled_deck() -> Vec<Card> {
    let mut deck = STANDARD_DECK;
    let mut rng = RNG.lock().unwrap();
    deck.shuffle(&mut *rng);
    deck.into()
}

pub struct Deck<G> {
    cards: Vec<Card>,
    _game: PhantomData<G>,
}

impl<G> Deck<G> {
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::shuffled_deck;
    use crate::{
        cho_dai_di::{ChoDaiDi, FOUR_PLAYERS},
        Deck,
    };

    #[test]
    fn test_shuffled_deck() {
        let deck = shuffled_deck();
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn test_cho_dai_di_draw_starting_hands() {
        let mut deck = Deck::<ChoDaiDi<FOUR_PLAYERS>>::new();
        let hands = deck.draw_starting_hands();

        for hand in hands {
            assert_eq!(hand.len(), 13);
        }
    }
}
