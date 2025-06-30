# PromptPay-RS

Welcome to `promptpay-rs`, a lightweight Rust library for generating PromptPay QR code payloads compliant with EMVCo standards. This library is designed with modern Rust practices, providing a simple and efficient interface to create QR code payloads for Thai payment systems, supporting phone numbers and Tax IDs.

## Features

- **EMVCo Compliance**: Generates PromptPay QR code payloads adhering to EMVCo Merchant Presented Mode standards.
- **Flexible Input**: Supports Thai phone numbers, Tax IDs, and E-Wallet IDs with proper formatting.
- **No External Dependencies**: Pure Rust implementation for minimal overhead and maximum portability.
- **Builder Pattern**: Intuitive API for constructing payloads with optional amount specification.

## Installation

Add `promptpay-rs` to your `Cargo.toml`:

```toml
[dependencies]
promptpay-rs = "0.1.0"
```

Then run:

```bash
cargo build
```

## Usage

Here is a quick example of how to use `promptpay-rs` to generate a PromptPay QR code payload:

```rust
use promptpay_rs::PromptPayQR;

fn main() {
    let mut qr = PromptPayQR::new("0812345678");
    qr.set_amount(100.50);
    match qr.create() {
        Ok(result) => println!("Payload: {}", result.to_string()),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

This will generate a payload like:

```
00020101021229370016A000000677010111011300668123456785802TH53037645406100.506304XXXX
```

You can use this payload with a QR code generation library (e.g., `qrcode`) to create a scannable QR code for Thai banking apps.

## Documentation

Comprehensive documentation is available at [docs.rs/promptpay-rs](https://docs.rs/promptpay-rs).

## Contributing

Contributions are welcome! Please submit a pull request to branch `develop` or open an issue on the [GitHub repository](https://github.com/mantvmass/promptpay-rs) for bug reports, feature requests, or suggestions.

## Disclaimer

This library is unofficial and is not affiliated with or endorsed by PromptPay or the Bank of Thailand. Use it at your own risk.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.