# STAGE 1: Rust Builder (Installs Rust + rust-analyzer)
# Use the Dev Containers base as it provides common prerequisites.
# ------------------------------------------------------------------
FROM mcr.microsoft.com/devcontainers/base:ubuntu AS rust_builder

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set default toolchain to 'stable', update it, 
# and add the rust-analyzer component all in one layer.
RUN rustup default stable && \
    rustup update stable && \
    rustup component add rust-analyzer && \
    # Verify installations
    rust-analyzer --version && \
    rustc --version

# STAGE 2: Final Environment (Latest Elixir + Copied Rust Tools)
# Use the official 'latest' Elixir tag.
# ------------------------------------------------------------------
FROM elixir:latest

COPY --from=rust_builder /root/.cargo /root/.cargo
COPY --from=rust_builder /root/.rustup /root/.rustup

# Set the PATH environment variable for the copied Rust tools
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Hex and Rebar for Elixir (essential for mix tasks)
RUN mix local.hex --force && mix local.rebar --force

WORKDIR /workspace