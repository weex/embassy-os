#!/bin/bash

product_key=$(cat product_key)
loopdev=$(losetup -f -P embassy.img --show)
root_mountpoint="/mnt/start9-${product_key}-root"
boot_mountpoint="/mnt/start9-${product_key}-boot"
mkdir -p "${root_mountpoint}"
mkdir -p "${boot_mountpoint}"
mount "${loopdev}p2" "${root_mountpoint}"
mount "${loopdev}p1" "${boot_mountpoint}"
