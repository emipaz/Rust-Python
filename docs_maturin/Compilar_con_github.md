# Distribuir Wheels de Maturin via GitHub Releases

> Sin PyPI, sin infraestructura extra — solo GitHub y una cuenta normal — Marzo 2026

---

## 1. La idea general

En lugar de publicar en PyPI, los wheels compilados para cada sistema quedan adjuntos
al **GitHub Release** de tu repositorio. Son archivos permanentes, descargables por
cualquiera, e instalables directamente con `pip` desde la URL.

```
tu repo en GitHub
└── Releases
    └── v0.1.0
        ├── mi_libreria-0.1.0-cp312-cp312-manylinux_2_17_x86_64.whl  ← Linux
        ├── mi_libreria-0.1.0-cp312-cp312-win_amd64.whl               ← Windows
        ├── mi_libreria-0.1.0-cp312-cp312-macosx_arm64.whl            ← macOS Apple Silicon
        ├── mi_libreria-0.1.0-cp312-cp312-macosx_x86_64.whl           ← macOS Intel
        └── Source code (zip / tar.gz)                                 ← GitHub lo agrega solo
```

El usuario descarga el wheel de su sistema o lo instala directo desde la URL,
sin necesidad de tener Rust instalado.

---

## 2. Qué necesitás

- **Cuenta de GitHub normal** (gratuita). No necesitás permisos especiales.
- **Un repositorio** con tu proyecto Maturin (público o privado).
- **Nada más.** GitHub Actions corre en los servidores de GitHub, no en tu máquina.

El plan gratuito de GitHub incluye **2000 minutos de Actions por mes**,
más que suficiente para compilar releases esporádicos.

---

## 3. Estructura del proyecto

```
mi_libreria/
├── .github/
│   └── workflows/
│       └── release.yml      ← el workflow que hace todo
├── Cargo.toml
├── pyproject.toml
└── src/
    └── lib.rs
```

---

## 4. El workflow completo

Guardá este archivo en `.github/workflows/release.yml`:

```yaml
name: Release

# Se activa solo cuando subís un tag que empieza con "v"
# Ejemplo: v0.1.0, v1.2.3
on:
  push:
    tags:
      - 'v*'

permissions:
  contents: write   # necesario para crear el Release y subir archivos

jobs:

  # ── Linux ──────────────────────────────────────────────────────────────────
  linux:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target:
          - x86_64    # la mayoría de los servidores y PCs Linux
          - aarch64   # ARM 64 bits (Raspberry Pi 4, AWS Graviton, etc.)

    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-python@v5
        with:
          python-version: '3.12'

      - name: Build wheels (manylinux — compatible con casi todo Linux)
        uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.target }}
          manylinux: auto
          args: --release --out dist --find-interpreter

      - name: Build wheels (musllinux — para Alpine Linux / Docker slim)
        uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.target }}
          manylinux: musllinux_1_2
          args: --release --out dist --find-interpreter

      - name: Guardar wheels como artefacto temporal
        uses: actions/upload-artifact@v4
        with:
          name: wheels-linux-${{ matrix.target }}
          path: dist/*.whl

  # ── Windows ────────────────────────────────────────────────────────────────
  windows:
    runs-on: windows-latest
    strategy:
      matrix:
        target:
          - x64   # Windows 64 bits (la gran mayoría)
          - x86   # Windows 32 bits (legacy)

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

      - name: Guardar wheels como artefacto temporal
        uses: actions/upload-artifact@v4
        with:
          name: wheels-windows-${{ matrix.target }}
          path: dist/*.whl

  # ── macOS ──────────────────────────────────────────────────────────────────
  macos:
    runs-on: macos-latest
    strategy:
      matrix:
        target:
          - x86_64   # Macs Intel (anteriores a 2020)
          - aarch64  # Macs Apple Silicon M1/M2/M3

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

      - name: Guardar wheels como artefacto temporal
        uses: actions/upload-artifact@v4
        with:
          name: wheels-macos-${{ matrix.target }}
          path: dist/*.whl

  # ── Crear el GitHub Release y subir todos los wheels ───────────────────────
  release:
    runs-on: ubuntu-latest
    needs: [linux, windows, macos]   # espera que los tres builds terminen

    steps:
      - name: Descargar todos los wheels
        uses: actions/download-artifact@v4
        with:
          pattern: wheels-*          # descarga todos los artefactos
          merge-multiple: true       # los junta en una sola carpeta
          path: dist

      - name: Crear Release y subir wheels
        uses: softprops/action-gh-release@v2
        with:
          files: dist/*.whl
          # El nombre del release se toma del tag: "v0.1.0" → Release "v0.1.0"
          generate_release_notes: true   # GitHub genera las notas automáticamente
```

---

## 5. Cómo publicar una nueva versión

Desde tu máquina, con el código listo:

```bash
# 1. Asegurarse de que todo está commiteado
git add .
git commit -m "release v0.1.0"

# 2. Crear el tag con la versión
git tag v0.1.0

# 3. Subir el código y el tag
git push origin main --tags
```

Eso es todo. GitHub detecta el tag y arranca el workflow automáticamente.
En unos minutos (el build en paralelo tarda ~5-10 minutos) el Release aparece
en tu repositorio con todos los wheels adjuntos.

---

## 6. Cómo instala el usuario

### Opción 1 — Descargar el archivo manualmente

El usuario va a `github.com/tu_usuario/mi_libreria/releases`, descarga el
`.whl` correspondiente a su sistema y lo instala:

```bash
pip install mi_libreria-0.1.0-cp312-cp312-win_amd64.whl
```

### Opción 2 — Instalar directo desde la URL (sin descargar)

```bash
# Linux x86_64
pip install "https://github.com/tu_usuario/mi_libreria/releases/download/v0.1.0/mi_libreria-0.1.0-cp312-cp312-manylinux_2_17_x86_64.whl"

# Windows 64 bits
pip install "https://github.com/tu_usuario/mi_libreria/releases/download/v0.1.0/mi_libreria-0.1.0-cp312-cp312-win_amd64.whl"

# macOS Apple Silicon
pip install "https://github.com/tu_usuario/mi_libreria/releases/download/v0.1.0/mi_libreria-0.1.0-cp312-cp312-macosx_11_0_arm64.whl"
```

### Opción 3 — Listar en un requirements.txt

```text
# requirements.txt
mi_libreria @ https://github.com/tu_usuario/mi_libreria/releases/download/v0.1.0/mi_libreria-0.1.0-cp312-cp312-manylinux_2_17_x86_64.whl
```

```bash
pip install -r requirements.txt
```

---

## 7. Cómo saber qué wheel corresponde a cada sistema

El nombre del wheel sigue este formato:

```
mi_libreria - 0.1.0 - cp312 - cp312 - manylinux_2_17_x86_64 .whl
     │           │       │       │             │
   nombre     versión  Python  Python       plataforma
```

### Plataformas más comunes

| Sistema                        | Nombre en el wheel                          |
|--------------------------------|---------------------------------------------|
| Linux x86_64 (PC/servidor)     | `manylinux_2_17_x86_64`                     |
| Linux ARM 64 (Raspberry Pi 4+) | `manylinux_2_17_aarch64`                    |
| Linux Alpine / Docker slim     | `musllinux_1_2_x86_64`                      |
| Windows 64 bits                | `win_amd64`                                 |
| Windows 32 bits                | `win32`                                     |
| macOS Apple Silicon (M1/M2/M3) | `macosx_11_0_arm64`                         |
| macOS Intel                    | `macosx_10_12_x86_64`                       |

### Versiones de Python

| Código  | Python |
|---------|--------|
| `cp39`  | 3.9    |
| `cp310` | 3.10   |
| `cp311` | 3.11   |
| `cp312` | 3.12   |
| `cp313` | 3.13   |

Si no sabés qué versión de Python tenés:

```bash
python --version
```

---

## 8. Diferencia entre artefactos y releases

Es importante no confundirlos:

| Característica    | Artefactos de Actions       | GitHub Release              |
|-------------------|-----------------------------|-----------------------------|
| Dónde aparecen    | Pestaña Actions del repo    | Pestaña Releases del repo   |
| Duración          | **90 días** y se borran     | **Permanentes**             |
| Acceso            | Solo logueados en GitHub    | Público (si el repo es público) |
| Uso recomendado   | Verificar builds internos   | Distribuir a usuarios       |

El workflow de arriba usa artefactos **solo como paso intermedio** para juntar
los wheels de los tres sistemas operativos, y luego los sube al Release que es permanente.

---

## 9. Soporte para múltiples versiones de Python

El workflow de arriba compila solo para Python 3.12. Para generar wheels
para 3.10, 3.11, 3.12 y 3.13 en cada plataforma, reemplazá el paso de build
de cada job por esto:

```yaml
    steps:
      - uses: actions/checkout@v4

      # Instalar todas las versiones de Python necesarias
      - uses: actions/setup-python@v5
        with:
          python-version: |
            3.10
            3.11
            3.12
            3.13

      - name: Build wheels para todas las versiones de Python
        uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.target }}
          manylinux: auto
          # --find-interpreter detecta automáticamente todos los Python instalados
          args: --release --out dist --find-interpreter
```

Esto genera un wheel separado por cada versión de Python, resultando en una
matriz completa de compatibilidad.

---

## 10. Verificar que el workflow funcionó

1. Ir a tu repo en GitHub
2. Click en la pestaña **Actions**
3. Ver el workflow **Release** corriendo (círculo amarillo = en progreso, verde = ok, rojo = error)
4. Al terminar, ir a la pestaña **Releases**
5. El release `v0.1.0` aparece con todos los `.whl` adjuntos

Si algo falla, en Actions podés ver el log detallado de cada paso para
identificar el error.

---

## 11. Resumen del flujo completo

```
tu máquina                    GitHub                        usuario final
──────────                    ──────                        ─────────────
git tag v0.1.0   →   Actions compila en paralelo:
git push --tags              ├── Ubuntu  → wheels Linux
                             ├── Windows → wheels Windows
                             └── macOS   → wheels macOS
                                    │
                             Release v0.1.0  ←──────────  pip install <URL>
                             con todos los .whl adjuntos
```

---

*GitHub Actions gratuito | Maturin >= 1.7 | softprops/action-gh-release@v2*