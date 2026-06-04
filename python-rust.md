# Guía de Migración de Python a Rust

> Conceptos, patrones y equivalencias prácticas — Marzo 2026

---

## 1. Introducción

Rust y Python son lenguajes con filosofías muy distintas. Python prioriza la legibilidad y rapidez de desarrollo, mientras que Rust prioriza el rendimiento, la seguridad de memoria y la concurrencia sin condiciones de carrera.

Esta guía cubre los patrones más comunes que encontrarás al migrar código Python a Rust, con ejemplos lado a lado para facilitar la transición.

---

## 2. Tipos de Datos Básicos

### 2.1 Tipos primitivos

| Descripción   | Python          | Rust                      |
|---------------|-----------------|---------------------------|
| Entero        | `int`           | `i32, i64, u32, u64`      |
| Flotante      | `float`         | `f32, f64`                |
| Booleano      | `bool`          | `bool`                    |
| Cadena        | `str / str`     | `str / String`            |
| Carácter      | (no existe)     | `char`                    |
| Ninguno/Nulo  | `None`          | `Option<T> = None`        |

### 2.2 Variables y mutabilidad

En Python todas las variables son mutables por defecto. En Rust, las variables son **inmutables por defecto** y debes usar `mut` para hacerlas mutables.

**Python**
```python
x = 10
x = 20  # OK, siempre mutable

nombre = 'Ana'
nombre = 'Luis'  # OK
```

**Rust**
```rust
let x = 10;
// x = 20; // ERROR: inmutable
let mut x = 10;
x = 20;  // OK con mut

let mut nombre = String::from("Ana");
nombre = String::from("Luis");
```

---

## 3. Colecciones

### 3.1 Listas / Vectores

**Python — `list`**
```python
mi_lista = [1, 2, 3]
mi_lista.append(4)
mi_lista[0]   # => 1
len(mi_lista) # => 4
for x in mi_lista:
    ...
```

**Rust — `Vec<T>`**
```rust
let mut mi_vec: Vec<i32> = vec![1, 2, 3];
mi_vec.push(4);
mi_vec[0]   // => 1
mi_vec.len() // => 4
for x in &mi_vec {
    ...
}
```

### 3.2 Diccionarios / HashMap

**Python — `dict`**
```python
d = {'a': 1, 'b': 2}
d['c'] = 3
d.get('x', 0)  # 0 si no existe
for k, v in d.items():
    ...
```

**Rust — `HashMap`**
```rust
use std::collections::HashMap;
let mut d = HashMap::new();
d.insert("a", 1);
d.get("x").copied().unwrap_or(0);
for (k, v) in &d {
    ...
}
```

### 3.3 Tuplas

**Python**
```python
t = (1, 'hola', 3.14)
x, y, z = t  # desestructurar
t[0]  # => 1
```

**Rust**
```rust
let t = (1, "hola", 3.14);
let (x, y, z) = t;  // desestructurar
t.0  // => 1
```

---

## 4. Control de Flujo

### 4.1 Condicionales

**Python**
```python
if x > 0:
    print('positivo')
elif x == 0:
    print('cero')
else:
    print('negativo')
```

**Rust**
```rust
if x > 0 {
    println!("positivo");
} else if x == 0 {
    println!("cero");
} else {
    println!("negativo");
}
```

### 4.2 Bucles

**Python**
```python
# for con rango
for i in range(10):
    print(i)

# while
while condicion:
    ...
```

**Rust**
```rust
// for con rango
for i in 0..10 {
    println!("{}", i);
}

// while
while condicion {
    ...
}
```

### 4.3 match (equivalente a match/case de Python 3.10+)

**Python (3.10+)**
```python
match valor:
    case 1:
        print('uno')
    case 2 | 3:
        print('dos o tres')
    case _:
        print('otro')
```

**Rust**
```rust
match valor {
    1 => println!("uno"),
    2 | 3 => println!("dos o tres"),
    _ => println!("otro"),
}
```

---

## 5. Funciones

### 5.1 Definición básica

**Python**
```python
def suma(a: int, b: int) -> int:
    return a + b

resultado = suma(3, 4)  # 7
```

**Rust**
```rust
fn suma(a: i32, b: i32) -> i32 {
    a + b  // sin return ni punto y coma
}

let resultado = suma(3, 4);  // 7
```

### 5.2 Closures (lambdas)

**Python**
```python
doble = lambda x: x * 2
cuadrado = lambda x: x ** 2

numeros = [1, 2, 3, 4]
dobles = list(map(doble, numeros))
```

**Rust**
```rust
let doble = |x| x * 2;
let cuadrado = |x: i32| x.pow(2);

let numeros = vec![1, 2, 3, 4];
let dobles: Vec<i32> = numeros.iter()
    .map(|x| x * 2)
    .collect();
```

---

## 6. Manejo de Errores

Este es uno de los cambios conceptuales más importantes. Python usa excepciones; Rust usa tipos de retorno `Result<T, E>` y `Option<T>`.

### 6.1 try/except vs Result

**Python**
```python
try:
    resultado = dividir(10, 0)
except ZeroDivisionError as e:
    print(f'Error: {e}')
except Exception as e:
    print(f'Error inesperado: {e}')
```

**Rust**
```rust
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("División por cero".to_string())
    } else {
        Ok(a / b)
    }
}

match dividir(10.0, 0.0) {
    Ok(r)  => println!("Resultado: {}", r),
    Err(e) => println!("Error: {}", e),
}
```

### 6.2 Option\<T\> en lugar de None

**Python**
```python
def buscar(lista, valor):
    for i, x in enumerate(lista):
        if x == valor:
            return i
    return None  # si no se encuentra

indice = buscar([1, 2, 3], 5)
if indice is not None:
    print(indice)
```

**Rust**
```rust
fn buscar(lista: &[i32], valor: i32) -> Option<usize> {
    lista.iter().position(|&x| x == valor)
}

let indice = buscar(&[1, 2, 3], 5);
if let Some(i) = indice {
    println!("{}", i);
}
```

### 6.3 El operador `?` (propagación de errores)

El operador `?` en Rust es equivalente al patrón común de Python de relanzar excepciones hacia arriba en la pila de llamadas.

**Python**
```python
def leer_numero(path):
    with open(path) as f:
        contenido = f.read()   # lanza IOError
    return int(contenido)      # lanza ValueError
```

**Rust**
```rust
use std::fs;

fn leer_numero(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let contenido = fs::read_to_string(path)?;  // propaga error
    let numero: i32 = contenido.trim().parse()?; // propaga error
    Ok(numero)
}
```

---

## 7. Clases y Structs

### 7.1 Definición básica

**Python — `class`**
```python
class Persona:
    def __init__(self, nombre: str, edad: int):
        self.nombre = nombre
        self.edad = edad

    def saludar(self) -> str:
        return f'Hola, soy {self.nombre}'

p = Persona('Ana', 30)
print(p.saludar())
```

**Rust — `struct` + `impl`**
```rust
struct Persona {
    nombre: String,
    edad: u32,
}

impl Persona {
    fn nuevo(nombre: &str, edad: u32) -> Self {
        Persona { nombre: nombre.to_string(), edad }
    }

    fn saludar(&self) -> String {
        format!("Hola, soy {}", self.nombre)
    }
}

let p = Persona::nuevo("Ana", 30);
println!("{}", p.saludar());
```

### 7.2 Herencia vs Traits

Python usa herencia de clases. Rust no tiene herencia; en cambio usa **traits** (similares a interfaces o protocolos abstractos).

**Python — herencia**
```python
class Animal:
    def hablar(self) -> str:
        raise NotImplementedError

class Perro(Animal):
    def hablar(self) -> str:
        return 'Guau!'

class Gato(Animal):
    def hablar(self) -> str:
        return 'Miau!'
```

**Rust — traits**
```rust
trait Animal {
    fn hablar(&self) -> &str;
}

struct Perro;
impl Animal for Perro {
    fn hablar(&self) -> &str { "Guau!" }
}

struct Gato;
impl Animal for Gato {
    fn hablar(&self) -> &str { "Miau!" }
}
```

---

## 8. Ownership y Préstamos

Este es el concepto más importante y diferente de Rust. **No existe en Python.** El sistema de ownership garantiza seguridad de memoria sin necesidad de un recolector de basura.

### 8.1 Reglas de Ownership

- Cada valor tiene exactamente un propietario (owner).
- Cuando el propietario sale del scope, el valor se libera.
- Solo puede haber un propietario a la vez (mover o prestar).

### 8.2 Move vs Clone

**Python — siempre referencias**
```python
lista1 = [1, 2, 3]
lista2 = lista1   # comparten referencia
lista2.append(4)
print(lista1)     # [1, 2, 3, 4] !!
```

**Rust — move semantics**
```rust
let lista1 = vec![1, 2, 3];
let lista2 = lista1;  // MOVE: lista1 ya no es válida
// println!("{:?}", lista1);  // ERROR!

// Para copiar, usar .clone()
let lista1 = vec![1, 2, 3];
let lista2 = lista1.clone();  // copia profunda
```

### 8.3 Referencias y Borrowing

**Python — no hay equivalente directo**
```python
def imprimir(v):
    print(v)  # recibe referencia automáticamente

lista = [1, 2, 3]
imprimir(lista)  # lista sigue siendo válida
```

**Rust — referencias explícitas**
```rust
fn imprimir(v: &Vec<i32>) {  // recibe préstamo
    println!("{:?}", v);
}

let lista = vec![1, 2, 3];
imprimir(&lista);             // presta sin mover
println!("{:?}", lista);      // sigue siendo válida
```

---

## 9. Iteradores y Programación Funcional

Rust tiene un sistema de iteradores muy poderoso, equivalente a las list comprehensions y funciones de Python.

### 9.1 Map, Filter, Fold

**Python**
```python
numeros = [1, 2, 3, 4, 5, 6]

# filter + map
pares_dobles = [x * 2 for x in numeros if x % 2 == 0]

# reduce (suma)
from functools import reduce
total = reduce(lambda a, b: a + b, numeros, 0)
```

**Rust**
```rust
let numeros = vec![1, 2, 3, 4, 5, 6];

// filter + map + collect
let pares_dobles: Vec<i32> = numeros.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * 2)
    .collect();

// fold (equivalente a reduce) / o simplemente .sum()
let total: i32 = numeros.iter().sum();
```

---

## 10. Concurrencia

### 10.1 Threads

**Python — threading (limitado por el GIL)**
```python
import threading

def tarea(nombre):
    print(f'Hola desde {nombre}')

t = threading.Thread(target=tarea, args=('hilo1',))
t.start()
t.join()
```

**Rust — threads reales sin GIL**
```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Hola desde el hilo");
});

handle.join().unwrap();
```

### 10.2 async/await

**Python — asyncio**
```python
import asyncio

async def tarea():
    await asyncio.sleep(1)
    return 42

async def main():
    resultado = await tarea()
    print(resultado)

asyncio.run(main())
```

**Rust — tokio**
```rust
use std::time::Duration;

#[tokio::main]
async fn main() {
    let resultado = tarea().await;
    println!("{}", resultado);
}

async fn tarea() -> i32 {
    tokio::time::sleep(Duration::from_secs(1)).await;
    42
}
```

---

## 11. Tabla Resumen de Equivalencias

| Concepto            | Python                    | Rust                        |
|---------------------|---------------------------|-----------------------------|
| Lista dinámica      | `list`                    | `Vec<T>`                    |
| Mapa clave-valor    | `dict`                    | `HashMap<K, V>`             |
| Conjunto            | `set`                     | `HashSet<T>`                |
| Valor opcional      | `None / valor`            | `Option<T>`                 |
| Manejo de errores   | `try/except`              | `Result<T, E>`              |
| Inmutabilidad       | no por defecto            | `let` (por defecto)         |
| Clase               | `class`                   | `struct + impl`             |
| Interfaz            | `ABC / Protocol`          | `trait`                     |
| Lambda              | `lambda x: x * 2`        | `\|x\| x * 2`               |
| Comprensión         | `[x for x in ...]`       | `iter().map().collect()`    |
| Concurrencia        | `threading / asyncio`     | `thread / tokio async`      |
| Paquetes            | `pip + pyproject.toml`    | `cargo + Cargo.toml`        |
| Formato de cadena   | `f'hola {nombre}'`        | `format!("hola {}", nombre)`|
| Imprimir            | `print()`                 | `println!()`                |

---

## 12. Consejos para la Migración

- Empieza por aprender el sistema de ownership leyendo el [Rust Book](https://doc.rust-lang.org/book).
- Usa `cargo new` para iniciar proyectos y `cargo run` / `cargo test` para ejecutarlos.
- El compilador de Rust es tu mejor amigo: sus mensajes de error son muy descriptivos.
- Usa `clippy` (`cargo clippy`) para sugerencias de estilo y buenas prácticas.
- Para code Python-like rápido, usa `.unwrap()` al principio y mejora el manejo de errores después.
- La crate `rayon` te permite paralelizar iteradores fácilmente, como `Pool` de `multiprocessing`.
- Para scripting rápido, considera `PyO3` para mezclar Python y Rust en el mismo proyecto.

---

*Rust Edition 2024 | `cargo build --release`*