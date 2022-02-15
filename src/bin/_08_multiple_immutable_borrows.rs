/*
    "emprestando objetos" - de forma imutavel
    Read-only reference
    Immutable borrows

    Este código funciona?
*/
fn main() {
    let a = "Esta string é constante".to_string();

    println!("a: {}\n", a);

    let b : &String = &a;
    let c : &String = b;
    let d : &String = &a;

    // mesmo comportamento de passar uma constante

    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}\n", d);

    println!("a: {}", a);
}