/// Removes all non-digit characters from a merchant identifier.
///
/// Used to clean inputs like phone numbers with hyphens or spaces.
///
/// # Arguments
/// * `id` - Raw merchant ID (e.g., `"+66-8-123-4567"`)
///
/// # Returns
/// String containing only digits (e.g., `"6681234567"`)
///
/// # Example
/// ```rust
/// use promptpay_rs::utils::sanitize_target;
/// assert_eq!(sanitize_target("+66-8-123-4567"), "6681234567");
/// ```
pub fn sanitize_target(id: &str) -> String {
    // กรองเฉพาะตัวเลข 0-9 ออกมา
    id.chars().filter(|c| c.is_digit(10)).collect()
}

/// Formats a **sanitized** merchant ID according to PromptPay rules.
///
/// # Rules
/// - If length ≥ 13 → Use as-is (Tax ID or E-Wallet ID)
/// - If starts with `'0'` → Replace first `'0'` with `'66'` and pad to 13 digits
/// - Otherwise → Pad with leading zeros to 13 digits
///
/// # Arguments
/// * `id` - Sanitized ID (digits only)
///
/// # Returns
/// Formatted 13-digit string (or longer for Tax/E-Wallet)
///
/// # Example
/// ```rust
/// use promptpay_rs::utils::format_target;
/// assert_eq!(format_target("0812345678"), "0066812345678");
/// ```
pub fn format_target(id: &str) -> String {
    if id.len() >= 13 {
        // Tax ID หรือ E-Wallet ID → ใช้ตามเดิม
        id.to_string()
    } else if id.starts_with("0") {
        // เบอร์โทร → เปลี่ยน 0 เป็น 66 (รหัสประเทศไทย)
        let replaced = id.replacen("0", "66", 1);
        format!("{:0>13}", replaced) // เติม 0 ด้านซ้ายให้ครบ 13 หลัก
    } else {
        // กรณีอื่น (เช่น ขึ้นต้นด้วย 66 อยู่แล้ว)
        format!("{:0>13}", id)
    }
}