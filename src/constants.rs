use std::fmt;

/// Country code according to **ISO 3166-1 alpha-2** standard.
///
/// Currently only supports **Thailand** (`TH`) as PromptPay is Thailand-specific.
///
/// # Variants
/// * `Thailand` - Thailand (`"TH"`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountryCode {
    /// Thailand - ISO 3166-1 alpha-2 code: `"TH"`
    Thailand,
}

impl CountryCode {
    /// Returns the 2-letter country code as a static string.
    ///
    /// # Returns
    /// `"TH"` for Thailand
    pub fn as_str(&self) -> &'static str {
        match self {
            CountryCode::Thailand => "TH",
        }
    }

    /// Parses a string into a `CountryCode` (case-insensitive).
    ///
    /// # Arguments
    /// * `s` - Input string (e.g., `"TH"`, `"th"`, `"Thailand"`)
    ///
    /// # Returns
    /// * `Some(CountryCode)` if valid
    /// * `None` if unknown
    ///
    /// # Example
    /// ```rust
    /// use promptpay_rs::CountryCode;
    /// assert_eq!(CountryCode::from_str("th"), Some(CountryCode::Thailand));
    /// ```
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

/// Currency code according to **ISO 4217** standard.
///
/// Only supports **Thai Baht (THB)** as required by PromptPay.
///
/// # Variants
/// * `THB` - Thai Baht (numeric: `"764"`, alphabetic: `"THB"`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrencyCode {
    /// Thai Baht
    THB,
}

impl CurrencyCode {
    /// Returns the **3-digit numeric code** used in EMVCo QR (e.g., `"764"`).
    pub fn numeric_code(&self) -> &'static str {
        match self {
            CurrencyCode::THB => "764",
        }
    }

    /// Returns the **3-letter alphabetic code** (ISO 4217).
    pub fn alphabetic_code(&self) -> &'static str {
        match self {
            CurrencyCode::THB => "THB",
        }
    }

    /// Creates from numeric code string.
    pub fn from_numeric(s: &str) -> Option<Self> {
        match s.trim() {
            "764" => Some(CurrencyCode::THB),
            _ => None,
        }
    }

    /// Creates from alphabetic code string (case-insensitive).
    pub fn from_alphabetic(s: &str) -> Option<Self> {
        match s.trim().to_uppercase().as_str() {
            "THB" => Some(CurrencyCode::THB),
            _ => None,
        }
    }
}

impl fmt::Display for CurrencyCode {
    /// Displays as **numeric code** (required in QR payload).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.numeric_code())
    }
}

/// Type of merchant identifier used in PromptPay.
///
/// Determines the tag used in Merchant Account Information field:
/// - `"01"` → Mobile Number
/// - `"02"` → Tax ID
/// - `"03"` → E-Wallet ID
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MerchantType {
    MobileNumber,
    TaxId,
    EWalletId,
}

impl MerchantType {
    /// Returns the 2-digit tag used in the payload.
    pub fn as_str(&self) -> &'static str {
        match self {
            MerchantType::MobileNumber => "01",
            MerchantType::TaxId => "02",
            MerchantType::EWalletId => "03",
        }
    }

    /// Infers the merchant type from a **sanitized** ID (digits only).
    ///
    /// # Arguments
    /// * `id` - Sanitized merchant ID (only digits)
    ///
    /// # Returns
    /// Appropriate `MerchantType` based on length:
    /// - ≥15 digits → `EWalletId`
    /// - ≥13 digits → `TaxId`
    /// - <13 digits → `MobileNumber`
    ///
    /// # Example
    /// ```rust
    /// use promptpay_rs::constants::MerchantType;
    /// assert_eq!(MerchantType::from_merchant_id("1234567890123"), MerchantType::TaxId);
    /// ```
    pub fn from_merchant_id(id: &str) -> Self {
        let digits_only: String = id.chars().filter(|c| c.is_digit(10)).collect();
        match digits_only.len() {
            len if len >= 15 => MerchantType::EWalletId,
            len if len >= 13 => MerchantType::TaxId,
            _ => MerchantType::MobileNumber,
        }
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
        );
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
