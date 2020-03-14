use clap::*;
use fair;
use std::process;

fn main() {
    let matches = clap_app!(myapp =>
        (name: crate_name!())
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg game: +required "Game")
        (@arg client_seed: +required "Client seed")
        (@arg server_seed: +required "Server seed")
        (@arg nonce: +required "Nonce (positive integer)")
    )
    .get_matches();

    let game = matches.value_of("game").unwrap().to_lowercase();
    let client_seed = matches.value_of("client_seed").unwrap();
    let server_seed = matches.value_of("server_seed").unwrap();

    // TODO use value_t! to parse game.. ensure game is in valid list of strings...
    let nonce: u64 = value_t!(matches, "nonce", u64).unwrap_or_else(|e| e.exit());
    // println!("{:?}", matches);
    println!("Client seed: {}", client_seed);
    println!("Server seed: {}", server_seed);
    println!("Nonce: {}", nonce);
    println!("");

    match game.as_str() {
        "baccarat" => {
            let result = fair::games::baccarat::simulate(client_seed, server_seed, nonce);
            println!("{}", result);
        }
        "dice" => {
            let result = fair::games::dice::simulate(client_seed, server_seed, nonce);
            println!("{}", result);
        }
        "limbo" => {
            let result = fair::games::limbo::simulate(client_seed, server_seed, nonce);
            println!("{}", result);
        }
        "hilo" => {
            let result = fair::games::hilo::simulate(client_seed, server_seed, nonce);
            println!("{}", result);
        }
        _ => {
            eprintln!("{} is not a supported game.", game);
            process::exit(1);
        }
    };
}
