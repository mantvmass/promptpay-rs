use dioxus::prelude::*;
use dioxus_logger::tracing::{self, Level};
use promptpay_rs::{
    qrcode::{render::svg, EcLevel},
    FormatterTrait, PromptPayQR,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus_logger::init(Level::ERROR).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}

    }
}

#[component]
fn Hero() -> Element {
    let mut target = use_signal(|| String::new());
    let mut amount = use_signal(|| String::new());

    let format_amount = move |value: String| -> String {
        if value.is_empty() {
            return String::new();
        }

        let cleaned: String = value
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.')
            .collect();

        if cleaned.is_empty() {
            return String::new();
        }

        match cleaned.parse::<f64>() {
            Ok(num) => {
                format!("{:.2}", num)
            }
            Err(_) => value,
        }
    };

    let qr_result = use_memo(move || {
        let target_val = target();
        if target_val.is_empty() {
            return None;
        }

        let mut qr = PromptPayQR::new(&target_val);
        if let Ok(amt) = format_amount(amount()).parse::<f64>() {
            if amt > 0.0 {
                qr.set_amount(amt);
            }
        }

        match qr.create() {
            Ok(payload) => match payload.to_image(EcLevel::M) {
                Ok(qr_code) => {
                    let svg = qr_code
                        .render::<svg::Color>()
                        .min_dimensions(256, 256)
                        .build();
                    Some((svg, payload.to_string()))
                }
                Err(e) => {
                    tracing::info!("QR generation error: {:?}", e);
                    None
                }
            },
            Err(e) => {
                tracing::info!("Payload error: {:?}", e);
                None
            }
        }
    });

    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-blue-50 to-indigo-50 flex items-center justify-center p-3 sm:p-6 lg:p-8",
            div { class: "w-full max-w-sm sm:max-w-md lg:max-w-lg",

                div { class: "bg-white rounded-xl sm:rounded-2xl shadow-lg sm:shadow-xl border border-gray-100 overflow-hidden",

                    div { class: "bg-gradient-to-r from-blue-600 to-indigo-700 px-4 py-5 sm:px-6 sm:py-6 text-white text-center",
                        h1 { class: "text-xl sm:text-2xl lg:text-3xl font-bold mb-1", "PromptPay QR Generator" }
                        p { class: "text-blue-100 text-xs sm:text-sm", "สร้าง QR Code สำหรับรับเงินผ่าน PromptPay" }
                    }

                    div { class: "px-4 py-5 sm:px-6 sm:py-6 space-y-4 sm:space-y-5",

                        div {
                            label {
                                class: "block text-sm sm:text-base font-medium text-gray-700 mb-2",
                                "เบอร์มือถือ, รหัสประจำตัวประชาชน, Tax IDs, e-Wallet"
                            }
                            input {
                                r#type: "text",
                                placeholder: "",
                                class: "w-full px-3 py-3 sm:px-4 sm:py-3 text-sm sm:text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors",
                                value: "{target}",
                                oninput: move |e| target.set(e.value()),
                            }
                        }

                        div {
                            label {
                                class: "block text-sm sm:text-base font-medium text-gray-700 mb-2",
                                "จำนวนเงิน (ไม่จำเป็นต้องระบุ)"
                            }
                            div {
                                input {
                                    r#type: "number",
                                    placeholder: "0.00",
                                    class: "w-full px-3 py-3 sm:px-4 sm:py-3 text-sm sm:text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors",
                                    value: "{amount}",
                                    step: "0.01",
                                    oninput: move |e| amount.set(e.value()),
                                }
                            }
                        }
                    }

                    div { class: "px-4 pb-5 sm:px-6 sm:pb-6",
                        div { class: "flex justify-center",
                            if let Some((svg, _)) = qr_result() {
                                div { class: "p-3 sm:p-4 bg-white rounded-lg shadow-md border border-gray-200 transition-all duration-300 hover:shadow-lg",
                                    dangerous_inner_html: "{svg}"
                                }
                            } else {
                                div { class: "w-48 h-48 sm:w-64 sm:h-64 lg:w-72 lg:h-72 bg-gray-50 border-2 border-dashed border-gray-300 rounded-lg flex flex-col items-center justify-center transition-colors",
                                    div { class: "text-4xl sm:text-5xl text-gray-400 mb-2", "+" }
                                    p { class: "text-xs sm:text-sm text-gray-500 text-center px-4", "กรอกข้อมูลด้านบนเพื่อสร้าง QR Code" }
                                }
                            }
                        }
                    }

                    div { class: "bg-gray-50 border-t border-gray-200 px-4 py-4 sm:px-6 sm:py-5 space-y-3",
                        div { class: "text-sm sm:text-base text-gray-700 space-y-2 sm:space-y-3",
                            div { class: "flex justify-between items-center",
                                span { "ผู้รับเงิน:" }
                                span {
                                    class: "font-mono font-medium text-gray-900 text-sm sm:text-base truncate max-w-[120px] sm:max-w-[180px] lg:max-w-[220px]",
                                    if target().is_empty() {
                                        span { class: "text-gray-500", "-" }
                                    } else {
                                        "{target}"
                                    }
                                }
                            }
                            div { class: "flex justify-between items-center",
                                span { "จำนวนเงิน:" }
                                span {
                                    class: "font-semibold text-blue-700 text-sm sm:text-base",
                                    if amount().is_empty() {
                                        span { class: "text-gray-500", "-" }
                                    } else {
                                        { format!("฿{}", format_amount(amount())) }
                                    }
                                }
                            }
                        }

                        if qr_result().is_some() {
                            div { class: "pt-3 border-t border-gray-200 mt-3",
                                div { class: "flex items-center justify-center space-x-2 text-xs sm:text-sm text-gray-600 bg-green-50 px-3 py-2 rounded-lg border border-green-100",
                                    span { class: "text-green-600 font-medium", "✓" }
                                    span { "แสกน QR Code นี้ด้วยแอปพลิเคชันธนาคารของคุณ" }
                                }
                            }
                        }
                    }
                }

                div { class: "mt-4 text-center",
                    p { class: "text-md text-gray-500",
                        "สร้างด้วย "
                        a {
                            href: "",
                            class: "text-blue-600 hover:text-blue-800 underline",
                            "promptpay-rs"
                        }
                    }
                }
            }
        }
    }
}
