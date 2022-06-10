ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld \
   && apt-get autoremove -y && apt-get clean -y \
   && su vscode -c "rustup component add clippy rust-analysis rust-src rls"

