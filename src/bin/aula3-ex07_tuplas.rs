fn main() {
    let aluno = (String::from("Felipe"),21,8.7);

    //Acessando pelo indice

    println!("Nome: {}", aluno.0);
    println!("Idade: {}", aluno.1);
    println!("Nota: {}", aluno.2);

// Desestruturando a tupla

let (nome, idade, nota) = aluno;
println!("Nome: {}, idade {}, nota: {}", nome, idade, nota );

}
