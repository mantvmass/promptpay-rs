use serde::{Deserialize, Serialize};
use crate::validation::MerchantType;

/// โครงสร้างสำหรับข้อมูล PromptPay ที่สามารถ serialize ได้
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptPayData {
    pub merchant_id: String,
    pub merchant_type: String,
    pub amount: Option<f64>,
    pub country_code: String,
    pub currency_code: String,
    pub payload: String,
}

/// โครงสร้างสำหรับผลลัพธ์การสร้าง QR Code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QRResult {
    pub payload: String,
    pub svg: Option<String>,
    pub png_base64: Option<String>,
    pub html_img: Option<String>,
    pub merchant_info: PromptPayData,
}

/// รูปแบบการ output ที่รองรับ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Payload,
    SVG,
    PNG,
    Base64PNG,
    HTML,
    JSON,
    All,
}

/// โครงสร้างสำหรับการตั้งค่าการสร้าง PromptPay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptPayConfig {
    pub country_code: String,
    pub currency_code: String,
    pub qr_size: u32,
    pub qr_dark_color: String,
    pub qr_light_color: String,
    pub qr_quiet_zone: u32,
    pub validate_input: bool,
}

impl Default for PromptPayConfig {
    fn default() -> Self {
        PromptPayConfig {
            country_code: "TH".to_string(),
            currency_code: "764".to_string(),
            qr_size: 256,
            qr_dark_color: "#000000".to_string(),
            qr_light_color: "#FFFFFF".to_string(),
            qr_quiet_zone: 4,
            validate_input: true,
        }
    }
}

impl PromptPayData {
    /// สร้าง instance ใหม่จากข้อมูลพื้นฐาน
    pub fn new(
        merchant_id: String,
        merchant_type: MerchantType,
        amount: Option<f64>,
        country_code: String,
        currency_code: String,
        payload: String,
    ) -> Self {
        PromptPayData {
            merchant_id,
            merchant_type: merchant_type.to_string(),
            amount,
            country_code,
            currency_code,
            payload,
        }
    }
    
    /// แปลงเป็น JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl QRResult {
    /// สร้าง instance ใหม่
    pub fn new(payload: String, merchant_info: PromptPayData) -> Self {
        QRResult {
            payload,
            svg: None,
            png_base64: None,
            html_img: None,
            merchant_info,
        }
    }
    
    /// เพิ่ม SVG
    pub fn with_svg(mut self, svg: String) -> Self {
        self.svg = Some(svg);
        self
    }
    
    /// เพิ่ม PNG Base64
    pub fn with_png_base64(mut self, png_base64: String) -> Self {
        self.png_base64 = Some(png_base64);
        self
    }
    
    /// เพิ่ม HTML img tag
    pub fn with_html_img(mut self, html_img: String) -> Self {
        self.html_img = Some(html_img);
        self
    }
    
    /// แปลงเป็น JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
} 