use anyhow::Context;
use card_games::collections::SortCardsBy;
use card_games::{cho_dai_di::ChoDaiDi, collections::Cards, player::ai::Strategy};
use core::fmt;
use rand::seq::SliceRandom;
use rand::{rngs::SmallRng, SeedableRng};
use std::fmt::Write;

pub trait Player {
    fn name(&self) -> &str;

    fn take_turn(&mut self, game: &ChoDaiDi, hand: Cards<ChoDaiDi>) -> anyhow::Result<TurnAction>;
}

impl fmt::Display for dyn Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub enum TurnAction {
    PlayCards(Cards<ChoDaiDi>),
    Pass,
}

impl Player for card_games::player::ai::Player {
    fn name(&self) -> &str {
        self.name()
    }

    /// An AI player's turn-taking logic.
    ///
    /// This is where the AI decides what cards to play.
    fn take_turn(
        &mut self,
        game: &ChoDaiDi,
        mut hand: Cards<ChoDaiDi>,
    ) -> anyhow::Result<TurnAction> {
        let mut small_rng = SmallRng::from_entropy();

        hand.sort_by_rank();

        match self.strategy() {
            Strategy::Random => {
                let possible_plays = game.possible_plays(&hand);
                if let Some(play) = possible_plays.choose(&mut small_rng) {
                    tracing::warn!(
                        "{} possible play(s) found for {}",
                        possible_plays.len(),
                        self.name()
                    );

                    return Ok(TurnAction::PlayCards(play.clone()));
                } else {
                    tracing::warn!("no possible plays found for {}", self.name());
                }
            }
        }

        Ok(TurnAction::Pass)
    }
}

impl Player for card_games::player::human::Player {
    fn name(&self) -> &str {
        self.name()
    }

    fn take_turn(
        &mut self,
        game: &ChoDaiDi,
        mut hand: Cards<ChoDaiDi>,
    ) -> anyhow::Result<TurnAction> {
        // Enough capacity for a full hand of cards, plus some extra for the commas and spaces.
        let mut buf = String::with_capacity(16);
        let cards = loop {
            buf.clear();
            match self.sort_cards_by() {
                SortCardsBy::Rank => hand.sort_by_rank(),
                SortCardsBy::Suit => hand.sort_by_suit(),
            }
            println!();
            if let Some(last_play) = game.last_play() {
                println!("Last play: {last_play}");
            }
            let hand_str = hand.iter().fold(String::new(), |mut s, card| {
                write!(s, "{card}, ").expect("write to string will never fail");
                s
            });
            println!("{}'s hand: {hand_str}", self.name());
            print!("Your play: ");
            // We flush to guarantee that the prompt is displayed before reading input.
            std::io::Write::flush(&mut std::io::stdout())
                .context("flushing 'Your play: ' prompt")?;
            std::io::stdin().read_line(&mut buf)?;
            let input = buf.trim();
            match input {
                "p" | "pass" => return Ok(TurnAction::Pass),
                "q" | "quit" => {
                    println!("Quitting immediately. Thanks for playing.");
                    std::process::exit(0);
                }
                "" | "help" => {
                    println!("Enter the space-separated list of the cards you want to play");
                    println!("For example: '2c 3h 4d 5s 6s' or '2C 2D 2H' or 'jc'");
                    println!("You may pass your turn: enter 'p' or 'pass'");
                    println!("You may quit the game: enter 'q' or 'quit'");
                    println!(
                        "You may toggle between sorting by rank and sorting by suit: enter 'sort'"
                    );
                    continue;
                }
                "sort" => {
                    self.toggle_precedence();
                    println!("hand rearranged by {}", self.sort_cards_by());
                    continue;
                }
                input => match input.parse::<Cards<ChoDaiDi>>() {
                    Ok(cards) => break cards,
                    Err(e) => {
                        println!("invalid input: {e}");
                        continue;
                    }
                },
            }
        };

        Ok(TurnAction::PlayCards(cards))
    }
}
