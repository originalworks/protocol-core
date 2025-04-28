echo "Installing AWS-CLI..."
apt update
apt install unzip -y
wget -P /workspace https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip
unzip /workspace/awscli-exe-linux-x86_64.zip -d /workspace
./workspace/aws/install