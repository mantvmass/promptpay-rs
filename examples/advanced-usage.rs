use promptpay_rs::{
    PromptPayQR, PromptPayConfig, OutputFormat, QRResult,
    Validator, validate_merchant_id, quick_generate, generate_with_svg
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 PromptPay-RS Advanced Usage Examples");
    println!("======================================");

    // ตัวอย่างที่ 1: การใช้งานแบบพื้นฐาน
    println!("\n📱 ตัวอย่างที่ 1: การใช้งานแบบพื้นฐาน");
    basic_usage()?;

    // ตัวอย่างที่ 2: การใช้งานกับ Tax ID
    println!("\n🏢 ตัวอย่างที่ 2: การใช้งานกับ Tax ID");
    tax_id_usage()?;

    // ตัวอย่างที่ 3: การใช้งานกับ E-Wallet ID
    println!("\n💳 ตัวอย่างที่ 3: การใช้งานกับ E-Wallet ID");
    ewallet_usage()?;

    // ตัวอย่างที่ 4: การใช้งานกับ custom configuration
    println!("\n⚙️ ตัวอย่างที่ 4: การใช้งานกับ custom configuration");
    custom_config_usage()?;

    // ตัวอย่างที่ 5: การสร้าง QR Code ในรูปแบบต่างๆ
    println!("\n🎨 ตัวอย่างที่ 5: การสร้าง QR Code ในรูปแบบต่างๆ");
    multiple_formats_usage()?;

    // ตัวอย่างที่ 6: การบันทึกไฟล์
    println!("\n💾 ตัวอย่างที่ 6: การบันทึกไฟล์");
    file_save_usage()?;

    // ตัวอย่างที่ 7: การตรวจสอบข้อมูล
    println!("\n✅ ตัวอย่างที่ 7: การตรวจสอบข้อมูล");
    validation_usage()?;

    // ตัวอย่างที่ 8: การใช้งาน convenience functions
    println!("\n⚡ ตัวอย่างที่ 8: การใช้งาน convenience functions");
    convenience_functions_usage()?;

    println!("\n🎉 ตัวอย่างการใช้งานเสร็จสิ้น!");
    Ok(())
}

fn basic_usage() -> Result<(), Box<dyn std::error::Error>> {
    let mut qr = PromptPayQR::new("0812345678");
    qr.set_amount(100.50);
    
    let payload = qr.generate_payload()?;
    println!("Payload: {}", payload);
    
    let result = qr.generate_qr(OutputFormat::SVG)?;
    if let Some(svg) = result.svg {
        println!("SVG generated successfully! ({} characters)", svg.len());
    }
    
    Ok(())
}

fn tax_id_usage() -> Result<(), Box<dyn std::error::Error>> {
    let tax_id = "1234567890123"; // ตัวอย่าง Tax ID
    
    // ตรวจสอบ Tax ID ก่อน
    if Validator::is_valid_tax_id(tax_id) {
        let mut qr = PromptPayQR::new(tax_id);
        qr.set_amount(500.00);
        
        let merchant_type = qr.get_merchant_type();
        println!("Merchant Type: {}", merchant_type);
        
        let result = qr.generate_qr(OutputFormat::All)?;
        println!("Tax ID QR Code generated with all formats!");
        println!("  - Has SVG: {}", result.svg.is_some());
        println!("  - Has PNG: {}", result.png_base64.is_some());
        println!("  - Has HTML: {}", result.html_img.is_some());
    } else {
        println!("Invalid Tax ID: {}", tax_id);
    }
    
    Ok(())
}

fn ewallet_usage() -> Result<(), Box<dyn std::error::Error>> {
    let ewallet_id = "123456789012345"; // ตัวอย่าง E-Wallet ID
    
    if Validator::is_valid_ewallet_id(ewallet_id) {
        let qr = PromptPayQR::new(ewallet_id);
        
        let merchant_type = qr.get_merchant_type();
        println!("Merchant Type: {}", merchant_type);
        
        let result = qr.generate_qr(OutputFormat::Base64PNG)?;
        if let Some(png) = result.png_base64 {
            println!("E-Wallet QR Code PNG generated! ({} characters)", png.len());
        }
    } else {
        println!("Invalid E-Wallet ID: {}", ewallet_id);
    }
    
    Ok(())
}

fn custom_config_usage() -> Result<(), Box<dyn std::error::Error>> {
    let config = PromptPayConfig {
        country_code: "TH".to_string(),
        currency_code: "764".to_string(),
        qr_size: 512,
        qr_dark_color: "#1a1a1a".to_string(),
        qr_light_color: "#ffffff".to_string(),
        qr_quiet_zone: 8,
        validate_input: true,
    };
    
    let qr = PromptPayQR::with_config("0812345678", config);
    qr.set_amount(150.25);
    
    let result = qr.generate_qr(OutputFormat::SVG)?;
    if let Some(svg) = result.svg {
        println!("Custom config QR Code generated! ({} characters)", svg.len());
    }
    
    Ok(())
}

fn multiple_formats_usage() -> Result<(), Box<dyn std::error::Error>> {
    let mut qr = PromptPayQR::new("0812345678");
    qr.set_amount(75.50);
    
    // สร้างในรูปแบบต่างๆ
    let formats = vec![
        OutputFormat::Payload,
        OutputFormat::SVG,
        OutputFormat::Base64PNG,
        OutputFormat::HTML,
        OutputFormat::All,
    ];
    
    for format in formats {
        let result = qr.generate_qr(format)?;
        match format {
            OutputFormat::Payload => println!("Payload format: {} characters", result.payload.len()),
            OutputFormat::SVG => println!("SVG format: {} characters", result.svg.unwrap_or_default().len()),
            OutputFormat::Base64PNG => println!("PNG format: {} characters", result.png_base64.unwrap_or_default().len()),
            OutputFormat::HTML => println!("HTML format: {} characters", result.html_img.unwrap_or_default().len()),
            OutputFormat::All => println!("All formats: SVG={}, PNG={}, HTML={}", 
                result.svg.is_some(), result.png_base64.is_some(), result.html_img.is_some()),
            _ => println!("Other format generated"),
        }
    }
    
    Ok(())
}

fn file_save_usage() -> Result<(), Box<dyn std::error::Error>> {
    let mut qr = PromptPayQR::new("0812345678");
    qr.set_amount(200.00);
    
    // บันทึกเป็นไฟล์ PNG
    match qr.save_qr("output.png", "png") {
        Ok(()) => println!("PNG file saved as output.png"),
        Err(e) => println!("Failed to save PNG: {}", e),
    }
    
    // บันทึกเป็นไฟล์ SVG
    match qr.save_qr("output.svg", "svg") {
        Ok(()) => println!("SVG file saved as output.svg"),
        Err(e) => println!("Failed to save SVG: {}", e),
    }
    
    Ok(())
}

fn validation_usage() -> Result<(), Box<dyn std::error::Error>> {
    // ตรวจสอบเบอร์โทรศัพท์
    let phones = vec!["0812345678", "66812345678", "1234567890"];
    for phone in phones {
        let is_valid = Validator::is_valid_thai_phone(phone);
        let clean = Validator::sanitize_phone(phone);
        println!("Phone {}: valid={}, cleaned={}", phone, is_valid, clean);
    }
    
    // ตรวจสอบ Tax ID
    let tax_ids = vec!["1234567890123", "1234567890124"];
    for tax_id in tax_ids {
        let is_valid = Validator::is_valid_tax_id(tax_id);
        println!("Tax ID {}: valid={}", tax_id, is_valid);
    }
    
    // ตรวจสอบจำนวนเงิน
    let amounts = vec![100.50, -10.0, 1000000000.0];
    for amount in amounts {
        let is_valid = Validator::is_valid_amount(amount);
        println!("Amount {}: valid={}", amount, is_valid);
    }
    
    // ตรวจสอบ merchant ID และระบุประเภท
    let merchant_ids = vec!["0812345678", "1234567890123", "123456789012345"];
    for merchant_id in merchant_ids {
        match validate_merchant_id(merchant_id) {
            Ok(merchant_type) => println!("Merchant ID {}: type={}", merchant_id, merchant_type),
            Err(e) => println!("Merchant ID {}: error={}", merchant_id, e),
        }
    }
    
    Ok(())
}

fn convenience_functions_usage() -> Result<(), Box<dyn std::error::Error>> {
    // ใช้ quick_generate function
    match quick_generate("0812345678", Some(100.50)) {
        Ok(payload) => println!("Quick generate payload: {} characters", payload.len()),
        Err(e) => println!("Quick generate error: {}", e),
    }
    
    // ใช้ generate_with_svg function
    match generate_with_svg("0812345678", Some(100.50)) {
        Ok(result) => {
            println!("Generate with SVG: {} characters", result.payload.len());
            if let Some(svg) = result.svg {
                println!("SVG: {} characters", svg.len());
            }
        },
        Err(e) => println!("Generate with SVG error: {}", e),
    }
    
    Ok(())
} 