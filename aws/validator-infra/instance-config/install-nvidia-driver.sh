echo "Installing NVIDIA drivers..."
wget https://us.download.nvidia.com/tesla/570.124.06/nvidia-driver-local-repo-ubuntu2404-570.124.06_1.0-1_amd64.deb
cp /var/nvidia-driver-local-repo-ubuntu2404-570.124.06/nvidia-driver-local-D67F55A1-keyring.gpg /usr/share/keyrings/
dpkg -i nvidia-driver-local-repo-ubuntu2404-570.124.06_1.0-1_amd64.deb