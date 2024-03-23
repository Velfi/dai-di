use crate::{card::Card, collections::Cards, rank::Rank, shuffled_deck, suit::Suit, Deck};
use anyhow::bail;
use std::{cmp::Ordering, marker::PhantomData};

pub const FOUR_PLAYERS: usize = 4;

pub fn new_4p_game() -> ChoDaiDi<FOUR_PLAYERS> {
    ChoDaiDi::new_game()
}

/// Given a hand size, calculate a score
///
/// The score is calculated as follows:
/// - 10 or less cards in hand, -1 point per card
/// - 11-12 cards in hand, -2 points per card
/// - 13 cards in hand, -3 points per card
pub fn hand_size_to_score(hand_size: usize) -> isize {
    let score = match hand_size {
        0..=10 => hand_size as isize,
        11..=12 => (hand_size as isize) * 2,
        13 => (hand_size as isize) * 3,
        _ => unreachable!("valid hand sizes for a four-player game are between 0 and 13 inclusive"),
    };

    -score
}

pub struct ChoDaiDi<const PLAYERS: usize = FOUR_PLAYERS> {
    card_pile: Vec<Card>,
    last_play: Option<Cards<Self>>,
    deck: Deck<Self>,
    hands: [Cards<Self>; PLAYERS],
    scores: [usize; PLAYERS],
    turn: usize,
    pass_counter: usize,
}

impl<const PLAYERS: usize> ChoDaiDi<PLAYERS> {
    /// Create a new game of Cho Dai Di
    pub fn new_game() -> Self {
        let mut deck = Deck::new();
        let hands: [Cards<Self>; PLAYERS] = deck.draw_starting_hands();
        let hand_with_three_of_diamonds = hands
            .iter()
            .enumerate()
            .find(|(_, hand)| hand.contains(&Card::THREE_OF_DIAMONDS))
            .map(|(i, _)| i)
            .unwrap();

        Self {
            card_pile: Vec::new(),
            last_play: None,
            deck,
            hands,
            scores: [0; PLAYERS],
            // The player with the three of diamonds goes first
            turn: hand_with_three_of_diamonds,
            pass_counter: 0,
        }
    }

    /// Reset the pass counter. his should happen whenever a new round starts.
    pub fn reset_pass_counter(&mut self) {
        self.pass_counter = 0;
    }

    /// Unset the last play. This should happen whenever a new round starts.
    pub fn unset_last_play(&mut self) {
        self.last_play = None;
    }

    /// Increment the turn counter
    pub fn increment_turn_counter(&mut self) {
        self.turn += 1;
    }

    /// Get the current player's hand
    pub fn get_current_players_hand(&self) -> Cards<Self> {
        self.hands
            .get(self.whose_turn())
            .cloned()
            .expect("all games will a non-zero number of players")
    }

    /// Get the player's hands
    pub fn hands(&self) -> &[Cards<Self>; PLAYERS] {
        &self.hands
    }

    pub fn deck(&self) -> &Deck<Self> {
        &self.deck
    }

    /// Get the last play
    pub fn last_play(&self) -> Option<Cards<Self>> {
        self.last_play.clone()
    }

    /// Get the number of passes.
    ///
    /// If all players pass, then the round is over.
    pub fn pass_counter(&self) -> usize {
        self.pass_counter
    }

    /// Get the player's scores
    pub fn scores(&self) -> [usize; PLAYERS] {
        self.scores
    }

    /// Get the number of players
    pub fn number_of_players(&self) -> usize {
        PLAYERS
    }

    /// Get the current player's turn
    pub fn whose_turn(&self) -> usize {
        self.turn % PLAYERS
    }

    /// A round has ended when all players have passed. Returns true if that's
    /// the case.
    pub fn is_round_ended(&self) -> bool {
        self.pass_counter >= PLAYERS
    }

    /// A game has ended when a player has no cards left. Returns true if that's
    /// the case.
    pub fn is_game_ended(&self) -> bool {
        // A game is over when a player has no cards left
        self.hands.iter().any(|hand| hand.is_empty())
    }

    /// Play a card or cards
    ///
    /// If the play is valid, the cards are removed from the player's hand and added to the card pile.
    /// If the play is invalid, the cards are returned to the player's hand and an error is returned.
    pub fn play_cards(&mut self, cards: Cards<Self>) -> Result<(), anyhow::Error> {
        self.is_valid_play(&cards)?;
        // Remove the played cards from the player's hand
        self.hands[self.whose_turn()].retain(|c| !cards.contains(c));
        // Update the last play
        self.last_play = Some(cards.clone());
        // Add the played cards to the card pile
        self.card_pile.extend(cards);

        Ok(())
    }

    /// Pass the turn
    pub fn pass(&mut self) {
        self.pass_counter += 1;
    }

    /// Check if a play is valid.
    pub fn is_valid_play(&self, cards: &Cards<Self>) -> anyhow::Result<()> {
        match self.last_play() {
            Some(last_play) => last_play.may_be_followed_by(cards),
            None => {
                cards.is_valid_hand()?;
                if self.card_pile.is_empty() {
                    // If card pile is empty, then we must be starting a new game.
                    // In this case, the first played hand must contain the three of diamonds.
                    if cards.contains(&Card::THREE_OF_DIAMONDS) {
                        Ok(())
                    } else {
                        Err(anyhow::anyhow!(
                            "the first play must contain the three of diamonds"
                        ))
                    }
                } else {
                    // If the card pile is not empty, then we must be starting a new round.
                    // In this case, any valid hand is acceptable.
                    Ok(())
                }
            }
        }
    }

    pub fn highest_card_still_in_play(&self) -> Option<&Card> {
        self.hands
            .iter()
            .flat_map(|hand| hand.iter())
            .max_by(|a, b| Cards::<ChoDaiDi>::cmp_card(a, b))
    }

    /// Calculate the possible plays from a given hand.
    ///
    /// If no plays are possible, the player must pass.
    pub fn possible_plays(&self, hand: &Cards<Self>) -> Vec<Cards<Self>> {
        match self.last_play() {
            Some(last_play) => {
                let mut possible_plays: Vec<_> = match last_play.len() {
                    1 => hand.iter().map(|it| Cards::from(*it)).collect(),
                    n @ 2 | n @ 3 | n @ 5 => hand.permutations(n).collect(),
                    _ => unreachable!("all possible cases have been handled"),
                };

                // Filter out invalid hands
                possible_plays.retain(|play| last_play.may_be_followed_by(play).is_ok());
                possible_plays
            }
            None => {
                // If the card pile is empty, then we must be starting a new game.
                // The first play of a game must contain the three of diamonds.
                let mut possible_plays: Vec<_> = if self.card_pile.is_empty() {
                    hand.contains(&Card::THREE_OF_DIAMONDS)
                        .then(|| {
                            std::iter::once(Cards::from(Card::THREE_OF_DIAMONDS))
                                .chain(hand.permutations(2))
                                .chain(hand.permutations(3))
                                .chain(hand.permutations(5))
                                .filter(|it| it.contains(&Card::THREE_OF_DIAMONDS))
                                .collect()
                        })
                        .unwrap_or_default()
                // If the card pile is not empty, then we must be starting a new round.
                // Any valid hand is acceptable.
                } else {
                    hand.iter()
                        .map(|it| Cards::from(*it))
                        .chain(hand.permutations(2))
                        .chain(hand.permutations(3))
                        .chain(hand.permutations(5))
                        .collect()
                };

                // Filter out invalid hands
                possible_plays.retain(|play| play.is_valid_hand().is_ok());
                possible_plays
            }
        }
    }

    pub fn current_players_hand_includes(&self, cards: &Cards<ChoDaiDi<4>>) -> bool {
        let current_players_hand = self.get_current_players_hand();
        cards.iter().all(|card| current_players_hand.contains(card))
    }
}

impl<const PLAYERS: usize> Deck<ChoDaiDi<PLAYERS>> {
    pub fn new() -> Self {
        Self {
            cards: shuffled_deck(),
            _game: PhantomData,
        }
    }

    pub fn draw_starting_hands(&mut self) -> [Cards<ChoDaiDi<PLAYERS>>; PLAYERS] {
        const INIT: Vec<Card> = Vec::new();
        let mut hands = [INIT; PLAYERS];
        // Dole out cards to each player in turn until the deck has less than PLAYERS cards
        while self.len() >= PLAYERS {
            for item in hands.iter_mut() {
                item.push(self.cards.pop().unwrap());
            }
        }

        hands.map(Cards::from)
    }
}

impl<const PLAYERS: usize> Cards<ChoDaiDi<PLAYERS>> {
    /// The precedence of suits in Cho Dai Di, from lowest to highest.
    pub const SUIT_PRECEDENCE: &'static [Suit] =
        &[Suit::Diamonds, Suit::Clubs, Suit::Hearts, Suit::Spades];

    /// The precedence of ranks in Cho Dai Di, from lowest to highest.
    ///
    /// In Cho Dai Di, the lowest rank is three and the highest rank is two.
    pub const RANK_PRECEDENCE: &'static [Rank] = &[
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
        Rank::Two,
    ];

    /// Compare two suits by their precedence.
    ///
    /// In Cho Dai Di, the precedence of suits is as follows:
    /// - Diamonds, Clubs, Hearts, Spades
    pub(crate) fn cmp_suit(a: &Suit, b: &Suit) -> Ordering {
        Self::SUIT_PRECEDENCE
            .iter()
            .position(|&s| s == *a)
            .cmp(&Self::SUIT_PRECEDENCE.iter().position(|&s| s == *b))
    }

    /// Compare two ranks by their precedence.
    ///
    /// In Cho Dai Di, the precedence of ranks is as follows:
    /// - Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace, Two
    pub(crate) fn cmp_rank(a: &Rank, b: &Rank) -> Ordering {
        Self::RANK_PRECEDENCE
            .iter()
            .position(|&r| r == *a)
            .cmp(&Self::RANK_PRECEDENCE.iter().position(|&r| r == *b))
    }

    /// Compare two cards by their precedence.
    ///
    /// In Cho Dai Di, the precedence of cards is as follows:
    /// - Three of Diamonds, Three of Clubs, Three of Hearts, Three of Spades, Four of Diamonds, Four of Clubs, etc.
    pub(crate) fn cmp_card(a: &Card, b: &Card) -> Ordering {
        if a.rank() == b.rank() {
            Self::cmp_suit(&a.suit(), &b.suit())
        } else {
            Self::cmp_rank(&a.rank(), &b.rank())
        }
    }

    /// Sort this hand by rank.
    ///
    /// In Cho Dai Di, the precedence of ranks is as follows:
    /// - Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace, Two
    pub fn sort_by_rank(&mut self) {
        self.sort_by(|a, b| {
            if a.rank() == b.rank() {
                Self::cmp_suit(&a.suit(), &b.suit())
            } else {
                Self::cmp_rank(&a.rank(), &b.rank())
            }
        });
    }

    /// Sort this hand by suit.
    ///
    /// In Cho Dai Di, the precedence of suits is as follows:
    /// - Diamonds, Clubs, Hearts, Spades
    pub fn sort_by_suit(&mut self) {
        self.sort_by(|a, b| {
            if a.suit() == b.suit() {
                Self::cmp_rank(&a.rank(), &b.rank())
            } else {
                Self::cmp_suit(&a.suit(), &b.suit())
            }
        });
    }

    /// Sort this hand by precedence.
    ///
    /// In Cho Dai Di, the precedence of cards is as follows:
    /// - Three of Diamonds, Three of Clubs, Three of Hearts, Three of Spades, Four of Diamonds, Four of Clubs, etc.
    pub fn sort_by_precedence(&mut self) {
        self.sort_by(Self::cmp_card);
    }

    /// Get the lowest card in the hand.
    ///
    /// In Cho Dai Di, the lowest card is the three of diamonds.
    pub fn lowest_card(&self) -> Option<&Card> {
        self.iter().min_by_key(|card| {
            (
                Self::RANK_PRECEDENCE
                    .iter()
                    .position(|&r| r == card.rank())
                    .unwrap(),
                Self::SUIT_PRECEDENCE
                    .iter()
                    .position(|&s| s == card.suit())
                    .unwrap(),
            )
        })
    }

    /// Get the highest card in the hand.
    ///
    /// In Cho Dai Di, the highest card is the two of spades.
    pub fn highest_card(&self) -> Option<&Card> {
        self.iter().max_by_key(|card| {
            (
                Self::RANK_PRECEDENCE.iter().position(|&r| r == card.rank()),
                Self::SUIT_PRECEDENCE.iter().position(|&s| s == card.suit()),
            )
        })
    }

    /// When checking five-card hands, the rank of the largest group is the one
    /// that determines the rank of the hand.
    pub(crate) fn rank_of_largest_group(&self) -> Option<Rank> {
        if self.is_empty() {
            return None;
        }

        let rank_counts = self.iter().fold([0; 13], |mut acc, card| {
            acc[Self::RANK_PRECEDENCE
                .iter()
                .position(|&r| r == card.rank())
                .unwrap()] += 1;
            acc
        });

        let max_count = rank_counts.iter().max().unwrap();
        let max_rank =
            Self::RANK_PRECEDENCE[rank_counts.iter().position(|&c| c == *max_count).unwrap()];
        Some(max_rank)
    }

    pub fn may_be_followed_by(&self, other: &Self) -> anyhow::Result<()> {
        // NOTE: We assume that this hand is valid or else
        // we wouldn't be checking if something could follow it.
        debug_assert!(self.is_valid_hand().is_ok(), "self is not a valid hand");

        if self.len() != other.len() {
            bail!("during a trick, all hand must contain the same number of cards");
        }

        // Ensure that the other hand is valid
        other.is_valid_hand()?;

        match (self.len(), other.len()) {
            // Any card from the deck, ordered by rank with suit being the
            // tie-breaker. (For instance, Spade A beats Heart A, which beats
            // Heart K.)
            (1, 1) => {
                let s_card = self.highest_card().unwrap();
                let o_card = other.highest_card().unwrap();

                match Self::cmp_card(s_card, o_card) {
                    Ordering::Less => Ok(()),
                    Ordering::Greater | Ordering::Equal => Err(anyhow::anyhow!(
                        "the played card must be higher than the previous card"
                    )),
                }
            }
            // Any two cards of matching rank, ordered as with singular cards by
            // the card of the higher suit. (A pair consisting of the Spade K
            // and Diamond K beats a pair consisting of Hearts K and Clubs K.)
            (2, 2) => {
                let s_card = self.highest_card().unwrap();
                let o_card = other.highest_card().unwrap();

                match Self::cmp_card(s_card, o_card) {
                    Ordering::Less => Ok(()),
                    Ordering::Greater | Ordering::Equal => Err(anyhow::anyhow!(
                        "the played pair must be higher than the previous pair"
                    )),
                }
            }
            // Three equal ranked cards, three twos are highest, then aces,
            // kings, etc. down to three threes, which is the lowest triple.
            (3, 3) => {
                let s_card = self.highest_card().unwrap();
                let o_card = other.highest_card().unwrap();

                match Self::cmp_card(s_card, o_card) {
                    Ordering::Less => Ok(()),
                    Ordering::Greater | Ordering::Equal => Err(anyhow::anyhow!(
                        "the played triplet must be higher than the previous triplet"
                    )),
                }
            }
            // There are five (var. 2) different valid five-card hands, ranking
            // from low to high as follows (the same ranking as in poker, where
            // applicable)
            (5, 5) => {
                let s_card = self.highest_card().unwrap();
                let o_card = other.highest_card().unwrap();

                if self.is_a_straight_flush() && other.is_a_straight_flush() {
                    match Self::cmp_card(s_card, o_card) {
                        Ordering::Less => Ok(()),
                        Ordering::Greater | Ordering::Equal => Err(anyhow::anyhow!(
                            "the played straight flush must be higher than the previous straight flush"
                        )),
                    }
                } else if self.is_a_straight() && other.is_a_straight() {
                    match Self::cmp_card(s_card, o_card) {
                        Ordering::Less => Ok(()),
                        Ordering::Greater | Ordering::Equal => Err(anyhow::anyhow!(
                            "the played straight must be higher than the previous straight"
                        )),
                    }
                } else if self.is_a_flush() && other.is_a_flush() {
                    match Self::cmp_card(s_card, o_card) {
                        Ordering::Less => Ok(()),
                        Ordering::Greater | Ordering::Equal => Err(anyhow::anyhow!(
                            "the played flush must be higher than the previous flush"
                        )),
                    }
                } else if self.is_a_full_house() && other.is_a_full_house() {
                    match Self::cmp_card(s_card, o_card) {
                        Ordering::Less => Ok(()),
                        Ordering::Greater | Ordering::Equal => Err(anyhow::anyhow!(
                            "the played full house must be higher than the previous full house"
                        )),
                    }
                } else if self.is_four_of_a_kind_plus_one() && other.is_four_of_a_kind_plus_one() {
                    // We want to judge this based on the four of a kind cards, not the extra card
                    let s_rank = self.rank_of_largest_group().expect("hand is not empty");
                    let o_rank = other.rank_of_largest_group().expect("hand is not empty");

                    match Self::cmp_rank(&s_rank, &o_rank) {
                        Ordering::Less => Ok(()),
                        Ordering::Greater | Ordering::Equal => Err(anyhow::anyhow!(
                            "the played four of a kind plus one must be higher than the previous four of a kind plus one"
                        )),
                    }
                } else {
                    Err(anyhow::anyhow!("todo"))
                }
            }
            (_, _) => unreachable!(),
        }
    }

    // TODO memoize this or create a lookup table to see if that saves time
    fn is_valid_hand(&self) -> anyhow::Result<()> {
        if self.is_empty() {
            bail!("a hand must contain at least one card");
        }

        let is_the_right_size =
            self.len() == 1 || self.len() == 2 || self.len() == 3 || self.len() == 5;
        if !is_the_right_size {
            bail!("plays must be either a single card, a pair, a triplet, or a quintuple");
        };

        if (1..=3).contains(&self.len()) {
            let is_all_of_same_rank = self.are_all_of_same_rank()?;
            if is_all_of_same_rank {
                return Ok(());
            }

            bail!("1-3 card plays may only contain cards of the same rank");
        }

        if self.len() == 5 {
            let is_four_of_a_kind_plus_one = self.is_four_of_a_kind_plus_one();
            let is_a_straight = self.is_a_straight();
            let is_a_flush = self.is_a_flush();
            let is_a_full_house = self.is_a_full_house();
            let is_a_straight_flush = self.is_a_straight_flush();

            if is_four_of_a_kind_plus_one
                || is_a_straight
                || is_a_flush
                || is_a_full_house
                || is_a_straight_flush
            {
                return Ok(());
            }

            bail!("5 card plays must be a straight, a flush, a full house, or a straight flush");
        }

        unreachable!("all possible cases have been handled")
    }

    /// If this hand is a straight, return true.
    ///
    /// A straight is a hand with five cards of consecutive ranks.
    /// In Cho Dai Di, three 'special' straights are also possible:
    /// - `[Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five]`
    /// - `[Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six]`
    /// - `[Rank::Jack, Rank::Queen, Rank::King, Rank::Ace, Rank::Two]`
    pub fn is_a_straight(&self) -> bool {
        if self.len() != 5 {
            return false;
        }

        let mut ranks: Vec<Rank> = self.iter().map(|card| card.rank()).collect();
        ranks.sort_by(Self::cmp_rank);

        // Check for the basic straights
        let is_basic_straight = ranks
            == [Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven]
            || ranks == [Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight]
            || ranks == [Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine]
            || ranks == [Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten]
            || ranks == [Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack]
            || ranks == [Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen]
            || ranks == [Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King]
            || ranks == [Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];

        // Check for the special straights
        let is_special_straight = ranks
            == [Rank::Three, Rank::Four, Rank::Five, Rank::Ace, Rank::Two]
            || ranks == [Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Two]
            || ranks == [Rank::Jack, Rank::Queen, Rank::King, Rank::Ace, Rank::Two];

        is_basic_straight || is_special_straight
    }

    /// If this hand is a straight flush, return true.
    pub fn is_a_straight_flush(&self) -> bool {
        self.is_a_straight() && self.is_a_flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::STANDARD_DECK;

    #[test]
    fn test_cmp_suit() {
        assert!(Cards::<ChoDaiDi>::cmp_suit(&Suit::Spades, &Suit::Spades) == Ordering::Equal);
        assert!(Cards::<ChoDaiDi>::cmp_suit(&Suit::Hearts, &Suit::Hearts) == Ordering::Equal);
        assert!(Cards::<ChoDaiDi>::cmp_suit(&Suit::Clubs, &Suit::Clubs) == Ordering::Equal);
        assert!(Cards::<ChoDaiDi>::cmp_suit(&Suit::Diamonds, &Suit::Diamonds) == Ordering::Equal);

        assert!(Cards::<ChoDaiDi>::cmp_suit(&Suit::Diamonds, &Suit::Clubs) == Ordering::Less);
        assert!(Cards::<ChoDaiDi>::cmp_suit(&Suit::Clubs, &Suit::Hearts) == Ordering::Less);
        assert!(Cards::<ChoDaiDi>::cmp_suit(&Suit::Hearts, &Suit::Spades) == Ordering::Less);
        assert!(Cards::<ChoDaiDi>::cmp_suit(&Suit::Spades, &Suit::Diamonds) == Ordering::Greater);
    }

    #[test]
    fn test_cmp_rank() {
        // Ensure every rank is equal to itself
        for rank in Cards::<ChoDaiDi>::RANK_PRECEDENCE {
            assert!(Cards::<ChoDaiDi>::cmp_rank(rank, rank) == Ordering::Equal);
        }

        assert!(Cards::<ChoDaiDi>::cmp_rank(&Rank::Three, &Rank::Four) == Ordering::Less);
        assert!(Cards::<ChoDaiDi>::cmp_rank(&Rank::Four, &Rank::Five) == Ordering::Less);
        assert!(Cards::<ChoDaiDi>::cmp_rank(&Rank::Five, &Rank::Six) == Ordering::Less);
        assert!(Cards::<ChoDaiDi>::cmp_rank(&Rank::Six, &Rank::Seven) == Ordering::Less);
        assert!(Cards::<ChoDaiDi>::cmp_rank(&Rank::Seven, &Rank::Eight) == Ordering::Less);
        assert!(Cards::<ChoDaiDi>::cmp_rank(&Rank::Eight, &Rank::Nine) == Ordering::Less);
        assert!(Cards::<ChoDaiDi>::cmp_rank(&Rank::Nine, &Rank::Ten) == Ordering::Less);
        assert!(Cards::<ChoDaiDi>::cmp_rank(&Rank::Ten, &Rank::Jack) == Ordering::Less);
        assert!(Cards::<ChoDaiDi>::cmp_rank(&Rank::Jack, &Rank::Queen) == Ordering::Less);
        assert!(Cards::<ChoDaiDi>::cmp_rank(&Rank::Queen, &Rank::King) == Ordering::Less);
        assert!(Cards::<ChoDaiDi>::cmp_rank(&Rank::King, &Rank::Ace) == Ordering::Less);
        assert!(Cards::<ChoDaiDi>::cmp_rank(&Rank::Ace, &Rank::Two) == Ordering::Less);
    }

    #[test]
    fn test_cmp_card() {
        // Ensure every card is equal to itself
        for card in STANDARD_DECK.iter() {
            assert!(Cards::<ChoDaiDi>::cmp_card(card, card) == Ordering::Equal);
        }

        // ensure the three of diamonds is less than any other card
        for card in STANDARD_DECK {
            if card == Card::THREE_OF_DIAMONDS {
                continue;
            }

            assert!(Cards::<ChoDaiDi>::cmp_card(&Card::THREE_OF_DIAMONDS, &card) == Ordering::Less);
        }

        // ensure the two of spades is greater than any other card
        for card in STANDARD_DECK {
            if card == Card::TWO_OF_SPADES {
                continue;
            }

            assert!(Cards::<ChoDaiDi>::cmp_card(&card, &Card::TWO_OF_SPADES) == Ordering::Less);
        }
    }

    #[test]
    fn test_is_a_straight() {
        assert!(
            Cards::<ChoDaiDi>::try_from(vec!["2S", "3D", "4S", "5C", "6H"])
                .unwrap()
                .is_a_straight()
        );
        assert!(
            Cards::<ChoDaiDi>::try_from(vec!["AC", "2D", "3S", "4D", "5H"])
                .unwrap()
                .is_a_straight()
        );
        assert!(
            Cards::<ChoDaiDi>::try_from(vec!["JD", "QC", "KS", "AD", "2H"])
                .unwrap()
                .is_a_straight()
        );
        assert!(
            Cards::<ChoDaiDi>::try_from(vec!["5C", "6H", "7S", "8D", "9H"])
                .unwrap()
                .is_a_straight()
        );
    }

    #[test]
    fn test_highest_card() {
        let hand = Cards::<ChoDaiDi>::try_from(vec!["2S", "3D", "4S", "5C", "6H"]).unwrap();
        assert_eq!(Card::TWO_OF_SPADES, *hand.highest_card().unwrap());

        let hand = Cards::<ChoDaiDi>::try_from(vec!["3S", "4D", "5H"]).unwrap();
        assert_eq!(Card::FIVE_OF_HEARTS, *hand.highest_card().unwrap());

        let hand = Cards::<ChoDaiDi>::try_from(vec!["JD", "QC", "KS", "AD"]).unwrap();
        assert_eq!(Card::ACE_OF_DIAMONDS, *hand.highest_card().unwrap());

        let hand = Cards::<ChoDaiDi>::try_from(vec!["5C", "6H", "7S", "8D", "9H"]).unwrap();
        assert_eq!(Card::NINE_OF_HEARTS, *hand.highest_card().unwrap());
    }

    #[test]
    fn test_lowest_card() {
        let hand = Cards::<ChoDaiDi>::try_from(vec!["2S", "3D", "4S", "5C", "6H"]).unwrap();
        assert_eq!(Card::THREE_OF_DIAMONDS, *hand.lowest_card().unwrap());

        let hand = Cards::<ChoDaiDi>::try_from(vec!["4D", "3S", "5H"]).unwrap();
        assert_eq!(Card::THREE_OF_SPADES, *hand.lowest_card().unwrap());

        let hand = Cards::<ChoDaiDi>::try_from(vec!["QC", "KS", "JD", "AD"]).unwrap();
        assert_eq!(Card::JACK_OF_DIAMONDS, *hand.lowest_card().unwrap());

        let hand = Cards::<ChoDaiDi>::try_from(vec!["7S", "8D", "9H", "5C", "6H"]).unwrap();
        assert_eq!(Card::FIVE_OF_CLUBS, *hand.lowest_card().unwrap());
    }

    #[test]
    fn test_sort_by_rank() {
        let mut hand = Cards::<ChoDaiDi>::try_from(vec!["2S", "3D", "JS", "AC", "6H"]).unwrap();
        hand.sort_by_rank();
        assert_eq!(
            Cards::try_from(vec!["3D", "6H", "JS", "AC", "2S"]).unwrap(),
            hand
        );

        let mut hand = Cards::<ChoDaiDi>::try_from(vec!["AD", "AS", "AH", "AC"]).unwrap();
        hand.sort_by_rank();
        assert_eq!(Cards::try_from(vec!["AD", "AC", "AH", "AS"]).unwrap(), hand);
    }

    #[test]
    fn test_sort_by_suit() {
        let mut hand = Cards::<ChoDaiDi>::try_from(vec!["2S", "3D", "JS", "AC", "6H"]).unwrap();
        hand.sort_by_suit();
        assert_eq!(
            Cards::try_from(vec!["3D", "AC", "6H", "JS", "2S"]).unwrap(),
            hand
        );

        let mut hand = Cards::<ChoDaiDi>::try_from(vec!["AD", "AS", "AH", "AC"]).unwrap();
        hand.sort_by_suit();
        assert_eq!(Cards::try_from(vec!["AD", "AC", "AH", "AS"]).unwrap(), hand);
    }

    #[test]
    fn test_may_be_followed_by_singles() {
        let two_of_spades = Cards::<ChoDaiDi>::from(Card::TWO_OF_SPADES);
        // ensure the two of spades can follow any other single card
        for card in STANDARD_DECK {
            if card == Card::TWO_OF_SPADES {
                continue;
            }

            let last_play: Cards<ChoDaiDi> = card.into();
            match last_play.may_be_followed_by(&two_of_spades) {
                Ok(_) => continue,
                Err(e) => panic!(
                    "expected {two_of_spades} to be able to follow {last_play} but got an error: {e}",
                ),
            }
        }
        let three_of_diamonds = Cards::<ChoDaiDi>::from(Card::THREE_OF_DIAMONDS);
        // ensure any card can follow the three of diamonds
        for card in STANDARD_DECK {
            if card == Card::THREE_OF_DIAMONDS {
                continue;
            }

            let last_play: Cards<ChoDaiDi> = card.into();
            match three_of_diamonds.may_be_followed_by(&last_play) {
                Ok(_) => continue,
                Err(e) => panic!(
                    "expected {last_play} to be able to follow {three_of_diamonds} but got an error: {e}",
                ),
            }
        }

        let last_play = Cards::<ChoDaiDi>::try_from(vec!["3D"]).unwrap();
        let single = Cards::<ChoDaiDi>::try_from(vec!["3S"]).unwrap();
        assert!(last_play.may_be_followed_by(&single).is_ok());

        let last_play = Cards::<ChoDaiDi>::try_from(vec!["AC"]).unwrap();
        let single = Cards::<ChoDaiDi>::try_from(vec!["AH"]).unwrap();
        assert!(last_play.may_be_followed_by(&single).is_ok());

        let last_play = Cards::<ChoDaiDi>::try_from(vec!["6C"]).unwrap();
        let single = Cards::<ChoDaiDi>::try_from(vec!["6H"]).unwrap();
        assert!(last_play.may_be_followed_by(&single).is_ok());
    }

    #[test]
    fn test_may_not_be_followed_by_singles() {
        let last_play = Cards::<ChoDaiDi>::try_from(vec!["7D"]).unwrap();
        let single = Cards::<ChoDaiDi>::try_from(vec!["5C"]).unwrap();
        assert!(last_play.may_be_followed_by(&single).is_err());

        let last_play = Cards::<ChoDaiDi>::try_from(vec!["AS"]).unwrap();
        let single = Cards::<ChoDaiDi>::try_from(vec!["KH"]).unwrap();
        assert!(last_play.may_be_followed_by(&single).is_err());

        let last_play = Cards::<ChoDaiDi>::try_from(vec!["8S"]).unwrap();
        let single = Cards::<ChoDaiDi>::try_from(vec!["8H"]).unwrap();
        assert!(last_play.may_be_followed_by(&single).is_err());
    }

    #[test]
    fn test_may_be_followed_by_pairs() {
        let last_play = Cards::<ChoDaiDi>::try_from(vec!["3D", "3H"]).unwrap();
        let pair = Cards::<ChoDaiDi>::try_from(vec!["3C", "3S"]).unwrap();
        assert!(last_play.may_be_followed_by(&pair).is_ok());

        let last_play = Cards::<ChoDaiDi>::try_from(vec!["3S", "3H"]).unwrap();
        let pair = Cards::<ChoDaiDi>::try_from(vec!["4C", "4D"]).unwrap();
        assert!(last_play.may_be_followed_by(&pair).is_ok());
    }

    #[test]
    fn test_may_not_be_followed_by_pairs() {
        let last_play = Cards::<ChoDaiDi>::try_from(vec!["2D", "2H"]).unwrap();
        let pair = Cards::<ChoDaiDi>::try_from(vec!["AC", "AS"]).unwrap();
        assert!(last_play.may_be_followed_by(&pair).is_err());

        let last_play = Cards::<ChoDaiDi>::try_from(vec!["3S", "3H"]).unwrap();
        let pair = Cards::<ChoDaiDi>::try_from(vec!["3C", "3D"]).unwrap();
        assert!(last_play.may_be_followed_by(&pair).is_err());

        let last_play = Cards::<ChoDaiDi>::try_from(vec!["5S", "5H"]).unwrap();
        let full_house = Cards::<ChoDaiDi>::try_from(vec!["6C", "6D", "6H", "7C", "7D"]).unwrap();
        assert!(last_play.may_be_followed_by(&full_house).is_err());
    }

    #[test]
    fn test_may_be_followed_by_triplets() {
        let last_play = Cards::<ChoDaiDi>::try_from(vec!["3D", "3H", "3S"]).unwrap();
        let triplet = Cards::<ChoDaiDi>::try_from(vec!["4C", "4D", "4H"]).unwrap();
        assert!(last_play.may_be_followed_by(&triplet).is_ok());

        let last_play = Cards::<ChoDaiDi>::try_from(vec!["9S", "9H", "9D"]).unwrap();
        let triplet = Cards::<ChoDaiDi>::try_from(vec!["JC", "JD", "JH"]).unwrap();
        assert!(last_play.may_be_followed_by(&triplet).is_ok());
    }

    #[ignore]
    #[test]
    fn test_may_not_be_followed_by_triplets() {
        let last_play = Cards::<ChoDaiDi>::try_from(vec!["2D", "2H", "2S"]).unwrap();
        let triplet = Cards::<ChoDaiDi>::try_from(vec!["AC", "AS", "AH"]).unwrap();
        assert!(last_play.may_be_followed_by(&triplet).is_err());

        let last_play = Cards::<ChoDaiDi>::try_from(vec!["AS", "AH", "AD"]).unwrap();
        let triplet = Cards::<ChoDaiDi>::try_from(vec!["JC", "JD", "JH"]).unwrap();
        assert!(last_play.may_be_followed_by(&triplet).is_err());

        let last_play = Cards::<ChoDaiDi>::try_from(vec!["5S", "5H", "5D"]).unwrap();
        let full_house = Cards::<ChoDaiDi>::try_from(vec!["6C", "6D", "6H", "7C", "7D"]).unwrap();
        assert!(last_play.may_be_followed_by(&full_house).is_err());
    }

    #[test]
    fn test_may_be_followed_by_four_of_a_kind_plus_one() {
        let last_play = Cards::<ChoDaiDi>::try_from(vec!["3D", "3H", "3S", "3C", "2S"]).unwrap();
        let four_of_a_kind_plus_one =
            Cards::<ChoDaiDi>::try_from(vec!["4C", "4D", "4H", "4S", "5H"]).unwrap();
        assert!(last_play
            .may_be_followed_by(&four_of_a_kind_plus_one)
            .is_ok());

        let last_play = Cards::<ChoDaiDi>::try_from(vec!["9S", "9H", "9D", "9C", "4C"]).unwrap();
        let four_of_a_kind_plus_one =
            Cards::<ChoDaiDi>::try_from(vec!["JC", "JD", "JH", "JS", "5H"]).unwrap();
        assert!(last_play
            .may_be_followed_by(&four_of_a_kind_plus_one)
            .is_ok());

        let last_play = Cards::<ChoDaiDi>::try_from(vec!["AS", "AH", "AD", "AC", "3S"]).unwrap();
        let four_of_a_kind_plus_one =
            Cards::<ChoDaiDi>::try_from(vec!["2C", "2D", "2H", "2S", "JH"]).unwrap();
        assert!(last_play
            .may_be_followed_by(&four_of_a_kind_plus_one)
            .is_ok());
    }

    #[ignore]
    #[test]
    fn test_may_not_be_followed_by_four_of_a_kind_plus_one() {
        let last_play = Cards::<ChoDaiDi>::try_from(vec!["2D", "2H", "2S", "2C", "3S"]).unwrap();
        let four_of_a_kind_plus_one =
            Cards::<ChoDaiDi>::try_from(vec!["AC", "AS", "AH", "AD", "5H"]).unwrap();
        assert!(last_play
            .may_be_followed_by(&four_of_a_kind_plus_one)
            .is_err());

        let last_play = Cards::<ChoDaiDi>::try_from(vec!["AS", "AH", "AD", "AC", "3S"]).unwrap();
        let four_of_a_kind_plus_one =
            Cards::<ChoDaiDi>::try_from(vec!["JC", "JD", "JH", "JS", "5H"]).unwrap();
        assert!(last_play
            .may_be_followed_by(&four_of_a_kind_plus_one)
            .is_err());
    }

    #[ignore]
    #[test]
    fn test_may_be_followed_by_full_house() {
        todo!()
    }

    #[ignore]
    #[test]
    fn test_may_not_be_followed_by_full_house() {
        todo!()
    }

    #[ignore]
    #[test]
    fn test_may_be_followed_by_straight() {
        todo!()
    }

    #[ignore]
    #[test]
    fn test_may_not_be_followed_by_straight() {
        todo!()
    }

    #[ignore]
    #[test]
    fn test_may_be_followed_by_flush() {
        todo!()
    }

    #[ignore]
    #[test]
    fn test_may_not_be_followed_by_flush() {
        todo!()
    }

    #[ignore]
    #[test]
    fn test_may_be_followed_by_straight_flush() {
        todo!()
    }

    #[ignore]
    #[test]
    fn test_may_not_be_followed_by_straight_flush() {
        todo!()
    }
}
