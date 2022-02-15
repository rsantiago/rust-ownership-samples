/* Passando constantes entre variáveis imutáveis */
fn main() {
    let a = "Esta string é constante";
    println!("a: {}\n", a);

    let b = a;

    println!("b: {}", b);
    println!("a: {}\n\n", a);
}
