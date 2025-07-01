echo "Installing apt packages..."
apt update && apt install gcc make npm pkg-config -y

./deps/docker.sh
./deps/cuda.sh
./deps/rust.sh
./deps/foundry.sh
./deps/risc0.sh