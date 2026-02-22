 Rust + Python Dev Container

Entorno de desarrollo con **Rust 1.89** (edición 2024) y **Python 3.13** dentro de un contenedor Docker, con soporte para Jupyter Notebooks con kernels de Rust y Python.

## Stack

| Herramienta | Versión | Descripción |
|---|---|---|
| Rust | 1.89 | Con edición 2024 |
| Python | 3.13 | Gestionado con `uv` |
| Jupyter | — | Kernels de Rust y Python |
| maturin | latest | Extensiones Python en Rust |

---

## Inicio rápido

```bash
# Construir y levantar el contenedor
docker compose build
docker compose up -d

# Entrar al contenedor
docker compose exec rustpython bash
```

---

## 1. Hola Mundo en Rust

```bash
cargo new hola_mundo
cd hola_mundo
cargo run
```

---

## 2. Jupyter con kernels de Rust y Python

Dentro del contenedor:

```bash
jupyter lab --ip=0.0.0.0 --no-browser --allow-root
```

Abrí en el navegador: `http://localhost:8888` con el token que aparece en la terminal.

Los kernels disponibles son:

```bash
jupyter kernelspec list
# python3  →  /opt/venv/share/jupyter/kernels/python3
# rust     →  /root/.local/share/jupyter/kernels/rust
```

---

## 3. Extensiones Python con Rust (maturin)

```bash
# Crear nuevo proyecto maturin
maturin new mi_extension
cd mi_extension

# Compilar e instalar en el venv
maturin develop
```

---

## Opcionales (instalación manual dentro del contenedor)

### Midnight Commander

Administrador de archivos visual en la terminal.

```bash
apt-get update && apt-get install -y mc
mc
```

### Compilación cruzada para Windows

Permite generar ejecutables `.exe` desde Linux.

```bash
# Instalar el linker de Windows
apt-get update && apt-get install -y gcc-mingw-w64

# Agregar el target de Windows a rustup
rustup target add x86_64-pc-windows-gnu

# Compilar
cargo build --target x86_64-pc-windows-gnu --release
```

Los binarios quedan en `target/<target>/release/` y nunca se pisan entre sí:

```
target/
├── debug/                            # cargo run (Linux)
├── release/                          # cargo build --release (Linux)
└── x86_64-pc-windows-gnu/
    └── release/
        └── mi_proyecto.exe           # ejecutable de Windows
```

---

## Estructura del proyecto

```
.
├── Dockerfile
├── docker-compose.yml
├── .devcontainer/
│   └── devcontainer.json
└── README.md
```

El Objetivo de este Repositorio es aprender rust