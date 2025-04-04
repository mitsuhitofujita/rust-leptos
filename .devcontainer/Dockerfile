FROM mcr.microsoft.com/devcontainers/rust:1

# Install required dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        pkg-config \
        libssl-dev \
        binaryen
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs
RUN apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Install wasm-bindgen and rust targets
RUN rustup target add wasm32-unknown-unknown \
    && rustup component add rust-src
