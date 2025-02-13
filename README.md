# github-actions-rust

This project is a Rust-based application that implements the "Secret Number Challenge" game. The application is designed to guess a secret number between 1 and 100, with the user providing guesses until they discover the correct number.

Additionally, this project is set up with **GitHub Actions** for continuous integration (CI) to automatically build, test, and run the code every time changes are pushed to the repository.

[![Rust CI](https://github.com/alexpaulo100/secret-number-challenge/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/alexpaulo100/secret-number-challenge/actions/workflows/rust.yml)

## Features

- **Secret Number Challenge**: A simple game where you try to guess a secret number between 1 and 100.
- **GitHub Actions**: The project uses GitHub Actions to automatically build, test, and lint the Rust code.
  
## Requirements

- Rust (installed via `rustup`)

## Running the Game Locally

To run the game locally, make sure you have Rust installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).

Once Rust is installed, clone the repository and run the following commands:

```bash
# Clone the repository
git clone https://github.com/alexpaulo100/github-actions-rust.git

cd github-actions-rust/secret_number_challenge

# Build the project
cargo build

# Run the application
cargo run -- start
