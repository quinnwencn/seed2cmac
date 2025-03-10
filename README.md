# Introduction

Seed2CMAC is a command-line tool designed to demonstrate the functionality of the UDS (Unified Diagnostics Service) SecurityAccess service. Specifically, it simulates the process of generating a Cryptographic Message Authentication Code (CMAC) based on a provided seed and key, which is a common step in the SecurityAccess authentication process. This tool is particularly useful for developers, testers, and engineers working on automotive diagnostic systems or embedded software where UDS protocol compliance is required.

# How It Works

In the UDS protocol, the SecurityAccess service ensures secure access between an external diagnostic tool and an ECU (Electronic Control Unit). The process typically involves the following steps:

1. The diagnostic tool requests a "seed" from the ECU.
2. The ECU generates and sends a random seed value to the diagnostic tool.
3. The diagnostic tool computes a CMAC using the seed and a pre-shared key.
4. The computed CMAC is sent back to the ECU for verification.
5. If the CMAC is valid, access is granted.
`Seed2CMAC` simplifies this process by allowing users to manually input a seed and key to generate the corresponding CMAC, enabling testing and validation of the SecurityAccess service logic.

# Examples

1. Generate CMAC with a specific seed and key:
```bash
seed2cmac -s fbfc3cb800dd58abd5142231a4a0a053 -k f9ddec4c374f98578aa06e78d1ab7a24  
```
Output:
```
CMAC: 1da90ff600d8fa94111f47f805053b6c
```

2. Display help info
```bash
seed2cmac -h
```