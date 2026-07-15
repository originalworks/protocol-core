FROM ubuntu:22.04

RUN apt-get update && apt install curl wget npm pkg-config nasm -y

# rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

# foundry
RUN curl -L https://getfoundry.sh/install | bash
ENV PATH="/root/.foundry/bin:$PATH"
RUN foundryup


# cargo-lambda
RUN pip3 install cargo-lambda
