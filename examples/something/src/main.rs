use promptpay_rs::PromptPayQR;

fn main() {
    let mut qr = PromptPayQR::new("0812345678");
    qr.set_amount(100.50);
    match qr.generate() {
        Ok(payload) => println!("Payload: {}", payload),
        Err(e) => eprintln!("Error: {}", e),
    }
}