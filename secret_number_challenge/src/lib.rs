use rand::Rng;
use std::io::{self, Write};

/// Função que gera um número secreto aleatório entre 1 e 100
pub fn generate_secret_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

/// Função que compara a tentativa do jogador com o número secreto
pub fn check_guess(guess: u32, secret_number: u32) -> String {
    if guess < secret_number {
        "Muito baixo! Tente novamente.".to_string()
    } else if guess > secret_number {
        "Muito alto! Tente novamente.".to_string()
    } else {
        "Parabéns! Você decifrou o número secreto em 1 tentativas.".to_string()
    }
}

/// Função que lida com o jogo e verifica a tentativa do jogador
pub fn start_game() {
    println!("Bem-vindo ao Desafio do Número Secreto!");
    println!("Tente decifrar o número entre 1 e 100!");

    let secret_number = generate_secret_number();
    let mut attempts = 0;

    loop {
        print!("Digite sua tentativa: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, insira um número válido.");
                continue;
            }
        };

        attempts += 1;

        let result = check_guess(guess, secret_number);
        println!("{}", result);

        if guess == secret_number {
            break;
        }
    }
}