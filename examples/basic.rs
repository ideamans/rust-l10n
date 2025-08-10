use ctor::ctor;
use rust_l10n::{register_translations, t, f, force_language, reset_language, detect_language};

// ctorで自動初期化される翻訳登録
#[ctor]
fn init_translations() {
    register_translations! {
        ja: {
            "Hello" => "こんにちは",
            "Welcome to {}" => "{}へようこそ",
            "Error: {}" => "エラー: {}",
            "File not found" => "ファイルが見つかりません",
            "Success" => "成功",
        },
        es: {
            "Hello" => "Hola",
            "Welcome to {}" => "Bienvenido a {}",
            "Error: {}" => "Error: {}",
            "File not found" => "Archivo no encontrado",
            "Success" => "Éxito",
        }
    }
}

fn main() {
    println!("=== rust-l10n Demo ===\n");

    // 現在の言語を検出
    println!("Detected language: {}\n", detect_language());

    // デフォルト（環境変数による）
    println!("Default (from environment):");
    println!("  {}", t!("Hello"));
    println!("  {}", f!("Welcome to {}", "Rust"));
    println!("  {}\n", t!("Success"));

    // 日本語に切り替え
    force_language("ja");
    println!("Japanese (forced):");
    println!("  {}", t!("Hello"));
    println!("  {}", f!("Welcome to {}", "Rust"));
    println!("  {}", t!("File not found"));
    println!("  {}\n", f!("Error: {}", "接続失敗"));

    // スペイン語に切り替え
    force_language("es");
    println!("Spanish (forced):");
    println!("  {}", t!("Hello"));
    println!("  {}", f!("Welcome to {}", "Rust"));
    println!("  {}\n", t!("Success"));

    // 環境変数による自動検出に戻す
    reset_language();
    println!("Back to auto-detection:");
    println!("  {}", t!("Hello"));
}