declare module 'promptpay-rs' {
  export interface PromptPayConfig {
    country_code: string;
    currency_code: string;
    qr_size: number;
    qr_dark_color: string;
    qr_light_color: string;
    qr_quiet_zone: number;
    validate_input: boolean;
  }

  export interface PromptPayData {
    merchant_id: string;
    merchant_type: string;
    amount?: number;
    country_code: string;
    currency_code: string;
    payload: string;
  }

  export interface QRResult {
    payload: string;
    svg?: string;
    png_base64?: string;
    html_img?: string;
    merchant_info: PromptPayData;
  }

  export type MerchantType = 'Phone' | 'Tax ID' | 'E-Wallet' | 'Unknown';
  export type OutputFormat = 'payload' | 'svg' | 'png' | 'base64png' | 'html' | 'json' | 'all';

  export class PromptPayWasm {
    constructor(merchant_id: string);
    
    static with_config(merchant_id: string, config: PromptPayConfig): Promise<PromptPayWasm>;
    
    set_amount(amount: number): PromptPayWasm;
    validate(): Promise<boolean>;
    get_merchant_type(): string;
    generate_payload(): Promise<string>;
    generate_qr(format: OutputFormat): Promise<QRResult>;
    generate_svg(): Promise<string>;
    generate_base64_png(): Promise<string>;
    generate_html(): Promise<string>;
    generate_all(): Promise<QRResult>;
  }

  // WebAssembly initialization function
  export default function init(): Promise<void>;

  // Convenience functions
  export function quick_generate_wasm(merchant_id: string, amount?: number): Promise<string>;
  export function generate_with_svg_wasm(merchant_id: string, amount?: number): Promise<QRResult>;
  export function validate_merchant_id_wasm(merchant_id: string): Promise<MerchantType>;
  
  // Validation functions
  export function is_valid_thai_phone(phone: string): boolean;
  export function is_valid_tax_id(tax_id: string): boolean;
  export function is_valid_ewallet_id(ewallet_id: string): boolean;
  export function is_valid_amount(amount: number): boolean;
  
  // Utility functions
  export function sanitize_phone(phone: string): string;
  export function sanitize_numbers(input: string): string;
  export function identify_merchant_type(merchant_id: string): MerchantType;
} 