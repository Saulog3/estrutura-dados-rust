fn main(){
    let nomes = vec!["Sarah", "saulo", "Kleber", "Miguel"];
    for nome in &nomes {
        println!("Olá {}", nome);
    }
}
