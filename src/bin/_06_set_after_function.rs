/*
    Tentando atribuir valor a partir de uma variável
    que não tem mais a propriedade (ownership) do objeto

    Por que este código está quebrado?
*/
fn main() {
    let a = "Esta string é constante".to_string();

    println!("a: {}\n", a);

    imprime(a);

    let b = a; // valor movido para funcao
}

fn imprime(uma_string: String) {
    println!("dentro da funcao: {}", uma_string);
}

// STACK                             HEAP
// uma_string -------------------->  "Esta string é uma constante."