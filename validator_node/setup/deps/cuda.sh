echo "Installing NVIDIA and CUDA..."
wget -P /workspace -q https://developer.download.nvidia.com/compute/cuda/12.9.1/local_installers/cuda_12.9.1_575.57.08_linux.run
sh /workspace/cuda_12.9.1_575.57.08_linux.run --silent --toolkit --driver
echo "export PATH=/usr/local/cuda/bin:\$PATH" >> $HOME/.bashrc
echo "export LD_LIBRARY_PATH=/usr/local/cuda/lib64:\$LD_LIBRARY_PATH" >> $HOME/.bashrc
source $HOME/.bashrc