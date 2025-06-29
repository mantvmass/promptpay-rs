use std::error::Error;
use std::fmt;

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
    merchant_id: String,   // รหัสผู้รับเงิน (เช่น เบอร์โทรศัพท์, Tax ID, หรือ E-Wallet ID)
    amount: Option<f64>,   // จำนวนเงิน (ถ้ามี)
    country_code: String,  // รหัสประเทศ (เช่น "TH" สำหรับประเทศไทย)
    currency_code: String, // รหัสสกุลเงิน (เช่น "764" สำหรับบาทไทย)
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
            country_code: "TH".to_string(),
            currency_code: "764".to_string(),
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
        let numbers = self.sanitize_target(id);
        if numbers.len() >= 13 {
            numbers
        } else {
            let formatted = numbers.replace("0", "66");
            format!("{:0>13}", formatted)
        }
    }

    /// สร้าง payload สำหรับ QR Code PromptPay ตามมาตรฐาน EMVCo
    /// # Returns
    /// ผลลัพธ์เป็น `Result` ที่มีสตริง payload หรือข้อผิดพลาด
    pub fn generate(&self) -> Result<String, PromptPayError> {
        if self.merchant_id.is_empty() {
            return Err(PromptPayError::new("Merchant ID is required"));
        }

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
        let target_type = if self.merchant_id.len() >= 15 {
            "03" // E-Wallet ID
        } else if self.merchant_id.len() >= 13 {
            "02" // Tax ID
        } else {
            "01" // Phone Number
        };
        let formatted_target = self.format_target(&self.merchant_id);
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

        Ok(payload)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ทดสอบการสร้าง payload สำหรับ QR Code ที่มีจำนวนเงิน
    #[test]
    fn test_generate_qr() {
        let mut qr = PromptPayQR::new("0812345678");
        qr.set_amount(100.50);
        let result = qr.generate().unwrap();
        assert!(!result.is_empty());
        assert!(result.starts_with("000201"));
        assert!(result.contains("01130066812345678")); // ตรวจสอบเบอร์โทรศัพท์ที่ถูกจัดรูปแบบ
        assert!(result.contains("5406100.50")); // ตรวจสอบจำนวนเงิน (ความยาว 6 สำหรับ "100.50")
        assert!(result.len() >= 8);
        let crc_part = &result[result.len() - 8..];
        assert!(crc_part.starts_with("6304"));
        assert!(crc_part[4..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// ทดสอบการสร้าง payload สำหรับ QR Code ที่ไม่มีจำนวนเงิน
    #[test]
    fn test_generate_qr_no_amount() {
        let qr = PromptPayQR::new("0812345678");
        let result = qr.generate().unwrap();
        assert!(!result.is_empty());
        assert!(result.starts_with("000201010211"));
        assert!(result.contains("01130066812345678")); // ตรวจสอบเบอร์โทรศัพท์ที่ถูกจัดรูปแบบ
        assert!(!result.contains("54")); // ไม่มีฟิลด์จำนวนเงิน
        assert!(result.len() >= 8);
        let crc_part = &result[result.len() - 8..];
        assert!(crc_part.starts_with("6304"));
        assert!(crc_part[4..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// ทดสอบการสร้าง payload สำหรับ QR Code ด้วย Tax ID
    #[test]
    fn test_generate_qr_tax_id() {
        let qr = PromptPayQR::new("1234567890123");
        let result = qr.generate().unwrap();
        assert!(!result.is_empty());
        assert!(result.starts_with("000201010211"));
        assert!(result.contains("02131234567890123")); // ตรวจสอบ Tax ID
        assert!(!result.contains("54")); // ไม่มีฟิลด์จำนวนเงิน
        assert!(result.len() >= 8);
        let crc_part = &result[result.len() - 8..];
        assert!(crc_part.starts_with("6304"));
        assert!(crc_part[4..].chars().all(|c| c.is_ascii_hexdigit()));
    }
}
