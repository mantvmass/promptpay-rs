use thiserror::Error;

/// ข้อผิดพลาดที่เกิดขึ้นในระหว่างการสร้าง PromptPay QR code
#[derive(Error, Debug)]
pub enum PromptPayError {
    #[error("Merchant ID is required")]
    MissingMerchantId,
    
    #[error("Invalid merchant ID format: {0}")]
    InvalidMerchantId(String),
    
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    
    #[error("QR code generation failed: {0}")]
    QrGenerationFailed(String),
    
    #[error("Image generation failed: {0}")]
    ImageGenerationFailed(String),
    
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
    
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

impl PromptPayError {
    /// สร้าง instance ใหม่ของ `PromptPayError::InvalidMerchantId`
    pub fn invalid_merchant_id(id: &str) -> Self {
        PromptPayError::InvalidMerchantId(id.to_string())
    }
    
    /// สร้าง instance ใหม่ของ `PromptPayError::InvalidAmount`
    pub fn invalid_amount(amount: f64) -> Self {
        PromptPayError::InvalidAmount(amount.to_string())
    }
} 