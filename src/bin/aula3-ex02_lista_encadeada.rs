use std::collections::LinkedList;

fn main() {
    let mut lista:LinkedList<i32> = LinkedList::new();
    //inserindo valores
    lista.push_back(10); //entra no fim
    lista.push_back(20); //entra no fim
    lista.push_front(5); //entra no começo

    println!("Lista depois das inserções: {:?}", lista);

    lista.pop_back();
    lista.pop_front();

    println!("Lista depois das remoções: {:?}", lista)
}