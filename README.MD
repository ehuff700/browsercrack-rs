# browsercrack-rs

**browsercrack-rs** is a Rust-based library designed to extract and decrypt passwords stored in popular web browsers. This library is aimed at security professionals and developers who need to audit or recover passwords for security testing, forensics, or other legitimate purposes.

## Features

- **Multi-Browser Support**: Supports popular web browsers such as Google Chrome, Mozilla Firefox, Microsoft Edge, and more.
- **Decryption of Stored Passwords**: Decrypts passwords stored in browser-specific encrypted formats.
- **Cross-Platform**: Works on Windows, macOS, and Linux.
- **Efficient and Fast**: Built in Rust for high performance and memory safety.
- **Easy Integration**: Simple API for integrating into existing Rust projects.

## Installation

To use `browsercrack-rs` in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
browsercrack-rs = "0.1.0"
