curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
exec /bin/bash
dnf install -y gcc gcc-c++ glibc-devel make python3-pip openssl openssl-devel
pip3 install cargo-lambda