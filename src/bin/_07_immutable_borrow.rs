/*
    "emprestando objetos" - de forma imutavel
    Read-only reference
    Immutable borrows

    Este código funciona?
*/
fn main() {
    let a = "Esta string é constante".to_string();

    println!("a: {}\n", a);

    imprime(&a);

    println!("a: {}\n", a);
}

// observacoes -- agora a propriedade nao mudou, ela foi 'tomada emprestada'
fn imprime(uma_string: &String) {
    println!("tomou emprestado para dentro da funcao: {}\n", uma_string);
}