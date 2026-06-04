fn main() {
    let x = 5;
    let y = &x; // y es una referencia a x
    println!("Valor de x: {}", x); // 5
    println!("Valor de y: {}", y); // 5, pero a través de la referencia

    std::mem::drop(x); // ERROR: no se puede usar x después de esto, pero y sigue siendo válido

    println!("Valor de y después de drop: {}", y); // 5, la referencia sigue siendo válida
}
