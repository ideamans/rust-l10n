use rust_l10n::{force_language, register_translations, reset_language, t};

// 注意: 現在の実装では環境変数は起動時に読み込まれるため、
// 実行時の環境変数変更はテストできません。
// 代わりに force_language を使った動作確認を行います。

#[test]
fn test_force_language_works() {
    // 翻訳を登録
    register_translations! {
        ja: {
            "Test message" => "テストメッセージ",
            "Error occurred" => "エラーが発生しました",
        },
        es: {
            "Test message" => "Mensaje de prueba",
            "Error occurred" => "Ocurrió un error",
        }
    }

    // force_language で言語を切り替え
    force_language("ja");
    assert_eq!(t("Test message"), "テストメッセージ");
    assert_eq!(t("Error occurred"), "エラーが発生しました");

    force_language("es");
    assert_eq!(t("Test message"), "Mensaje de prueba");
    assert_eq!(t("Error occurred"), "Ocurrió un error");

    force_language("en");
    assert_eq!(t("Test message"), "Test message");
    assert_eq!(t("Error occurred"), "Error occurred");

    // reset_language で自動検出に戻す
    reset_language();
    // 環境変数に依存するため、具体的な言語はテストしない
}
