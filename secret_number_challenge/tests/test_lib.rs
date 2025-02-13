use secret_number_challenge::check_guess;

#[test]
fn test_guess_low() {
    let guess = 50;
    let secret_number = 75;
    let attempts = 1;  // Passando a quantidade de tentativas
    let result = check_guess(guess, secret_number, attempts);
    assert_eq!(result, "Muito baixo! Tente novamente.");
}

#[test]
fn test_guess_high() {
    let guess = 90;
    let secret_number = 75;
    let attempts = 1;  // Passando a quantidade de tentativas
    let result = check_guess(guess, secret_number, attempts);
    assert_eq!(result, "Muito alto! Tente novamente.");
}

#[test]
fn test_guess_correct() {
    let guess = 75;
    let secret_number = 75;
    let attempts = 1;  // Passando a quantidade de tentativas
    let result = check_guess(guess, secret_number, attempts);
    assert_eq!(result, "Parabéns! Você decifrou o número secreto em 1 tentativas.");
}
