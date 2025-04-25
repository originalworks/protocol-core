FROM ubuntu:24.04

# docker CLI (without docker daemon)
RUN apt-get update && apt-get install curl -y
RUN install -m 0755 -d /etc/apt/keyrings 
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc 
RUN chmod a+r /etc/apt/keyrings/docker.asc 
RUN echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
    $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}") stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null

RUN apt-get update && apt install wget npm pkg-config docker-ce-cli -y
RUN npm install -g @web3-storage/w3cli

# cuda
RUN wget https://developer.download.nvidia.com/compute/cuda/12.8.1/local_installers/cuda_12.8.1_570.124.06_linux.run
RUN sh cuda_12.8.1_570.124.06_linux.run --silent --toolkit
ENV PATH="/usr/local/cuda-12.8/bin:$PATH"
ENV LD_LIBRARY_PATH="/usr/local/cuda-12.8/lib64:$PATH"

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
RUN rzup install r0vm 1.2.0


RUN git clone https://github.com/originalworks/protocol-core.git

# initiate w3
# !!! it will ask to click the e-mail link during the build
# *temporary solution until we have an endpoint to upload files
USER root
RUN w3 login cezary.stroczynski@rumblefish.pl && w3 space use did:key:z6Mkeq9K7nxMYK3xu9SSnq6ova1xrLV8NyRLZL89NrS2RxTf

# compile contracts and build validator
RUN cd protocol-core/contracts && npm i && forge build && npx hardhat compile
RUN cd protocol-core/validator_node && cargo build --release -F cuda