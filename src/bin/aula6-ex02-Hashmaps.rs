use std::collections::HashMap;

fn main () {
    let mut usuarios: HashMap<u32,(String, i32, f64, bool, char)>=HashMap::new();

    usuarios.insert(1,("Ana".to_string(), 25, 3500.0, true, 'A'));
    usuarios.insert(2,("Brunno".to_string(), 30, 4200.5, false, 'B'));

    if let Some(usuario) = usuarios.get(&1){
        println!("Registro1 => {:?}", usuario);
        println!("Nome: {}, Salario: {}", usuario.0, usuario.2);
        
    }
}
