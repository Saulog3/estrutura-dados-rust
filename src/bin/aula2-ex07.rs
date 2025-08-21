// Fila completa em Rust
use std::collections::VecDeque;
fn main() {
    let mut fila: VecDeque<i32> = VecDeque::new();
    // Enfileirar (push)
    fila.push_back(10);
    fila.push_back(20);
    fila.push_back(30);

    println!("Fila inicial: {:?}", fila);
    // Desenfileirar (pop)
    if let Some(valor) = fila.pop_front() {
        println!("Removido da fila: {}", valor);
    }
    println!("Fila após remoção: {:?}", fila);
    
}