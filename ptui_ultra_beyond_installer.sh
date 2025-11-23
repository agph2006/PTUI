
#!/bin/bash
set -e
apt-get update
apt-get install -y curl build-essential zerotier-one virt-viewer spice-client-gtk bridge-utils ifstat iproute2 net-tools zfsutils-linux sysstat lm-sensors smartmontools espeak brltty python3 python3-pip jq dialog git wget avahi-daemon isc-dhcp-relay

curl https://sh.rustup.rs -sSf | sh -s -- -y
source ~/.cargo/env

cp -r /mnt/data/PTUI_ULTRA_BEYOND /root/ptui_ultra_beyond
cd /root/ptui_ultra_beyond
cargo build --release
echo "/root/ptui_ultra_beyond/target/release/ptui_ultra_beyond" >> ~/.bash_profile
