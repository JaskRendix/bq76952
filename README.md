# BQ76952 Battery Monitor Library (STM32, `no_std`, Rust)

This project provides a Rust implementation of the Texas Instruments BQ76952 3‑to‑16‑cell battery monitor and protector.

It is based on TI’s reference documentation and verified on STM32 microcontrollers, rewritten to use `no_std` and `embedded-hal` traits.

The goal is to expose a safe, typed API for configuration, telemetry, protection, and FET/balancing control.

---

## Purpose of the Library

TI’s BQ76952 uses a command/data‑memory architecture that requires:

- structured subcommands  
- checksummed data‑memory writes  
- specific sequencing rules  

Vendor examples often rely on blocking I²C and manual register handling.

This Rust port provides:

- a `no_std` Rust API  
- safe data‑memory access with automatic checksum  
- predictable behavior under concurrency  
- compatibility with any `embedded-hal` I²C backend  
- a fully testable driver using mock I²C interfaces  

The functional behavior matches TI’s reference implementation.

---

## Upstream Source

This driver is based on:

- [https://github.com/skriachko/bq76952](https://github.com/skriachko/bq76952)

---

## Hardware

- Monitor/Protector: Texas Instruments BQ76952  
- Cells: 3–16 series Li‑ion / Li‑polymer / LiFePO₄  
- MCU Tested: STM32WB55  
- Interfaces: I²C, ALERT pin, TSx thermistors, FET drivers, cell inputs

---

## Rust Setup

The driver uses:

- `embedded-hal` for I²C  
- `no_std`  
- mockable I²C for testing  

It exposes:

- register access  
- subcommand access  
- data‑memory read/write  
- voltage, current, temperature telemetry  
- protection configuration  
- permanent fault monitoring  
- FET and cell‑balancing control  
- security/unseal handling  
- sleep/shutdown mode control  

---

## Telemetry

The BQ76952 provides:

- Cell voltages (1–16)  
- Stack voltage  
- Min/max cell voltage  
- Current measurement (CC2)  
- Temperatures: internal die, TS1/TS2/TS3, HDQ, DCHG, DDSG  

Kelvin‑to‑Celsius helpers included.

---

## Protection Modes

Supported protection categories:

- Cell overvoltage / undervoltage  
- Charge/discharge overcurrent  
- Short‑circuit protections  
- Temperature protections  
- Permanent Faults (PF)

The driver exposes:

- Safety Status A/B/C  
- PF Status  
- threshold and delay configuration  
- recovery behavior

---

## Security States

The BQ76952 supports:

- Sealed  
- Unsealed  
- Full Access  

The driver includes unseal key handling and state tracking.

---

## FET & Balancing Control

Supported operations:

- Charge FET  
- Discharge FET  
- Combined FET control  
- Manual cell balancing

---

## Power Modes

Supported:

- Normal  
- Sleep  
- Shutdown  

Driver includes helpers for entering/exiting modes.

---

## Data Memory Access

The BQ76952 uses a structured data‑memory map with checksums.

The driver provides:

- safe read/write  
- automatic checksum  
- typed accessors  
- raw access for advanced configuration

---

## Examples

- `monitor` — full monitoring loop: voltages, current, temperatures, safety status  

Run examples:

```bash
cargo run --example monitor
```

---

## Documentation

- BQ76952 Technical Reference Manual  
  `https://www.ti.com/lit/ug/sluuby2a/sluuby2a.pdf` [(ti.com in Bing)](https://www.bing.com/search?q="https%3A%2F%2Fwww.ti.com%2Flit%2Fug%2Fsluuby2a%2Fsluuby2a.pdf")  
- Datasheet  
  `https://www.ti.com/lit/ds/symlink/bq76952.pdf` [(ti.com in Bing)](https://www.bing.com/search?q="https%3A%2F%2Fwww.ti.com%2Flit%2Fds%2Fsymlink%2Fbq76952.pdf")  
- Software Development Guide  
  `https://www.ti.com/lit/an/sluaa11a/sluaa11a.pdf` [(ti.com in Bing)](https://www.bing.com/search?q="https%3A%2F%2Fwww.ti.com%2Flit%2Fan%2Fsluaa11a%2Fsluaa11a.pdf")
