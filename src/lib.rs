use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

pub type LexiconMap = HashMap<String, String>;
pub type WorldMap = HashMap<String, LexiconMap>;

// 環境変数取得を抽象化するトレイト（テスト可能にするため）
pub trait EnvProvider: Send + Sync {
    fn var(&self, key: &str) -> Result<String, std::env::VarError>;
}

// 実際の環境変数を読むプロバイダ
pub struct SystemEnvProvider;

impl EnvProvider for SystemEnvProvider {
    fn var(&self, key: &str) -> Result<String, std::env::VarError> {
        std::env::var(key)
    }
}

// グローバルな翻訳ストレージ
struct L10n {
    world: RwLock<WorldMap>,
    forced_language: RwLock<Option<String>>,
    env_provider: Box<dyn EnvProvider>,
}

impl L10n {
    fn new(env_provider: Box<dyn EnvProvider>) -> Self {
        L10n {
            world: RwLock::new(HashMap::new()),
            forced_language: RwLock::new(None),
            env_provider,
        }
    }

    fn register(&self, lang: &str, lexicon: LexiconMap) {
        let mut world = self.world.write().unwrap();
        world
            .entry(lang.to_string())
            .or_insert_with(HashMap::new)
            .extend(lexicon);
    }

    fn detect_language(&self) -> String {
        // 強制言語設定があればそれを使用
        if let Some(ref lang) = *self.forced_language.read().unwrap() {
            return lang.clone();
        }

        // 環境変数から言語を検出
        let env_vars = [
            "L10N_TEST_MODE",
            "LANGUAGE",
            "LC_ALL",
            "LC_MESSAGES",
            "LANG",
        ];

        for var in &env_vars {
            if let Ok(value) = self.env_provider.var(var) {
                if !value.is_empty() {
                    // 言語コードを抽出 (例: "ja_JP.UTF-8" -> "ja")
                    let lang = value.split('_').next().unwrap_or(&value);
                    let lang = lang.split('.').next().unwrap_or(lang);
                    return lang.to_string();
                }
            }
        }

        // デフォルト言語
        self.env_provider
            .var("L10N_DEFAULT_LANGUAGE")
            .unwrap_or_else(|_| "en".to_string())
    }

    fn translate(&self, phrase: &str) -> String {
        let lang = self.detect_language();
        let world = self.world.read().unwrap();

        if let Some(lexicon) = world.get(&lang) {
            if let Some(translation) = lexicon.get(phrase) {
                return translation.clone();
            }
        }

        phrase.to_string()
    }

    fn format(&self, phrase: &str, args: &[&str]) -> String {
        let mut result = self.translate(phrase);

        // シンプルな置換実装 ({}を順番に置き換える)
        for arg in args {
            if let Some(pos) = result.find("{}") {
                result.replace_range(pos..pos + 2, arg);
            }
        }

        result
    }

    fn force_language(&self, lang: &str) {
        *self.forced_language.write().unwrap() = Some(lang.to_string());
    }

    fn reset_language(&self) {
        *self.forced_language.write().unwrap() = None;
    }
}

// グローバルインスタンス
static GLOBAL_L10N: Lazy<L10n> = Lazy::new(|| L10n::new(Box::new(SystemEnvProvider)));

// パブリックAPI
pub fn register(lang: &str, lexicon: LexiconMap) {
    GLOBAL_L10N.register(lang, lexicon);
}

pub fn t(phrase: &str) -> String {
    GLOBAL_L10N.translate(phrase)
}

pub fn f(phrase: &str, args: &[&str]) -> String {
    GLOBAL_L10N.format(phrase, args)
}

pub fn e(phrase: &str, args: &[&str]) -> String {
    GLOBAL_L10N.format(phrase, args)
}

pub fn force_language(lang: &str) {
    GLOBAL_L10N.force_language(lang);
}

pub fn reset_language() {
    GLOBAL_L10N.reset_language();
}

pub fn detect_language() -> String {
    GLOBAL_L10N.detect_language()
}

// マクロ定義
#[macro_export]
macro_rules! t {
    ($phrase:expr) => {
        $crate::t($phrase)
    };
}

#[macro_export]
macro_rules! f {
    ($phrase:expr, $($arg:expr),* $(,)?) => {
        $crate::f($phrase, &[$($arg),*])
    };
}

#[macro_export]
macro_rules! e {
    ($phrase:expr) => {
        $crate::e($phrase, &[])
    };
    ($phrase:expr, $($arg:expr),* $(,)?) => {
        $crate::e($phrase, &[$($arg),*])
    };
}

// 翻訳登録用マクロ
#[macro_export]
macro_rules! register_translations {
    (
        $(
            $lang:ident: {
                $($key:expr => $value:expr),* $(,)?
            }
        ),* $(,)?
    ) => {
        $(
            {
                let mut lexicon = $crate::LexiconMap::new();
                $(
                    lexicon.insert($key.to_string(), $value.to_string());
                )*
                $crate::register(stringify!($lang), lexicon);
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    // テスト用の環境変数プロバイダ
    struct MockEnvProvider {
        vars: Mutex<HashMap<String, String>>,
    }

    impl MockEnvProvider {
        fn new() -> Self {
            MockEnvProvider {
                vars: Mutex::new(HashMap::new()),
            }
        }

        fn set(&self, key: &str, value: &str) {
            self.vars
                .lock()
                .unwrap()
                .insert(key.to_string(), value.to_string());
        }
    }

    impl EnvProvider for MockEnvProvider {
        fn var(&self, key: &str) -> Result<String, std::env::VarError> {
            self.vars
                .lock()
                .unwrap()
                .get(key)
                .cloned()
                .ok_or(std::env::VarError::NotPresent)
        }
    }

    #[test]
    fn test_basic_translation() {
        let env_provider = Box::new(MockEnvProvider::new());
        let l10n = L10n::new(env_provider);

        let mut ja_lexicon = LexiconMap::new();
        ja_lexicon.insert("Hello".to_string(), "こんにちは".to_string());
        l10n.register("ja", ja_lexicon);

        l10n.force_language("ja");
        assert_eq!(l10n.translate("Hello"), "こんにちは");

        l10n.force_language("en");
        assert_eq!(l10n.translate("Hello"), "Hello");
    }

    #[test]
    fn test_environment_detection() {
        let mock_env = Box::new(MockEnvProvider::new());
        mock_env.set("LANGUAGE", "ja_JP.UTF-8");

        let l10n = L10n::new(mock_env);
        assert_eq!(l10n.detect_language(), "ja");
    }

    #[test]
    fn test_format_translation() {
        let env_provider = Box::new(MockEnvProvider::new());
        let l10n = L10n::new(env_provider);

        let mut ja_lexicon = LexiconMap::new();
        ja_lexicon.insert("Hello, {}!".to_string(), "こんにちは、{}さん！".to_string());
        l10n.register("ja", ja_lexicon);

        l10n.force_language("ja");
        assert_eq!(
            l10n.format("Hello, {}!", &["Alice"]),
            "こんにちは、Aliceさん！"
        );
    }

    #[test]
    fn test_force_and_reset_language() {
        let mock_env = Box::new(MockEnvProvider::new());
        mock_env.set("LANGUAGE", "ja");

        let l10n = L10n::new(mock_env);

        assert_eq!(l10n.detect_language(), "ja");

        l10n.force_language("en");
        assert_eq!(l10n.detect_language(), "en");

        l10n.reset_language();
        assert_eq!(l10n.detect_language(), "ja");
    }

    #[test]
    fn test_register_translations_macro() {
        register_translations! {
            ja: {
                "Yes" => "はい",
                "No" => "いいえ",
            },
            es: {
                "Yes" => "Sí",
                "No" => "No",
            }
        }

        force_language("ja");
        assert_eq!(t("Yes"), "はい");
        assert_eq!(t("No"), "いいえ");

        force_language("es");
        assert_eq!(t("Yes"), "Sí");
    }

    #[test]
    fn test_format_macro() {
        let mut ja_lexicon = LexiconMap::new();
        ja_lexicon.insert("Welcome, {}!".to_string(), "ようこそ、{}さん！".to_string());
        register("ja", ja_lexicon);

        force_language("ja");
        assert_eq!(f!("Welcome, {}!", "Bob"), "ようこそ、Bobさん！");
    }
}
