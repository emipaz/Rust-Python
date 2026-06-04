# Distribución de Librerías Maturin entre Plataformas

> Cómo compilar y distribuir wheels de Rust+Python para Linux, Windows y macOS — Marzo 2026

---

## 1. El problema de la portabilidad

Cuando corrés `maturin develop`, se genera un archivo binario compilado específicamente para:

- **Tu sistema operativo** (Linux, Windows, macOS)
- **Tu arquitectura de CPU** (x86_64, arm64)
- **Tu versión de Python** (3.10, 3.11, 3.12, etc.)
- **Tu versión de glibc** (solo en Linux)

Ese archivo (`.so` en Linux/macOS, `.pyd` en Windows) **no funciona en otros entornos**. Para distribuirlo necesitás generar un **wheel** (`.whl`) compilado para cada combinación de plataforma y Python que querés soportar.

---

## 2. Opciones de distribución

### Opción A — Distribuir código fuente (los destinos compilan)

La más simple. Compartís el código fuente y cada usuario ejecuta:

```bash
pip install maturin
maturin develop --release
```

**Ventajas:** sin infraestructura de build, siempre compatible.  
**Desventajas:** el destino necesita tener Rust instalado, lo cual no es razonable para usuarios finales.

**Cuándo usarla:** proyectos internos donde todos los desarrolladores ya tienen Rust.

---

### Opción B — Wheel local (misma plataforma)

```bash
maturin build --release
```

Genera un wheel en `target/wheels/` para tu plataforma y versión de Python actual.

```
target/wheels/
└── mi_libreria-0.1.0-cp312-cp312-linux_x86_64.whl
```

El nombre del archivo indica exactamente para qué sirve:
- `cp312` → CPython 3.12
- `linux_x86_64` → Linux de 64 bits

Para instalarlo en el mismo entorno:

```bash
pip install target/wheels/mi_libreria-*.whl
```

**Ventajas:** rápido, sin Docker ni CI.  
**Desventajas:** solo funciona en tu plataforma exacta.

---

### Opción C — manylinux (Linux portable)

El estándar **manylinux** resuelve la compatibilidad en Linux. Compila dentro de un contenedor Docker con una versión muy antigua de glibc, garantizando que el wheel funcione en prácticamente cualquier distro Linux moderna (Ubuntu, Debian, Fedora, Arch, etc.).

```bash
# Requiere Docker instalado y corriendo
maturin build --release --manylinux auto
```

Maturin elige automáticamente la imagen Docker adecuada. El wheel resultante tiene un nombre como:

```
mi_libreria-0.1.0-cp312-cp312-manylinux_2_17_x86_64.manylinux2014_x86_64.whl
```

#### Variantes de manylinux

| Variante          | glibc mínimo | Compatibilidad          |
|-------------------|--------------|-------------------------|
| `manylinux2014`   | 2.17         | Recomendado, muy amplio |
| `manylinux_2_28`  | 2.28         | Distros más recientes   |
| `musllinux_1_2`   | musl libc    | Alpine Linux            |
| `auto`            | automático   | Maturin elige por vos   |

#### Para Alpine Linux (musl)

```bash
maturin build --release --manylinux musllinux_1_2
```

**Ventajas:** un solo wheel cubre casi todo Linux.  
**Desventajas:** requiere Docker, no cubre Windows ni macOS.

---

### Opción D — Build para múltiples versiones de Python

Por defecto `maturin build` compila solo para la versión de Python activa. Para generar wheels para Python 3.9, 3.10, 3.11, 3.12 y 3.13 a la vez:

```bash
maturin build --release --manylinux auto \
  --interpreter python3.9 python3.10 python3.11 python3.12 python3.13
```

O usando `pyenv` para tener múltiples versiones instaladas:

```bash
# instalar versiones con pyenv
pyenv install 3.10 3.11 3.12

# maturin las detecta automáticamente
maturin build --release --manylinux auto
```

Resultado:

```
target/wheels/
├── mi_libreria-0.1.0-cp310-cp310-manylinux_2_17_x86_64.whl
├── mi_libreria-0.1.0-cp311-cp311-manylinux_2_17_x86_64.whl
└── mi_libreria-0.1.0-cp312-cp312-manylinux_2_17_x86_64.whl
```

---

### Opción E — Cross-compilation (compilar para otra arquitectura)

Podés compilar para una arquitectura diferente a la tuya sin tener esa máquina física. Requiere instalar el target de Rust correspondiente.

#### Desde Linux x86_64 → Linux arm64 (ej: Raspberry Pi, AWS Graviton)

```bash
# instalar el target de Rust
rustup target add aarch64-unknown-linux-gnu

# instalar el linker cruzado
sudo apt install gcc-aarch64-linux-gnu

# compilar
maturin build --release --target aarch64-unknown-linux-gnu --manylinux auto
```

#### Desde macOS arm64 → macOS x86_64

```bash
rustup target add x86_64-apple-darwin
maturin build --release --target x86_64-apple-darwin
```

#### Universal binary para macOS (arm64 + x86_64 en un solo wheel)

```bash
rustup target add x86_64-apple-darwin aarch64-apple-darwin
maturin build --release --universal2
```

**Ventajas:** no necesitás máquinas físicas de cada arquitectura.  
**Desventajas:** la configuración del linker cruzado puede ser compleja.

---

### Opción F — GitHub Actions (recomendada para distribución real)

La solución completa: GitHub Actions corre el build en paralelo en Linux, Windows y macOS, genera todos los wheels y los publica en PyPI automáticamente al crear un release.

#### Estructura del proyecto

```
mi_libreria/
├── .github/
│   └── workflows/
│       ├── CI.yml       # tests en cada push
│       └── publish.yml  # build + PyPI en cada release
├── Cargo.toml
├── pyproject.toml
└── src/
    └── lib.rs
```

#### .github/workflows/CI.yml — Tests en cada push

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        python-version: ["3.10", "3.11", "3.12"]

    steps:
      - uses: actions/checkout@v4

      - name: Instalar Python
        uses: actions/setup-python@v5
        with:
          python-version: ${{ matrix.python-version }}

      - name: Instalar Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Instalar Maturin
        run: pip install maturin pytest

      - name: Compilar y instalar
        run: maturin develop --release

      - name: Correr tests
        run: pytest tests/
```

#### .github/workflows/publish.yml — Build completo y publicación en PyPI

```yaml
name: Publish

on:
  push:
    tags:
      - 'v*'  # se activa al crear un tag como v0.1.0

permissions:
  contents: read

jobs:

  # ── Linux (manylinux + musllinux) ─────────────────────────────────────────
  linux:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target: [x86_64, aarch64, armv7]  # Intel 64, ARM 64, ARM 32
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: '3.12'

      - name: Build wheels (manylinux)
        uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.target }}
          args: --release --out dist --find-interpreter
          manylinux: auto

      - name: Build wheels (musllinux / Alpine)
        uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.target }}
          args: --release --out dist --find-interpreter
          manylinux: musllinux_1_2

      - name: Subir wheels como artefacto
        uses: actions/upload-artifact@v4
        with:
          name: wheels-linux-${{ matrix.target }}
          path: dist

  # ── Windows ───────────────────────────────────────────────────────────────
  windows:
    runs-on: windows-latest
    strategy:
      matrix:
        target: [x64, x86]  # 64 bits y 32 bits
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: '3.12'

      - name: Build wheels
        uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.target }}
          args: --release --out dist --find-interpreter

      - name: Subir wheels como artefacto
        uses: actions/upload-artifact@v4
        with:
          name: wheels-windows-${{ matrix.target }}
          path: dist

  # ── macOS ─────────────────────────────────────────────────────────────────
  macos:
    runs-on: macos-latest
    strategy:
      matrix:
        target: [x86_64, aarch64]  # Intel y Apple Silicon
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: '3.12'

      - name: Build wheels
        uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.target }}
          args: --release --out dist --find-interpreter

      - name: Subir wheels como artefacto
        uses: actions/upload-artifact@v4
        with:
          name: wheels-macos-${{ matrix.target }}
          path: dist

  # ── Source distribution (sdist) ───────────────────────────────────────────
  # Permite que usuarios con Rust instalado compilen desde fuente como fallback
  sdist:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build sdist
        uses: PyO3/maturin-action@v1
        with:
          command: sdist
          args: --out dist

      - name: Subir sdist como artefacto
        uses: actions/upload-artifact@v4
        with:
          name: wheels-sdist
          path: dist

  # ── Publicar en PyPI ──────────────────────────────────────────────────────
  publish:
    runs-on: ubuntu-latest
    needs: [linux, windows, macos, sdist]  # espera que todos los builds terminen
    environment:
      name: pypi
      url: https://pypi.org/project/mi-libreria/
    permissions:
      id-token: write  # necesario para trusted publishing (sin API key)

    steps:
      - name: Descargar todos los wheels
        uses: actions/download-artifact@v4
        with:
          pattern: wheels-*
          merge-multiple: true
          path: dist

      - name: Publicar en PyPI
        uses: pypa/gh-action-pypi-publish@release/v1
```

#### Crear un release

```bash
git tag v0.1.0
git push origin v0.1.0
```

Esto dispara el workflow automáticamente. En unos minutos todos los wheels estarán en PyPI y cualquier usuario podrá instalar la librería con:

```bash
pip install mi_libreria
```

---

## 3. Trusted Publishing (sin API keys)

La forma moderna y segura de publicar en PyPI es **Trusted Publishing**: PyPI reconoce a GitHub Actions como fuente confiable y no necesitás guardar ninguna API key como secreto.

Para configurarlo:

1. En PyPI, ir a tu proyecto → **Publishing** → **Add a new publisher**
2. Completar: owner de GitHub, nombre del repo, nombre del workflow (`publish.yml`), y el environment (`pypi`)
3. Listo — el workflow de arriba ya está configurado para usarlo con `id-token: write`

---

## 4. Resumen: cuándo usar cada opción

| Situación                                      | Opción recomendada                        |
|------------------------------------------------|-------------------------------------------|
| Uso interno, todos tienen Rust                 | Código fuente + `maturin develop`         |
| Distribuir solo en tu Linux actual             | `maturin build --release`                 |
| Distribuir en cualquier Linux                  | `--manylinux auto`                        |
| Distribuir en Alpine/Docker                    | `--manylinux musllinux_1_2`               |
| Distribuir en múltiples versiones de Python    | `--find-interpreter` o `--interpreter`    |
| Compilar para ARM sin tener la máquina         | Cross-compilation con `--target`          |
| Distribuir en Linux + Windows + macOS + PyPI   | GitHub Actions con `maturin-action`       |

---

## 5. Verificar un wheel antes de publicar

```bash
# instalar herramientas de verificación
pip install twine auditwheel

# verificar que el wheel es válido
twine check dist/*.whl

# verificar compatibilidad manylinux (solo en Linux)
auditwheel show dist/mi_libreria-*.whl
```

`auditwheel show` te dice exactamente con qué plataformas es compatible el wheel.

---

## 6. Recursos

- [Documentación de Maturin — distribución](https://www.maturin.rs/distribution)
- [maturin-action en GitHub](https://github.com/PyO3/maturin-action)
- [Trusted Publishing en PyPI](https://docs.pypi.org/trusted-publishers/)
- [Estándar manylinux (PEP 599/600)](https://peps.python.org/pep-0599/)
- [auditwheel](https://github.com/pypa/auditwheel)

---

*Maturin >= 1.7 | maturin-action v1 | GitHub Actions*