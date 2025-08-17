// Crie um programa que: Peça para o usuario digitar 3 nomes, armazene em um vetor. 
// Depois imprima: Bem vindo, {nome}!
// Para cada um
// Em Rust
use std::io;

fn main() {
    let mut nomes = Vec::new();

    // Pedir para o usuário digitar 3 nomes
    for i in 1..=3 {
        println!("Digite o {}º nome:", i);
        let mut nome = String::new();
        io::stdin().read_line(&mut nome).expect("Falha ao ler a linha");
        nomes.push(nome.trim().to_string());
    }

    // Imprimir mensagem de boas-vindas para cada nome
    for nome in nomes {
        println!("Bem-vindo, {}!", nome);
    }
}
