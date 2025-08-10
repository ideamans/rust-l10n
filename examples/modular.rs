// 各モジュールで独自の翻訳を登録する例

mod auth {
    use ctor::ctor;
    use rust_l10n::{register_translations, t};

    #[ctor]
    fn init() {
        register_translations! {
            ja: {
                "Invalid credentials" => "認証情報が無効です",
                "Login successful" => "ログインに成功しました",
                "Account locked" => "アカウントがロックされています",
                "Password expired" => "パスワードの有効期限が切れています",
            },
            es: {
                "Invalid credentials" => "Credenciales inválidas",
                "Login successful" => "Inicio de sesión exitoso",
                "Account locked" => "Cuenta bloqueada",
                "Password expired" => "Contraseña expirada",
            }
        }
    }

    pub fn login(username: &str, password: &str) -> Result<String, String> {
        // 簡単なデモロジック
        if username == "admin" && password == "secret" {
            Ok(t!("Login successful"))
        } else if username == "locked" {
            Err(t!("Account locked"))
        } else {
            Err(t!("Invalid credentials"))
        }
    }
}

mod file_manager {
    use ctor::ctor;
    use rust_l10n::{register_translations, t, f};

    #[ctor]
    fn init() {
        register_translations! {
            ja: {
                "File saved" => "ファイルを保存しました",
                "File deleted" => "ファイルを削除しました",
                "Cannot open {}" => "{}を開けません",
                "Permission denied" => "アクセスが拒否されました",
            },
            es: {
                "File saved" => "Archivo guardado",
                "File deleted" => "Archivo eliminado",
                "Cannot open {}" => "No se puede abrir {}",
                "Permission denied" => "Permiso denegado",
            }
        }
    }

    pub fn save_file(name: &str) -> String {
        // デモ実装
        if name.starts_with('.') {
            t!("Permission denied")
        } else {
            t!("File saved")
        }
    }

    pub fn open_file(name: &str) -> Result<String, String> {
        if name == "secret.txt" {
            Err(f!("Cannot open {}", name))
        } else {
            Ok(format!("Contents of {}", name))
        }
    }
}

use rust_l10n::force_language;

fn main() {
    println!("=== Modular Translation Example ===\n");

    // 各言語でテスト
    for lang in &["en", "ja", "es"] {
        force_language(lang);
        println!("Language: {}", lang);
        println!("---");

        // 認証モジュールのテスト
        match auth::login("admin", "secret") {
            Ok(msg) => println!("  Auth: {}", msg),
            Err(msg) => println!("  Auth: {}", msg),
        }

        match auth::login("user", "wrong") {
            Ok(msg) => println!("  Auth: {}", msg),
            Err(msg) => println!("  Auth: {}", msg),
        }

        // ファイル管理モジュールのテスト
        println!("  File: {}", file_manager::save_file("document.txt"));
        
        match file_manager::open_file("secret.txt") {
            Ok(msg) => println!("  File: {}", msg),
            Err(msg) => println!("  File: {}", msg),
        }

        println!();
    }
}