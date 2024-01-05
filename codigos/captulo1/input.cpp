use std::io;

fn main() {
    // Le uma linha de texto do teclado
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Remove o caractere de nova linha da string
    input.pop();

    // Imprime a string lida
    println!("Voce digitou: {}", input);
}