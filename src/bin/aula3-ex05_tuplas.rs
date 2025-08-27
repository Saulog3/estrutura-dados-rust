fn main() {
    let pessoa: (&str, u8, f32) = ("Maria", 25, 1.65);
    
    println!("Nome: {}",pessoa.0);
    println!("Idade: {}",pessoa.1);
    println!("Altura: {}",pessoa.2);


}
