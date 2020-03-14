# fair

CLI tool and library for verifying provably fair games. Compatible with Stake's [provably fair algorithms](https://stake.com/provably-fair/overview).

[![Build Status](https://travis-ci.org/lucasholder/fair.svg?branch=master)](https://travis-ci.org/lucasholder/fair)
[![Build Status Appveyor](https://ci.appveyor.com/api/projects/status/github/lucasholder/fair)](https://ci.appveyor.com/project/lucasholder/fair)
[![crates.io](https://meritbadge.herokuapp.com/fair)](https://crates.io/crates/fair)

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
fair <game> <client_seed> <server_seed> <nonce>
```

Example usage:

```bash
$ fair baccarat "client seed" "server seed" 2
Client seed: client seed
Server seed: server seed
Nonce: 2

Player won

Player (9): ♦9 - ♦10
Banker (7): ♥4 - ♦3
```

As expected, we get the same result as on
[Stake.com](https://stake.com/casino/games/baccarat?clientSeed=client%20seed&game=baccarat&modal=verify&nonce=2&serverSeed=server%20seed).

## Supported Games

Work In Progress... more coming!

- Baccarat
- Dice

## Rust API docs

[fair](https://docs.rs/fair/)
