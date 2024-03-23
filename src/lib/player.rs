use rand::seq::SliceRandom;
use std::sync::Mutex;

pub mod human {
    use crate::collections::SortCardsBy;

    pub struct Player {
        name: String,
        sort_cards_by: SortCardsBy,
    }

    impl Player {
        pub fn new(name: impl Into<String>) -> Self {
            Player {
                name: name.into(),
                sort_cards_by: SortCardsBy::Rank,
            }
        }

        pub fn name(&self) -> &str {
            self.name.as_str()
        }

        pub fn toggle_precedence(&mut self) {
            self.sort_cards_by = match self.sort_cards_by {
                SortCardsBy::Rank => SortCardsBy::Suit,
                SortCardsBy::Suit => SortCardsBy::Rank,
            };
        }

        pub fn sort_cards_by(&self) -> SortCardsBy {
            self.sort_cards_by
        }
    }
}

pub mod ai {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Strategy {
        Random,
    }

    pub struct Player {
        name: String,
        strategy: Strategy,
    }

    impl Player {
        pub fn new(name: impl Into<String>) -> Self {
            Player {
                name: name.into(),
                strategy: Strategy::Random,
            }
        }

        pub fn name(&self) -> &str {
            self.name.as_str()
        }

        pub fn strategy(&self) -> Strategy {
            self.strategy
        }
    }
}

const AI_NAMES: &[&str] = &["AIshley", "FelAIcity", "AImy", "ChoBot", "Hirayama"];

pub fn new_ai_player() -> ai::Player {
    static CHOSEN_NAMES: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

    let mut rng = rand::thread_rng();
    let name = loop {
        let chosen_names = CHOSEN_NAMES.lock().unwrap();
        debug_assert_ne!(chosen_names.len(), AI_NAMES.len(), "ran out of AI names");

        let name = AI_NAMES.choose(&mut rng).unwrap();
        let mut chosen_names = chosen_names;
        if !chosen_names.contains(name) {
            chosen_names.push(name);
            break *name;
        }
    };

    ai::Player::new(name)
}

pub fn new_human_player(name: &str) -> human::Player {
    human::Player::new(name)
}
