# rust-l10n

A lightweight internationalization (i18n) library for Rust, inspired by [go-l10n](https://github.com/ideamans/go-l10n). This library provides simple, distributed translation support with automatic language detection from environment variables.

[日本語版 README](README_ja.md)

## Features

- 🌍 **Automatic Language Detection** - Detects language from environment variables (`LANGUAGE`, `LC_ALL`, `LC_MESSAGES`, etc.)
- 📦 **Distributed Translation Registration** - Each module can register its own translations independently
- 🚀 **Simple API** - Easy-to-use functions: `t()`, `f()`, `e()`
- 🔧 **Zero Configuration** - Works out of the box with sensible defaults
- 🧪 **Test-Friendly** - Dependency injection for environment variables makes testing easy
- ⚡ **Lightweight** - Minimal dependencies and small binary size

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-l10n = "0.1"
ctor = "0.2"  # Required for automatic initialization
```

## Quick Start

```rust
use ctor::ctor;
use rust_l10n::{register_translations, t, f};

// Register translations at module initialization
#[ctor]
fn init() {
    register_translations! {
        ja: {
            "Hello" => "こんにちは",
            "Welcome to {}" => "{}へようこそ",
        },
        es: {
            "Hello" => "Hola",
            "Welcome to {}" => "Bienvenido a {}",
        }
    }
}

fn main() {
    // Automatic language detection from environment
    println!("{}", t!("Hello"));
    println!("{}", f!("Welcome to {}", "Rust"));
    
    // Force a specific language
    rust_l10n::force_language("ja");
    println!("{}", t!("Hello"));  // Output: こんにちは
}
```

## Distributed Translation Registration

Each module can register its own translations independently:

```rust
// auth.rs
mod auth {
    use ctor::ctor;
    use rust_l10n::{register_translations, t};

    #[ctor]
    fn init() {
        register_translations! {
            ja: {
                "Invalid credentials" => "認証情報が無効です",
                "Login successful" => "ログインに成功しました",
            }
        }
    }

    pub fn login(user: &str, pass: &str) -> Result<String, String> {
        // Your authentication logic here
        Ok(t!("Login successful"))
    }
}

// file_manager.rs
mod file_manager {
    use ctor::ctor;
    use rust_l10n::{register_translations, t};

    #[ctor]
    fn init() {
        register_translations! {
            ja: {
                "File not found" => "ファイルが見つかりません",
                "File saved" => "ファイルを保存しました",
            }
        }
    }
    
    pub fn save_file(name: &str) -> String {
        t!("File saved")
    }
}
```

## Language Detection Priority

1. Forced language via `force_language()`
2. `L10N_TEST_MODE` environment variable
3. Standard locale environment variables:
   - `LANGUAGE`
   - `LC_ALL`
   - `LC_MESSAGES`
   - `LANG`
4. `L10N_DEFAULT_LANGUAGE` environment variable
5. Default fallback: `"en"`

## API Reference

### Core Functions

- `t(phrase)` - Translate a phrase
- `f(phrase, args)` - Format and translate with arguments
- `e(phrase, args)` - Create translated error message
- `register(lang, lexicon)` - Register translations for a language
- `force_language(lang)` - Force a specific language
- `reset_language()` - Reset to automatic detection
- `detect_language()` - Get the currently detected language

### Macros

- `t!("phrase")` - Translate macro
- `f!("phrase {}", arg)` - Format macro
- `e!("error: {}", arg)` - Error macro
- `register_translations! { ... }` - Bulk registration macro

## Environment Variables

- `LANGUAGE`, `LC_ALL`, `LC_MESSAGES`, `LANG` - Standard locale variables
- `L10N_DEFAULT_LANGUAGE` - Set default language (fallback)
- `L10N_TEST_MODE` - Force specific language for testing
- `L10N_SKIP_DETECTION` - Disable automatic detection

## Testing

The library provides dependency injection for environment variables, making it easy to test:

```rust
#[cfg(test)]
mod tests {
    use rust_l10n::{force_language, t};

    #[test]
    fn test_japanese_translation() {
        force_language("ja");
        assert_eq!(t("Hello"), "こんにちは");
    }
}
```

## Examples

Run the examples:

```bash
# Basic usage
cargo run --example basic

# Run with Japanese locale
LANGUAGE=ja cargo run --example basic

# Modular translations
cargo run --example modular
```

## Comparison with Other i18n Libraries

Unlike other Rust i18n libraries that use compile-time optimization and centralized translation files, rust-l10n follows a distributed approach where each module manages its own translations. This makes it particularly suitable for:

- Porting projects from Go
- Microservices and modular applications
- Projects where translations should live close to the code
- Applications requiring runtime translation registration

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Credits

Inspired by [go-l10n](https://github.com/ideamans/go-l10n) - A lightweight i18n library for Go.