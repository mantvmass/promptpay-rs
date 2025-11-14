use qrcode::{EcLevel, QrCode};
use std::error::Error;
use std::fmt;

// re-export qrcode
pub use qrcode;

use crate::constants::{CountryCode, CurrencyCode};
pub mod constants;

/// ข้อผิดพลาดที่เกิดขึ้นในระหว่างการสร้าง PromptPay QR code
#[derive(Debug)]
pub struct PromptPayError {
    details: String,
}

impl PromptPayError {
    /// สร้าง instance ใหม่ของ `PromptPayError` ด้วยข้อความข้อผิดพลาด
    /// # Arguments
    /// * `msg` - ข้อความที่อธิบายข้อผิดพลาด
    /// # Returns
    /// instance ของ `PromptPayError`
    fn new(msg: &str) -> PromptPayError {
        PromptPayError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for PromptPayError {
    /// จัดรูปแบบการแสดงผลข้อผิดพลาดสำหรับ `PromptPayError`
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for PromptPayError {
    /// คืนค่าคำอธิบายของข้อผิดพลาด
    fn description(&self) -> &str {
        &self.details
    }
}

/// โครงสร้างสำหรับสร้าง PromptPay QR code ตามมาตรฐาน EMVCo
pub struct PromptPayQR {
    merchant_id: String,       // รหัสผู้รับเงิน (เช่น เบอร์โทรศัพท์, Tax ID, หรือ E-Wallet ID)
    amount: Option<f64>,       // จำนวนเงิน (ถ้ามี)
    country_code: CountryCode, // รหัสประเทศ (เช่น "TH" สำหรับประเทศไทย)
    currency_code: CurrencyCode, // รหัสสกุลเงิน (เช่น "764" สำหรับบาทไทย)
}

/// Trait สำหรับ Formatter ที่สามารถแปลงผลลัพธ์เป็นรูปแบบต่างๆ
pub trait FormatterTrait {
    /// แปลง payload เป็น String
    fn to_string(&self) -> String;
    fn to_image(&self, ec_level: EcLevel) -> Result<QrCode, PromptPayError>;
}

/// โครงสร้างสำหรับจัดการผลลัพธ์
#[derive(Debug)]
pub struct Formatter {
    payload: String,
}

impl Formatter {
    /// สร้าง instance ใหม่ของ `Formatter`
    /// # Arguments
    /// * `payload` - ข้อมูลที่ได้จากการสร้าง QRCode
    /// # Returns
    /// instance ของ `Formatter`
    pub fn new(payload: &str) -> Self {
        Self {
            payload: payload.to_string(),
        }
    }
}

impl FormatterTrait for Formatter {
    /// คืนค่า payload ในรูปแบบ String
    fn to_string(&self) -> String {
        self.payload.clone()
    }

    /// สร้าง QrCode (0.14.1) instance จาก payload
    /// # Arguments
    /// * `version` - รุ่นของ QR Code
    /// * `ec_level` - ระดับการแก้ไขข้อผิดพลาด
    /// # Returns
    /// `Result` ที่มี `QrCode` หากสำเร็จ หรือ `PromptPayError` หากล้มเหลว
    fn to_image(&self, ec_level: EcLevel) -> Result<QrCode, PromptPayError> {
        if self.payload.is_empty() {
            return Err(PromptPayError::new("Payload cannot be empty"));
        }

        QrCode::with_error_correction_level(self.payload.as_bytes(), ec_level)
            .map_err(|e| PromptPayError::new(&format!("Failed to create QRCode: {}", e)))
    }
}

impl PromptPayQR {
    /// สร้าง instance ใหม่ของ `PromptPayQR`
    /// # Arguments
    /// * `merchant_id` - รหัสผู้รับเงิน (เบอร์โทรศัพท์, Tax ID, หรือ E-Wallet ID)
    /// # Returns
    /// instance ของ `PromptPayQR` ด้วยค่าเริ่มต้นสำหรับประเทศไทย (TH, 764)
    pub fn new(merchant_id: &str) -> Self {
        PromptPayQR {
            merchant_id: merchant_id.to_string(),
            amount: None,
            country_code: CountryCode::Thailand,
            currency_code: CurrencyCode::THB,
        }
    }

    /// กำหนดจำนวนเงินสำหรับการทำธุรกรรม
    /// # Arguments
    /// * `amount` - จำนวนเงิน (ในหน่วยบาท, รูปแบบทศนิยมสองตำแหน่ง)
    /// # Returns
    /// อ้างอิงถึง instance นี้เพื่อให้สามารถ chain method ได้
    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self
    }

    /// ลบตัวอักษรที่ไม่ใช่ตัวเลขออกจากรหัสผู้รับเงิน
    /// # Arguments
    /// * `id` - รหัสผู้รับเงิน (เช่น เบอร์โทรศัพท์หรือ Tax ID)
    /// # Returns
    /// สตริงที่มีเฉพาะตัวเลข
    fn sanitize_target(&self, id: &str) -> String {
        id.chars().filter(|c| c.is_digit(10)).collect()
    }

    /// จัดรูปแบบรหัสผู้รับเงินให้เป็นไปตามมาตรฐาน PromptPay
    /// - ถ้าเป็นเบอร์โทรศัพท์ (< 13 หลัก): แปลงรหัสประเทศจาก "0" เป็น "66" และเติมศูนย์ให้ครบ 13 หลัก
    /// - ถ้าเป็น Tax ID หรือ E-Wallet ID (≥ 13 หลัก): ใช้ตามเดิม
    /// # Arguments
    /// * `id` - รหัสผู้รับเงิน
    /// # Returns
    /// รหัสผู้รับเงินที่ถูกจัดรูปแบบแล้ว
    fn format_target(&self, id: &str) -> String {
        if id.len() >= 13 {
            id.to_string()
        } else if id.starts_with("0") {
            // แปลงรหัสประเทศจาก "0" เป็น "66" เฉพาะตัวแรก และเติมศูนย์ให้ครบ 13 หลัก
            let replaced = id.replacen("0", "66", 1);
            format!("{:0>13}", replaced)
        } else {
            // เติมศูนย์ให้ครบ 13 หลัก
            format!("{:0>13}", id)
        }
    }

    /// สร้าง payload สำหรับ QR Code PromptPay ตามมาตรฐาน EMVCo
    /// # Returns
    /// ผลลัพธ์เป็น `Result` ที่มี Formatter หรือข้อผิดพลาด
    pub fn create(&self) -> Result<Formatter, PromptPayError> {
        if self.merchant_id.is_empty() {
            return Err(PromptPayError::new("Merchant ID is required"));
        }

        // sanitize ข้อมูลที่รับมา
        let merchant_id = self.sanitize_target(&self.merchant_id);

        let mut payload = String::new();

        // เพิ่ม Payload Format Indicator (ID 00, ค่า "01" สำหรับ EMVCo QR)
        payload.push_str("000201");

        // เพิ่ม Point of Initiation Method
        // - "010211" สำหรับ QR แบบ static (ไม่มีจำนวนเงิน)
        // - "010212" สำหรับ QR แบบ dynamic (มีจำนวนเงิน)
        payload.push_str(if self.amount.is_some() {
            "010212"
        } else {
            "010211"
        });

        // สร้าง Merchant Account Information (ID 29)
        let mut merchant_info = String::new();
        // เพิ่ม PromptPay AID (Application Identifier)
        merchant_info.push_str("0016A000000677010111"); // PromptPay AID
        // กำหนดประเภทของรหัสผู้รับเงิน
        // - "01" สำหรับเบอร์โทรศัพท์
        // - "02" สำหรับ Tax ID
        // - "03" สำหรับ E-Wallet ID
        let target_type = if merchant_id.len() >= 15 {
            "03" // E-Wallet ID
        } else if merchant_id.len() >= 13 {
            "02" // Tax ID
        } else {
            "01" // Phone Number
        };
        let formatted_target = self.format_target(&merchant_id);
        let merchant_id_field = format!(
            "{}{:02}{}",
            target_type,
            formatted_target.len(),
            formatted_target
        );
        merchant_info.push_str(&merchant_id_field);

        // เพิ่มความยาวและข้อมูล Merchant Account Information
        let merchant_info_len = format!("{:02}", merchant_info.len());
        payload.push_str(&format!("29{}", merchant_info_len));
        payload.push_str(&merchant_info);

        // เพิ่ม Country Code (ID 58, "TH" สำหรับประเทศไทย)
        payload.push_str(&format!("5802{}", self.country_code));

        // เพิ่ม Currency Code (ID 53, "764" สำหรับบาทไทย)
        payload.push_str(&format!("5303{}", self.currency_code));

        // เพิ่มจำนวนเงิน (ถ้ามี) (ID 54)
        if let Some(amount) = self.amount {
            let amount_str = format!("{:.2}", amount);
            let amount_len = format!("{:02}", amount_str.len());
            payload.push_str(&format!("54{}", amount_len));
            payload.push_str(&amount_str);
        }

        // เพิ่ม CRC (ID 63)
        payload.push_str("6304");
        let crc = self.calculate_crc(&payload);
        payload.push_str(&format!("{:04X}", crc));

        Ok(Formatter::new(&payload))
    }

    /// คำนวณ CRC-16 (CCITT) สำหรับ payload เพื่อใช้ใน QR Code
    /// ใช้ polynomial 0x1021 และค่าเริ่มต้น 0xFFFF ตามมาตรฐาน EMVCo
    /// # Arguments
    /// * `data` - สตริง payload ที่ใช้คำนวณ CRC (รวม "6304")
    /// # Returns
    /// ค่า CRC ในรูปแบบ u16
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

    // Getters
    pub fn merchant_id(&self) -> &str {
        &self.merchant_id
    }
    pub fn amount(&self) -> Option<f64> {
        self.amount
    }
    pub fn country_code(&self) -> CountryCode {
        self.country_code
    }
    pub fn currency_code(&self) -> CurrencyCode {
        self.currency_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ทดสอบการสร้าง payload สำหรับ QR Code ด้วยหมายเลขโทรศัพท์และจำนวนเงิน
    #[test]
    fn test_create_qr_phone_with_amount() {
        let mut qr = PromptPayQR::new("0812345678");
        qr.set_amount(100.50);
        let result = qr.create().unwrap();
        let data = result.to_string();
        assert!(!data.is_empty());
        assert!(data.starts_with("000201010212")); // Dynamic QR
        assert!(data.contains("01130066812345678")); // ตรวจสอบหมายเลขโทรศัพท์
        assert!(data.contains("5406100.50")); // ตรวจสอบจำนวนเงิน
        assert!(data.contains("5802TH")); // Country Code
        assert!(data.contains("5303764")); // Currency Code
        assert!(data.len() >= 8);
        let crc_part = &data[data.len() - 8..];
        assert!(crc_part.starts_with("6304"));
        assert!(crc_part[4..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// ทดสอบการสร้าง payload สำหรับ QR Code ด้วยหมายเลขโทรศัพท์ที่ขึ้นต้นด้วย +66
    #[test]
    fn test_create_qr_phone_plus_66() {
        let mut qr = PromptPayQR::new("+66-8-1234-500 0");
        qr.set_amount(100.50);
        let result = qr.create().unwrap();
        let data = result.to_string();
        assert!(!data.is_empty());
        assert!(data.starts_with("000201010212")); // Dynamic QR
        assert!(data.contains("01130066812345000")); // ตรวจสอบหมายเลขโทรศัพท์
        assert!(data.contains("5406100.50")); // ตรวจสอบจำนวนเงิน
        assert!(data.contains("5802TH"));
        assert!(data.contains("5303764"));
        assert!(data.len() >= 8);
        let crc_part = &data[data.len() - 8..];
        assert!(crc_part.starts_with("6304"));
        assert!(crc_part[4..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// ทดสอบการสร้าง payload สำหรับ QR Code ด้วยหมายเลขโทรศัพท์ที่ไม่มีจำนวนเงิน
    #[test]
    fn test_create_qr_phone_no_amount() {
        let qr = PromptPayQR::new("0812345678");
        let result = qr.create().unwrap();
        let data = result.to_string();
        assert!(!data.is_empty());
        assert!(data.starts_with("000201010211")); // Static QR
        assert!(data.contains("01130066812345678")); // ตรวจสอบหมายเลขโทรศัพท์
        assert!(!data.contains("54")); // ไม่มีฟิลด์จำนวนเงิน
        assert!(data.contains("5802TH"));
        assert!(data.contains("5303764"));
        assert!(data.len() >= 8);
        let crc_part = &data[data.len() - 8..];
        assert!(crc_part.starts_with("6304"));
        assert!(crc_part[4..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// ทดสอบการสร้าง payload สำหรับ QR Code ด้วย Tax ID
    #[test]
    fn test_create_qr_tax_id() {
        let qr = PromptPayQR::new("1234567890123");
        let result = qr.create().unwrap();
        let data = result.to_string();
        assert!(!data.is_empty());
        assert!(data.starts_with("000201010211")); // Static QR
        assert!(data.contains("02131234567890123")); // ตรวจสอบ Tax ID
        assert!(!data.contains("54")); // ไม่มีฟิลด์จำนวนเงิน
        assert!(data.contains("5802TH"));
        assert!(data.contains("5303764"));
        assert!(data.len() >= 8);
        let crc_part = &data[data.len() - 8..];
        assert!(crc_part.starts_with("6304"));
        assert!(crc_part[4..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// ทดสอบการสร้าง payload สำหรับ QR Code ด้วย E-Wallet ID
    #[test]
    fn test_create_qr_ewallet_id() {
        let qr = PromptPayQR::new("123456789012345");
        let result = qr.create().unwrap();
        let data = result.to_string();
        assert!(!data.is_empty());
        assert!(data.starts_with("000201010211")); // Static QR
        assert!(data.contains("0315123456789012345")); // ตรวจสอบ E-Wallet ID
        assert!(!data.contains("54")); // ไม่มีฟิลด์จำนวนเงิน
        assert!(data.contains("5802TH"));
        assert!(data.contains("5303764"));
        assert!(data.len() >= 8);
        let crc_part = &data[data.len() - 8..];
        assert!(crc_part.starts_with("6304"));
        assert!(crc_part[4..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// ทดสอบการจัดการข้อผิดพลาดเมื่อ merchant_id ว่างเปล่า
    #[test]
    fn test_create_qr_empty_merchant_id() {
        let qr = PromptPayQR::new("");
        let result = qr.create();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Merchant ID is required");
    }

    /// ทดสอบการล้างข้อมูล (sanitize_target) สำหรับหมายเลขโทรศัพท์ที่มีตัวอักษรพิเศษ
    #[test]
    fn test_sanitize_target_phone() {
        let qr = PromptPayQR::new("+66-8-1234-500 0");
        let sanitized = qr.sanitize_target(&qr.merchant_id);
        assert_eq!(sanitized, "66812345000");
    }

    /// ทดสอบการจัดรูปแบบ (format_target) สำหรับหมายเลขโทรศัพท์
    #[test]
    fn test_format_target_phone() {
        let qr = PromptPayQR::new("0812345678");
        let formatted = qr.format_target(&qr.sanitize_target(&qr.merchant_id));
        assert_eq!(formatted, "0066812345678");
    }

    /// ทดสอบการจัดรูปแบบ (format_target) สำหรับ Tax ID
    #[test]
    fn test_format_target_tax_id() {
        let qr = PromptPayQR::new("1234567890123");
        let formatted = qr.format_target(&qr.sanitize_target(&qr.merchant_id));
        assert_eq!(formatted, "1234567890123");
    }

    /// ทดสอบการจัดรูปแบบ (format_target) สำหรับ E-Wallet ID
    #[test]
    fn test_format_target_ewallet_id() {
        let qr = PromptPayQR::new("123456789012345");
        let formatted = qr.format_target(&qr.sanitize_target(&qr.merchant_id));
        assert_eq!(formatted, "123456789012345");
    }

    /// ทดสอบการคำนวณ CRC - ใช้ payload จริงที่สร้างจาก create() method
    #[test]
    fn test_calculate_crc() {
        let qr = PromptPayQR::new("0812345678");
        let result = qr.create().unwrap();
        let full_payload = result.to_string();

        // แยก payload ที่ไม่รวม CRC (ตัด 4 หลักสุดท้ายออก) และเพิ่ม "6304"
        let payload_without_crc = &full_payload[..full_payload.len() - 4];
        let crc = qr.calculate_crc(payload_without_crc);
        let expected_crc = &full_payload[full_payload.len() - 4..];

        assert_eq!(format!("{:04X}", crc), expected_crc);
    }

    /// ทดสอบการคำนวณ CRC ด้วยค่าที่ทราบแน่นอน
    #[test]
    fn test_calculate_crc_known_value() {
        let qr = PromptPayQR::new("0812345678");
        // สร้าง payload จริงและใช้ส่วนที่ไม่รวม CRC
        let result = qr.create().unwrap();
        let full_payload = result.to_string();
        let payload_without_crc = &full_payload[..full_payload.len() - 4];
        let crc = qr.calculate_crc(payload_without_crc);
        // ค่า CRC ที่คำนวณได้จริง
        assert_eq!(format!("{:04X}", crc), "5D82");
    }

    #[test]
    fn test_promptpay_qr_creation() {
        let mut qr = PromptPayQR::new("1234567890123");
        qr.set_amount(150.75);
        assert_eq!(qr.merchant_id(), "1234567890123");
        assert_eq!(qr.amount(), Some(150.75));
        assert_eq!(qr.country_code(), CountryCode::Thailand);
        assert_eq!(qr.currency_code(), CurrencyCode::THB);
    }

    #[test]
    fn test_promptpay_qr_no_amount() {
        let qr = PromptPayQR::new("9876543210987");
        assert_eq!(qr.amount(), None);
        assert_eq!(qr.country_code().as_str(), "TH");
        assert_eq!(qr.currency_code().numeric_code(), "764");
    }
}
