use wasm_bindgen::prelude::*;
use js_sys::{Object, JsString};
use serde::{Serialize, Deserialize};
use crate::{
    PromptPayQR, PromptPayError, OutputFormat, PromptPayConfig,
    QRResult, PromptPayData, MerchantType, Validator
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

/// WebAssembly wrapper สำหรับ PromptPay QR Generator
#[wasm_bindgen]
pub struct PromptPayWasm {
    qr: PromptPayQR,
}

#[wasm_bindgen]
impl PromptPayWasm {
    /// สร้าง instance ใหม่
    #[wasm_bindgen(constructor)]
    pub fn new(merchant_id: &str) -> Self {
        PromptPayWasm {
            qr: PromptPayQR::new(merchant_id),
        }
    }

    /// สร้าง instance ใหม่ด้วยการตั้งค่า
    pub fn with_config(merchant_id: &str, config: JsValue) -> Result<PromptPayWasm, JsValue> {
        let config: PromptPayConfig = config.into_serde()
            .map_err(|e| JsValue::from_str(&format!("Invalid config: {}", e)))?;
        
        Ok(PromptPayWasm {
            qr: PromptPayQR::with_config(merchant_id, config),
        })
    }

    /// กำหนดจำนวนเงิน
    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        self.qr.set_amount(amount);
        self
    }

    /// ตรวจสอบ merchant ID
    pub fn validate(&self) -> Result<JsValue, JsValue> {
        match self.qr.validate() {
            Ok(()) => Ok(JsValue::TRUE),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    /// รับประเภทของ merchant ID
    pub fn get_merchant_type(&self) -> String {
        self.qr.get_merchant_type().to_string()
    }

    /// สร้าง payload
    pub fn generate_payload(&self) -> Result<String, JsValue> {
        self.qr.generate_payload()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// สร้าง QR Code ในรูปแบบต่างๆ
    pub fn generate_qr(&self, format: &str) -> Result<JsValue, JsValue> {
        let output_format = match format {
            "payload" => OutputFormat::Payload,
            "svg" => OutputFormat::SVG,
            "png" => OutputFormat::PNG,
            "base64png" => OutputFormat::Base64PNG,
            "html" => OutputFormat::HTML,
            "json" => OutputFormat::JSON,
            "all" => OutputFormat::All,
            _ => return Err(JsValue::from_str("Invalid format")),
        };

        let result = self.qr.generate_qr(output_format)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        JsValue::from_serde(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// สร้าง QR Code พร้อม SVG
    pub fn generate_svg(&self) -> Result<String, JsValue> {
        let result = self.qr.generate_qr(OutputFormat::SVG)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        result.svg.ok_or_else(|| JsValue::from_str("SVG generation failed"))
    }

    /// สร้าง QR Code พร้อม Base64 PNG
    pub fn generate_base64_png(&self) -> Result<String, JsValue> {
        let result = self.qr.generate_qr(OutputFormat::Base64PNG)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        result.png_base64.ok_or_else(|| JsValue::from_str("PNG generation failed"))
    }

    /// สร้าง QR Code พร้อม HTML img tag
    pub fn generate_html(&self) -> Result<String, JsValue> {
        let result = self.qr.generate_qr(OutputFormat::HTML)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        result.html_img.ok_or_else(|| JsValue::from_str("HTML generation failed"))
    }

    /// สร้าง QR Code ทั้งหมด
    pub fn generate_all(&self) -> Result<JsValue, JsValue> {
        let result = self.qr.generate_qr(OutputFormat::All)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        JsValue::from_serde(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
}

/// ฟังก์ชัน convenience สำหรับการใช้งานอย่างรวดเร็ว
#[wasm_bindgen]
pub fn quick_generate_wasm(merchant_id: &str, amount: Option<f64>) -> Result<String, JsValue> {
    crate::quick_generate(merchant_id, amount)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// ฟังก์ชัน convenience สำหรับการสร้าง QR Code พร้อม SVG
#[wasm_bindgen]
pub fn generate_with_svg_wasm(merchant_id: &str, amount: Option<f64>) -> Result<JsValue, JsValue> {
    let result = crate::generate_with_svg(merchant_id, amount)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    JsValue::from_serde(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// ฟังก์ชันสำหรับตรวจสอบ merchant ID
#[wasm_bindgen]
pub fn validate_merchant_id_wasm(merchant_id: &str) -> Result<String, JsValue> {
    let merchant_type = crate::validate_merchant_id(merchant_id)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(merchant_type.to_string())
}

/// ฟังก์ชันสำหรับตรวจสอบเบอร์โทรศัพท์ไทย
#[wasm_bindgen]
pub fn is_valid_thai_phone(phone: &str) -> bool {
    Validator::is_valid_thai_phone(phone)
}

/// ฟังก์ชันสำหรับตรวจสอบ Tax ID
#[wasm_bindgen]
pub fn is_valid_tax_id(tax_id: &str) -> bool {
    Validator::is_valid_tax_id(tax_id)
}

/// ฟังก์ชันสำหรับตรวจสอบ E-Wallet ID
#[wasm_bindgen]
pub fn is_valid_ewallet_id(ewallet_id: &str) -> bool {
    Validator::is_valid_ewallet_id(ewallet_id)
}

/// ฟังก์ชันสำหรับตรวจสอบจำนวนเงิน
#[wasm_bindgen]
pub fn is_valid_amount(amount: f64) -> bool {
    Validator::is_valid_amount(amount)
}

/// ฟังก์ชันสำหรับทำความสะอาดเบอร์โทรศัพท์
#[wasm_bindgen]
pub fn sanitize_phone(phone: &str) -> String {
    Validator::sanitize_phone(phone)
}

/// ฟังก์ชันสำหรับทำความสะอาดตัวเลข
#[wasm_bindgen]
pub fn sanitize_numbers(input: &str) -> String {
    Validator::sanitize_numbers(input)
}

/// ฟังก์ชันสำหรับระบุประเภทของ merchant ID
#[wasm_bindgen]
pub fn identify_merchant_type(merchant_id: &str) -> String {
    Validator::identify_merchant_type(merchant_id).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_quick_generate_wasm() {
        let result = quick_generate_wasm("0812345678", Some(100.50));
        assert!(result.is_ok());
        let payload = result.unwrap();
        assert!(payload.starts_with("000201"));
    }

    #[wasm_bindgen_test]
    fn test_validate_merchant_id_wasm() {
        let result = validate_merchant_id_wasm("0812345678");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Phone");
    }

    #[wasm_bindgen_test]
    fn test_is_valid_thai_phone() {
        assert!(is_valid_thai_phone("0812345678"));
        assert!(is_valid_thai_phone("66812345678"));
        assert!(!is_valid_thai_phone("1234567890"));
    }
} 