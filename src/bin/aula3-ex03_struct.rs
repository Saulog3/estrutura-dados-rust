
struct Pessoa {
    nome: String,
    idade: u8, // Só pode receber numeros positivos de 8 bits 0 à 255
}

fn main() {
    let aluno = Pessoa{
        nome: String::from("Maria"),
        idade:20,
    };
println!("{} tem {} anos", aluno.nome, aluno.idade)
}