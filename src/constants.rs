use std::fmt;

/// รหัสประเทศตามมาตรฐาน ISO 3166-1 alpha-2
/// ปัจจุบันรองรับเฉพาะประเทศไทย เนื่องจาก PromptPay เป็นระบบเฉพาะของไทย
/// # Variants
/// * `Thailand` - ประเทศไทย (รหัส: `"TH"`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountryCode {
    /// ประเทศไทย (ISO 3166-1: TH)
    Thailand,
}

impl CountryCode {
    /// แปลงเป็นรหัสประเทศ 2 ตัวอักษร (ISO 3166-1 alpha-2)
    /// # Returns
    /// `&'static str` เช่น `"TH"`
    pub fn as_str(&self) -> &'static str {
        match self {
            CountryCode::Thailand => "TH",
        }
    }

    /// สร้าง `CountryCode` จาก string (ไม่สนใจ case)
    /// # Arguments
    /// * `s` - ข้อความ เช่น `"TH"`, `"th"`, `"Thailand"`, `"THAILAND"`
    /// # Returns
    /// `Some(CountryCode)` ถ้าถูกต้อง, `None` ถ้าไม่รู้จัก
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_uppercase().as_str() {
            "TH" | "THAILAND" => Some(CountryCode::Thailand),
            _ => None,
        }
    }
}

impl fmt::Display for CountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// รหัสสกุลเงินตามมาตรฐาน ISO 4217
/// ปัจจุบันรองรับเฉพาะบาทไทย เนื่องจาก PromptPay ใช้เฉพาะสกุลเงิน THB
/// # Variants
/// * `THB` - บาทไทย (รหัสตัวเลข: `"764"`, รหัสตัวอักษร: `"THB"`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrencyCode {
    /// บาทไทย (Thai Baht)
    THB,
}

impl CurrencyCode {
    /// รหัสสกุลเงินแบบตัวเลข 3 หลัก (ตาม EMVCo)
    /// # Returns
    /// เช่น `"764"` สำหรับบาทไทย
    pub fn numeric_code(&self) -> &'static str {
        match self {
            CurrencyCode::THB => "764",
        }
    }

    /// รหัสสกุลเงินแบบตัวอักษร 3 ตัว (ISO 4217)
    /// # Returns
    /// เช่น `"THB"`
    pub fn alphabetic_code(&self) -> &'static str {
        match self {
            CurrencyCode::THB => "THB",
        }
    }

    /// สร้างจากรหัสตัวเลข
    pub fn from_numeric(s: &str) -> Option<Self> {
        match s.trim() {
            "764" => Some(CurrencyCode::THB),
            _ => None,
        }
    }

    /// สร้างจากรหัสตัวอักษร
    pub fn from_alphabetic(s: &str) -> Option<Self> {
        match s.trim().to_uppercase().as_str() {
            "THB" => Some(CurrencyCode::THB),
            _ => None,
        }
    }
}

impl fmt::Display for CurrencyCode {
    /// แสดงผลเป็นรหัสตัวเลข (ตามที่ EMVCo ต้องการใน QR)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.numeric_code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_country_code() {
        let code = CountryCode::Thailand;
        assert_eq!(code.as_str(), "TH");
        assert_eq!(code.to_string(), "TH");

        assert_eq!(CountryCode::from_str("TH"), Some(CountryCode::Thailand));
        assert_eq!(CountryCode::from_str("th"), Some(CountryCode::Thailand));
        assert_eq!(
            CountryCode::from_str("Thailand"),
            Some(CountryCode::Thailand)
        );
        assert_eq!(
            CountryCode::from_str("THAILAND"),
            Some(CountryCode::Thailand)
        ); // เพิ่มบรรทัดนี้
        assert_eq!(CountryCode::from_str("US"), None);
    }

    #[test]
    fn test_currency_code() {
        let cur = CurrencyCode::THB;
        assert_eq!(cur.numeric_code(), "764");
        assert_eq!(cur.alphabetic_code(), "THB");
        assert_eq!(cur.to_string(), "764");

        assert_eq!(CurrencyCode::from_numeric("764"), Some(CurrencyCode::THB));
        assert_eq!(
            CurrencyCode::from_alphabetic("THB"),
            Some(CurrencyCode::THB)
        );
        assert_eq!(CurrencyCode::from_numeric("840"), None);
    }
}
