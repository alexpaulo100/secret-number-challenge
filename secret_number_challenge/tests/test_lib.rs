use secret_number_challenge::{generate_secret_number, start_game};

#[test]
fn test_guess_low() {
    // Teste simula um valor baixo para a tentativa (espera que o jogo informe "Muito baixo")
    let guess = 50;
    let secret_number = 75; // Fixando um número secreto para o teste
    let result = check_guess(guess, secret_number); // Função fictícia, que simula a lógica de comparar as tentativas.
    assert_eq!(result, "Muito baixo! Tente novamente.");
}

#[test]
fn test_guess_high() {
    // Teste simula um valor alto para a tentativa (espera que o jogo informe "Muito alto")
    let guess = 90;
    let secret_number = 75; // Fixando um número secreto para o teste
    let result = check_guess(guess, secret_number); // Função fictícia, que simula a lógica de comparar as tentativas.
    assert_eq!(result, "Muito alto! Tente novamente.");
}

#[test]
fn test_guess_correct() {
    // Teste simula o acerto do número secreto
    let guess = 75;
    let secret_number = 75; // Valor simulado
    let result = check_guess(guess, secret_number); // Função fictícia
    assert_eq!(result, "Parabéns! Você decifrou o número secreto em 1 tentativas.");
}

// Simula a função de comparação entre a tentativa e o número secreto
fn check_guess(guess: u32, secret_number: u32) -> String {
    if guess < secret_number {
        "Muito baixo! Tente novamente.".to_string()
    } else if guess > secret_number {
        "Muito alto! Tente novamente.".to_string()
    } else {
        "Parabéns! Você decifrou o número secreto em 1 tentativas.".to_string()
    }
}
