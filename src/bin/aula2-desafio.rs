fn main() {
    // Matriz com os preços: cada linha = [preço normal, preço desconto]
    let precos = vec![
        vec![15.0, 12.5],
        vec![13.0, 7.5],
        vec![100.0, 97.0],
    ];

    // Vetor para armazenar as médias
    let mut medias: Vec<f64> = Vec::new();

    // Calcular a média de cada linha
    for linha in &precos {
        let soma: f64 = linha.iter().sum();
        let media = soma / linha.len() as f64;
        medias.push(media);
    }

    // Exibir o vetor de médias
    println!("{:?}", medias);
}