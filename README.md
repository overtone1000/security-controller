# security-controller
Security controller using embedded rust

# Development Environment
- Depends on rustup (taken care of by default.nix)
- Have to install ravedude from https://github.com/Rahix/avr-hal
- Must use nightly toolchain? (rust-toolchain.toml) Yes, and need to use the version from the commit to avr-hal repo in its rust-toolchain.toml.
- .cargo/config.toml, .Cargo.toml, Ravedude.toml contents are very specific.

# Flashing
```
sudo avrdude -c usbtiny -p m328p -B 32 -v
```
Frequency should be 8 MHz for atmega328p running with 8MHz oscillator at 3.3V, typically (at 5V) it's at 16 MHz.

# Binary Size Investigation

```
cargo-tree tree
cargo-size
cargo bloat --crates --full-fn
```

To run these if the build is failing due to build size, change `.cargo/config.toml` to target a bigger board.

smoltcp makes the binary prohibitively large. This is unnecessary; the w5500 has a tcp stack on it!

Should adapt Ethernet_W5500 library?
https://github.com/RoboCore/Ethernet_W5500
This seems like a real pain.

Switch to something with more flash memory like W5500-EVB-Pico (better hardware: RP2040)?
https://thepihut.com/products/wiznet-w5100s-evb-pico-rp2040-board-with-ethernet
Would need to check the power for everything.

Try this crate?
https://crates.io/crates/w5500-hl