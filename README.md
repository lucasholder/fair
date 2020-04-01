# fair

CLI tool and library for verifying provably fair games. Compatible with Stake's [provably fair algorithms](https://stake.com/provably-fair/overview).

[![crates.io](https://meritbadge.herokuapp.com/fair)](https://crates.io/crates/fair)
[![documentation](https://docs.rs/fair/badge.svg)](https://docs.rs/fair)
[![Build Status](https://travis-ci.org/lucasholder/fair.svg?branch=master)](https://travis-ci.org/lucasholder/fair)
[![Build Status Appveyor](https://ci.appveyor.com/api/projects/status/github/lucasholder/fair)](https://ci.appveyor.com/project/lucasholder/fair)

## Install

On Mac or Linux:

```bash
curl -sL https://raw.githubusercontent.com/lucasholder/fair/master/install.sh | sh
```

If you have Rust:

```bash
cargo install fair
```

## Examples

```bash
$ fair baccarat "client seed" "server seed" 1
Client seed: client seed
Server seed: server seed
Nonce: 1

Player won

Player (9): ♠J - ♥10 - ♥9
Banker (5): ♥5 - ♣K
```

```bash
$ fair mines "client seed" "server seed" 1
Squares: [18, 15, 5]


💠       💠      💠      💠      💠
💣       💠      💠      💠      💠
💠       💠      💠      💠      💠
💣       💠      💠      💣      💠
💠       💠      💠      💠      💠
```

## Usage

```bash
$ fair --help
fair 0.0.5-alpha.0
Lucas Holder <lucasholderx@gmail.com>
CLI tool and library for verifying provably fair games (baccarat, etc.).

USAGE:
    fair [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    baccarat         Baccarat game
    blackjack        Blackjack
    crash            Crash game (uses Stake.com's parameters). Does not use client/server seed and nonce arguments.
    diamond_poker    Diamond poker
    dice             Dice game
    help             Prints this message or the help of the given subcommand(s)
    hilo             Hilo game
    keno             Keno
    limbo            Limbo game
    mines            Mines game
    plinko           Plinko game
    roulette         Roulette
    slots            Slots game(s)
    video_poker      Video Poker
    wheel            Wheel game

```

As expected, we get the same result as on
[Stake.com](https://stake.com/casino/games/baccarat?clientSeed=client%20seed&game=baccarat&modal=verify&nonce=2&serverSeed=server%20seed).

## Supported Games

Work In Progress... more game support coming!

- [x] Blackjack
- [x] Hilo
- [x] Baccarat
- [x] Diamond Poker
- [x] Dice Roll
- [x] Limbo
- [x] Plinko
- [x] Roulette Roll
- [x] Keno
- [x] Mines
- [x] Video Poker
- [x] Wheel
- [x] Slots
- [x] Crash

## Rust API docs

[fair](https://docs.rs/fair/)
