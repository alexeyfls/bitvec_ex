FROM mcr.microsoft.com/devcontainers/base:ubuntu

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Elixir + Erlang
RUN apt-get update && \
    apt-get install -y elixir erlang-dev erlang-parsetools erlang-ssl erlang-inets && \
    mix local.hex --force && mix local.rebar --force

WORKDIR /workspace
