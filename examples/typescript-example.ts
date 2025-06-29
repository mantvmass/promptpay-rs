import init, { 
  PromptPayWasm, 
  quick_generate_wasm, 
  generate_with_svg_wasm,
  validate_merchant_id_wasm,
  is_valid_thai_phone,
  is_valid_tax_id,
  is_valid_ewallet_id,
  is_valid_amount,
  sanitize_phone
} from 'promptpay-rs';

// ตัวอย่างการใช้งาน PromptPay QR Generator ใน TypeScript

async function main() {
  // เริ่มต้น WebAssembly module
  await init();
  
  console.log('🚀 PromptPay QR Generator - TypeScript Example');
  console.log('==============================================');

  // ตัวอย่างที่ 1: การใช้งานอย่างรวดเร็ว
  console.log('\n📱 ตัวอย่างที่ 1: การใช้งานอย่างรวดเร็ว');
  try {
    const payload = await quick_generate_wasm('0812345678', 100.50);
    console.log('Payload:', payload);
  } catch (error) {
    console.error('Error:', error);
  }

  // ตัวอย่างที่ 2: การใช้งานแบบ class
  console.log('\n🏗️ ตัวอย่างที่ 2: การใช้งานแบบ class');
  try {
    const promptpay = new PromptPayWasm('0812345678');
    promptpay.set_amount(250.75);
    
    const merchantType = promptpay.get_merchant_type();
    console.log('Merchant Type:', merchantType);
    
    const payload = await promptpay.generate_payload();
    console.log('Payload:', payload);
    
    const svg = await promptpay.generate_svg();
    console.log('SVG generated successfully!');
    
    const html = await promptpay.generate_html();
    console.log('HTML img tag generated!');
    
    const allFormats = await promptpay.generate_all();
    console.log('All formats generated:', {
      hasSvg: !!allFormats.svg,
      hasPng: !!allFormats.png_base64,
      hasHtml: !!allFormats.html_img
    });
  } catch (error) {
    console.error('Error:', error);
  }

  // ตัวอย่างที่ 3: การใช้งานกับ Tax ID
  console.log('\n🏢 ตัวอย่างที่ 3: การใช้งานกับ Tax ID');
  try {
    const taxId = '1234567890123'; // ตัวอย่าง Tax ID
    const promptpay = new PromptPayWasm(taxId);
    promptpay.set_amount(500.00);
    
    const merchantType = promptpay.get_merchant_type();
    console.log('Merchant Type:', merchantType);
    
    const svg = await promptpay.generate_svg();
    console.log('Tax ID QR Code SVG generated!');
  } catch (error) {
    console.error('Error:', error);
  }

  // ตัวอย่างที่ 4: การใช้งานกับ E-Wallet ID
  console.log('\n💳 ตัวอย่างที่ 4: การใช้งานกับ E-Wallet ID');
  try {
    const ewalletId = '123456789012345'; // ตัวอย่าง E-Wallet ID
    const promptpay = new PromptPayWasm(ewalletId);
    
    const merchantType = promptpay.get_merchant_type();
    console.log('Merchant Type:', merchantType);
    
    const svg = await promptpay.generate_svg();
    console.log('E-Wallet QR Code SVG generated!');
  } catch (error) {
    console.error('Error:', error);
  }

  // ตัวอย่างที่ 5: การตรวจสอบข้อมูล
  console.log('\n✅ ตัวอย่างที่ 5: การตรวจสอบข้อมูล');
  
  // ตรวจสอบเบอร์โทรศัพท์
  console.log('เบอร์โทรศัพท์ 0812345678:', is_valid_thai_phone('0812345678'));
  console.log('เบอร์โทรศัพท์ 66812345678:', is_valid_thai_phone('66812345678'));
  console.log('เบอร์โทรศัพท์ 1234567890:', is_valid_thai_phone('1234567890'));
  
  // ตรวจสอบ Tax ID
  console.log('Tax ID 1234567890123:', is_valid_tax_id('1234567890123'));
  
  // ตรวจสอบจำนวนเงิน
  console.log('จำนวนเงิน 100.50:', is_valid_amount(100.50));
  console.log('จำนวนเงิน -10.00:', is_valid_amount(-10.00));
  console.log('จำนวนเงิน 1000000000.00:', is_valid_amount(1000000000.00));
  
  // ทำความสะอาดเบอร์โทรศัพท์
  console.log('ทำความสะอาดเบอร์โทรศัพท์:', sanitize_phone('081-234-5678'));
  console.log('ทำความสะอาดเบอร์โทรศัพท์:', sanitize_phone('+66 81 234 5678'));

  // ตัวอย่างที่ 6: การใช้งานกับ custom config
  console.log('\n⚙️ ตัวอย่างที่ 6: การใช้งานกับ custom config');
  try {
    const config = {
      country_code: 'TH',
      currency_code: '764',
      qr_size: 512,
      qr_dark_color: '#1a1a1a',
      qr_light_color: '#ffffff',
      qr_quiet_zone: 8,
      validate_input: true
    };
    
    const promptpay = await PromptPayWasm.with_config('0812345678', config);
    promptpay.set_amount(150.25);
    
    const svg = await promptpay.generate_svg();
    console.log('Custom config QR Code generated!');
  } catch (error) {
    console.error('Error:', error);
  }

  console.log('\n🎉 ตัวอย่างการใช้งานเสร็จสิ้น!');
}

// เรียกใช้ฟังก์ชันหลัก
main().catch(console.error);

// ตัวอย่างการใช้งานใน Node.js environment
export async function generatePromptPayQR(merchantId: string, amount?: number) {
  await init();
  
  const promptpay = new PromptPayWasm(merchantId);
  if (amount) {
    promptpay.set_amount(amount);
  }
  
  const result = await promptpay.generate_all();
  return result;
}

// ตัวอย่างการใช้งาน validation functions
export function validatePromptPayInput(merchantId: string, amount?: number) {
  const errors: string[] = [];
  
  // ตรวจสอบ merchant ID
  if (!is_valid_thai_phone(merchantId) && 
      !is_valid_tax_id(merchantId) && 
      !is_valid_ewallet_id(merchantId)) {
    errors.push('Invalid merchant ID format');
  }
  
  // ตรวจสอบจำนวนเงิน
  if (amount !== undefined && !is_valid_amount(amount)) {
    errors.push('Invalid amount');
  }
  
  return {
    isValid: errors.length === 0,
    errors
  };
} 