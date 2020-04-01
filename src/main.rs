use clap::*;
use std::process;

use fair::games::*;
use fair::ProvablyFairConfig;

// TODO: implement game as subcommands? cause plinko games has some additional parameters (e.g.
// risk and rows)

// TODO: refactor so that client_seed, server_see and nonce required for all games except crash
fn main() {
    let matches = clap_app!(myapp =>
        (name: crate_name!())
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@subcommand baccarat =>
            (about: "Baccarat game"))
        (@subcommand dice =>
            (about: "Dice game")
        )
        (@subcommand limbo =>
            (about: "Limbo game")
        )
        (@subcommand hilo =>
            (about: "Hilo game")
        )
        (@subcommand blackjack =>
            (about: "Blackjack")
        )
        (@subcommand diamond_poker =>
            (about: "Diamond poker")
        )
        (@subcommand roulette =>
            (about: "Roulette")
        )
        (@subcommand plinko =>
            (about: "Plinko game")
            (@arg risk: --risk +takes_value
                 default_value("low")
                 possible_value[low]
                 possible_value[medium]
                 possible_value[high]
                 "Risk")
            (@arg rows: --rows +takes_value
                 default_value("8")
                 {validate_plinko_rows}
                 "Rows")
        )
        (@subcommand keno =>
            (about: "Keno")
        )
        (@subcommand mines =>
            (about: "Mines game")
            (@arg mines: --mines +takes_value
                 default_value("3")
                 {validate_mines_mines}
                 "Number of Mines")
        )
        (@subcommand video_poker =>
            (about: "Video Poker")
        )
        (@subcommand wheel =>
            (about: "Wheel game")
            (@arg risk: --risk +takes_value
                 default_value("low")
                 possible_value[low]
                 possible_value[medium]
                 possible_value[high]
                 "Risk")
            (@arg segments: --segments +takes_value
                 default_value("10")
                 possible_value("10")
                 possible_value("20")
                 possible_value("30")
                 possible_value("40")
                 possible_value("50")
                 "Segments")
        )
        (@subcommand slots =>
            (about: "Slots game(s)")
            (@arg round: --round +takes_value
                 default_value("0")
                 "Round #")
        )
        (@subcommand crash =>
            (about: "Crash game (uses Stake.com's parameters). Does not use client/server seed and nonce arguments.")
            (@arg verify: --verify "Verify whether the hash is valid (can be slow)")
            (@arg game_hash: +required
                 {validate_hex}
                 "Game hash")
        )
        (@arg client_seed: +required "Client seed")
        (@arg server_seed: +required "Server seed")
        (@arg nonce: +required "Nonce (positive integer)")
    )
    .get_matches();

    // let game = matches.value_of("game").unwrap().to_lowercase();
    let client_seed = matches.value_of("client_seed").unwrap();
    let server_seed = matches.value_of("server_seed").unwrap();
    // println!("{:?}", matches);

    // TODO: list supported games!
    // TODO use value_t! to parse game.. ensure game is in valid list of strings...
    let nonce: u64 = value_t!(matches, "nonce", u64).unwrap_or_else(|e| e.exit());
    // println!("{:?}", matches);
    println!("Client seed: {}", client_seed);
    println!("Server seed: {}", server_seed);
    println!("Nonce: {}", nonce);
    println!("");

    let config = ProvablyFairConfig::new(client_seed, server_seed, nonce);

    let res = match matches.subcommand() {
        ("baccarat", _) => baccarat::simulate(config).to_string(),
        ("dice", _) => dice::simulate(config).to_string(),
        ("limbo", _) => limbo::simulate(config).to_string(),
        ("hilo", _) => hilo::simulate(config).to_string(),
        ("blackjack", _) => blackjack::simulate(config).to_string(),
        ("diamond_poker", _) => diamond_poker::simulate(config).to_string(),
        ("roulette", _) => roulette::simulate(config).to_string(),
        ("plinko", Some(sub_matches)) => {
            let rows: u8 = value_t!(sub_matches, "rows", u8).unwrap_or_else(|e| e.exit());
            let risk = sub_matches.value_of("risk").unwrap_or("low");

            let risk = match risk {
                "low" => plinko::Risk::Low,
                "medium" => plinko::Risk::Medium,
                "high" => plinko::Risk::High,
                _ => panic!("Uknown risk {}", risk),
            };

            let opts = plinko::Opts::new(rows, risk);
            plinko::simulate(config, Some(opts)).to_string()
        }
        ("keno", _) => keno::simulate(config).to_string(),
        ("mines", Some(sub_matches)) => {
            let mines: u8 = value_t!(sub_matches, "mines", u8).unwrap_or_else(|e| e.exit());
            mines::simulate(config, mines).to_string()
        }
        ("video_poker", _) => video_poker::simulate(config).to_string(),
        ("wheel", Some(sub_matches)) => {
            let segments: u8 = value_t!(sub_matches, "segments", u8).unwrap_or_else(|e| e.exit());
            let risk = sub_matches.value_of("risk").unwrap_or("low");
            let risk = wheel::Risk::from_str(risk);
            let opts = wheel::Opts::new(segments, risk);
            wheel::simulate(config, Some(opts)).to_string()
        }
        ("slots", Some(sub_matches)) => {
            let round: usize = value_t!(sub_matches, "round", usize).unwrap_or_else(|e| e.exit());
            slots::simulate(config, round).to_string()
        }
        ("crash", Some(sub_matches)) => {
            // TODO: ensure game_hash is valid hex with hex::decode
            let game_hash = sub_matches.value_of("game_hash").unwrap();

            let game_hash = crash::Hash::from_hex(game_hash);
            let config = crash::Config::for_stake();
            println!("{}", crash::simulate(config, game_hash));
            if sub_matches.is_present("verify") {
                println!("\nVerifying game hash, this could take a while...\n");
                if crash::verify_hash(config, game_hash) {
                    println!("Game hash is valid.");
                } else {
                    println!("!!! Game hash is INVALID !!!");
                }
            } else {
                println!("");
                println!("IMPORTANT: use --verify to verify the game hash is valid");
            }
            "".to_string()
        }
        _ => die("This branch should never execute. Unimplemented game?"),
    };
    println!("{}", res);
}

fn validate_plinko_rows(rows: String) -> std::result::Result<(), String> {
    let rows: u8 = rows.parse().unwrap_or(0);
    if rows >= 8 && rows <= 16 {
        Ok(())
    } else {
        Err("must be between 8 to 16 inclusive".to_string())
    }
}

fn validate_mines_mines(mines: String) -> std::result::Result<(), String> {
    let mines: u8 = mines.parse().unwrap_or(0);
    if mines >= 1 && mines <= 24 {
        Ok(())
    } else {
        Err("must be between 1 to 24 inclusive".to_string())
    }
}
fn validate_hex(hex: String) -> std::result::Result<(), String> {
    if hex.len() != 64 {
        return Err("must be 64 characters hexadecimal".to_string());
    }
    match hex::decode(hex) {
        Ok(_) => Ok(()),
        _ => Err("must be valid hexadecimal".to_string()),
    }
}

fn die(msg: &str) -> ! {
    eprintln!("{}", msg);
    process::exit(1);
}
