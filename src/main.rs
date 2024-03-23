mod player;

use tracing::info;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let mut state_machine = StateMachine {
        inner: Some(State::StartNewGame),
    };

    loop {
        state_machine.tick()?;
        if state_machine.is_end() {
            info!("Thank you for playing!");
            break Ok(());
        }
    }
}

struct StateMachine {
    inner: Option<State>,
}

impl StateMachine {
    fn tick(&mut self) -> anyhow::Result<()> {
        let inner = self.inner.take().unwrap();
        let next_state = inner.tick()?;
        self.inner = Some(next_state);
        Ok(())
    }

    fn is_end(&self) -> bool {
        self.inner.as_ref().map_or(false, |state| state.is_end())
    }
}

#[allow(clippy::large_enum_variant)]
enum State {
    StartNewGame,
    Play(play_game::State),
    PostGame(post_game::State),
    End,
}

impl State {
    fn tick(self) -> anyhow::Result<State> {
        match self {
            State::StartNewGame => start_new_game::tick(),
            State::Play(play_state) => play_game::run(play_state),
            State::PostGame(post_game_state) => post_game::run(post_game_state),
            State::End => Ok(State::End),
        }
    }

    fn is_end(&self) -> bool {
        matches!(self, State::End)
    }
}

mod start_new_game {
    use std::env;

    use crate::play_game;
    use card_games::player::{new_ai_player, new_human_player};

    pub fn tick() -> anyhow::Result<super::State> {
        println!("Starting a new four-player game");
        let player_name = env::var("DAI_DI_PLAYER_NAME").unwrap_or_else(|_| "Player".to_string());
        let play_state = play_game::State {
            game: card_games::cho_dai_di::new_4p_game(),
            players: vec![
                Box::new(new_human_player(&player_name)),
                Box::new(new_ai_player()),
                Box::new(new_ai_player()),
                Box::new(new_ai_player()),
            ],
        };
        let next_state = super::State::Play(play_state);
        println!("Good luck Player! Enter \"help\" if you need some guidance.");
        println!("The player with the 3♦ will go first.");

        Ok(next_state)
    }
}

mod play_game {
    use crate::player::{Player, TurnAction};
    use anyhow::Context;
    use card_games::{cho_dai_di::ChoDaiDi, collections::Cards};

    pub struct State {
        pub game: ChoDaiDi,
        pub players: Vec<Box<dyn Player>>,
    }

    impl State {
        fn play_cards(&mut self, cards: Cards<ChoDaiDi>) -> anyhow::Result<()> {
            self.game.play_cards(cards)
        }

        fn pass(&mut self) {
            self.game.pass();
        }

        pub fn get_current_player_name(&self) -> Option<&str> {
            let current_player = self.game.whose_turn();
            self.players.get(current_player).map(|it| it.name())
        }

        fn take_turn(&mut self) -> anyhow::Result<TurnAction> {
            let current_player = self.game.whose_turn();
            let player = self
                .players
                .get_mut(current_player)
                .context("taking turn")?;
            let hand = self.game.get_current_players_hand();
            player.take_turn(&self.game, hand)
        }

        fn longest_name_length(&self) -> usize {
            self.players
                .iter()
                .map(|it| it.name().len())
                .max()
                .unwrap_or(0)
        }
    }

    pub fn run(mut state: State) -> anyhow::Result<super::State> {
        // Pad things out
        println!();

        if state.game.is_game_ended() {
            let post_game_state = super::post_game::State {
                longest_name_length: state.longest_name_length(),
                hand_sizes: state.game.hands().iter().map(|it| it.len()).collect(),
                players: state.players,
            };
            return Ok(super::State::PostGame(post_game_state));
        }

        let mut highest_card_was_played = false;
        let current_player_name = state.get_current_player_name().unwrap().to_owned();
        loop {
            let turn_action = state
                .take_turn()
                .with_context(|| format!("{}'s turn", current_player_name))?;
            match turn_action {
                TurnAction::PlayCards(cards) => {
                    // Verify that the player has the cards they want to play.
                    if !state.game.current_players_hand_includes(&cards) {
                        println!("{current_player_name} doesn't have {cards} in their hand so the play is invalid",);
                        continue;
                    }

                    let current_player_name = current_player_name.to_owned();
                    match state.play_cards(cards.clone()) {
                        Ok(()) => {
                            if let Some(card) = state.game.highest_card_still_in_play() {
                                if cards.contains(card) {
                                    highest_card_was_played = true;
                                }
                            }
                            state.game.reset_pass_counter();
                        }
                        Err(e) => {
                            println!("can't play '{cards}': {e}");
                            continue;
                        }
                    }

                    if highest_card_was_played {
                        println!("{current_player_name} plays {cards}, ending the round.");
                    } else {
                        println!("{current_player_name} plays {cards}");
                    }

                    break;
                }
                TurnAction::Pass => {
                    println!("{} will pass", current_player_name);
                    state.pass();

                    // If all players pass, then a new round is started
                    if state.game.pass_counter() == state.game.number_of_players() - 1 {
                        state.game.reset_pass_counter();
                        state.game.unset_last_play();
                    }

                    break;
                }
            }
        }

        // If the highest card was played, then a new round is started
        if highest_card_was_played {
            state.game.reset_pass_counter();
            state.game.unset_last_play();
        } else {
            state.game.increment_turn_counter();
        }

        Ok(super::State::Play(state))
    }
}

mod post_game {
    use crate::player::Player;
    use card_games::cho_dai_di::hand_size_to_score;

    pub struct State {
        pub hand_sizes: Vec<usize>,
        pub players: Vec<Box<dyn Player>>,
        pub longest_name_length: usize,
    }

    pub fn run(state: State) -> anyhow::Result<super::State> {
        let hand_sizes = state.hand_sizes;
        let players = state.players;
        let lnl = state.longest_name_length;

        let player_scores: Vec<_> = hand_sizes.into_iter().map(hand_size_to_score).collect();
        // The winning player collects the sum of all other players' scores
        let sum = player_scores.iter().sum::<isize>().abs();
        let player_scores: Vec<_> = player_scores.into_iter().zip(players).collect();

        println!("Game over. Let's see the scores:");
        println!();

        for (score, player) in player_scores.iter() {
            let score = if *score == 0 { sum } else { *score };
            println!("\t{:n$}:\t{score:+}", player.name(), n = lnl);
        }

        println!();
        println!("Congratulations {}!", player_scores[0].1.name());

        Ok(super::State::End)
    }
}
