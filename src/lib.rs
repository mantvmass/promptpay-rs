//! # PromptPay QR Code Generator
//!
//! A Rust library for generating **PromptPay QR codes** according to the **EMVCo QR Code Specification**.
//! Supports mobile numbers, Tax ID, E-Wallet ID, with optional amount, and full CRC-16 validation.
//!
//! ## Features
//! - Static and Dynamic QR (with/without amount)
//! - Automatic target formatting (e.g. `0` → `66` for Thai mobile)
//! - CRC-16/CCITT calculation
//! - Output as `String` or `QrCode` image via `qrcode` crate
//! - Full error handling with `PromptPayError`
//!
//! ## Example
//! ```rust
//! use promptpay_rs::{PromptPayQR, FormatterTrait};
//!
//! let mut qr = PromptPayQR::new("0812345678");
//! qr.set_amount(150.75);
//! let payload = qr.create().unwrap().to_string();
//! println!("{}", payload); // EMVCo-compliant payload
//! ```

pub mod constants;
pub mod error;
pub mod formatter;
pub mod promptpay;
pub mod crc;
pub mod utils;

pub use qrcode;
pub use error::PromptPayError;
pub use formatter::{Formatter, FormatterTrait};
pub use promptpay::PromptPayQR;
pub use constants::{CountryCode, CurrencyCode};