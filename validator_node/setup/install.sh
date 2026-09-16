echo "Installing apt packages..."
apt update && apt install gcc make npm pkg-config clang libclang-dev libc6-dev protobuf-compiler -y

./deps/docker.sh
./deps/cuda.sh
./deps/rust.sh
./deps/foundry.sh
# ./deps/risc0.sh
