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

## Usage

```bash
fair <client_seed> <server_seed> <nonce> <game>
```

Example usage:

```bash
$ fair "client seed" "server seed" 2 baccarat
Client seed: client seed
Server seed: server seed
Nonce: 2

Player won

Player (9): ♦9 - ♦10
Banker (7): ♥4 - ♦3

$ fair plinko --help
Plinko game

USAGE:
    fair <client_seed> <server_seed> <nonce> plinko [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --risk <risk>    Risk [possible values: low, medium, high]
        --rows <rows>    Rows
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
- [ ] Wheel
- [ ] Slots
- [ ] Crash

## Rust API docs

[fair](https://docs.rs/fair/)
