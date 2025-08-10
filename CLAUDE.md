# rust-l10n

go-l10nのRust移植版 - 軽量な国際化(i18n)ライブラリ

## 概要

rust-l10nは、[go-l10n](https://github.com/ideamans/go-l10n)をRustに移植した国際化ライブラリです。シンプルで分散的な翻訳サポートを提供し、環境変数から自動的に言語を検出する機能を持ちます。

## Rustの既存i18nライブラリとの比較

### 主要なRust i18nライブラリ

#### 1. **rust-i18n** (最も人気)
- **特徴**: コンパイル時に翻訳ファイル（YAML/JSON/TOML）を読み込み
- **API**: グローバルな`t!`マクロ
- **管理**: 集中管理型（localesディレクトリ）
- **初期化**: `i18n!("locales")`マクロで一括読み込み

#### 2. **ICU4X** (最も包括的)
- **特徴**: Unicode標準準拠、複雑な複数形、日付・数値フォーマット対応
- **API**: 型安全だが複雑
- **管理**: DataProviderパターン
- **用途**: 大規模で国際標準準拠が必要なアプリケーション

#### 3. **fluent-rs** (Mozilla製)
- **特徴**: 自然言語的な翻訳記法、複雑な文法規則対応
- **API**: `.ftl`ファイル形式
- **管理**: FluentBundleによる管理
- **用途**: 複雑な言語ルールが必要な場合

#### 4. **gettext-rs** (GNU互換)
- **特徴**: GNU Gettext互換、`.po`ファイル使用
- **API**: 標準的なgettext API
- **管理**: 既存のgettextツールチェーン利用可能

### rust-l10nの位置づけと利点

#### 🎯 **rust-l10nの独自性**

1. **分散登録アーキテクチャ**
   ```rust
   // rust-i18n: 集中管理
   i18n!("locales");  // 全翻訳を一箇所で読み込み
   
   // rust-l10n: 分散管理（各モジュールで登録）
   #[ctor]
   fn init() {
       register("ja", lexicon);  // 各ファイルで必要な翻訳のみ
   }
   ```

2. **環境変数による自動言語検出**
   - 他のライブラリ: アプリケーションで明示的に設定
   - rust-l10n: `LANGUAGE`、`LC_ALL`等から自動検出

3. **実行時の動的登録**
   - 他のライブラリ: コンパイル時固定
   - rust-l10n: 実行時に翻訳を追加可能

4. **Go互換性**
   - go-l10nと同じAPIデザイン
   - 既存のGoプロジェクトからの移植が容易

#### 📊 **比較表**

| 機能 | rust-l10n | rust-i18n | ICU4X | fluent-rs |
|------|-----------|-----------|--------|-----------|
| 分散登録 | ✅ | ❌ | ❌ | ❌ |
| 環境変数検出 | ✅ | ❌ | ❌ | ❌ |
| 実行時登録 | ✅ | ❌ | ✅ | ✅ |
| コンパイル時最適化 | ❌ | ✅ | ✅ | ❌ |
| 複雑な文法 | ❌ | ❌ | ✅ | ✅ |
| 学習コスト | 低 | 低 | 高 | 中 |
| バイナリサイズ | 小 | 中 | 大 | 中 |

#### 💡 **rust-l10nが適している場合**

1. **マイクロサービスや分散システム**
   - 各サービスが独自の翻訳を管理
   - 動的なモジュール読み込み

2. **Goからの移植プロジェクト**
   - go-l10nと同じ設計思想
   - 最小限のコード変更で移植可能

3. **開発者体験重視**
   - 翻訳とコードが同じ場所
   - 環境変数による言語切り替え
   - テスト時の言語制御が簡単

4. **軽量なアプリケーション**
   - 最小限の依存関係
   - シンプルなAPI
   - 小さなバイナリサイズ

#### ⚠️ **rust-l10nの制限事項**

1. **コンパイル時最適化なし**
   - rust-i18nのような事前最適化は不可
   - 実行時のオーバーヘッドあり

2. **複雑な言語機能なし**
   - 複数形の自動処理なし
   - 性別による活用なし
   - 複雑な文法規則非対応

3. **ツールチェーンが限定的**
   - 専用のCLIツールなし
   - IDE統合なし
   - 翻訳ファイル管理ツールなし

### 結論

rust-l10nは既存のRust i18nライブラリとは異なるアプローチを採用しています：

- **既存ライブラリ**: 「コンパイル時に全翻訳を集約」というWebアプリ的アプローチ
- **rust-l10n**: 「各モジュールが必要な翻訳を持つ」という分散アプローチ

これは特に**大規模なモジュラーシステム**や**Goからの移植**において優位性があります。一方、**Webフロントエンド**や**単一バイナリアプリ**では、rust-i18nのような既存ライブラリの方が適している場合があります。

## 主要機能

### 1. 自動言語検出
- 環境変数（`LANGUAGE`、`LC_ALL`、`LC_MESSAGES`など）から言語を自動検出
- 言語が検出されない場合は英語にフォールバック
- 日本語と英語をデフォルトでサポート

### 2. 翻訳メカニズム
- `register()` - 各言語の翻訳を登録
- `t()` - 基本的な翻訳
- `f()` - フォーマット付き翻訳
- `e()` - エラーメッセージの翻訳

### 3. 分散翻訳登録
- 複数のモジュールから独立して翻訳を登録可能
- 各パッケージが独自の翻訳を追加できる柔軟な設計

## 主要な型とインターフェース

### 型定義
```rust
// 基本フレーズから翻訳済みフレーズへのマッピング
type LexiconMap = HashMap<String, String>;

// 言語コードから辞書へのマッピング
type WorldMap = HashMap<String, LexiconMap>;
```

### 主要な関数

#### 翻訳登録
```rust
// 特定言語の翻訳を登録
fn register(lang: &str, lexicon: LexiconMap)
```

#### 翻訳機能
```rust
// フレーズを翻訳
fn t(phrase: &str) -> String

// フォーマット付き翻訳
fn f(phrase: &str, args: &[&str]) -> String

// エラーメッセージとして翻訳
fn e(phrase: &str, args: &[&str]) -> Error
```

#### 言語制御
```rust
// 言語を手動で設定
fn force_language(lang: &str)

// 自動言語検出に戻す
fn reset_language()

// 現在の言語を自動検出
fn detect_language() -> String
```

## 環境変数

### 言語設定
- `LANGUAGE` - 優先言語
- `LC_ALL` - すべてのロケール設定
- `LC_MESSAGES` - メッセージのロケール
- `LANG` - デフォルトロケール

### ライブラリ固有の設定
- `L10N_DEFAULT_LANGUAGE` - デフォルト言語を設定
- `L10N_SKIP_DETECTION` - 自動検出を無効化
- `L10N_TEST_MODE` - テストモードで一貫した言語動作を強制

## 使用例

### 基本的な使用方法
```rust
use rust_l10n::{register, t, LexiconMap};

// 初期化時に翻訳を登録
fn init() {
    let mut lexicon = LexiconMap::new();
    lexicon.insert("Hello, World!".to_string(), "こんにちは、世界！".to_string());
    
    register("ja", lexicon);
}

fn main() {
    init();
    
    // 翻訳を使用
    println!("{}", t("Hello, World!"));
    // 環境変数がjaの場合: "こんにちは、世界！"
    // それ以外: "Hello, World!"
}
```

### フォーマット付き翻訳
```rust
use rust_l10n::{register, f, LexiconMap};

fn init() {
    let mut lexicon = LexiconMap::new();
    lexicon.insert("Hello, {}!".to_string(), "こんにちは、{}さん！".to_string());
    
    register("ja", lexicon);
}

fn main() {
    init();
    
    // フォーマット付き翻訳
    println!("{}", f("Hello, {}!", &["Alice"]));
    // 環境変数がjaの場合: "こんにちは、Aliceさん！"
}
```

### テスト時の言語制御
```rust
#[cfg(test)]
mod tests {
    use rust_l10n::{force_language, reset_language, t};
    
    #[test]
    fn test_japanese_translation() {
        // テスト用に言語を固定
        force_language("ja");
        
        assert_eq!(t("Hello"), "こんにちは");
        
        // 自動検出に戻す
        reset_language();
    }
}
```

## 言語検出の優先順位

1. `force_language()`による強制設定
2. テストモード（`L10N_TEST_MODE`環境変数）
3. 環境変数による自動検出
4. デフォルト言語（英語）

## 特徴

- **軽量**: 最小限の依存関係
- **シンプル**: 直感的なAPI
- **柔軟**: 分散的な翻訳登録
- **テストフレンドリー**: テスト時の言語制御機能
- **環境変数対応**: 標準的なロケール環境変数をサポート

## サポート言語

- 日本語 (`ja`)
- 英語 (`en`, フォールバック)
- その他の言語も`register()`で追加可能

## 移植における考慮事項

### Rustでの初期化アプローチ（`ctor`クレート採用）

このプロジェクトでは、Goの`init()`関数と同等の機能を実現するため、**`ctor`クレートを採用**します。

#### なぜ`ctor`クレートを選んだか

1. **Goとの互換性**: Goの`init()`と同じように、各ファイルで自動実行される初期化が可能
2. **シンプルさ**: `#[ctor]`属性を付けるだけで動作
3. **分散登録**: 各モジュールで独立して翻訳を管理できる
4. **メンテナンス性**: コードと翻訳が同じファイルに配置される

#### 基本的な使用方法

**Cargo.tomlへの追加:**
```toml
[dependencies]
ctor = "0.2"
rust-l10n = { path = "." }  # または適切なパス/バージョン
```

**各ソースファイルでの実装:**
```rust
use ctor::ctor;
use rust_l10n::{register, LexiconMap};

#[ctor]
fn init_translations() {
    // 日本語翻訳
    let mut ja = LexiconMap::new();
    ja.insert("Error".to_string(), "エラー".to_string());
    ja.insert("Success".to_string(), "成功".to_string());
    register("ja", ja);
    
    // 必要に応じて他の言語も
    let mut es = LexiconMap::new();
    es.insert("Error".to_string(), "Error".to_string());
    es.insert("Success".to_string(), "Éxito".to_string());
    register("es", es);
}

// 以降、通常のコード
pub fn process() -> Result<String, String> {
    use rust_l10n::t;
    
    if some_condition {
        Ok(t("Success"))
    } else {
        Err(t("Error"))
    }
}
```

#### マクロを使った簡潔な記法

登録を更に簡潔にするため、専用マクロも提供：

```rust
use ctor::ctor;
use rust_l10n::register_translations;

#[ctor]
fn init() {
    register_translations! {
        ja: {
            "Welcome" => "ようこそ",
            "Goodbye" => "さようなら",
            "Thank you" => "ありがとうございます",
        },
        en: {
            "Welcome" => "Welcome",
            "Goodbye" => "Goodbye", 
            "Thank you" => "Thank you",
        }
    }
}
```

#### プロジェクト構成例

```
src/
├── lib.rs           # ライブラリのエントリーポイント
├── core/
│   ├── mod.rs
│   └── engine.rs    # エンジン関連の翻訳
├── auth/
│   ├── mod.rs  
│   └── login.rs     # 認証関連の翻訳
└── api/
    ├── mod.rs
    └── handlers.rs  # API関連の翻訳
```

各ファイルで関連する翻訳を定義：

```rust
// src/auth/login.rs
#[ctor]
fn init() {
    register_translations! {
        ja: {
            "Invalid credentials" => "認証情報が無効です",
            "Login successful" => "ログインに成功しました",
            "Account locked" => "アカウントがロックされています",
        }
    }
}

// src/api/handlers.rs
#[ctor]
fn init() {
    register_translations! {
        ja: {
            "Bad request" => "不正なリクエスト",
            "Not found" => "見つかりません",
            "Internal error" => "内部エラー",
        }
    }
}
```

#### 注意事項

1. **初期化順序**: `ctor`の実行順序は保証されないため、翻訳の登録順序に依存しない設計にする
2. **重複キー**: 同じキーが複数回登録された場合、最後の登録が有効になる
3. **パフォーマンス**: プログラム起動時に全ての`#[ctor]`関数が実行されるため、大量の翻訳がある場合は起動時間に影響する可能性がある

#### テストでの使用

テスト時も`ctor`が自動実行されるため、特別な初期化は不要：

```rust
#[cfg(test)]
mod tests {
    use rust_l10n::{t, force_language};
    
    #[test]
    fn test_japanese_messages() {
        force_language("ja");
        assert_eq!(t("Error"), "エラー");
        assert_eq!(t("Success"), "成功");
    }
}
```

#### その他の初期化方法（参考）

以下は`ctor`を使わない場合の代替方法です：

1. **`std::sync::OnceLock` による遅延初期化**
```rust
use std::sync::OnceLock;
use rust_l10n::{register, LexiconMap};

static INIT: OnceLock<()> = OnceLock::new();

fn ensure_translations() {
    INIT.get_or_init(|| {
        let mut lexicon = LexiconMap::new();
        lexicon.insert("Hello".to_string(), "こんにちは".to_string());
        register("ja", lexicon);
    });
}

// 翻訳を使う前に呼び出す
pub fn my_function() {
    ensure_translations();
    // 翻訳を使用...
}
```

#### 2. マクロによる登録の簡略化
```rust
// マクロ定義
#[macro_export]
macro_rules! register_translations {
    ($lang:expr, { $($key:expr => $value:expr),* $(,)? }) => {
        {
            use std::sync::OnceLock;
            static INIT: OnceLock<()> = OnceLock::new();
            INIT.get_or_init(|| {
                let mut lexicon = LexiconMap::new();
                $(
                    lexicon.insert($key.to_string(), $value.to_string());
                )*
                register($lang, lexicon);
            });
        }
    };
}

// 使用例
register_translations!("ja", {
    "Hello" => "こんにちは",
    "Goodbye" => "さようなら",
});
```

#### 3. ctor クレートを使用した起動時初期化
```rust
// Cargo.tomlに追加: ctor = "0.2"
use ctor::ctor;

#[ctor]
fn init_translations() {
    let mut lexicon = LexiconMap::new();
    lexicon.insert("Hello".to_string(), "こんにちは".to_string());
    register("ja", lexicon);
}
```

#### 4. linkme クレートによる分散登録
```rust
// Cargo.tomlに追加: linkme = "0.3"
use linkme::distributed_slice;

#[distributed_slice]
pub static TRANSLATIONS: [fn()] = [..];

#[distributed_slice(TRANSLATIONS)]
fn register_module_a_translations() {
    // モジュールAの翻訳を登録
}

#[distributed_slice(TRANSLATIONS)]
fn register_module_b_translations() {
    // モジュールBの翻訳を登録
}

// アプリケーション起動時に一度実行
pub fn init_all_translations() {
    for init_fn in TRANSLATIONS {
        init_fn();
    }
}
```

#### 推奨アプローチ

1. **開発の簡便性重視**: `ctor`クレートが最もGoの`init()`に近い
2. **標準ライブラリのみ**: `OnceLock`と遅延初期化パターン
3. **分散登録が必要**: `linkme`クレートによる収集
4. **パフォーマンス重視**: マクロと`OnceLock`の組み合わせ

### 各ソースファイルでの翻訳辞書の記載

各ソースファイルにその機能に関連する翻訳を記載したい場合、以下の方法が有効です：

#### ✅ 実現可能な方法

**1. `ctor`クレート（最も簡単）**
```rust
// src/user.rs
use ctor::ctor;
use rust_l10n::{register, LexiconMap};

#[ctor]
fn init() {
    let mut ja = LexiconMap::new();
    ja.insert("User not found".to_string(), "ユーザーが見つかりません".to_string());
    ja.insert("Invalid email".to_string(), "無効なメールアドレス".to_string());
    register("ja", ja);
}

// この後、通常のコード
pub fn validate_user(email: &str) -> Result<(), String> {
    // t("Invalid email") を使用可能
}
```

**2. `linkme`クレート（明示的な収集）**
```rust
// src/user.rs
use linkme::distributed_slice;
use rust_l10n::{TRANSLATIONS, register, LexiconMap};

#[distributed_slice(TRANSLATIONS)]
fn register_user_translations() {
    let mut ja = LexiconMap::new();
    ja.insert("User not found".to_string(), "ユーザーが見つかりません".to_string());
    register("ja", ja);
}

// main.rsで一度だけ呼び出し
fn main() {
    rust_l10n::init_all_translations();
    // アプリケーション開始
}
```

**3. 遅延初期化パターン（関数呼び出し必要）**
```rust
// src/user.rs
use std::sync::OnceLock;

static INIT: OnceLock<()> = OnceLock::new();

fn ensure_translations() {
    INIT.get_or_init(|| {
        let mut ja = LexiconMap::new();
        ja.insert("User not found".to_string(), "ユーザーが見つかりません".to_string());
        register("ja", ja);
    });
}

pub fn validate_user(email: &str) -> Result<(), String> {
    ensure_translations();  // 各関数で呼び出し必要
    // t("User not found") を使用
}
```

#### ⚠️ 制限事項

**マクロのみの方法では困難**
- マクロは呼び出し時に実行されるため、自動登録には不向き
- 各ファイルで手動実行が必要

#### 💡 ベストプラクティス

**推奨構成（`ctor`使用）:**
```rust
// src/features/auth.rs
#[ctor]
fn init() {
    register_translations!("ja", {
        "Login failed" => "ログインに失敗しました",
        "Password required" => "パスワードが必要です",
    });
}

// src/features/profile.rs  
#[ctor]
fn init() {
    register_translations!("ja", {
        "Profile updated" => "プロフィールを更新しました",
        "Invalid username" => "無効なユーザー名です",
    });
}
```

**メリット:**
- 各モジュールで関連する翻訳を管理
- 自動的に登録（main関数での初期化不要）
- Goの`init()`と同じ使用感
- コードと翻訳が近い場所に配置

**デメリット:**
- 外部クレート（`ctor`）への依存
- コンパイル時間がわずかに増加

### Rustらしい実装
- 所有権とライフタイムを適切に管理
- `Result`型でエラーハンドリング
- `Arc`と`Mutex`/`RwLock`で安全な並行アクセス
- マクロによる便利な翻訳登録

### パフォーマンス最適化
- 遅延初期化
- 読み取り専用アクセスには`RwLock`を使用
- 文字列の不要なコピーを避ける

### 互換性
- Go版と同等の機能を提供
- 同じ環境変数をサポート
- 同様のAPIインターフェース