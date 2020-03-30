use clap::*;
use fair;
use std::process;

// TODO: implement game as subcommands? cause plinko games has some additional parameters (e.g.
// risk and rows)

fn main() {
    let matches = clap_app!(myapp =>
        (name: crate_name!())
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@subcommand baccarat =>
            (about: "Baccarat game")
        )
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
        (@subcommand plinko =>
            (about: "Plinko game")
            (@arg risk:
                 possible_value[low]
                 possible_value[medium]
                 possible_value[high]
                 "Risk")
            (@arg rows: "Rows")
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

    if let (game, Some(_)) = matches.subcommand() {
        let result = fair::simulate(game, client_seed, server_seed, nonce);

        match result {
            Ok(s) => println!("{}", s),
            Err(s) => {
                eprintln!("{}", s);
                process::exit(1);
            }
        }
    }
}
