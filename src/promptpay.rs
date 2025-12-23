use crate::{
    CountryCode, CurrencyCode, PromptPayError,
    constants::MerchantType,
    crc::calculate_crc,
    utils::{format_target, sanitize_target},
};

/// Main struct for generating **PromptPay QR codes** compliant with **EMVCo** standards.
///
/// Supports:
/// - Mobile number, Tax ID, E-Wallet ID
/// - Optional amount (static/dynamic QR)
/// - Automatic formatting and CRC calculation
///
/// # Example
/// ```rust
/// use promptpay_rs::PromptPayQR;
/// let mut qr = PromptPayQR::new("0812345678");
/// qr.set_amount(100.0);
/// let payload = qr.create().unwrap();
/// ```
pub struct PromptPayQR {
    merchant_id: String,      // รหัสผู้รับเงินดิบ (เช่น "0812345678")
    merchant_type: MerchantType, // ชนิดของรหัส (Mobile, Tax, EWallet)
    amount: Option<f64>,      // จำนวนเงิน (ถ้ามี)
    country_code: CountryCode,   // รหัสประเทศ (default: TH)
    currency_code: CurrencyCode, // รหัสสกุลเงิน (default: 764)
}

impl PromptPayQR {
    /// Creates a new `PromptPayQR` instance with a merchant identifier.
    ///
    /// Automatically:
    /// - Sanitizes the input
    /// - Detects `merchant_type`
    /// - Sets default country (`TH`) and currency (`THB`)
    ///
    /// # Arguments
    /// * `merchant_id` - Phone number, Tax ID, or E-Wallet ID
    ///
    /// # Returns
    /// A new `PromptPayQR` instance
    pub fn new(merchant_id: &str) -> Self {
        let sanitized = sanitize_target(merchant_id); // ล้างตัวอักษรที่ไม่ใช่ตัวเลข
        let merchant_type = MerchantType::from_merchant_id(&sanitized); // ตรวจจับประเภท
        PromptPayQR {
            merchant_id: merchant_id.to_string(),
            merchant_type,
            amount: None,
            country_code: CountryCode::Thailand,
            currency_code: CurrencyCode::THB,
        }
    }

    /// Sets the transaction amount (enables **dynamic QR**).
    ///
    /// # Arguments
    /// * `amount` - Amount in THB (e.g., `150.75`)
    ///
    /// # Returns
    /// `&mut self` for method chaining
    ///
    /// # Example
    /// ```rust
    /// use promptpay_rs::PromptPayQR;
    /// let mut qr = PromptPayQR::new("0812345678");
    /// qr.set_amount(99.50);
    /// ```
    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self
    }

    /// Generates the complete **EMVCo-compliant payload** and wraps it in a `Formatter`.
    ///
    /// # Returns
    /// * `Ok(Formatter)` - Ready for `.to_string()` or `.to_image()`
    /// * `Err(PromptPayError)` - If merchant ID is empty
    ///
    /// # Payload Structure (TLV format)
    /// - `00` Payload Format Indicator
    /// - `01` Point of Initiation Method (`11` = static, `12` = dynamic)
    /// - `29` Merchant Account Information (with PromptPay AID)
    /// - `53` Currency Code
    /// - `54` Amount (if present)
    /// - `58` Country Code
    /// - `63` CRC-16
    pub fn create(&self) -> Result<String, PromptPayError> {
        // ตรวจสอบว่ามีรหัสผู้รับเงินหรือไม่
        if self.merchant_id.trim().is_empty() {
            return Err(PromptPayError::new("Merchant ID is required"));
        }

        let mut payload = String::new();

        // ID 00: Payload Format Indicator = "01"
        payload.push_str("000201");

        // ID 01: Point of Initiation Method
        // 11 = Static QR (no amount), 12 = Dynamic QR (with amount)
        payload.push_str(if self.amount.is_some() { "010212" } else { "010211" });

        // ID 29: Merchant Account Information
        let mut merchant_info = String::new();
        merchant_info.push_str("0016A000000677010111"); // PromptPay AID

        let target_type = self.merchant_type.as_str(); // "01", "02", or "03"
        let formatted_target = format_target(&sanitize_target(&self.merchant_id)); // จัดรูปแบบให้ถูกต้อง

        let merchant_id_field = format!(
            "{}{:02}{}",
            target_type,
            formatted_target.len(),
            formatted_target
        );
        merchant_info.push_str(&merchant_id_field);

        // เพิ่มความยาวของ Merchant Info
        let merchant_info_len = format!("{:02}", merchant_info.len());
        payload.push_str(&format!("29{}", merchant_info_len));
        payload.push_str(&merchant_info);

        // ID 58: Country Code
        payload.push_str(&format!("5802{}", self.country_code));

        // ID 53: Currency Code
        payload.push_str(&format!("5303{}", self.currency_code));

        // ID 54: Amount (ถ้ามี)
        if let Some(amount) = self.amount {
            let amount_str = format!("{:.2}", amount); // 2 ทศนิยม
            let amount_len = format!("{:02}", amount_str.len());
            payload.push_str(&format!("54{}", amount_len));
            payload.push_str(&amount_str);
        }

        // ID 63: CRC (คำนวณจาก payload + "6304")
        payload.push_str("6304");
        let crc = calculate_crc(&payload);
        payload.push_str(&format!("{:04X}", crc)); // แปลงเป็น hex 4 หลัก

        Ok(payload)
    }

    // --- Getters ---
    pub fn merchant_id(&self) -> &str { &self.merchant_id }
    pub fn amount(&self) -> Option<f64> { self.amount }
    pub fn country_code(&self) -> CountryCode { self.country_code }
    pub fn currency_code(&self) -> CurrencyCode { self.currency_code }
    pub fn merchant_type(&self) -> MerchantType { self.merchant_type }
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
        assert!(!result.is_empty());
        assert!(result.starts_with("000201010212")); // Dynamic QR
        assert!(result.contains("01130066812345678")); // ตรวจสอบหมายเลขโทรศัพท์
        assert!(result.contains("5406100.50")); // ตรวจสอบจำนวนเงิน
        assert!(result.contains("5802TH")); // Country Code
        assert!(result.contains("5303764")); // Currency Code
        assert!(result.len() >= 8);
        let crc_part = &result[result.len() - 8..];
        assert!(crc_part.starts_with("6304"));
        assert!(crc_part[4..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// ทดสอบการสร้าง payload สำหรับ QR Code ด้วยหมายเลขโทรศัพท์ที่ขึ้นต้นด้วย +66
    #[test]
    fn test_create_qr_phone_plus_66() {
        let mut qr = PromptPayQR::new("+66-8-1234-500 0");
        qr.set_amount(100.50);
        let result = qr.create().unwrap();
        assert!(!result.is_empty());
        assert!(result.starts_with("000201010212")); // Dynamic QR
        assert!(result.contains("01130066812345000")); // ตรวจสอบหมายเลขโทรศัพท์
        assert!(result.contains("5406100.50")); // ตรวจสอบจำนวนเงิน
        assert!(result.contains("5802TH"));
        assert!(result.contains("5303764"));
        assert!(result.len() >= 8);
        let crc_part = &result[result.len() - 8..];
        assert!(crc_part.starts_with("6304"));
        assert!(crc_part[4..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// ทดสอบการสร้าง payload สำหรับ QR Code ด้วยหมายเลขโทรศัพท์ที่ไม่มีจำนวนเงิน
    #[test]
    fn test_create_qr_phone_no_amount() {
        let qr = PromptPayQR::new("0812345678");
        let result = qr.create().unwrap();
        assert!(!result.is_empty());
        assert!(result.starts_with("000201010211")); // Static QR
        assert!(result.contains("01130066812345678")); // ตรวจสอบหมายเลขโทรศัพท์
        assert!(!result.contains("54")); // ไม่มีฟิลด์จำนวนเงิน
        assert!(result.contains("5802TH"));
        assert!(result.contains("5303764"));
        assert!(result.len() >= 8);
        let crc_part = &result[result.len() - 8..];
        assert!(crc_part.starts_with("6304"));
        assert!(crc_part[4..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// ทดสอบการสร้าง payload สำหรับ QR Code ด้วย Tax ID
    #[test]
    fn test_create_qr_tax_id() {
        let qr = PromptPayQR::new("1234567890123");
        let result = qr.create().unwrap();
        assert!(!result.is_empty());
        assert!(result.starts_with("000201010211")); // Static QR
        assert!(result.contains("02131234567890123")); // ตรวจสอบ Tax ID
        assert!(!result.contains("54")); // ไม่มีฟิลด์จำนวนเงิน
        assert!(result.contains("5802TH"));
        assert!(result.contains("5303764"));
        assert!(result.len() >= 8);
        let crc_part = &result[result.len() - 8..];
        assert!(crc_part.starts_with("6304"));
        assert!(crc_part[4..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// ทดสอบการสร้าง payload สำหรับ QR Code ด้วย E-Wallet ID
    #[test]
    fn test_create_qr_ewallet_id() {
        let qr = PromptPayQR::new("123456789012345");
        let result = qr.create().unwrap();
        assert!(!result.is_empty());
        assert!(result.starts_with("000201010211")); // Static QR
        assert!(result.contains("0315123456789012345")); // ตรวจสอบ E-Wallet ID
        assert!(!result.contains("54")); // ไม่มีฟิลด์จำนวนเงิน
        assert!(result.contains("5802TH"));
        assert!(result.contains("5303764"));
        assert!(result.len() >= 8);
        let crc_part = &result[result.len() - 8..];
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
        let sanitized = sanitize_target(&qr.merchant_id);
        assert_eq!(sanitized, "66812345000");
    }

    /// ทดสอบการจัดรูปแบบ (format_target) สำหรับหมายเลขโทรศัพท์
    #[test]
    fn test_format_target_phone() {
        let qr = PromptPayQR::new("0812345678");
        let formatted = format_target(&sanitize_target(&qr.merchant_id));
        assert_eq!(formatted, "0066812345678");
    }

    /// ทดสอบการจัดรูปแบบ (format_target) สำหรับ Tax ID
    #[test]
    fn test_format_target_tax_id() {
        let qr = PromptPayQR::new("1234567890123");
        let formatted = format_target(&sanitize_target(&qr.merchant_id));
        assert_eq!(formatted, "1234567890123");
    }

    /// ทดสอบการจัดรูปแบบ (format_target) สำหรับ E-Wallet ID
    #[test]
    fn test_format_target_ewallet_id() {
        let qr = PromptPayQR::new("123456789012345");
        let formatted = format_target(&sanitize_target(&qr.merchant_id));
        assert_eq!(formatted, "123456789012345");
    }

    /// ทดสอบการคำนวณ CRC - ใช้ payload จริงที่สร้างจาก create() method
    #[test]
    fn test_calculate_crc() {
        let qr = PromptPayQR::new("0812345678");
        let result = qr.create().unwrap();

        // แยก payload ที่ไม่รวม CRC (ตัด 4 หลักสุดท้ายออก) และเพิ่ม "6304"
        let payload_without_crc = &result[..result.len() - 4];
        let crc = calculate_crc(payload_without_crc);
        let expected_crc = &result[result.len() - 4..];

        assert_eq!(format!("{:04X}", crc), expected_crc);
    }

    /// ทดสอบการคำนวณ CRC ด้วยค่าที่ทราบแน่นอน
    #[test]
    fn test_calculate_crc_known_value() {
        let qr = PromptPayQR::new("0812345678");
        // สร้าง payload จริงและใช้ส่วนที่ไม่รวม CRC
        let result = qr.create().unwrap();
        let payload_without_crc = &result[..result.len() - 4];
        let crc = calculate_crc(payload_without_crc);
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
