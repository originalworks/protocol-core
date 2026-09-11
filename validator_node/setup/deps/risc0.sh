echo "Installing Risc0..."

curl -L https://risczero.com/install | bash
source $HOME/.bashrc
rzup install r0vm 3.0.6
