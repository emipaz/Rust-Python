//! Variables y tipos en Rust
//! En este programa se muestra cómo declarar variables, constantes y tipos de datos en Rust.

/// importamos la biblioteca de entrada/salida para poder leer datos del usuario
/// También importamos el trait Write para poder usar la función flush() 
/// y asegurarnos de que el mensaje se imprima antes de esperar la entrada del usuario.
use std::io::{self, Write}; 

// Constante que representa el saludo base.
/// 
/// Se utiliza en la función [`saludo`] para construir el mensaje.
const SALUDO: &str = "Hola";

/// Constante que representa el mensaje de solicitud de nombre.
/// 
/// Se pasa como argumento a la función [`input`].
const PEDIR_NOMBRE: &str = "Ingrese su nombre: ";


/// Devuelve un saludo personalizado utilizando la constante [`SALUDO`].
///
/// # Argumentos
///
/// * `nombre` - Una referencia a un string con el nombre del usuario.
///
/// # Ejemplo
/// ```
/// let mensaje = saludo("Emi");
/// assert_eq!(mensaje, "Hola Emi");
/// ``
fn saludo(nombre: &str) -> String {
    
    return format!("{} {}", SALUDO, nombre);
}

// Solicita entrada al usuario mostrando un mensaje en consola.
///
/// El prompt se imprime en la misma línea (sin salto de línea) y se fuerza
/// el `flush` para que aparezca inmediatamente antes de leer.
///
/// # Argumentos
///
/// * `prompt` - Mensaje que se muestra al usuario.
///
/// # Retorna
///
/// Un `String` con la entrada del usuario, sin espacios iniciales ni finales.
///
/// # Ejemplo
/// ```
/// let nombre = input("Ingrese su nombre: ");
/// println!("Hola {}", nombre);
///
fn input(prompt: &str) -> String {

    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Error al leer la entrada"); 
    
    return input.trim().to_string(); }   

/// Función principal del programa.
///
/// - Declara variables de temperatura.
/// - Solicita el nombre del usuario.
/// - Muestra un saludo y las temperaturas.
/// - Actualiza la temperatura actual sumando un valor decimal.

fn main() {
    
    let     temperatura_minima : i8 = -5;  // inmutable 
    
    let     temperatura_maxima : i8 = 50;  // inmutable
    
    let mut temperatura_actual : f32 = 10.0; // mutable
    
    let nombre = input(PEDIR_NOMBRE); 

    println!("{}", saludo(&nombre.trim())); 
    
    println!("La temperatura minima es: {}", temperatura_minima);
    println!("La temperatura maxima es: {}", temperatura_maxima);
    println!("La temperatura actual es: {}", temperatura_actual);

    temperatura_actual += 20.5; // temperatura_actual = temperatura_actual + 20;
    
    println!("La temperatura actual es: {}", temperatura_actual);
}