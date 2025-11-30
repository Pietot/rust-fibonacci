# 🔢 Rust Figonacci

![Location](https://img.shields.io/badge/Made_in-France-red?labelColor=blue)
![Language](https://img.shields.io/badge/Language-Rust-black?labelColor=f74c00)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/Pietot/rust-figonacci)

This repo is a simple implementation of the fastest algorithm (I found) to compute the largest Fibonacci numbers under a second. It's uses GMP (GNU Multiple Precision Arithmetic Library) via the `rug` crate to handle large integers efficiently.

## 📋 Summary

### 1. [Installation](#1---installation)
    
### 2. [Credits](#2---credits)


### 1 - Installation
Since it uses the `rug` crate, you need to have a Linux system or WSL with GMP installed. 

You can download the latest release from the [releases page](https://github.com/Pietot/rust-figonacci/releases/latest) and run it.

Or you can download the source code / clone the repo and build it:

```bash
cargo build --release
```

## 2 - Credits

- **[Algorithm explained](https://www.youtube.com/watch?v=6ZyTqfFCnjg)** by Pihedron.
