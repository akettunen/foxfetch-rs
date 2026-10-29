# foxfetch-rs

**A fast, minimal Windows system fetcher written in Rust, featuring an orange fox ASCII logo.**

`foxfetch-rs` is a lightweight command-line utility that displays basic Windows system information in your terminal. It uses the Windows registry to retrieve system details and presents them alongside a simple fox logo.

## Example output

The output will look similar to the following. Actual values depend on your Windows installation and detected hardware.

```text
    |\__/|      os: windows
   /     \      kernel: 10.0.26100.1
  /_.~ ~,_\     gpu: NVIDIA GeForce RTX ...
     \@/        gpu: ...
```

## Requirements

- Windows
- [Rust and Cargo](https://www.rust-lang.org/tools/install)

The current implementation relies on the Windows registry and is not intended to run natively on Linux or macOS.

## Getting started

### Build from source

Clone the repository:

```bash
git clone https://github.com/akettunen/foxfetch-rs.git
cd foxfetch-rs
```

Build and run the application:

```bash
cargo run --release
```

To build the optimized executable without running it:

```bash
cargo build --release
```

The resulting executable is located at:

```text
target/release/foxfetch-rs.exe
```

You can run it directly from PowerShell:

```powershell
.\target\release\foxfetch-rs.exe
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
