# Repository Guidelines

## プロジェクト構成とモジュール配置

このリポジトリは `rust_bmp` という小さな Rust バイナリクレートです。

- `Cargo.toml` はパッケージ情報と依存関係を定義します。
- `Cargo.lock` は再現可能なビルドのためにコミットします。
- `src/main.rs` は実行ファイルのエントリーポイントです。
- `context/` には開発時のプロンプトや参考資料を置きます。
- `target/` は Cargo が生成する成果物で、Git 管理対象外です。

機能が増えたら、再利用できる処理は `src/` 配下の別モジュールに分け、`main.rs` は引数処理、全体の呼び出し、終了処理に集中させてください。外部から実行結果を検証するテストは `tests/` に置きます。

## ビルド・テスト・開発コマンド

- `cargo build`: デバッグビルドを作成します。
- `cargo run`: ビルドしてローカルで実行します。
- `cargo test`: 単体テストと統合テストを実行します。
- `cargo fmt`: `rustfmt` で Rust コードを整形します。
- `cargo clippy --all-targets --all-features`: クレート全体の lint を実行します。

変更を提出する前に `cargo fmt` と `cargo test` を実行してください。実装変更がある場合は `cargo clippy --all-targets --all-features` も実行します。

## コーディングスタイルと命名規則

Rust 2024 edition の標準的な慣習に従います。インデントは 4 スペース、関数とモジュールは `snake_case`、型とトレイトは `PascalCase`、定数は `SCREAMING_SNAKE_CASE` を使います。

本番経路では安易な `panic!` を避け、明示的なエラー処理を優先してください。`use` は必要最小限にし、`target/` の生成物はコミットしません。

## テスト方針

現在の初期構成にはテストがありません。単体テストは対象コードの近くに `#[cfg(test)] mod tests` として追加し、CLI やエンドツーエンドの挙動は `tests/` 配下の統合テストで検証します。

テスト名は検証する挙動が分かる名前にします。例: `writes_valid_bmp_header`、`rejects_invalid_dimensions`。BMP 関連の処理では、小さく決定的な fixture とバイト単位の検証を優先してください。

## コミットとプルリクエスト

現在の履歴は `initial commit` のみで、厳密なプロジェクト固有ルールはまだありません。コミットメッセージは `Add BMP header writer` や `Validate image dimensions` のように、短い命令形の件名にしてください。

プルリクエストには、変更概要、実行した検証コマンド、関連 issue があればそのリンクを含めます。BMP の出力形式に影響する変更では、サンプル出力や生成ファイルに関するメモも記載してください。

## エージェント向け指示

変更範囲は狭く保ち、無関係なリファクタリングは避けてください。コードを編集するときは既存の Cargo 慣習を維持し、正確性や保守性が明確に向上する場合を除いて標準ライブラリを優先します。
