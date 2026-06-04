# Guía de Maturin: Librerías Python escritas en Rust

> Cómo crear, compilar y publicar extensiones Python de alto rendimiento usando Maturin y PyO3 — Marzo 2026

---

## 1. ¿Qué es Maturin?

**Maturin** es una herramienta de build que permite compilar código Rust y empaquetarlo como una librería Python nativa (`.pyd` en Windows, `.so` en Linux/macOS). Internamente usa **PyO3**, el binding entre Rust y Python.

El flujo es simple:

```
Código Rust (PyO3)  →  maturin build  →  wheel (.whl)  →  pip install
```

Casos de uso ideales:
- Funciones computacionalmente intensivas (procesamiento de datos, algoritmos, parsers)
- Reemplazar cuellos de botella en código Python existente
- Librerías con requisitos estrictos de rendimiento o seguridad de memoria

---

## 2. Instalación y requisitos

### Requisitos previos

- Python 3.8+
- Rust (instalar desde [rustup.rs](https://rustup.rs))
- pip

### Instalar Maturin

```bash
pip install maturin
```

O con `uv` (recomendado):

```bash
uv tool install maturin
```

Verificar instalación:

```bash
maturin --version
rustc --version
```

---

## 3. Crear un proyecto nuevo

```bash
maturin new mi_libreria
cd mi_libreria
```

Maturin preguntará el tipo de binding. Elegí **pyo3** (la opción más común):

```
? Which kind of bindings to use?
  ❯ pyo3
    cffi
    uniffi
    bin
```

Los otros bindings que ofrece Maturin son:

- cffi — C Foreign Function Interface. En lugar de usar PyO3 (que es específico de Python), genera una interfaz C estándar. Es útil si querés que tu librería Rust sea usable también desde otros lenguajes (Ruby, Julia, etc.), no solo Python. Más portable pero más verboso y sin las comodidades de PyO3.
- uniffi — Desarrollado por Mozilla. Permite escribir el código Rust una sola vez y generar bindings automáticamente para múltiples lenguajes: Python, Kotlin, Swift y Go. Es ideal si estás construyendo una librería de lógica de negocio compartida entre una app mobile (Android/iOS) y un backend Python, por ejemplo. Mozilla lo usa internamente para Firefox y sus apps.
- bin — No genera una librería sino un ejecutable binario. Es útil cuando querés distribuir una herramienta de línea de comandos escrita en Rust como si fuera un paquete Python instalable con pip, sin que el usuario necesite tener Rust instalado. Por ejemplo, herramientas como ruff (el linter de Python) están distribuidas así.

En resumen: para el caso típico de "quiero acelerar código Python con Rust", PyO3 es casi siempre la opción correcta. Los otros tienen sentido en escenarios más específicos de distribución multiplataforma o multi-lenguaje.

### Estructura generada

```
mi_libreria/
├── Cargo.toml          # Configuración de Rust
├── pyproject.toml      # Configuración de Python/pip
└── src/
    └── lib.rs          # Tu código Rust
```

### Cargo.toml

```toml
[package]
name = "mi_libreria"
version = "0.1.0"
edition = "2021"

[lib]
name = "mi_libreria"
crate-type = ["cdylib"]  # necesario para generar .so / .pyd

[dependencies]
pyo3 = { version = "0.22", features = ["extension-module"] }
```

### pyproject.toml

```toml
[build-system]
requires = ["maturin>=1.7,<2.0"]
build-backend = "maturin"

[project]
name = "mi_libreria"
requires-python = ">=3.8"
classifiers = [
    "Programming Language :: Rust",
    "Programming Language :: Python :: Implementation :: CPython",
]

[tool.maturin]
features = ["pyo3/extension-module"]
```

---

## 4. Tu primer módulo: funciones básicas

### src/lib.rs

```rust
use pyo3::prelude::*;

/// Suma dos enteros. Documentación visible desde Python.
#[pyfunction]
fn suma(a: i64, b: i64) -> i64 {
    a + b
}

/// Saluda a una persona por su nombre.
#[pyfunction]
fn saludar(nombre: &str) -> String {
    format!("Hola, {}! Escrito en Rust.", nombre)
}

/// El módulo Python se define aquí.
#[pymodule]
fn mi_libreria(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(suma, m)?)?;
    m.add_function(wrap_pyfunction!(saludar, m)?)?;
    Ok(())
}
```

### Compilar y probar en modo desarrollo

```bash
maturin develop
```

Esto compila e instala la librería en el entorno Python activo, sin crear un wheel.

### Usar desde Python

```python
import mi_libreria

print(mi_libreria.suma(3, 4))       # 7
print(mi_libreria.saludar("Ana"))   # Hola, Ana! Escrito en Rust.

# El docstring de Rust también es visible
help(mi_libreria.suma)
```

---

## 5. Tipos de datos: Python ↔ Rust

PyO3 convierte automáticamente los tipos más comunes:

| Python          | Rust (PyO3)              |
|-----------------|--------------------------|
| `int`           | `i32, i64, u64, isize`   |
| `float`         | `f32, f64`               |
| `bool`          | `bool`                   |
| `str`           | `&str, String`           |
| `bytes`         | `&[u8], Vec<u8>`         |
| `list`          | `Vec<T>`                 |
| `dict`          | `HashMap<K, V>`          |
| `tuple`         | `(T1, T2, ...)`          |
| `None`          | `Option<T>`              |
| `Exception`     | `PyErr`                  |
| objeto Python   | `PyObject / Py<T>`       |

### Ejemplo con varios tipos

```rust
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyfunction]
fn procesar_lista(numeros: Vec<f64>) -> f64 {
    numeros.iter().sum()
}

#[pyfunction]
fn contar_palabras(texto: &str) -> HashMap<String, usize> {
    let mut mapa = HashMap::new();
    for palabra in texto.split_whitespace() {
        *mapa.entry(palabra.to_lowercase()).or_insert(0) += 1;
    }
    mapa
}

#[pyfunction]
fn valor_opcional(x: Option<i64>) -> String {
    match x {
        Some(n) => format!("Recibí: {}", n),
        None    => "No recibí nada".to_string(),
    }
}
```

Desde Python:

```python
import mi_libreria

mi_libreria.procesar_lista([1.5, 2.5, 3.0])   # 7.0
mi_libreria.contar_palabras("hola mundo hola") # {'hola': 2, 'mundo': 1}
mi_libreria.valor_opcional(None)               # 'No recibí nada'
mi_libreria.valor_opcional(42)                 # 'Recibí: 42'
```

---

## 6. Clases Python desde Rust

Podés exponer structs de Rust como clases Python usando `#[pyclass]`.

```rust
use pyo3::prelude::*;

#[pyclass]
struct Contador {
    valor: i64,
    nombre: String,
}

#[pymethods]
impl Contador {
    /// Constructor: Contador(nombre, valor_inicial=0)
    #[new]
    #[pyo3(signature = (nombre, valor_inicial=0))]
    fn new(nombre: String, valor_inicial: i64) -> Self {
        Contador { valor: valor_inicial, nombre }
    }

    fn incrementar(&mut self) {
        self.valor += 1;
    }

    fn incrementar_en(&mut self, n: i64) {
        self.valor += n;
    }

    fn reset(&mut self) {
        self.valor = 0;
    }

    /// Getter: accesible como propiedad en Python
    #[getter]
    fn valor(&self) -> i64 {
        self.valor
    }

    #[getter]
    fn nombre(&self) -> &str {
        &self.nombre
    }

    /// __repr__ para Python
    fn __repr__(&self) -> String {
        format!("Contador(nombre='{}', valor={})", self.nombre, self.valor)
    }
}

#[pymodule]
fn mi_libreria(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Contador>()?;
    Ok(())
}
```

Desde Python:

```python
from mi_libreria import Contador

c = Contador("visitas", 10)
c.incrementar()
c.incrementar_en(5)
print(c.valor)   # 16
print(c)         # Contador(nombre='visitas', valor=16)
c.reset()
print(c.valor)   # 0
```

---

## 7. Manejo de errores

Podés lanzar excepciones Python desde Rust usando `PyErr` y los tipos de error de PyO3.

```rust
use pyo3::prelude::*;
use pyo3::exceptions::{PyValueError, PyZeroDivisionError};

#[pyfunction]
fn dividir(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        return Err(PyZeroDivisionError::new_err("No se puede dividir por cero"));
    }
    Ok(a / b)
}

#[pyfunction]
fn raiz(x: f64) -> PyResult<f64> {
    if x < 0.0 {
        return Err(PyValueError::new_err(
            format!("No se puede calcular la raíz de {}", x)
        ));
    }
    Ok(x.sqrt())
}
```

Desde Python, se comportan como excepciones normales:

```python
import mi_libreria

try:
    mi_libreria.dividir(10.0, 0.0)
except ZeroDivisionError as e:
    print(e)  # No se puede dividir por cero

try:
    mi_libreria.raiz(-4.0)
except ValueError as e:
    print(e)  # No se puede calcular la raíz de -4
```

### Tipos de error disponibles en PyO3

```rust
use pyo3::exceptions::{
    PyValueError,
    PyTypeError,
    PyRuntimeError,
    PyIOError,
    PyIndexError,
    PyKeyError,
    PyZeroDivisionError,
    PyOverflowError,
};
```

---

## 8. Trabajar con NumPy (numpy + PyO3)

Para trabajar con arrays NumPy desde Rust, usá la crate `numpy`.

### Cargo.toml

```toml
[dependencies]
pyo3 = { version = "0.22", features = ["extension-module"] }
numpy = "0.22"
```

### src/lib.rs

```rust
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Multiplica cada elemento de un array por un escalar
#[pyfunction]
fn escalar<'py>(
    py: Python<'py>,
    array: PyReadonlyArray1<'py, f64>,
    factor: f64,
) -> Bound<'py, PyArray1<f64>> {
    let resultado: Vec<f64> = array
        .as_array()
        .iter()
        .map(|x| x * factor)
        .collect();
    resultado.into_pyarray(py)
}

/// Suma de todos los elementos
#[pyfunction]
fn suma_array(array: PyReadonlyArray1<'_, f64>) -> f64 {
    array.as_array().iter().sum()
}

#[pymodule]
fn mi_libreria(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(escalar, m)?)?;
    m.add_function(wrap_pyfunction!(suma_array, m)?)?;
    Ok(())
}
```

Desde Python:

```python
import numpy as np
import mi_libreria

arr = np.array([1.0, 2.0, 3.0, 4.0])
print(mi_libreria.escalar(arr, 3.0))   # [3. 6. 9. 12.]
print(mi_libreria.suma_array(arr))     # 10.0
```

---

## 9. Paralelismo con Rayon

Una de las grandes ventajas de Rust es la concurrencia segura. La crate `rayon` permite paralelizar iteradores con un cambio mínimo de código.

### Cargo.toml

```toml
[dependencies]
pyo3 = { version = "0.22", features = ["extension-module"] }
rayon = "1.10"
```

### src/lib.rs

```rust
use pyo3::prelude::*;
use rayon::prelude::*;

/// Versión secuencial
#[pyfunction]
fn suma_cuadrados(numeros: Vec<f64>) -> f64 {
    numeros.iter().map(|x| x * x).sum()
}

/// Versión paralela con rayon (útil para listas grandes)
#[pyfunction]
fn suma_cuadrados_paralelo(numeros: Vec<f64>) -> f64 {
    numeros.par_iter().map(|x| x * x).sum()
}
```

Desde Python:

```python
import mi_libreria

datos = list(range(10_000_000))

# secuencial
resultado = mi_libreria.suma_cuadrados(datos)

# paralelo (usa todos los núcleos disponibles)
resultado = mi_libreria.suma_cuadrados_paralelo(datos)
```

> **Nota:** Para liberar el GIL de Python durante operaciones paralelas largas, usá `py.allow_threads(|| { ... })`.

---

## 10. Modo desarrollo vs build de producción

### Desarrollo (rápido, sin optimizaciones)

```bash
maturin develop
```

### Desarrollo con optimizaciones

```bash
maturin develop --release
```

### Build de producción (genera un `.whl`)

```bash
maturin build --release
```

El wheel quedará en `target/wheels/`. Para instalarlo:

```bash
pip install target/wheels/mi_libreria-*.whl
```

### Entornos virtuales

Maturin detecta automáticamente el entorno virtual activo. Se recomienda:

```bash
python -m venv .venv
source .venv/bin/activate   # Linux/macOS
# .venv\Scripts\activate    # Windows

maturin develop
```

---

## 11. Publicar en PyPI

### Prerequisitos

```bash
pip install twine  # opcional, maturin puede publicar directamente
```

### Build para múltiples plataformas (con manylinux)

```bash
# Requiere Docker instalado
maturin build --release --manylinux auto
```

### Publicar directamente con Maturin

```bash
maturin publish
```

Maturin pedirá tus credenciales de PyPI (o usará un token de `MATURIN_PYPI_TOKEN`).

### Con GitHub Actions (CI/CD)

```yaml
# .github/workflows/publish.yml
name: Publish

on:
  push:
    tags: ['v*']

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: PyO3/maturin-action@v1
        with:
          command: publish
          args: --release
        env:
          MATURIN_PYPI_TOKEN: ${{ secrets.PYPI_API_TOKEN }}
```

---

## 12. Ejemplo completo: librería de procesamiento de texto

Un ejemplo real que integra todo lo anterior: una librería para contar palabras, limpiar texto y calcular estadísticas, exponiendo funciones y una clase a Python.

### src/lib.rs

```rust
use pyo3::prelude::*;
use std::collections::HashMap;

// ── Funciones ──────────────────────────────────────────────

#[pyfunction]
fn limpiar(texto: &str) -> String {
    texto
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
}

#[pyfunction]
fn contar_palabras(texto: &str) -> HashMap<String, usize> {
    let mut mapa = HashMap::new();
    for palabra in texto.split_whitespace() {
        let limpia = limpiar(palabra);
        if !limpia.is_empty() {
            *mapa.entry(limpia).or_insert(0) += 1;
        }
    }
    mapa
}

#[pyfunction]
fn palabras_mas_frecuentes(texto: &str, n: usize) -> Vec<(String, usize)> {
    let conteo = contar_palabras(texto);
    let mut pares: Vec<(String, usize)> = conteo.into_iter().collect();
    pares.sort_by(|a, b| b.1.cmp(&a.1));
    pares.truncate(n);
    pares
}

// ── Clase ──────────────────────────────────────────────────

#[pyclass]
struct AnalizadorTexto {
    texto: String,
}

#[pymethods]
impl AnalizadorTexto {
    #[new]
    fn new(texto: String) -> Self {
        AnalizadorTexto { texto }
    }

    fn palabras_unicas(&self) -> usize {
        contar_palabras(&self.texto).len()
    }

    fn total_palabras(&self) -> usize {
        self.texto.split_whitespace().count()
    }

    fn top(&self, n: usize) -> Vec<(String, usize)> {
        palabras_mas_frecuentes(&self.texto, n)
    }

    fn __repr__(&self) -> String {
        format!(
            "AnalizadorTexto({} palabras, {} únicas)",
            self.total_palabras(),
            self.palabras_unicas()
        )
    }
}

// ── Módulo ─────────────────────────────────────────────────

#[pymodule]
fn mi_libreria(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(limpiar, m)?)?;
    m.add_function(wrap_pyfunction!(contar_palabras, m)?)?;
    m.add_function(wrap_pyfunction!(palabras_mas_frecuentes, m)?)?;
    m.add_class::<AnalizadorTexto>()?;
    Ok(())
}
```

Desde Python:

```python
from mi_libreria import AnalizadorTexto, palabras_mas_frecuentes

texto = """
    Rust es rápido. Rust es seguro. Python es popular.
    Python es fácil de aprender. Rust y Python son geniales.
"""

analizador = AnalizadorTexto(texto)
print(analizador)                  # AnalizadorTexto(13 palabras, 10 únicas)
print(analizador.total_palabras()) # 13
print(analizador.palabras_unicas())# 10
print(analizador.top(3))           # [('es', 4), ('rust', 3), ('python', 3)]

print(palabras_mas_frecuentes(texto, 2))  # [('es', 4), ('rust', 3)]
```

---

## 13. Referencia rápida de comandos

| Comando                        | Descripción                                      |
|--------------------------------|--------------------------------------------------|
| `maturin new <nombre>`         | Crear proyecto nuevo                             |
| `maturin develop`              | Compilar e instalar en modo debug                |
| `maturin develop --release`    | Compilar e instalar con optimizaciones           |
| `maturin build --release`      | Generar wheel de producción                      |
| `maturin build --manylinux auto` | Wheel compatible con Linux (requiere Docker)   |
| `maturin publish`              | Publicar en PyPI                                 |
| `maturin list-python`          | Ver intérpretes Python detectados                |

---

## 14. Recursos

- [Documentación de Maturin](https://www.maturin.rs)
- [Guía de PyO3](https://pyo3.rs)
- [Repositorio de Maturin en GitHub](https://github.com/PyO3/maturin)
- [Ejemplos de PyO3](https://github.com/PyO3/pyo3/tree/main/examples)
- [Crate numpy para PyO3](https://github.com/PyO3/rust-numpy)

---

*Maturin >= 1.7 | PyO3 >= 0.22 | Rust Edition 2021*