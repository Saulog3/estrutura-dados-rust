use std::collections::VecDeque;
fn main() {
    let mut deque: VecDeque<i32> = VecDeque::new();
    //Inserindo elementos no final
    deque.push_back(10);
    deque.push_back(20);
    //Inserindo elementos no início
    deque.push_front(5);
    println!("{:?}", deque);

    println!("{:?}", deque.pop_front()); //Saida: Some 10
    println!("{:?}", deque.pop_back()); //Saida: Some 5
    println!("{:?} Após a remoção", deque);
}