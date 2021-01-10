#!/bin/bash

product_key=$(cat product_key)
loopdev=$(losetup -f -P embassy.img --show)
root_mountpoint="/mnt/start9-${product_key}-root"
boot_mountpoint="/mnt/start9-${product_key}-boot"
umount "${root_mountpoint}"
rm -r "${root_mountpoint}"
umount "${boot_mountpoint}"
rm -r "${boot_mountpoint}"
losetup -d ${loopdev}
