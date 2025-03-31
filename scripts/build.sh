#!/bin/bash

# Exit on error
set -e

# Build the bootloader
echo "Building bootloader..."
cargo build --release --target thumbv7em-none-eabihf

# Convert to binary
echo "Converting to binary..."
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/openblt openblt.bin

# Flash using JLink
echo "Flashing to device..."
JLinkExe -device S32K148 -if SWD -speed 4000 -CommanderScript flash.jlink

echo "Build and flash complete!" 
