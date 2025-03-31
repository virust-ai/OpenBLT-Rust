# OpenBLT-Rust

A Rust implementation of the OpenBLT bootloader for NXP S32K148 microcontrollers. This project provides a secure and efficient bootloader solution with CAN and UART communication interfaces.

## Features

- Bare metal Rust implementation (`no_std`)
- CAN communication interface for programming
- UART debug interface
- Flash programming capabilities
- Safe hardware abstraction layer (HAL)
- Support for application validation and jumping

## Hardware Requirements

- NXP S32K148 evaluation board
- CAN transceiver (e.g., TJA1050)
- UART-to-USB converter (for debug output)
- Programming pin (configurable GPIO)

## Software Requirements

- Rust toolchain (latest stable)
- `arm-none-eabi-gcc` for C startup code
- OpenOCD for programming
- CAN monitoring tool (e.g., `candump` from `can-utils`)

## Project Structure

```
.
├── hal/                    # Hardware Abstraction Layer
│   └── s32k148-hal/       # S32K148 specific HAL implementation
├── openblt/               # OpenBLT core implementation
│   └── boards/
│       └── s32k148/      # S32K148 board support
│           ├── linker/   # Linker scripts
│           ├── startup/  # C startup code
│           └── src/      # Board specific code
└── .cargo/               # Cargo configuration
```

## Building

1. Clone the repository:
```bash
git clone https://github.com/yourusername/OpenBLT-Rust.git
cd OpenBLT-Rust
```

2. Build the bootloader:
```bash
# Debug build
cargo build -p s32k148-board --bin s32k148-bootloader

# Release build (optimized)
cargo build -p s32k148-board --bin s32k148-bootloader --release
```

The compiled binary will be located at:
- Debug: `target/thumbv7em-none-eabihf/debug/s32k148-bootloader`
- Release: `target/thumbv7em-none-eabihf/release/s32k148-bootloader`

## Programming

1. Convert the binary to S19 format:
```bash
arm-none-eabi-objcopy -O srec target/thumbv7em-none-eabihf/release/s32k148-bootloader bootloader.s19
```

2. Program using OpenOCD:
```bash
openocd -f openocd.cfg -c "program bootloader.s19 verify reset exit"
```

## Testing

### Hardware Setup

1. Connect the CAN transceiver:
   - CAN_H → Board CAN_H
   - CAN_L → Board CAN_L
   - VCC → 5V
   - GND → GND

2. Connect the UART debug interface:
   - TX → Board UART TX
   - RX → Board UART RX
   - GND → GND

3. Connect the programming pin:
   - Configured GPIO → GND (for programming mode)

### Testing Steps

1. Basic Boot Test:
   - Power cycle the board
   - Check UART output for bootloader startup message

2. CAN Programming Test:
   - Send CAN message with ID 0x7E0 to trigger programming mode
   - Verify UART output indicates programming mode entry

3. Flash Programming Test:
   - Send test application binary through CAN
   - Verify flash programming process
   - Check UART output for progress

4. Application Jump Test:
   - Verify bootloader jumps to application
   - Check application execution

## Development

### Adding New Features

1. HAL Layer:
   - Add new peripheral support in `hal/s32k148-hal/`
   - Implement safe abstractions for hardware access
   - Add proper error handling

2. Board Support:
   - Add board-specific implementations in `openblt/boards/s32k148/`
   - Configure hardware resources
   - Implement board-specific features

### Debugging

1. UART Debug Output:
   - Use `debug_println!` macro for debug messages
   - Configure baud rate in `uart.rs`

2. CAN Monitoring:
   - Use `candump` to monitor CAN traffic
   - Configure CAN bitrate in `can.rs`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- OpenBLT project for the original bootloader concept
- NXP for the S32K148 microcontroller
- Rust embedded community for tools and libraries 
```
cargo b -p s32k148-board --bin s32k148-bootloader
```
