FROM rust:1.76

RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    python3-venv \
    && rm -rf /var/lib/apt/lists/*

# Crear un entorno virtual y activarlo
RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"

# Instalar maturin dentro del venv
RUN pip install maturin

WORKDIR /usr/src/app
CMD ["bash"]

