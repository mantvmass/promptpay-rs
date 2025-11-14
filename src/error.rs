use std::{error::Error, fmt};

/// Custom error type for PromptPay QR generation failures.
///
/// This error is used throughout the library to indicate issues such as:
/// - Empty merchant ID
/// - Invalid payload
/// - QR code generation failure
#[derive(Debug)]
pub struct PromptPayError {
    details: String, // ข้อความอธิบายข้อผิดพลาด
}

impl PromptPayError {
    /// Creates a new `PromptPayError` with a custom message.
    ///
    /// # Arguments
    /// * `msg` - A descriptive error message
    ///
    /// # Returns
    /// A new `PromptPayError` instance
    ///
    /// # Example
    /// ```rust
    /// use promptpay_rs::PromptPayError;
    /// let err = PromptPayError::new("Invalid phone number");
    /// ```
    pub fn new(msg: &str) -> PromptPayError {
        PromptPayError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for PromptPayError {
    /// Formats the error for display (e.g., in `println!` or `eprintln!`).
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for PromptPayError {
    /// Returns a short description of the error (legacy method).
    ///
    /// **Note**: Prefer using `Display` implementation for user-facing messages.
    fn description(&self) -> &str {
        &self.details
    }
}
