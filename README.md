# Dai Di (a.k.a. Big Two)

## How do you play?

**Dai Di (a.k.a. Big Two)** is a trick-taking card game that originated in China. The game is played with a standard deck of 52 playing cards and is usually played by four players. The objective of the game is to be the first player to get rid of all of their cards.

For a detailed explanation of the rules, see the [Wikipedia page](https://en.wikipedia.org/wiki/Big_two).

**Tips:**

- Enter a space-separated list of the cards you want to play. For example: `2c 3h 4d 5s 6s` or `2C 2D 2H` or `jc`
- You may toggle between sorting by rank and sorting by suit: enter `sort`
- You may pass your turn: enter `p` or `pass`
- You may quit the game: enter `q` or `quit`

## Installation & Usage

An up-to-date version of [Rust](https://www.rust-lang.org/tools/install) is required.
The game can be installed with cargo and then run:

```txt
$ cargo install dai-di --locked
$ dai-di
Starting a new four-player game
Good luck Player! Enter "help" if you need some guidance.
The player with the 3♦ will go first.
```

To set your name in-game, set the `DAI_DI_PLAYER_NAME` environment variable.
