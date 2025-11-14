use std::{error::Error, fmt};

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
    pub fn new(msg: &str) -> PromptPayError {
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
