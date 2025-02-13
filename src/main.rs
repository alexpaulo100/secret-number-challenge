use clap::{Command};
use secret_number_challenge::start_game;

fn main() {
    // Define o comando principal
    let matches = Command::new("secret_number_challenge")
        .version("1.0")
        .author("Alex Silva")
        .about("Desafio do Número Secreto")
        .subcommand(
            Command::new("start")
                .about("Comece o desafio"),
        )
        .get_matches();

    // Lida com os subcomandos
    match matches.subcommand() {
        Some(("start", _)) => start_game(),
        None => {
            println!("Use o comando `secret_number_challenge start` para começar o desafio.");
        },
        _ => {
            println!("Comando desconhecido. Use `secret_number_challenge start` para começar.");
        },
    }
}
