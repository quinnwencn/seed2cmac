# Seed2cmac

A desktop application for calculating CMAC keys for automotive ECU security access operations. It provides a simple interface to generate security access keys from seed values using CMAC algorithms. The application supports various ECU types and security levels.

## Features

- User-friendly graphical interface built with Iced
- Support for multiple ECU types and security levels
- Hexadecimal input for seed and key values
- CMAC key calculation
- One-click copy to clipboard
- Input validation and error handling

## Requirements

- 64-bit operating system (Windows, Linux, or macOS)
- No additional runtime dependencies required

## Installation

### Building from Source

1. Ensure you have Rust and Cargo installed:
   
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   
   ```
   git clone [repository-url]
   cd seed2cmac
   ```

3. Build the application:
   
   ```
   cargo build --release
   ```

4. The executable will be available in `target/release/seed2cmac`

## Usage

1. Launch the application
2. Select the appropriate ECU type from the dropdown menu
3. Select the required security level
4. Enter the seed value in hexadecimal format (without 0x prefix)
5. Enter the key value in hexadecimal format (without 0x prefix)
6. Click "Calculate" to generate the CMAC key
7. Use the "Copy" button to copy the result to your clipboard
8. Click "Clear" to reset all inputs

## Notes

- Seed and key data must be 16 bytes in length, formatted as hexadecimal without "0x" prefix
- The calculated CMAC key will also be 16 bytes, displayed in hexadecimal format
