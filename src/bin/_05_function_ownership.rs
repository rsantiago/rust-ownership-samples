/*
    Passando ownership para funções

    Este código funciona?
*/
fn main() {
    let a = "Esta string é constante".to_string();

    println!("a: {}\n", a);

    imprime(a);

    println!("a: {}", a);
}

// observações - o ownership mudou para a funcao
// string é liberada da heap após a função sair de escopo
fn imprime(uma_string: String) {
    println!("dentro da funcao: {}", uma_string);
}