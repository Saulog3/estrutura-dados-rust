use std::collections::VecDeque;

fn main() {
    println!("Exercício: Filas (Queue)");

    let mut fila: VecDeque<i32> = VecDeque::new();

    fila.push_back(1);
    fila.push_back(2);
    fila.push_back(3);

    println!("Fila: {:?}", fila);

    while let Some(valor) = fila.pop_front() {
        println!("Saiu da fila: {}", valor);
    }
}
