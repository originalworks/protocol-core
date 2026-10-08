FROM public.ecr.aws/amazonlinux/amazonlinux:2023

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"


RUN dnf install -y gcc gcc-c++ glibc-devel make python3-pip openssl openssl-devel tar gzip git nasm


RUN pip3 install cargo-lambda



