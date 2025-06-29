pub mod error;
pub mod validation;
pub mod qr_generator;
pub mod types;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use error::PromptPayError;
pub use validation::{Validator, MerchantType};
pub use qr_generator::{QRGenerator, QRConfig};
pub use types::{PromptPayData, QRResult, OutputFormat, PromptPayConfig};

use crate::validation::MerchantType;
use crate::types::{PromptPayData, QRResult, OutputFormat, PromptPayConfig};

/// โครงสร้างสำหรับสร้าง PromptPay QR code ตามมาตรฐาน EMVCo
pub struct PromptPayQR {
    merchant_id: String,
    amount: Option<f64>,
    config: PromptPayConfig,
}

impl PromptPayQR {
    /// สร้าง instance ใหม่ของ `PromptPayQR`
    pub fn new(merchant_id: &str) -> Self {
        PromptPayQR {
            merchant_id: merchant_id.to_string(),
            amount: None,
            config: PromptPayConfig::default(),
        }
    }

    /// สร้าง instance ใหม่ด้วยการตั้งค่าที่กำหนดเอง
    pub fn with_config(merchant_id: &str, config: PromptPayConfig) -> Self {
        PromptPayQR {
            merchant_id: merchant_id.to_string(),
            amount: None,
            config,
        }
    }

    /// กำหนดจำนวนเงินสำหรับการทำธุรกรรม
    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        if self.config.validate_input && !Validator::is_valid_amount(amount) {
            return self;
        }
        self.amount = Some(amount);
        self
    }

    /// กำหนดการตั้งค่า
    pub fn set_config(&mut self, config: PromptPayConfig) -> &mut Self {
        self.config = config;
        self
    }

    /// ตรวจสอบ merchant ID
    pub fn validate(&self) -> Result<(), PromptPayError> {
        if self.config.validate_input {
            Validator::validate_merchant_id(&self.merchant_id)?;
        }
        Ok(())
    }

    /// รับประเภทของ merchant ID
    pub fn get_merchant_type(&self) -> MerchantType {
        Validator::identify_merchant_type(&self.merchant_id)
    }

    /// สร้าง payload สำหรับ QR Code PromptPay
    pub fn generate_payload(&self) -> Result<String, PromptPayError> {
        if self.config.validate_input {
            self.validate()?;
        }

        if self.merchant_id.is_empty() {
            return Err(PromptPayError::MissingMerchantId);
        }

        let mut payload = String::new();

        // Payload Format Indicator
        payload.push_str("000201");

        // Point of Initiation Method
        payload.push_str(if self.amount.is_some() { "010212" } else { "010211" });

        // Merchant Account Information
        let mut merchant_info = String::new();
        merchant_info.push_str("0016A000000677010111");
        
        let target_type = match self.get_merchant_type() {
            MerchantType::Phone => "01",
            MerchantType::TaxId => "02",
            MerchantType::EWallet => "03",
            MerchantType::Unknown => "01", // Default to phone
        };
        
        let formatted_target = self.format_target(&self.merchant_id);
        let merchant_id_field = format!(
            "{}{:02}{}",
            target_type,
            formatted_target.len(),
            formatted_target
        );
        merchant_info.push_str(&merchant_id_field);

        let merchant_info_len = format!("{:02}", merchant_info.len());
        payload.push_str(&format!("29{}", merchant_info_len));
        payload.push_str(&merchant_info);

        // Country Code
        payload.push_str(&format!("5802{}", self.config.country_code));

        // Currency Code
        payload.push_str(&format!("5303{}", self.config.currency_code));

        // Amount
        if let Some(amount) = self.amount {
            let amount_str = format!("{:.2}", amount);
            let amount_len = format!("{:02}", amount_str.len());
            payload.push_str(&format!("54{}", amount_len));
            payload.push_str(&amount_str);
        }

        // CRC
        payload.push_str("6304");
        let crc = self.calculate_crc(&payload);
        payload.push_str(&format!("{:04X}", crc));

        Ok(payload)
    }

    /// สร้าง QR Code ในรูปแบบต่างๆ
    pub fn generate_qr(&self, format: OutputFormat) -> Result<QRResult, PromptPayError> {
        let payload = self.generate_payload()?;
        let merchant_type = self.get_merchant_type();
        
        let merchant_data = PromptPayData::new(
            self.merchant_id.clone(),
            merchant_type,
            self.amount,
            self.config.country_code.clone(),
            self.config.currency_code.clone(),
            payload.clone(),
        );

        let mut result = QRResult::new(payload, merchant_data);

        match format {
            OutputFormat::Payload => Ok(result),
            OutputFormat::SVG => {
                let svg = QRGenerator::generate_svg(&result.payload, self.config.qr_size)?;
                Ok(result.with_svg(svg))
            }
            OutputFormat::Base64PNG => {
                let png_base64 = QRGenerator::generate_base64_png(&result.payload, self.config.qr_size)?;
                Ok(result.with_png_base64(png_base64))
            }
            OutputFormat::HTML => {
                let html_img = QRGenerator::generate_html_img(&result.payload, self.config.qr_size, None)?;
                Ok(result.with_html_img(html_img))
            }
            OutputFormat::All => {
                let svg = QRGenerator::generate_svg(&result.payload, self.config.qr_size)?;
                let png_base64 = QRGenerator::generate_base64_png(&result.payload, self.config.qr_size)?;
                let html_img = QRGenerator::generate_html_img(&result.payload, self.config.qr_size, None)?;
                
                Ok(result
                    .with_svg(svg)
                    .with_png_base64(png_base64)
                    .with_html_img(html_img))
            }
            _ => Ok(result),
        }
    }

    /// บันทึก QR Code เป็นไฟล์
    pub fn save_qr(&self, file_path: &str, format: &str) -> Result<(), PromptPayError> {
        let payload = self.generate_payload()?;
        
        match format.to_lowercase().as_str() {
            "png" => QRGenerator::save_png(&payload, file_path, self.config.qr_size),
            "svg" => QRGenerator::save_svg(&payload, file_path, self.config.qr_size),
            _ => Err(PromptPayError::ImageGenerationFailed("Unsupported format".to_string())),
        }
    }

    // Helper methods
    fn sanitize_target(&self, id: &str) -> String {
        Validator::sanitize_numbers(id)
    }

    fn format_target(&self, id: &str) -> String {
        let numbers = self.sanitize_target(id);
        if numbers.len() >= 13 {
            numbers
        } else {
            let formatted = numbers.replace("0", "66");
            format!("{:0>13}", formatted)
        }
    }

    fn calculate_crc(&self, data: &str) -> u16 {
        let mut crc: u16 = 0xFFFF;
        let polynomial: u16 = 0x1021;

        for byte in data.bytes() {
            crc ^= (byte as u16) << 8;
            for _ in 0..8 {
                if (crc & 0x8000) != 0 {
                    crc = (crc << 1) ^ polynomial;
                } else {
                    crc <<= 1;
                }
            }
        }
        crc
    }
}

// Convenience functions
/// สร้าง PromptPay QR Code อย่างรวดเร็ว
pub fn quick_generate(merchant_id: &str, amount: Option<f64>) -> Result<String, PromptPayError> {
    let mut qr = PromptPayQR::new(merchant_id);
    if let Some(amt) = amount {
        qr.set_amount(amt);
    }
    qr.generate_payload()
}

/// สร้าง PromptPay QR Code พร้อม SVG
pub fn generate_with_svg(merchant_id: &str, amount: Option<f64>) -> Result<QRResult, PromptPayError> {
    let mut qr = PromptPayQR::new(merchant_id);
    if let Some(amt) = amount {
        qr.set_amount(amt);
    }
    qr.generate_qr(OutputFormat::SVG)
}

/// ตรวจสอบ merchant ID
pub fn validate_merchant_id(merchant_id: &str) -> Result<MerchantType, PromptPayError> {
    Validator::validate_merchant_id(merchant_id)?;
    Ok(Validator::identify_merchant_type(merchant_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_generate() {
        let result = quick_generate("0812345678", Some(100.50));
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert!(payload.starts_with("000201"));
    }

    #[test]
    fn test_generate_with_svg() {
        let result = generate_with_svg("0812345678", Some(100.50));
        assert!(result.is_ok());
        let qr_result = result.unwrap();
        assert!(qr_result.svg.is_some());
    }

    #[test]
    fn test_validate_merchant_id() {
        let result = validate_merchant_id("0812345678");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), MerchantType::Phone);
    }
}
