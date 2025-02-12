use clap::{Command};
use secret_number_challenge::start_game;

fn main() {
    // Define o comando principal
    let matches = Command::new("guess")
        .version("1.0")
        .author("Alex Silva")
        .about("Desafio do Número Secreto")
        .subcommand(
            Command::new("start")
                .about("Comece o desafio"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("start", _)) => start_game(),
        None => println!("Use o comando `guess start` para começar o desafio."),
        _ => unreachable!(),
    }
}
