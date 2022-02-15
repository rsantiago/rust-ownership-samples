/*
    Conceito de Ownership
    Passando objetos na heap entre variáveis

    Será que este código funciona?
*/
fn main() {
    let a = "Esta string é constante?"

        // criando um objeto na heap
        .to_string();

    println!("a: {}\n", a);

    let mut b = a;
    b.push_str(" Não, não é constante.");
    println!("b: {}", b);

    // observações - strings estão na heap, e o que tem na heap, tem dono
    // o ownership mudou, nao da pra saber o que o dono vai fazer com o objeto
    println!("a: {}\n\n", a);
}