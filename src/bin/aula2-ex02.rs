use std::io;
fn main() {
    let mut entrada = String::new();
    println!("Digite seu nome: ");
    io::stdin().read_line(&mut entrada).unwrap();
    println!("Olá, {}!", entrada.trim());
}

