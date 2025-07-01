echo "Installing Foundry..."

curl -L https://foundry.paradigm.xyz | bash
echo "export PATH=\$PATH:/$HOME/.foundry/bin" >> $HOME/.bashrc
source $HOME/.bashrc

foundryup

source $HOME/.bashrc