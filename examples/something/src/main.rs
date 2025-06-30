use promptpay_rs::PromptPayQR;

fn main() {
    let mut qr = PromptPayQR::new("0812345678");
    qr.set_amount(100.50);
    match qr.create() {
        Ok(result) => println!("Payload: {}", result.to_string()),
        Err(e) => eprintln!("Error: {}", e),
    }
}