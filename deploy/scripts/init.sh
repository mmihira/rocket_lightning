#!/bin/bash

sudo apt-get update
sudo apt-get -y install python
sudo apt-get -y install python-pip
sudo apt-get -y install tree
sudo apt-get -y install htop
sudo apt-get -y install vim

if [ $(wc -l < /proc/swaps) -eq 1 ]; then
  sudo fallocate -l 1G /swapfile
  sudo chmod 600 /swapfile
  sudo mkswap /swapfile
  sudo swapon /swapfile
fi
