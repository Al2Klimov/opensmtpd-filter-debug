## About

OpenSMTPd "filter" which logs all smtp-in events for debugging.

## Build

Compile like any other Rust program: `cargo build -r`

Find the resulting binary directly under `target/release/`.

## Usage

1. Integrate this "filter" into smtpd.conf(5)
2. Watch smtpd(8)'s log
