FROM rust:1.89

RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    python3-venv \
    curl \
    cmake \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Instalar uv (gestor de entornos/paquetes Python moderno)
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
ENV PATH="/root/.cargo/bin:/root/.local/bin:$PATH"

# Crear entorno virtual con uv e instalar paquetes Python
RUN uv venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"
ENV VIRTUAL_ENV=/opt/venv

RUN uv pip install \
    maturin \
    ipykernel \
    jupyterlab

# Registrar el kernel de Python en Jupyter
RUN python3 -m ipykernel install --name python3 --display-name "Python 3"

# Instalar evcxr_jupyter (kernel de Rust para Jupyter)
# Requiere Rust edition 2024 (disponible desde 1.88)
RUN cargo install evcxr_jupyter --locked
RUN evcxr_jupyter --install

WORKDIR /usr/src/app
EXPOSE 8888
CMD ["bash"]
