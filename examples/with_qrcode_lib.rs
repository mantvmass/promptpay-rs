use promptpay_rs::{PromptPayError, PromptPayQR};
use qrcode::{EcLevel, QrCode, render::unicode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize a new PromptPay QR object with a phone number
    // This will be the recipient of the payment.
    let mut qr = PromptPayQR::new("081-234-5678");

    // Set the payment amount for the QR code
    // The amount is in THB (Thai Baht)
    qr.set_amount(250.75);

    // Create the QR payload using the formatter
    // This payload is a string representation following PromptPay's standard
    let payload = qr.create()?;

    // Convert the formatter payload into a string
    println!("Payload: {}", payload);

    // Generate the QR code image from the payload
    let qr_code = QrCode::with_error_correction_level(payload.as_bytes(), EcLevel::M)
        .map_err(|e| PromptPayError::new(&format!("Failed to create QRCode: {}", e)))?;

    // Render the QR code in Unicode format for terminal display
    let image = qr_code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

    // Print the QR code in the terminal
    println!("{}", image);

    Ok(())
}
