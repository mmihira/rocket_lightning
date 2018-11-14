#!/bin/bash
export TZ=Australia/Melbourne
sudo su root -c "ln -snf /usr/share/zoneinfo/$TZ /etc/localtime"
sudo su root -c "echo $TZ > /etc/timezone"
sudo su root -c "dpkg-reconfigure -f noninteractive tzdata"

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
