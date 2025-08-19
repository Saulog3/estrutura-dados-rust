//Pilha (Stack) em Rust
fn main() {
    let mut pilha: Vec<i32> = Vec::new();
    pilha.push(10); //Empilhar (push)
    pilha.push(20); 
    pilha.push(30); 
    println!("Pilha: {:?}", pilha);
if let Some(valor) = pilha.pop() { //Desempilhar (pop)
    println!("Removido da pilha: {}", valor);

    }
    println!("Pilha após remoção: {:?}", pilha);
}