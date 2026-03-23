# Ruta de Aprendizaje de Rust

Este documento organiza los cuadernos Jupyter creados hasta ahora y establece los próximos temas a tratar para continuar con un progreso de aprendizaje estructurado en Rust.

---

## 📚 1. Módulos Completados y Actuales

A continuación, el orden lógico sugerido para leer o revisar los cuadernos existentes:

### 1.1 Variables, Tipos de Datos y Operadores
- [Variables_Rust.ipynb](./Variables_Rust.ipynb): Declaración de variables, mutabilidad y tipos de datos principales.
- [Operadores Aritmeticos Rust.ipynb](./Operadores%20Aritmeticos%20Rust.ipynb): Operaciones matemáticas y cálculos básicos.

### 1.2 Control de Flujo
- [Flujos if else y shadown.ipynb](./Flujos%20if%20else%20y%20shadown.ipynb): Estructuras de decisión condicionales y el concepto de shadowing (ocultamiento) de variables.
- [Flujos_operadores_y_bucles.ipynb](./Flujos_operadores_y_bucles.ipynb): Operadores lógicos, control de flujo avanzado y bucles (`loop`, `while`, `for`).

### 1.3 Colecciones Comunes (Vectores y Slices)
- [Vectores.ipynb](./Vectores.ipynb): Introducción y creación de listas de tamaño dinámico.
- [Vectore_acceder_elementos.ipynb](./Vectore_acceder_elementos.ipynb): Métodos de lectura y acceso seguro usando índices o el método `get`.
- [Vectores_itereciones_for.ipynb](./Vectores_itereciones_for.ipynb): Cómo recorrer y modificar elementos iterando con bucles iterativos.
- [Slices_rust.ipynb](./Slices_rust.ipynb): Referencias parciales (vistas) a colecciones sin copiar la data original.

### 1.4 Gestión de Memoria Punteros y Referencias
- [Ciclo de vida de Referencias.ipynb](./Ciclo%20de%20vida%20de%20Referencias.ipynb): Conceptos fundamentales de memoria de Rust como el Ownership (propiedad), Borrowing (préstamos seguros) y nociones de Lifetimes (ciclos de vida).

---

## 🚀 2. Próximos Temas a Tratar (Roadmap)

Organización sugerida para los siguientes cuadernos o temas a estudiar:

### Nivel Intermedio
- [fundiones a fondo.ipynb](./fundiones%20a%20fondo.ipynb): **Concepto de Funciones a fondo:** Retorno de valores, expresiones vs sentencias `(statements vs expressions)`.
- [ ] **Structs (Estructuras):** Crear estructuras de datos personalizadas y bloques de implementación (`impl`) de métodos.
- [ ] **Enums y Pattern Matching:** Definir Enumeradores (`enum`), cómo controlarlos fuertemente con la estructura `match` y abreviaciones como `if let`.
- [ ] **Strings y Texto:** Comprendiendo a fondo la diferencia entre el tipo contenedor `String` y las referencias inmutables `&str`.
- [ ] **Hash Maps:** Almacenamiento mediante clave-valor (Diccionarios en Rust).
- [ ] **Manejo de Errores:** Trabajando con el enum `Result`, cómo manejar errores de ejecución suaves, uso de `Option` para manejar valores nulos (sin Null pointer) y uso del símbolo `?`.

### Nivel Avanzado
- [ ] **Sistema de Organización (Módulos):** Paquetes (Packages), Crates, e importación con `use` y la palabra clave `pub` para alcances de visibilidad.
- [ ] **Genéricos (Generics):** Crear funciones abstractas preparadas para aceptar y devolver múltiples tipos de datos.
- [ ] **Traits (Rasgos):** Definiendo comportamiento compartido y herencia de características de interfaces.
- [ ] **Lifetimes Avanzados:** Uso explícito de notación con comillas simples (ej. `'a`) en estructuras complejas y referencias avanzadas.
- [ ] **Closures e Iteradores:** Programación funcional con funciones anónimas y consumo de iteradores complejos de manera eficiente.

### Especialización y Desarrollo Práctico
- [ ] **Escribir Tests:** Pruebas unitarias directas en módulos, integración y pruebas de documentación.
- [ ] **Smart Pointers (Punteros Inteligentes):** Qué y cuándo usar tipos complejos como `Box<T>`, `Rc<T>` y `RefCell<T>`.
- [ ] **Concurrencia Práctica:** Hilos de ejecución (Threads), transferencia segura de datos entre hilos (Channels de `mpsc`) y gestión compartida multihilo con `Arc<T>` y `Mutex<T>`.
- [ ] **Macros y Metaprogramación:** Entendiendo qué hace que `println!` o `vec!` funcionen (Macros declarativas).
