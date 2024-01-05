fn main() {
    // Imprime um numero inteiro em binario
    println!("O numero em binario eh: {:b}", 42);

    // Imprime um caractere
    println!("O caractere eh: {}", 'A');

    // Imprime um numero inteiro em hexadecimal
    println!("O numero em hexadecimal eh: {:x}", 42);

    // Imprime um numero inteiro em octal
    println!("O numero em octal eh: {:o}", 42);

    // Imprime um ponteiro
    println!("O ponteiro eh: {:p}", &42);

    // Imprime um numero de ponto flutuante em notacao cientifica com um expoente em minusculas
    println!("O numero em notacao cientifica eh: {:e}", 3.14159);

    // Imprime um numero de ponto flutuante em notacao cientifica com um expoente em maiusculas
    println!("O numero em notacao cientifica eh: {:E}", 3.14159);

    // Imprime um numero de ponto flutuante com duas casas decimais
    println!("O numero eh: {:.2}", 3.14159);

    // Imprime um numero inteiro com separadores de milhar
    println!("O numero eh: {:#}", 1000);
}
