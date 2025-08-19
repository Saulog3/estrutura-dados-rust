// Fila em Rust
fn main() {
    let mut fila: Vec<i32> = Vec::new();
    // Enfileirar (push)
    fila.push(1);
    fila.push(2);
    fila.push(3);

    println!("Fila: {:?}", fila);

    // Desenfileirar (pop)
    if !fila.is_empty() {
        let valor = fila.remove(0);
        println!("Removido da fila: {}", valor);
        println!("Fila após remoção: {:?}", fila);

    }
}