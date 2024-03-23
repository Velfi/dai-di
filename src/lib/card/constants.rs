use super::Card;
use crate::{rank::Rank, suit::Suit};

pub const STANDARD_DECK: [Card; 52] = [
    Card::TWO_OF_DIAMONDS,
    Card::THREE_OF_DIAMONDS,
    Card::FOUR_OF_DIAMONDS,
    Card::FIVE_OF_DIAMONDS,
    Card::SIX_OF_DIAMONDS,
    Card::SEVEN_OF_DIAMONDS,
    Card::EIGHT_OF_DIAMONDS,
    Card::NINE_OF_DIAMONDS,
    Card::TEN_OF_DIAMONDS,
    Card::JACK_OF_DIAMONDS,
    Card::QUEEN_OF_DIAMONDS,
    Card::KING_OF_DIAMONDS,
    Card::ACE_OF_DIAMONDS,
    Card::TWO_OF_CLUBS,
    Card::THREE_OF_CLUBS,
    Card::FOUR_OF_CLUBS,
    Card::FIVE_OF_CLUBS,
    Card::SIX_OF_CLUBS,
    Card::SEVEN_OF_CLUBS,
    Card::EIGHT_OF_CLUBS,
    Card::NINE_OF_CLUBS,
    Card::TEN_OF_CLUBS,
    Card::JACK_OF_CLUBS,
    Card::QUEEN_OF_CLUBS,
    Card::KING_OF_CLUBS,
    Card::ACE_OF_CLUBS,
    Card::TWO_OF_HEARTS,
    Card::THREE_OF_HEARTS,
    Card::FOUR_OF_HEARTS,
    Card::FIVE_OF_HEARTS,
    Card::SIX_OF_HEARTS,
    Card::SEVEN_OF_HEARTS,
    Card::EIGHT_OF_HEARTS,
    Card::NINE_OF_HEARTS,
    Card::TEN_OF_HEARTS,
    Card::JACK_OF_HEARTS,
    Card::QUEEN_OF_HEARTS,
    Card::KING_OF_HEARTS,
    Card::ACE_OF_HEARTS,
    Card::TWO_OF_SPADES,
    Card::THREE_OF_SPADES,
    Card::FOUR_OF_SPADES,
    Card::FIVE_OF_SPADES,
    Card::SIX_OF_SPADES,
    Card::SEVEN_OF_SPADES,
    Card::EIGHT_OF_SPADES,
    Card::NINE_OF_SPADES,
    Card::TEN_OF_SPADES,
    Card::JACK_OF_SPADES,
    Card::QUEEN_OF_SPADES,
    Card::KING_OF_SPADES,
    Card::ACE_OF_SPADES,
];

impl Card {
    pub const TWO_OF_DIAMONDS: Card = Card {
        rank: Rank::Two,
        suit: Suit::Diamonds,
    };
    pub const THREE_OF_DIAMONDS: Card = Card {
        rank: Rank::Three,
        suit: Suit::Diamonds,
    };
    pub const FOUR_OF_DIAMONDS: Card = Card {
        rank: Rank::Four,
        suit: Suit::Diamonds,
    };
    pub const FIVE_OF_DIAMONDS: Card = Card {
        rank: Rank::Five,
        suit: Suit::Diamonds,
    };
    pub const SIX_OF_DIAMONDS: Card = Card {
        rank: Rank::Six,
        suit: Suit::Diamonds,
    };
    pub const SEVEN_OF_DIAMONDS: Card = Card {
        rank: Rank::Seven,
        suit: Suit::Diamonds,
    };
    pub const EIGHT_OF_DIAMONDS: Card = Card {
        rank: Rank::Eight,
        suit: Suit::Diamonds,
    };
    pub const NINE_OF_DIAMONDS: Card = Card {
        rank: Rank::Nine,
        suit: Suit::Diamonds,
    };
    pub const TEN_OF_DIAMONDS: Card = Card {
        rank: Rank::Ten,
        suit: Suit::Diamonds,
    };
    pub const JACK_OF_DIAMONDS: Card = Card {
        rank: Rank::Jack,
        suit: Suit::Diamonds,
    };
    pub const QUEEN_OF_DIAMONDS: Card = Card {
        rank: Rank::Queen,
        suit: Suit::Diamonds,
    };
    pub const KING_OF_DIAMONDS: Card = Card {
        rank: Rank::King,
        suit: Suit::Diamonds,
    };
    pub const ACE_OF_DIAMONDS: Card = Card {
        rank: Rank::Ace,
        suit: Suit::Diamonds,
    };

    pub const TWO_OF_CLUBS: Card = Card {
        rank: Rank::Two,
        suit: Suit::Clubs,
    };
    pub const THREE_OF_CLUBS: Card = Card {
        rank: Rank::Three,
        suit: Suit::Clubs,
    };
    pub const FOUR_OF_CLUBS: Card = Card {
        rank: Rank::Four,
        suit: Suit::Clubs,
    };
    pub const FIVE_OF_CLUBS: Card = Card {
        rank: Rank::Five,
        suit: Suit::Clubs,
    };
    pub const SIX_OF_CLUBS: Card = Card {
        rank: Rank::Six,
        suit: Suit::Clubs,
    };
    pub const SEVEN_OF_CLUBS: Card = Card {
        rank: Rank::Seven,
        suit: Suit::Clubs,
    };
    pub const EIGHT_OF_CLUBS: Card = Card {
        rank: Rank::Eight,
        suit: Suit::Clubs,
    };
    pub const NINE_OF_CLUBS: Card = Card {
        rank: Rank::Nine,
        suit: Suit::Clubs,
    };
    pub const TEN_OF_CLUBS: Card = Card {
        rank: Rank::Ten,
        suit: Suit::Clubs,
    };
    pub const JACK_OF_CLUBS: Card = Card {
        rank: Rank::Jack,
        suit: Suit::Clubs,
    };
    pub const QUEEN_OF_CLUBS: Card = Card {
        rank: Rank::Queen,
        suit: Suit::Clubs,
    };
    pub const KING_OF_CLUBS: Card = Card {
        rank: Rank::King,
        suit: Suit::Clubs,
    };
    pub const ACE_OF_CLUBS: Card = Card {
        rank: Rank::Ace,
        suit: Suit::Clubs,
    };

    pub const TWO_OF_HEARTS: Card = Card {
        rank: Rank::Two,
        suit: Suit::Hearts,
    };
    pub const THREE_OF_HEARTS: Card = Card {
        rank: Rank::Three,
        suit: Suit::Hearts,
    };
    pub const FOUR_OF_HEARTS: Card = Card {
        rank: Rank::Four,
        suit: Suit::Hearts,
    };
    pub const FIVE_OF_HEARTS: Card = Card {
        rank: Rank::Five,
        suit: Suit::Hearts,
    };
    pub const SIX_OF_HEARTS: Card = Card {
        rank: Rank::Six,
        suit: Suit::Hearts,
    };
    pub const SEVEN_OF_HEARTS: Card = Card {
        rank: Rank::Seven,
        suit: Suit::Hearts,
    };
    pub const EIGHT_OF_HEARTS: Card = Card {
        rank: Rank::Eight,
        suit: Suit::Hearts,
    };
    pub const NINE_OF_HEARTS: Card = Card {
        rank: Rank::Nine,
        suit: Suit::Hearts,
    };
    pub const TEN_OF_HEARTS: Card = Card {
        rank: Rank::Ten,
        suit: Suit::Hearts,
    };
    pub const JACK_OF_HEARTS: Card = Card {
        rank: Rank::Jack,
        suit: Suit::Hearts,
    };
    pub const QUEEN_OF_HEARTS: Card = Card {
        rank: Rank::Queen,
        suit: Suit::Hearts,
    };
    pub const KING_OF_HEARTS: Card = Card {
        rank: Rank::King,
        suit: Suit::Hearts,
    };
    pub const ACE_OF_HEARTS: Card = Card {
        rank: Rank::Ace,
        suit: Suit::Hearts,
    };

    pub const TWO_OF_SPADES: Card = Card {
        rank: Rank::Two,
        suit: Suit::Spades,
    };
    pub const THREE_OF_SPADES: Card = Card {
        rank: Rank::Three,
        suit: Suit::Spades,
    };
    pub const FOUR_OF_SPADES: Card = Card {
        rank: Rank::Four,
        suit: Suit::Spades,
    };
    pub const FIVE_OF_SPADES: Card = Card {
        rank: Rank::Five,
        suit: Suit::Spades,
    };
    pub const SIX_OF_SPADES: Card = Card {
        rank: Rank::Six,
        suit: Suit::Spades,
    };
    pub const SEVEN_OF_SPADES: Card = Card {
        rank: Rank::Seven,
        suit: Suit::Spades,
    };
    pub const EIGHT_OF_SPADES: Card = Card {
        rank: Rank::Eight,
        suit: Suit::Spades,
    };
    pub const NINE_OF_SPADES: Card = Card {
        rank: Rank::Nine,
        suit: Suit::Spades,
    };
    pub const TEN_OF_SPADES: Card = Card {
        rank: Rank::Ten,
        suit: Suit::Spades,
    };
    pub const JACK_OF_SPADES: Card = Card {
        rank: Rank::Jack,
        suit: Suit::Spades,
    };
    pub const QUEEN_OF_SPADES: Card = Card {
        rank: Rank::Queen,
        suit: Suit::Spades,
    };
    pub const KING_OF_SPADES: Card = Card {
        rank: Rank::King,
        suit: Suit::Spades,
    };
    pub const ACE_OF_SPADES: Card = Card {
        rank: Rank::Ace,
        suit: Suit::Spades,
    };
}
