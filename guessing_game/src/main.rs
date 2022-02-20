use std::io; // io (input/output)

fn main() {
    println!("\nJogo da Advinhação!");

    println!("\nDigite um número de 1 a 100\n");

    let mut guess = String::new(); // "mut" makes the variable mutable;
                                   // "new()" is a function presented in almost all the variable types,
                                   // for create an empty instance of the given type.

    io::stdin()
        .read_line(&mut guess)
        .expect("Não foi possível a leitura do número!");

    println!("\nVocê digitou: {}", guess)
}