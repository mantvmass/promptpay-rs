use qrcode::{EcLevel, QrCode};

use crate::PromptPayError;

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