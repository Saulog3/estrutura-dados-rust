fn main(){
    let matriz:Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 0],
    ];
    println!("Matriz {:?}", matriz);

    let valor = matriz[1][2]; //

println!("Elemento na linha 2, coluna 0: {}", valor);
}
