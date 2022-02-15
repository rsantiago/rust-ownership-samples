/*
    "emprestando objetos" - de forma mutavel
    Read-write reference
    Mutable borrows
    Por que este código funciona?

    see more @ https://stackoverflow.com/questions/50251487/what-are-non-lexical-lifetimes
*/
fn main() {
    let mut a = "Esta string é constante?".to_string();
    println!("a: {}\n", a);

    let b = &mut a;
    b.push_str(" Não é constante.");

    let c = &mut a;

    /* O que acontece se esta linha for descomentada? */
    // b.push_str("Será que dá pra adicionar mais coisas?");

    println!("c: {}",c);
}
