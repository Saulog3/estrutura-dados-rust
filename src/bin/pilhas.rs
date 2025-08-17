fn main() {
    println!("Exercício: Pilhas (Stack)");

    let mut pilha: Vec<i32> = Vec::new();

    // empilhar
    pilha.push(10);
    pilha.push(20);
    pilha.push(30);

    println!("Pilha: {:?}", pilha);

    // desempilhar
    while let Some(valor) = pilha.pop() {
        println!("Desempilhado: {}", valor);
    }
}
