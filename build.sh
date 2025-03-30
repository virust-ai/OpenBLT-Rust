#!/bin/bash

# Exit on error
set -e

# Build the bootloader
echo "Building bootloader..."
cargo build --release --target thumbv7em-none-eabihf

# Convert to binary format
echo "Converting to binary..."
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/openblt openblt.bin

# Flash the bootloader
echo "Flashing bootloader..."
# TODO: Replace with actual flashing command based on your programmer
# Example for SEGGER J-Link:
# JLinkExe -device S32K148 -if SWD -speed 4000 -CommanderScript flash.jlink

echo "Build and flash complete!" 
