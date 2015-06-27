# rust-cowbulls-tutorial
A simple entry-level tutorial for Rust (v1.1.0)

## Description
This is a simple tutorial for Rust language, written for rustc version 1.1.0.
It is a next little step after official documentation "Learn Rust" tutorials.
In this tutorial we implement the game known to me as "Cows and Bulls" and described below.

## Target audience
This tutorial is aimed at noobs like me, that have finished a few tutorials from official
documentation and were thinking on what to do next.

This tutorial doesn't build the code line by line, but rather explains it using comments.

## The game
Before we start, I'll tell about the game that is being implenented here. I believe it has many
names accross the globe, but to me it was known as "Cows and Bulls" since my school days when
we've played the game between the classes with my friends.

This game is for two players. One player has a passive role. He makes up a secret four-digit
number (digits must be unique and the number can start with zero) and writes it down somewhere
safe from gazes of the player two.

Then player two makes guesses on what this number could be and player one looks up the number
and counts the "cows" and "bulls", where "cows" is number of digits that player two guessed
right, but they are not on right positions in the original secret number. The "bulls" is number
of digits that are at their right place in the secret number. Player one doesn't tell which
digits are "cows" and "bulls".

For example, the secret number is 6437 and player two calls his guess:

> P2> 1234.

> P1> You've got one bull and one cow.

> P2> 1290.

> P1> There's nobody here.

> P2> 7364.

> P1> It's 4 cows, wow!

> P2> 6437.

> P1> Bingo! You've guessed it!

The challenge of the game for player two is guess the secret number in the smallest amount of
attempts. The pencil and a piece of paper usually helps to store collected data and analyze it
to lower attempts count to minimum.

In this game, which we program in Rust (v1.1.0) below, we let the computer be player one and
the user to play as a player two.

We'll also implement a simple tool to assist player with the collected data, which will help us
to undestand a bit more about a game logic.

## Build and run

First, you need to install Rust, if you don't have it yet. Follow [official instructions](http://www.rust-lang.org/install.html).

By default, you'll have [Cargo](https://crates.io/)(package manager for Rust) installed. If not, follow [official instructions](https://crates.io/install).

To build and run this tutorial you have to either clone it from [github repository](https://github.com/dread-deimos/rust-cowbulls-tutorial.git), or download and unzip it manually (use Download ZIP button). I recommend to use Git.

```
git clone https://github.com/dread-deimos/rust-cowbulls-tutorial.git
cd rust-cowbulls-tutorial
cargo run
```

This sequence will clone the repository, download dependencies, build and run the code.

## Disclaimer
I am not an experienced Rust developer and have written this tutorial mostly to familiarize with the Rust itself.

This tutorial doesn't cover two important branches of development: documentation and testing.
