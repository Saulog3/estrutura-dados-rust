fn main() {
    
    let nome = "Jõao";
    println!("Olá, {}!", nome);
    for i in 1..6{
        println!("Número: {}", i);
    }
    
    let mut contador = 0;
    while contador < 5{
        println!("Contador: {}", contador);
        contador += 1;
    }
    
    let numeros = vec![10, 20, 30, 40];
    println!("{:?}", numeros);
    
    let mut nomes = Vec::new(); //cria um vetor vazio
    nomes.push("Ana");
    nomes.push("Bruno");
    nomes.push("Carlos");
    println!("{:?}", nomes);
    
    let frutas = vec!["Maça", "Banana", "Uva"];
    println!("Primeira fruta {}", frutas[0]);
    println!("Segunda fruta {}", frutas[1]);
    
    let mut numerosb = vec![1, 2, 3];
    numerosb[1] = 10; // altera o valor da posição 1
    println!("{:?}", numerosb);
    
    let mut numeros = vec![10, 20, 30];
    numeros.pop(); // remove o último
    println!("{:?}", numeros);
    
    let nomes = vec!["Ana", "Bruno", "Carlos"];
    for nome in &nomes{
        println!("Olá, {}", nome);
    }
}