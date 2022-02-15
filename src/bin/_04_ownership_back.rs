/*
    Ownership - passando valor de volta entre variávies e lendo seus valores
    Este código funciona?
*/
fn main() {
    let mut a = "Esta string é constante".to_string();

    println!("a: {}\n", a);

    let b = a;

    println!("b: {}", b);

    a = b; // observacoes -- o ownership voltou para a primeira variavel
    println!("a: {}\n\n", a);

}