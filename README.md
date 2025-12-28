# security-controller
Security controller using embedded rust

# Development Environment
- Depends on rustup (taken care of by default.nix)
- Have to install ravedude from https://github.com/Rahix/avr-hal
- Must use nightly toolchain? (rust-toolchain.toml)
- .cargo/config.toml, .Cargo.toml, Ravedude.toml contents are very specific.

# Flashing
```
sudo avrdude -c usbtiny -p m328p -B 8MHz
```
Frequency should be 8 MHz for atmega328p running with 8MHz oscillator at 3.3V, typically (at 5V) it's at 16 MHz.