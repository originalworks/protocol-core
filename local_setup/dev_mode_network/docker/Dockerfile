FROM ubuntu:24.04

ARG HOST_UID
ARG HOST_GID



RUN apt-get update && apt install curl wget npm pkg-config nasm -y

RUN getent group "$HOST_GID" >/dev/null || groupadd --gid "$HOST_GID" usergroup
RUN id "$HOST_UID" &>/dev/null || useradd --uid $HOST_UID --gid $HOST_GID ubuntu
USER $HOST_UID:$HOST_GID
RUN mkdir /home/ubuntu/dupa
# rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/ubuntu/.cargo/bin:$PATH"

# foundry
RUN curl -L https://foundry.paradigm.xyz | bash
ENV PATH="/home/ubuntu/.foundry/bin:$PATH"
RUN foundryup

# risc0
RUN curl -L https://risczero.com/install | bash 
ENV PATH="/home/ubuntu/.risc0/bin:$PATH"
RUN rzup install r0vm 2.0.2