/*
    "emprestando objetos" - de forma imutavel
    Read-only reference
    Immutable borrows

    Por que a linha 21 não compila?
*/
fn main() {
    let a = "Esta string é constante".to_string();

    println!("a: {}\n", a);

    imprime(&a);

    println!("a: {}\n", a);
}

fn imprime(uma_string: &String) {

    // por que esta linha nao compila?
    uma_string.push_str(" tentando adicionar mais coisas à string");

    println!("tomou emprestado para dentro da funcao: {}\n", uma_string);
}