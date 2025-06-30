use promptpay_rs::{FormatterTrait, PromptPayQR, qrcode::EcLevel, qrcode::render::unicode};

fn main() {
    let mut qr = PromptPayQR::new("0812345678");
    qr.set_amount(100.50);
    match qr.create() {
        Ok(result) => {
            println!("*************************************************");
            println!("QR Value: {}", result.to_string());
            println!("*************************************************");
            println!("QRCode:"); 
            let code = result.to_image(EcLevel::L).unwrap();
            let image = code
                .render::<unicode::Dense1x2>()
                .dark_color(unicode::Dense1x2::Light)
                .light_color(unicode::Dense1x2::Dark)
                .build();
            println!("{}", image);
            println!("*************************************************");
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
