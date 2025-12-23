use promptpay_rs::PromptPayQR;

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

    // You can use this value with a QR code generation library (e.g., `qrcode`).
    println!("Payload: {}", payload);

    Ok(())
}
