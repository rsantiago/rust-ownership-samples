/* Simples demonstração de atribuição de tipos primitivos */
fn main() {
    let a = 5;
    println!("a: {}\n", a);

    let b = a;
    let c = a;

    println!("b: {}", b);
    println!("c: {}", c);
    println!("a: {}\n", a);

    // valor passado por cópia na stack
    let sum_result = sum(a, b, c);
    println!("sum_result: {}", sum_result);

    println!("a again: {}", a);
    println!("b again: {}", b);
    println!("c again: {}\n", c);
}

fn sum(mut x: i32, mut y: i32, mut z: i32) -> i32 {
    let k = x;
    let l = y;
    let m = z;

    x = 1;
    y = 2;
    z = 3;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}\n", z);

    return k + l + m;
}