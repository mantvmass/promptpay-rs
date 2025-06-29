# PromptPay-RS 🚀

Welcome to `promptpay-rs`, a powerful Rust library for generating PromptPay QR code payloads compliant with EMVCo standards. This library is designed with modern Rust practices and includes **WebAssembly support for TypeScript/JavaScript** applications.

## ✨ Features

### 🎯 Core Features
- **EMVCo Compliance**: Generates PromptPay QR code payloads adhering to EMVCo Merchant Presented Mode standards
- **Flexible Input**: Supports Thai phone numbers, Tax IDs, and E-Wallet IDs with proper formatting
- **No External Dependencies**: Pure Rust implementation for minimal overhead and maximum portability
- **Builder Pattern**: Intuitive API for constructing payloads with optional amount specification

### 🆕 New Features
- **QR Code Generation**: Generate QR codes in multiple formats (SVG, PNG, Base64, HTML)
- **Input Validation**: Comprehensive validation for Thai phone numbers, Tax IDs, and amounts
- **WebAssembly Support**: Use in TypeScript/JavaScript applications with full type safety
- **Multiple Output Formats**: Support for SVG, PNG, Base64 PNG, HTML img tags, and JSON
- **Advanced Configuration**: Customizable QR code appearance and validation settings
- **Error Handling**: Detailed error messages with proper error types
- **File Export**: Save QR codes as PNG or SVG files

## 🚀 Quick Start

### Rust Usage

Add `promptpay-rs` to your `Cargo.toml`:

```toml
[dependencies]
promptpay-rs = "0.1.0"
```

Basic usage:

```rust
use promptpay_rs::PromptPayQR;

fn main() {
    let mut qr = PromptPayQR::new("0812345678");
    qr.set_amount(100.50);
    
    match qr.generate_payload() {
        Ok(payload) => println!("Payload: {}", payload),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### TypeScript/JavaScript Usage

Install the package:

```bash
npm install promptpay-rs
```

Basic usage:

```typescript
import init, { PromptPayWasm } from 'promptpay-rs';

async function generateQR() {
    await init();
    
    const promptpay = new PromptPayWasm('0812345678');
    promptpay.set_amount(100.50);
    
    const svg = await promptpay.generate_svg();
    const payload = await promptpay.generate_payload();
    
    console.log('SVG:', svg);
    console.log('Payload:', payload);
}
```

## 📚 API Reference

### Rust API

#### Basic Usage

```rust
use promptpay_rs::{PromptPayQR, OutputFormat};

// Create QR code
let mut qr = PromptPayQR::new("0812345678");
qr.set_amount(100.50);

// Generate payload only
let payload = qr.generate_payload()?;

// Generate QR code with SVG
let result = qr.generate_qr(OutputFormat::SVG)?;
println!("SVG: {}", result.svg.unwrap());

// Generate all formats
let all_formats = qr.generate_qr(OutputFormat::All)?;
```

#### Advanced Configuration

```rust
use promptpay_rs::{PromptPayQR, PromptPayConfig};

let config = PromptPayConfig {
    country_code: "TH".to_string(),
    currency_code: "764".to_string(),
    qr_size: 512,
    qr_dark_color: "#1a1a1a".to_string(),
    qr_light_color: "#ffffff".to_string(),
    qr_quiet_zone: 8,
    validate_input: true,
};

let qr = PromptPayQR::with_config("0812345678", config);
```

#### Validation

```rust
use promptpay_rs::{Validator, validate_merchant_id};

// Validate merchant ID
let merchant_type = validate_merchant_id("0812345678")?;
println!("Merchant type: {}", merchant_type);

// Check specific types
let is_phone = Validator::is_valid_thai_phone("0812345678");
let is_tax_id = Validator::is_valid_tax_id("1234567890123");
let is_valid_amount = Validator::is_valid_amount(100.50);
```

### TypeScript/JavaScript API

#### Basic Usage

```typescript
import init, { PromptPayWasm, quick_generate_wasm } from 'promptpay-rs';

// Quick generation
const payload = await quick_generate_wasm('0812345678', 100.50);

// Class-based usage
const promptpay = new PromptPayWasm('0812345678');
promptpay.set_amount(100.50);

const svg = await promptpay.generate_svg();
const png = await promptpay.generate_base64_png();
const html = await promptpay.generate_html();
const all = await promptpay.generate_all();
```

#### Validation Functions

```typescript
import { 
    is_valid_thai_phone, 
    is_valid_tax_id, 
    is_valid_amount,
    sanitize_phone 
} from 'promptpay-rs';

// Validation
const isValidPhone = is_valid_thai_phone('0812345678');
const isValidTaxId = is_valid_tax_id('1234567890123');
const isValidAmount = is_valid_amount(100.50);

// Sanitization
const cleanPhone = sanitize_phone('081-234-5678'); // Returns: 66812345678
```

## 🎨 Output Formats

### Supported Formats

1. **Payload**: Raw EMVCo payload string
2. **SVG**: Scalable Vector Graphics
3. **PNG**: Portable Network Graphics (Base64 encoded)
4. **HTML**: HTML img tag with embedded Base64 image
5. **JSON**: Complete data structure with all information
6. **All**: All formats combined

### Example Outputs

```rust
// SVG Output
let svg = qr.generate_qr(OutputFormat::SVG)?;
// Returns: <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256">...</svg>

// Base64 PNG Output
let png = qr.generate_qr(OutputFormat::Base64PNG)?;
// Returns: data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAA...

// HTML Output
let html = qr.generate_qr(OutputFormat::HTML)?;
// Returns: <img src="data:image/png;base64,..." alt="PromptPay QR Code" width="256" height="256" />
```

## 🔧 Configuration

### QR Code Configuration

```rust
let config = PromptPayConfig {
    qr_size: 512,                    // QR code size in pixels
    qr_dark_color: "#000000",        // Dark color (QR code)
    qr_light_color: "#FFFFFF",       // Light color (background)
    qr_quiet_zone: 4,               // Quiet zone size
    validate_input: true,            // Enable input validation
    country_code: "TH",             // Country code
    currency_code: "764",           // Currency code (Thai Baht)
};
```

## 🧪 Testing

### Rust Tests

```bash
cargo test
```

### WebAssembly Tests

```bash
wasm-pack test --headless --firefox
wasm-pack test --headless --chrome
```

## 📦 Building

### Rust Library

```bash
cargo build
cargo build --release
```

### WebAssembly

```bash
# For web browsers
wasm-pack build --target web

# For Node.js
wasm-pack build --target nodejs

# For bundlers (webpack, rollup, etc.)
wasm-pack build --target bundler
```

## 🌐 Browser Support

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 79+

## 📋 Examples

### React Component

```tsx
import React, { useEffect, useState } from 'react';
import init, { PromptPayWasm } from 'promptpay-rs';

function PromptPayQR({ merchantId, amount }: { merchantId: string; amount?: number }) {
    const [svg, setSvg] = useState<string>('');
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        async function generateQR() {
            await init();
            const promptpay = new PromptPayWasm(merchantId);
            if (amount) promptpay.set_amount(amount);
            const svgResult = await promptpay.generate_svg();
            setSvg(svgResult);
            setLoading(false);
        }
        generateQR();
    }, [merchantId, amount]);

    if (loading) return <div>Generating QR Code...</div>;
    return <div dangerouslySetInnerHTML={{ __html: svg }} />;
}
```

### Vue Component

```vue
<template>
  <div>
    <div v-if="loading">Generating QR Code...</div>
    <div v-else v-html="svg"></div>
  </div>
</template>

<script>
import init, { PromptPayWasm } from 'promptpay-rs';

export default {
  props: ['merchantId', 'amount'],
  data() {
    return {
      svg: '',
      loading: true
    };
  },
  async mounted() {
    await init();
    const promptpay = new PromptPayWasm(this.merchantId);
    if (this.amount) promptpay.set_amount(this.amount);
    this.svg = await promptpay.generate_svg();
    this.loading = false;
  }
};
</script>
```

## 🤝 Contributing

Contributions are welcome! Please submit a pull request or open an issue on the [GitHub repository](https://github.com/mantvmass/promptpay-rs) for:

- Bug reports
- Feature requests
- Documentation improvements
- Performance optimizations

### Development Setup

1. Clone the repository
2. Install Rust and wasm-pack
3. Run `cargo build` to build the Rust library
4. Run `wasm-pack build` to build WebAssembly
5. Run `cargo test` to run tests

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This library is unofficial and is not affiliated with or endorsed by PromptPay or the Bank of Thailand. Use it at your own risk.

## 🆕 Changelog

### v0.1.0
- Initial release with basic PromptPay QR generation
- WebAssembly support for TypeScript/JavaScript
- QR code generation in multiple formats
- Comprehensive input validation
- Advanced configuration options
- File export capabilities

---

Made with ❤️ in Thailand 🇹🇭 