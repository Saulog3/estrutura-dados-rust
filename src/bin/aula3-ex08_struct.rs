fn main() {
    struct Pessoa{
        nome:String,
        idade:u8,
        altura:f32,
    }
    
    let pessoa = Pessoa{
        nome: String::from("Ana"),
        idade: 25,
        altura: 1.68,
    };
    let Pessoa {nome, idade, altura} = pessoa;
    println!("Nome: {}", nome);
    println!("Idade: {}", idade);
    println!("Altura: {}", altura);

}
