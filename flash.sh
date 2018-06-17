#!/bin/sh
cargo build --release
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/testboard testboard.bin
sudo dfu-util -d 1209:70b1 -D testboard.bin
