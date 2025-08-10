# rust-l10n

[go-l10n](https://github.com/ideamans/go-l10n)にインスパイアされた、Rust用の軽量な国際化（i18n）ライブラリです。環境変数から自動的に言語を検出し、シンプルで分散的な翻訳サポートを提供します。

[English README](README.md)

## 特徴

- 🌍 **自動言語検出** - 環境変数（`LANGUAGE`、`LC_ALL`、`LC_MESSAGES`など）から言語を自動検出
- 📦 **分散翻訳登録** - 各モジュールが独立して独自の翻訳を登録可能
- 🚀 **シンプルなAPI** - 使いやすい関数: `t()`、`f()`、`e()`
- 🔧 **ゼロ設定** - デフォルト設定ですぐに動作
- 🧪 **テストフレンドリー** - 環境変数の依存性注入によりテストが簡単
- ⚡ **軽量** - 最小限の依存関係と小さなバイナリサイズ

## インストール

`Cargo.toml`に以下を追加：

```toml
[dependencies]
rust-l10n = "0.1"
ctor = "0.2"  # 自動初期化に必要
```

## クイックスタート

```rust
use ctor::ctor;
use rust_l10n::{register_translations, t, f};

// モジュール初期化時に翻訳を登録
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
    // 環境から自動的に言語を検出
    println!("{}", t!("Hello"));
    println!("{}", f!("Welcome to {}", "Rust"));
    
    // 特定の言語を強制
    rust_l10n::force_language("ja");
    println!("{}", t!("Hello"));  // 出力: こんにちは
}
```

## 分散翻訳登録

各モジュールが独立して独自の翻訳を登録できます：

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
        // 認証ロジック
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

## 言語検出の優先順位

1. `force_language()`による強制設定
2. `L10N_TEST_MODE`環境変数
3. 標準ロケール環境変数：
   - `LANGUAGE`
   - `LC_ALL`
   - `LC_MESSAGES`
   - `LANG`
4. `L10N_DEFAULT_LANGUAGE`環境変数
5. デフォルトフォールバック: `"en"`

## APIリファレンス

### コア関数

- `t(phrase)` - フレーズを翻訳
- `f(phrase, args)` - 引数付きでフォーマットして翻訳
- `e(phrase, args)` - 翻訳されたエラーメッセージを作成
- `register(lang, lexicon)` - 言語の翻訳を登録
- `force_language(lang)` - 特定の言語を強制
- `reset_language()` - 自動検出にリセット
- `detect_language()` - 現在検出されている言語を取得

### マクロ

- `t!("phrase")` - 翻訳マクロ
- `f!("phrase {}", arg)` - フォーマットマクロ
- `e!("error: {}", arg)` - エラーマクロ
- `register_translations! { ... }` - 一括登録マクロ

## 環境変数

- `LANGUAGE`、`LC_ALL`、`LC_MESSAGES`、`LANG` - 標準ロケール変数
- `L10N_DEFAULT_LANGUAGE` - デフォルト言語を設定（フォールバック）
- `L10N_TEST_MODE` - テスト用に特定の言語を強制
- `L10N_SKIP_DETECTION` - 自動検出を無効化

## テスト

このライブラリは環境変数の依存性注入を提供し、テストを簡単にします：

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

## サンプル

サンプルの実行：

```bash
# 基本的な使用法
cargo run --example basic

# 日本語ロケールで実行
LANGUAGE=ja cargo run --example basic

# モジュラー翻訳
cargo run --example modular
```

## 他のi18nライブラリとの比較

コンパイル時最適化と集中管理型の翻訳ファイルを使用する他のRust i18nライブラリとは異なり、rust-l10nは各モジュールが独自の翻訳を管理する分散アプローチを採用しています。これは特に以下の場合に適しています：

- Goからのプロジェクト移植
- マイクロサービスとモジュラーアプリケーション
- 翻訳をコードの近くに配置したいプロジェクト
- 実行時の翻訳登録が必要なアプリケーション

### 既存ライブラリとの違い

| 機能 | rust-l10n | rust-i18n | fluent-rs |
|------|-----------|-----------|-----------|
| 分散登録 | ✅ | ❌ | ❌ |
| 環境変数検出 | ✅ | ❌ | ❌ |
| 実行時登録 | ✅ | ❌ | ✅ |
| コンパイル時最適化 | ❌ | ✅ | ❌ |
| Go互換性 | ✅ | ❌ | ❌ |

## なぜrust-l10nを選ぶか

### 適している場合

- **Goからの移植** - go-l10nと同じ設計思想
- **分散システム** - 各サービスが独自の翻訳を管理
- **開発者体験重視** - 翻訳とコードが同じ場所
- **軽量アプリケーション** - 最小限の依存関係

### 適していない場合

- **Webフロントエンド** - rust-i18nの方が適切
- **複雑な文法規則** - fluent-rsやICU4Xを検討
- **大規模な翻訳ファイル** - 専用の管理ツールが必要

## ライセンス

このプロジェクトは以下のいずれかのライセンスで提供されています：

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) または http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) または http://opensource.org/licenses/MIT)

お好きな方をお選びください。

## コントリビューション

コントリビューションを歓迎します！お気軽にPull Requestを送ってください。

## クレジット

[go-l10n](https://github.com/ideamans/go-l10n) - Go用の軽量i18nライブラリにインスパイアされました。