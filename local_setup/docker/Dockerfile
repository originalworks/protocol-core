FROM ubuntu:24.04


RUN apt-get update && apt install curl wget npm pkg-config nasm -y

# rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

# foundry
RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH="/root/.foundry/bin:$PATH"
RUN foundryup

# risc0
RUN curl -L https://risczero.com/install | bash 
ENV PATH="/root/.risc0/bin:$PATH"
RUN rzup install r0vm 2.3.1