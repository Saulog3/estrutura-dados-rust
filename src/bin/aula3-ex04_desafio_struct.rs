
struct Livro {
    titulo: String,
    autor: String,
    pagina: u32,
}

fn main() {
    let livros = Livro{
        titulo: String::from("O assassinato"),
        autor: String::from("Paulo Rangel"),
        pagina: 20,
    };
println!("O livro {} do autor {} tem {} paginas", livros.titulo, livros.autor, livros.pagina)
}