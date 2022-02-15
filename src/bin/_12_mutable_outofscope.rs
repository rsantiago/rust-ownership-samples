/*
    "emprestando objetos" - de forma mutavel
    Read-write reference
    Mutable borrows
    Variaveis entrando e saindo de escopo

    Este código funciona?
*/
fn main() {
    let mut a = "Esta string é constante?".to_string();

    println!("a: {}\n", a);

    {
        let b : &mut String = &mut a;
        b.push_str(" Não, ela é mutável.");
        println!("b: {}", b);

        let c : &String = b;
        b.push_str(" Adicionando mais alguma coisa");

        println!("c: {}", c);
    }

    let d : &String = &b;
    println!("d: {}\n", d);

    let e : &mut String = &mut a;
    e.push_str(" Penultimas palavras");
    a.push_str(" Ultimas palavras");
    println!("e: {}", e);
    println!("a: {}", a);

}
