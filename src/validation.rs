use regex::Regex;
use crate::error::PromptPayError;

/// ตรวจสอบและทำความสะอาดข้อมูล input
pub struct Validator;

impl Validator {
    /// ตรวจสอบว่าเป็นเบอร์โทรศัพท์ไทยที่ถูกต้องหรือไม่
    pub fn is_valid_thai_phone(phone: &str) -> bool {
        let phone_clean = Self::sanitize_phone(phone);
        let phone_regex = Regex::new(r"^(0[689]\d{8}|66[689]\d{8})$").unwrap();
        phone_regex.is_match(&phone_clean)
    }
    
    /// ตรวจสอบว่าเป็น Tax ID ที่ถูกต้องหรือไม่
    pub fn is_valid_tax_id(tax_id: &str) -> bool {
        let tax_clean = Self::sanitize_numbers(tax_id);
        if tax_clean.len() != 13 {
            return false;
        }
        
        // ตรวจสอบ checksum ของ Tax ID
        let mut sum = 0;
        for (i, digit) in tax_clean.chars().take(12).enumerate() {
            let digit_val = digit.to_digit(10).unwrap();
            sum += digit_val * (13 - i as u32);
        }
        
        let checksum = (11 - (sum % 11)) % 10;
        let last_digit = tax_clean.chars().last().unwrap().to_digit(10).unwrap();
        
        checksum == last_digit
    }
    
    /// ตรวจสอบว่าเป็น E-Wallet ID ที่ถูกต้องหรือไม่
    pub fn is_valid_ewallet_id(ewallet_id: &str) -> bool {
        let ewallet_clean = Self::sanitize_numbers(ewallet_id);
        ewallet_clean.len() >= 13 && ewallet_clean.len() <= 15
    }
    
    /// ตรวจสอบจำนวนเงิน
    pub fn is_valid_amount(amount: f64) -> bool {
        amount > 0.0 && amount <= 999999999.99
    }
    
    /// ทำความสะอาดเบอร์โทรศัพท์
    pub fn sanitize_phone(phone: &str) -> String {
        let mut cleaned = Self::sanitize_numbers(phone);
        
        // แปลงเบอร์ที่ขึ้นต้นด้วย 0 เป็น 66
        if cleaned.starts_with('0') && cleaned.len() == 10 {
            cleaned = format!("66{}", &cleaned[1..]);
        }
        
        cleaned
    }
    
    /// ลบตัวอักษรที่ไม่ใช่ตัวเลขออก
    pub fn sanitize_numbers(input: &str) -> String {
        input.chars().filter(|c| c.is_digit(10)).collect()
    }
    
    /// ตรวจสอบและระบุประเภทของ merchant ID
    pub fn identify_merchant_type(merchant_id: &str) -> MerchantType {
        let clean_id = Self::sanitize_numbers(merchant_id);
        
        if Self::is_valid_thai_phone(merchant_id) {
            MerchantType::Phone
        } else if Self::is_valid_tax_id(merchant_id) {
            MerchantType::TaxId
        } else if Self::is_valid_ewallet_id(merchant_id) {
            MerchantType::EWallet
        } else {
            MerchantType::Unknown
        }
    }
    
    /// ตรวจสอบ merchant ID และคืนค่าข้อผิดพลาดถ้าไม่ถูกต้อง
    pub fn validate_merchant_id(merchant_id: &str) -> Result<(), PromptPayError> {
        if merchant_id.trim().is_empty() {
            return Err(PromptPayError::MissingMerchantId);
        }
        
        let merchant_type = Self::identify_merchant_type(merchant_id);
        match merchant_type {
            MerchantType::Unknown => Err(PromptPayError::invalid_merchant_id(merchant_id)),
            _ => Ok(()),
        }
    }
}

/// ประเภทของ merchant ID
#[derive(Debug, Clone, PartialEq)]
pub enum MerchantType {
    Phone,
    TaxId,
    EWallet,
    Unknown,
}

impl std::fmt::Display for MerchantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MerchantType::Phone => write!(f, "Phone"),
            MerchantType::TaxId => write!(f, "Tax ID"),
            MerchantType::EWallet => write!(f, "E-Wallet"),
            MerchantType::Unknown => write!(f, "Unknown"),
        }
    }
} 