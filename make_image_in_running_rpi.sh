#!/bin/bash

product_key=$(cat product_key)
#loopdev=$(losetup -f -P embassy.img --show)
root_mountpoint=""
boot_mountpoint="/boot"
#mkdir -p "${root_mountpoint}"
#mkdir -p "${boot_mountpoint}"
#mount "${loopdev}p2" "${root_mountpoint}"
#mount "${loopdev}p1" "${boot_mountpoint}"
mkdir "${root_mountpoint}/root/agent"
echo "${product_key}" > "${root_mountpoint}/root/agent/product_key"

# generate hostname from product_key
echo -n "start9-" > "${root_mountpoint}/etc/hostname"
echo -n "${product_key}" | shasum -t -a 256 | cut -c1-8 >> "${root_mountpoint}/etc/hostname"

# rebuild /etc/hosts with our hostname
cat "${root_mountpoint}/etc/hosts" | grep -v "127.0.1.1" > "${root_mountpoint}/etc/hosts.tmp"
echo -ne "127.0.1.1\tstart9-" >> "${root_mountpoint}/etc/hosts.tmp"
echo -n "${product_key}" | shasum -t -a 256 | cut -c1-8 >> "${root_mountpoint}/etc/hosts.tmp"
mv "${root_mountpoint}/etc/hosts.tmp" "${root_mountpoint}/etc/hosts"

# copy binaries, scripts and configs into place
cp agent/dist/agent "${root_mountpoint}/usr/local/bin/agent"
chmod 700 "${root_mountpoint}/usr/local/bin/agent"
cp appmgr/target/armv7-unknown-linux-gnueabihf/release/appmgr "${root_mountpoint}/usr/local/bin/appmgr"
chmod 700 "${root_mountpoint}/usr/local/bin/appmgr"
cp lifeline/target/armv7-unknown-linux-gnueabihf/release/lifeline "${root_mountpoint}/usr/local/bin/lifeline"
chmod 700 "${root_mountpoint}/usr/local/bin/lifeline"
cp docker-daemon.json "${root_mountpoint}/etc/docker/daemon.json"
cp setup.sh "${root_mountpoint}/root/setup.sh"
chmod 700 "${root_mountpoint}/root/setup.sh"
cp setup.service "${root_mountpoint}/etc/systemd/system/setup.service"
cp lifeline/lifeline.service "${root_mountpoint}/etc/systemd/system/lifeline.service"
cp agent/config/agent.service "${root_mountpoint}/etc/systemd/system/agent.service"

# save a copy of boot config.txt with simplified dtoverlay line
cat "${boot_mountpoint}/config.txt" | grep -v "dtoverlay=pwm-2chan" > "${boot_mountpoint}/config.txt.tmp"
echo "dtoverlay=pwm-2chan" >> "${boot_mountpoint}/config.txt.tmp"

# clean-up
#umount "${root_mountpoint}"
#rm -r "${root_mountpoint}"
#umount "${boot_mountpoint}"
#rm -r "${boot_mountpoint}"
#losetup -d ${loopdev}
