use std::collections::HashMap;

fn main() {
    let mut capitais = HashMap::new();
    capitais.insert("Brasil", "Brasilia");
    capitais.insert("França", "Paris");
    capitais.insert("Japão", "Toquio");

    println!("{:?}", capitais);

    if let Some(capital) = capitais.get("Brasil"){
        println!("A Capital do Brasil é {}", capital);

    }
    capitais.insert("Brasil", "Rio de Janeiro");
    capitais.remove("França");

    for (pais, capital) in &capitais{
        println!("{}->{}", pais, capital);
    }

}
