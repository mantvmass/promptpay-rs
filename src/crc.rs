/// Calculates **CRC-16/CCITT** checksum for the PromptPay payload.
///
/// Used in **field ID 63** of the EMVCo QR code.
/// - Polynomial: `0x1021`
/// - Initial value: `0xFFFF`
/// - No final XOR (raw CRC)
///
/// # Arguments
/// * `data` - Payload **including** `"6304"` prefix (but **excluding** the CRC itself)
///
/// # Returns
/// 16-bit CRC value as `u16`
///
/// # Example
/// ```rust
/// use promptpay_rs::crc::calculate_crc;
/// let crc = calculate_crc("000201010212...6304");
/// assert_eq!(format!("{:04X}", crc), "E14F");
/// ```
pub fn calculate_crc(data: &str) -> u16 {
    let mut crc: u16 = 0xFFFF; // ค่าเริ่มต้นตามมาตรฐาน EMVCo
    let polynomial: u16 = 0x1021; // CCITT polynomial

    // วนลูปทุก byte ในข้อมูล
    for byte in data.bytes() {
        crc ^= (byte as u16) << 8; // XOR กับ byte ที่ shift ไปทางซ้าย 8 บิต
        for _ in 0..8 { // วน 8 บิต
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ polynomial; // ถ้า MSB เป็น 1 → XOR กับ polynomial
            } else {
                crc <<= 1; // ถ้าไม่ → เลื่อนซ้ายอย่างเดียว
            }
        }
    }
    crc // คืนค่า CRC ดิบ (ไม่ XOR 0xFFFF)
}