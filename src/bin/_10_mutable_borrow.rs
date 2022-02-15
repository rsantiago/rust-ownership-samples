/*
    "emprestando objetos" - de forma mutavel
    Read-write reference
    Mutable borrows

    Este código funciona? Por que?
*/
fn main() {
    let mut a = "Esta string é constante".to_string();

    println!("a: {}\n", a);

    imprime(&mut a);

    println!("a: {}\n", a);
}

fn imprime(uma_string: &mut String) {

    uma_string.push_str(" tentando adicionar mais coisas à string");

    println!("tomou emprestado para dentro da funcao: {}\n", uma_string);
}