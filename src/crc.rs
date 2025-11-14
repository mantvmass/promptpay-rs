/// คำนวณ CRC-16 (CCITT) สำหรับ payload เพื่อใช้ใน QR Code
/// ใช้ polynomial 0x1021 และค่าเริ่มต้น 0xFFFF ตามมาตรฐาน EMVCo
/// # Arguments
/// * `data` - สตริง payload ที่ใช้คำนวณ CRC (รวม "6304")
/// # Returns
/// ค่า CRC ในรูปแบบ u16
pub fn calculate_crc(data: &str) -> u16 {
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
