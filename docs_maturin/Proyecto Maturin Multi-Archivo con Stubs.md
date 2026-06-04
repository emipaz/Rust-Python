# Proyecto Maturin Multi-Archivo con Stubs

> Estructura real, módulos en Rust y generación de stub files (.pyi) — Marzo 2026

---

## 1. La equivalencia Python → Rust

Antes de ver el código, la traducción directa de conceptos:

| Python                        | Rust                                  |
|-------------------------------|---------------------------------------|
| `mi_paquete/` (carpeta)       | `src/` con módulos                    |
| `__init__.py`                 | `src/lib.rs`                          |
| `texto.py`                    | `src/texto.rs`                        |
| `from texto import limpiar`   | `mod texto; use texto::limpiar;`      |
| `__all__ = [...]`             | lo que registrás en `#[pymodule]`     |
| docstring `"""..."""`         | `/// ...` (doc comment de Rust)       |
| `mi_lib.pyi` (stub manual)    | generado con `pyo3-stub-gen`          |

---

## 2. Estructura del proyecto

```
mi_libreria/
├── Cargo.toml
├── pyproject.toml
├── python/
│   └── mi_libreria/
│       ├── __init__.pyi        ← stub del módulo raíz
│       ├── texto.pyi           ← stub del submódulo texto
│       └── numeros.pyi         ← stub del submódulo numeros
└── src/
    ├── lib.rs                  ← arma el módulo Python (como __init__.py)
    ├── texto.rs                ← funciones de texto
    ├── numeros.rs              ← funciones numéricas
    └── utils.rs                ← helpers internos (no expuestos a Python)
```

> `utils.rs` es interno — lo usan `texto.rs` y `numeros.rs` pero no se
> expone directamente a Python, igual que un helper privado en Python.

---

## 3. Cargo.toml

```toml
[package]
name = "mi_libreria"
version = "0.1.0"
edition = "2021"

[lib]
name = "mi_libreria"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.22", features = ["extension-module"] }

[dev-dependencies]
pyo3-stub-gen = "0.6"   # para generar los .pyi automáticamente
```

---

## 4. pyproject.toml

```toml
[build-system]
requires = ["maturin>=1.7,<2.0"]
build-backend = "maturin"

[project]
name = "mi_libreria"
version = "0.1.0"
requires-python = ">=3.9"
description = "Librería de procesamiento de texto y números"

[tool.maturin]
features = ["pyo3/extension-module"]
# incluir los stubs en el wheel
include = ["python/mi_libreria/*.pyi"]
python-packages = ["python/mi_libreria"]
```

---

## 5. El código Rust

### src/utils.rs — Helpers internos (no expuestos a Python)

```rust
// utils.rs no tiene nada de PyO3, es Rust puro.
// Es exactamente como un módulo Python de helpers internos.

/// Elimina caracteres no alfanuméricos de una cadena
pub fn solo_alfanumerico(texto: &str) -> String {
    texto
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

/// Redondea un f64 a N decimales
pub fn redondear(valor: f64, decimales: u32) -> f64 {
    let factor = 10f64.powi(decimales as i32);
    (valor * factor).round() / factor
}

/// Divide una cadena en palabras, ignorando vacíos
pub fn tokenizar(texto: &str) -> Vec<&str> {
    texto.split_whitespace().collect()
}
```

---

### src/texto.rs — Funciones de texto expuestas a Python

```rust
use pyo3::prelude::*;
use std::collections::HashMap;

// Importamos los helpers internos
use crate::utils::{solo_alfanumerico, tokenizar};

/// Limpia un texto: minúsculas y solo caracteres alfanuméricos
#[pyfunction]
pub fn limpiar(texto: &str) -> String {
    solo_alfanumerico(texto).to_lowercase()
}

/// Cuenta la frecuencia de cada palabra en el texto
#[pyfunction]
pub fn contar_palabras(texto: &str) -> HashMap<String, usize> {
    let mut mapa: HashMap<String, usize> = HashMap::new();
    for palabra in tokenizar(&limpiar(texto)) {
        *mapa.entry(palabra.to_string()).or_insert(0) += 1;
    }
    mapa
}

/// Devuelve las N palabras más frecuentes como lista de (palabra, cantidad)
#[pyfunction]
pub fn top_palabras(texto: &str, n: usize) -> Vec<(String, usize)> {
    let mut pares: Vec<(String, usize)> = contar_palabras(texto).into_iter().collect();
    pares.sort_by(|a, b| b.1.cmp(&a.1));
    pares.truncate(n);
    pares
}

/// Clase que representa un analizador de texto con estado
#[pyclass]
pub struct AnalizadorTexto {
    texto: String,
}

#[pymethods]
impl AnalizadorTexto {
    /// Crea un nuevo analizador con el texto dado
    #[new]
    pub fn new(texto: String) -> Self {
        AnalizadorTexto { texto }
    }

    /// Total de palabras en el texto
    pub fn total_palabras(&self) -> usize {
        tokenizar(&self.texto).len()
    }

    /// Cantidad de palabras únicas
    pub fn palabras_unicas(&self) -> usize {
        contar_palabras(&self.texto).len()
    }

    /// Las N palabras más frecuentes
    pub fn top(&self, n: usize) -> Vec<(String, usize)> {
        top_palabras(&self.texto, n)
    }

    /// Reemplaza una palabra por otra en el texto
    pub fn reemplazar(&mut self, original: &str, nuevo: &str) {
        self.texto = self.texto.replace(original, nuevo);
    }

    /// Devuelve el texto actual
    #[getter]
    pub fn texto(&self) -> &str {
        &self.texto
    }

    pub fn __repr__(&self) -> String {
        format!(
            "AnalizadorTexto(palabras={}, unicas={})",
            self.total_palabras(),
            self.palabras_unicas()
        )
    }
}

/// Registra las funciones y clases de este módulo en el submódulo Python "texto"
pub fn registrar(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(limpiar, m)?)?;
    m.add_function(wrap_pyfunction!(contar_palabras, m)?)?;
    m.add_function(wrap_pyfunction!(top_palabras, m)?)?;
    m.add_class::<AnalizadorTexto>()?;
    Ok(())
}
```

---

### src/numeros.rs — Funciones numéricas expuestas a Python

```rust
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::utils::redondear;

/// Calcula la media (promedio) de una lista de números
#[pyfunction]
pub fn media(datos: Vec<f64>) -> PyResult<f64> {
    if datos.is_empty() {
        return Err(PyValueError::new_err("La lista no puede estar vacía"));
    }
    Ok(redondear(datos.iter().sum::<f64>() / datos.len() as f64, 6))
}

/// Calcula la mediana de una lista de números
#[pyfunction]
pub fn mediana(mut datos: Vec<f64>) -> PyResult<f64> {
    if datos.is_empty() {
        return Err(PyValueError::new_err("La lista no puede estar vacía"));
    }
    datos.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = datos.len();
    if n % 2 == 0 {
        Ok(redondear((datos[n / 2 - 1] + datos[n / 2]) / 2.0, 6))
    } else {
        Ok(datos[n / 2])
    }
}

/// Calcula la desviación estándar de una lista de números
#[pyfunction]
pub fn desviacion_estandar(datos: Vec<f64>) -> PyResult<f64> {
    if datos.len() < 2 {
        return Err(PyValueError::new_err("Se necesitan al menos 2 valores"));
    }
    let m = media(datos.clone())?;
    let varianza = datos.iter().map(|x| (x - m).powi(2)).sum::<f64>() / datos.len() as f64;
    Ok(redondear(varianza.sqrt(), 6))
}

/// Normaliza una lista al rango [0, 1]
#[pyfunction]
pub fn normalizar(datos: Vec<f64>) -> PyResult<Vec<f64>> {
    if datos.is_empty() {
        return Err(PyValueError::new_err("La lista no puede estar vacía"));
    }
    let min = datos.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = datos.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    if (max - min).abs() < f64::EPSILON {
        return Err(PyValueError::new_err("Todos los valores son iguales, no se puede normalizar"));
    }
    Ok(datos.iter().map(|x| redondear((x - min) / (max - min), 6)).collect())
}

/// Clase con estadísticas de un conjunto de datos
#[pyclass]
pub struct Estadisticas {
    datos: Vec<f64>,
}

#[pymethods]
impl Estadisticas {
    /// Crea un objeto Estadisticas con los datos dados
    #[new]
    pub fn new(datos: Vec<f64>) -> PyResult<Self> {
        if datos.is_empty() {
            return Err(PyValueError::new_err("Los datos no pueden estar vacíos"));
        }
        Ok(Estadisticas { datos })
    }

    /// Media de los datos
    pub fn media(&self) -> PyResult<f64> {
        media(self.datos.clone())
    }

    /// Mediana de los datos
    pub fn mediana(&self) -> PyResult<f64> {
        mediana(self.datos.clone())
    }

    /// Desviación estándar
    pub fn desviacion(&self) -> PyResult<f64> {
        desviacion_estandar(self.datos.clone())
    }

    /// Valor mínimo
    pub fn minimo(&self) -> f64 {
        self.datos.iter().cloned().fold(f64::INFINITY, f64::min)
    }

    /// Valor máximo
    pub fn maximo(&self) -> f64 {
        self.datos.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }

    /// Versión normalizada de los datos
    pub fn normalizar(&self) -> PyResult<Vec<f64>> {
        normalizar(self.datos.clone())
    }

    /// Cantidad de elementos
    #[getter]
    pub fn n(&self) -> usize {
        self.datos.len()
    }

    pub fn __repr__(&self) -> String {
        format!("Estadisticas(n={})", self.datos.len())
    }
}

/// Registra las funciones y clases de este módulo en el submódulo Python "numeros"
pub fn registrar(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(media, m)?)?;
    m.add_function(wrap_pyfunction!(mediana, m)?)?;
    m.add_function(wrap_pyfunction!(desviacion_estandar, m)?)?;
    m.add_function(wrap_pyfunction!(normalizar, m)?)?;
    m.add_class::<Estadisticas>()?;
    Ok(())
}
```

---

### src/lib.rs — El punto de entrada (como `__init__.py`)

```rust
use pyo3::prelude::*;

// Declarar los módulos — equivalente a los imports en __init__.py
mod utils;   // interno, no se expone a Python
mod texto;
mod numeros;

/// Módulo raíz de mi_libreria.
/// Arma la estructura de submódulos que verá Python.
#[pymodule]
fn mi_libreria(m: &Bound<'_, PyModule>) -> PyResult<()> {

    // ── Submódulo "texto" ──────────────────────────────────────────────────
    let modulo_texto = PyModule::new(m.py(), "texto")?;
    texto::registrar(&modulo_texto)?;
    m.add_submodule(&modulo_texto)?;

    // ── Submódulo "numeros" ────────────────────────────────────────────────
    let modulo_numeros = PyModule::new(m.py(), "numeros")?;
    numeros::registrar(&modulo_numeros)?;
    m.add_submodule(&modulo_numeros)?;

    Ok(())
}
```

---

## 6. Cómo se usa desde Python

```python
# importar submódulos completos
from mi_libreria import texto, numeros

# --- texto ---
print(texto.limpiar("Hola, Mundo! 123"))
# "hola mundo 123"

print(texto.contar_palabras("rust es rapido rust es seguro"))
# {'rust': 2, 'es': 2, 'rapido': 1, 'seguro': 1}

print(texto.top_palabras("rust es rapido rust es seguro", 2))
# [('rust', 2), ('es', 2)]

analizador = texto.AnalizadorTexto("rust es rapido rust es seguro")
print(analizador.total_palabras())   # 6
print(analizador.palabras_unicas())  # 4
print(analizador.top(2))             # [('rust', 2), ('es', 2)]
print(analizador)                    # AnalizadorTexto(palabras=6, unicas=4)

# --- numeros ---
datos = [4.0, 8.0, 15.0, 16.0, 23.0, 42.0]

print(numeros.media(datos))               # 18.0
print(numeros.mediana(datos))             # 15.5
print(numeros.desviacion_estandar(datos)) # 12.296
print(numeros.normalizar(datos))          # [0.0, 0.105, 0.289, 0.315, 0.5, 1.0]

stats = numeros.Estadisticas(datos)
print(stats.media())     # 18.0
print(stats.minimo())    # 4.0
print(stats.maximo())    # 42.0
print(stats.n)           # 6
```

---

## 7. Qué son los stub files (.pyi)

Cuando compilás la librería, el resultado es un archivo binario (`.so` o `.pyd`).
Python no puede leer ese binario para saber qué funciones y tipos tiene,
así que herramientas como VSCode, PyCharm, mypy o pyright quedan "ciegas":
no hay autocompletado, no hay chequeo de tipos, no hay documentación inline.

Los **stub files** (`.pyi`) resuelven eso. Son archivos de texto plano que
describen la interfaz pública de la librería, sin lógica:

```
mi_libreria/
├── __init__.pyi     ← "esta librería tiene los submódulos texto y numeros"
├── texto.pyi        ← "texto tiene limpiar(), contar_palabras(), AnalizadorTexto..."
└── numeros.pyi      ← "numeros tiene media(), Estadisticas..."
```

---

## 8. Opciones para generar los stubs

### Opción A — Automático con pyo3-stub-gen (recomendado)

`pyo3-stub-gen` lee los tipos y docstrings del código Rust y genera los `.pyi`
automáticamente.

**1. Agregar al Cargo.toml:**

```toml
[dependencies]
pyo3 = { version = "0.22", features = ["extension-module"] }
pyo3-stub-gen = "0.6"
```

**2. Modificar lib.rs para activar la generación:**

```rust
use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

mod utils;
mod texto;
mod numeros;

#[pymodule]
fn mi_libreria(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let modulo_texto = PyModule::new(m.py(), "texto")?;
    texto::registrar(&modulo_texto)?;
    m.add_submodule(&modulo_texto)?;

    let modulo_numeros = PyModule::new(m.py(), "numeros")?;
    numeros::registrar(&modulo_numeros)?;
    m.add_submodule(&modulo_numeros)?;

    Ok(())
}

// Esta línea activa la recolección de info para los stubs
define_stub_info_gatherer!(stub_info);
```

**3. Generar los stubs:**

```bash
maturin develop
cargo run --bin stub_gen
```

Los archivos `.pyi` se generan en `python/mi_libreria/`.

---

### Opción B — Semi-automático con stubgen de mypy

Después de compilar con `maturin develop`:

```bash
pip install mypy
stubgen -p mi_libreria -o python/
```

Genera los stubs pero puede dejar algunos tipos como `Any` o incompletos.
Hay que revisarlos y completarlos a mano.

---

### Opción C — Manual (control total)

Escribís los `.pyi` directamente. Es más trabajo pero el resultado es exactamente
lo que querés que vea el usuario.

---

## 9. Los stub files del ejemplo

### python/mi_libreria/__init__.pyi

```python
# Declara que mi_libreria tiene dos submódulos
from mi_libreria import texto as texto
from mi_libreria import numeros as numeros
```

---

### python/mi_libreria/texto.pyi

```python
from typing import Final

def limpiar(texto: str) -> str:
    """
    Limpia un texto: convierte a minúsculas y elimina
    caracteres no alfanuméricos.

    Args:
        texto: El texto a limpiar.

    Returns:
        Texto limpio en minúsculas.

    Example:
        >>> limpiar("Hola, Mundo! 123")
        'hola mundo 123'
    """
    ...

def contar_palabras(texto: str) -> dict[str, int]:
    """
    Cuenta la frecuencia de cada palabra en el texto.

    Args:
        texto: El texto a analizar.

    Returns:
        Diccionario con palabra → cantidad de apariciones.

    Example:
        >>> contar_palabras("rust es rust")
        {'rust': 2, 'es': 1}
    """
    ...

def top_palabras(texto: str, n: int) -> list[tuple[str, int]]:
    """
    Devuelve las N palabras más frecuentes.

    Args:
        texto: El texto a analizar.
        n: Cantidad de palabras a devolver.

    Returns:
        Lista de tuplas (palabra, cantidad) ordenadas de mayor a menor.
    """
    ...

class AnalizadorTexto:
    """
    Analizador de texto con estado. Permite consultar estadísticas
    y modificar el texto de forma incremental.

    Example:
        >>> a = AnalizadorTexto("rust es rapido")
        >>> a.total_palabras()
        3
    """

    def __init__(self, texto: str) -> None: ...

    @property
    def texto(self) -> str:
        """El texto actual del analizador."""
        ...

    def total_palabras(self) -> int:
        """Cantidad total de palabras."""
        ...

    def palabras_unicas(self) -> int:
        """Cantidad de palabras únicas."""
        ...

    def top(self, n: int) -> list[tuple[str, int]]:
        """Las N palabras más frecuentes."""
        ...

    def reemplazar(self, original: str, nuevo: str) -> None:
        """Reemplaza todas las ocurrencias de una palabra por otra."""
        ...

    def __repr__(self) -> str: ...
```

---

### python/mi_libreria/numeros.pyi

```python
def media(datos: list[float]) -> float:
    """
    Calcula la media (promedio) de una lista de números.

    Args:
        datos: Lista de números. No puede estar vacía.

    Returns:
        La media aritmética, redondeada a 6 decimales.

    Raises:
        ValueError: Si la lista está vacía.
    """
    ...

def mediana(datos: list[float]) -> float:
    """
    Calcula la mediana de una lista de números.

    Raises:
        ValueError: Si la lista está vacía.
    """
    ...

def desviacion_estandar(datos: list[float]) -> float:
    """
    Calcula la desviación estándar poblacional.

    Raises:
        ValueError: Si la lista tiene menos de 2 elementos.
    """
    ...

def normalizar(datos: list[float]) -> list[float]:
    """
    Normaliza una lista al rango [0.0, 1.0].

    Raises:
        ValueError: Si la lista está vacía o todos los valores son iguales.
    """
    ...

class Estadisticas:
    """
    Calcula y expone estadísticas de un conjunto de datos.

    Example:
        >>> s = Estadisticas([1.0, 2.0, 3.0])
        >>> s.media()
        2.0
    """

    def __init__(self, datos: list[float]) -> None:
        """
        Args:
            datos: Lista de números. No puede estar vacía.

        Raises:
            ValueError: Si la lista está vacía.
        """
        ...

    @property
    def n(self) -> int:
        """Cantidad de elementos."""
        ...

    def media(self) -> float: ...
    def mediana(self) -> float: ...
    def desviacion(self) -> float: ...
    def minimo(self) -> float: ...
    def maximo(self) -> float: ...

    def normalizar(self) -> list[float]:
        """Devuelve los datos normalizados al rango [0, 1]."""
        ...

    def __repr__(self) -> str: ...
```

---

## 10. Qué ve el usuario con los stubs instalados

Con los stubs incluidos en el wheel, el usuario tiene en su editor:

**Autocompletado:**
```
stats.med  →  media()    Calcula la media (promedio)...
             mediana()   Calcula la mediana...
```

**Chequeo de tipos:**
```python
stats = Estadisticas([1, 2, 3])
stats.media() + "texto"   # ← mypy/pyright marcan error en tiempo de desarrollo
```

**Documentación inline:**
Al pasar el mouse sobre `media()` en VSCode aparece el docstring completo
con los Args, Returns y Raises.

---

## 11. Compilar y verificar

```bash
# instalar en modo desarrollo
maturin develop

# verificar que los submódulos funcionan
python -c "from mi_libreria import texto, numeros; print(texto.limpiar('Hola!'))"

# verificar tipos con mypy
pip install mypy
mypy mi_script.py

# build de producción con stubs incluidos
maturin build --release
```

---

## 12. Resumen del flujo completo

```
src/utils.rs    ← Rust puro, helpers internos
src/texto.rs    ← funciones + clases con #[pyfunction] / #[pyclass]
src/numeros.rs  ← funciones + clases con #[pyfunction] / #[pyclass]
src/lib.rs      ← arma los submódulos Python, punto de entrada
      │
      │  maturin develop / maturin build --release
      ▼
mi_libreria.so / mi_libreria.pyd    ← binario que importa Python
      +
python/mi_libreria/*.pyi            ← stubs para autocompletado y tipos
      │
      ▼
from mi_libreria import texto, numeros   ← el usuario lo usa como cualquier lib Python
```

---

*Maturin >= 1.7 | PyO3 >= 0.22 | pyo3-stub-gen >= 0.6 | Rust Edition 2021*