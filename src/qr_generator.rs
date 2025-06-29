use qrcode::{QrCode, render::svg};
use image::{ImageBuffer, Luma};
use base64::{Engine as _, engine::general_purpose};
use crate::error::PromptPayError;

/// โครงสร้างสำหรับการสร้าง QR Code
pub struct QRGenerator;

impl QRGenerator {
    /// สร้าง QR Code จาก payload และคืนค่าเป็น SVG string
    pub fn generate_svg(payload: &str, size: u32) -> Result<String, PromptPayError> {
        let code = QrCode::new(payload)
            .map_err(|e| PromptPayError::QrGenerationFailed(e.to_string()))?;
        
        let svg_string = code.render()
            .min_dimensions(size, size)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#FFFFFF"))
            .build();
        
        Ok(svg_string)
    }
    
    /// สร้าง QR Code จาก payload และคืนค่าเป็น PNG image bytes
    pub fn generate_png(payload: &str, size: u32) -> Result<Vec<u8>, PromptPayError> {
        let code = QrCode::new(payload)
            .map_err(|e| PromptPayError::QrGenerationFailed(e.to_string()))?;
        
        let image_buffer = code.render::<Luma<u8>>()
            .module_dimensions(size / code.width() as u32, size / code.width() as u32)
            .build();
        
        let mut png_bytes = Vec::new();
        image_buffer.write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png)
            .map_err(|e| PromptPayError::ImageGenerationFailed(e.to_string()))?;
        
        Ok(png_bytes)
    }
    
    /// สร้าง QR Code จาก payload และคืนค่าเป็น Base64 encoded PNG
    pub fn generate_base64_png(payload: &str, size: u32) -> Result<String, PromptPayError> {
        let png_bytes = Self::generate_png(payload, size)?;
        let base64_string = general_purpose::STANDARD.encode(&png_bytes);
        Ok(format!("data:image/png;base64,{}", base64_string))
    }
    
    /// สร้าง QR Code จาก payload และคืนค่าเป็น HTML img tag
    pub fn generate_html_img(payload: &str, size: u32, alt: Option<&str>) -> Result<String, PromptPayError> {
        let base64_png = Self::generate_base64_png(payload, size)?;
        let alt_text = alt.unwrap_or("PromptPay QR Code");
        Ok(format!(
            r#"<img src="{}" alt="{}" width="{}" height="{}" />"#,
            base64_png, alt_text, size, size
        ))
    }
    
    /// สร้าง QR Code จาก payload และบันทึกเป็นไฟล์ PNG
    pub fn save_png(payload: &str, file_path: &str, size: u32) -> Result<(), PromptPayError> {
        let png_bytes = Self::generate_png(payload, size)?;
        std::fs::write(file_path, png_bytes)
            .map_err(|e| PromptPayError::ImageGenerationFailed(e.to_string()))?;
        Ok(())
    }
    
    /// สร้าง QR Code จาก payload และบันทึกเป็นไฟล์ SVG
    pub fn save_svg(payload: &str, file_path: &str, size: u32) -> Result<(), PromptPayError> {
        let svg_string = Self::generate_svg(payload, size)?;
        std::fs::write(file_path, svg_string)
            .map_err(|e| PromptPayError::ImageGenerationFailed(e.to_string()))?;
        Ok(())
    }
}

/// โครงสร้างสำหรับการตั้งค่า QR Code
#[derive(Debug, Clone)]
pub struct QRConfig {
    pub size: u32,
    pub dark_color: String,
    pub light_color: String,
    pub quiet_zone: u32,
}

impl Default for QRConfig {
    fn default() -> Self {
        QRConfig {
            size: 256,
            dark_color: "#000000".to_string(),
            light_color: "#FFFFFF".to_string(),
            quiet_zone: 4,
        }
    }
}

impl QRConfig {
    /// สร้าง QR Code ด้วยการตั้งค่าที่กำหนดเอง
    pub fn generate_svg(&self, payload: &str) -> Result<String, PromptPayError> {
        let code = QrCode::new(payload)
            .map_err(|e| PromptPayError::QrGenerationFailed(e.to_string()))?;
        
        let svg_string = code.render()
            .min_dimensions(self.size, self.size)
            .dark_color(svg::Color(&self.dark_color))
            .light_color(svg::Color(&self.light_color))
            .quiet_zone(self.quiet_zone)
            .build();
        
        Ok(svg_string)
    }
} 