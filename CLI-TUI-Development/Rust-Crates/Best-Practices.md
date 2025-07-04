# Rust CLI/TUI Development - ベストプラクティス

## 🏗️ プロジェクト構造

### 推奨ディレクトリ構造

```
my-cli-tool/
├── Cargo.toml
├── README.md
├── LICENSE
├── .gitignore
├── src/
│   ├── main.rs              # エントリーポイント
│   ├── lib.rs               # ライブラリ機能
│   ├── cli/
│   │   ├── mod.rs           # CLIモジュール
│   │   ├── args.rs          # 引数定義
│   │   └── commands.rs      # コマンド実装
│   ├── config/
│   │   ├── mod.rs           # 設定モジュール
│   │   └── settings.rs      # 設定構造体
│   ├── error.rs             # エラー定義
│   └── utils.rs             # ユーティリティ
├── tests/
│   ├── integration/
│   └── fixtures/
├── examples/
├── docs/
└── config/
    ├── default.toml
    └── example.toml
```

### Cargo.tomlの設定

```toml
[package]
name = "my-cli-tool"
version = "0.1.0"
edition = "2021"
description = "A CLI tool built with Rust"
license = "MIT OR Apache-2.0"
repository = "https://github.com/username/my-cli-tool"
keywords = ["cli", "tool"]
categories = ["command-line-utilities"]

[[bin]]
name = "my-cli-tool"
path = "src/main.rs"

[lib]
name = "my_cli_tool"
path = "src/lib.rs"

[dependencies]
# CLI
clap = { version = "4.4", features = ["derive"] }
# ログ
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
# エラーハンドリング
anyhow = "1.0"
thiserror = "1.0"
# 設定
serde = { version = "1.0", features = ["derive"] }
config = "0.14"
# ユーティリティ
indicatif = "0.17"
console = "0.15"

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.8"

[profile.release]
lto = true
codegen-units = 1
strip = true
```

---

## 🔧 エラーハンドリング

### カスタムエラー型の定義

```rust
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("設定ファイルが見つかりません: {path}")]
    ConfigNotFound { path: String },

    #[error("無効な入力: {message}")]
    InvalidInput { message: String },

    #[error("ファイル操作エラー")]
    FileError(#[from] std::io::Error),

    #[error("JSON解析エラー")]
    JsonError(#[from] serde_json::Error),

    #[error("ネットワークエラー")]
    NetworkError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, CliError>;
```

### エラーハンドリングパターン

```rust
// src/main.rs
use anyhow::{Context, Result};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        error!("アプリケーションエラー: {:?}", err);
        
        // エラーチェーンを表示
        let mut source = err.source();
        while let Some(err) = source {
            error!("原因: {}", err);
            source = err.source();
        }
        
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    // ログ初期化
    init_logging()?;
    
    // 設定読み込み
    let config = load_config()
        .context("設定ファイルの読み込みに失敗しました")?;
    
    // メイン処理
    process_data(&config)
        .await
        .context("データ処理に失敗しました")?;
    
    info!("処理が正常に完了しました");
    Ok(())
}
```

---

## 📝 ロギング・トレーシング

### 推奨ログ設定

```rust
// src/main.rs
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
    Registry,
};

fn init_logging() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    Registry::default()
        .with(fmt::layer()
            .with_target(false)
            .with_timer(fmt::time::UtcTime::rfc_3339())
            .with_level(true))
        .with(env_filter)
        .init();
    
    Ok(())
}

// instrumentマクロの活用
#[tracing::instrument(skip(data))]
async fn process_file(path: &str, data: &[u8]) -> Result<()> {
    tracing::info!(file_size = data.len(), "ファイル処理開始");
    
    // 処理...
    
    tracing::info!("ファイル処理完了");
    Ok(())
}
```

---

## ⚙️ 設定管理

### 階層化設定システム

```rust
// src/config/settings.rs
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub log: LogConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    
    #[serde(default)]
    pub file: Option<String>,
}

fn default_log_level() -> String {
    "info".to_string()
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let config = Config::builder()
            // デフォルト値
            .set_default("log.level", "info")?
            .set_default("server.host", "localhost")?
            .set_default("server.port", 3000)?
            
            // 設定ファイル（優先度順）
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name("config/local").required(false))
            
            // 環境変数
            .add_source(Environment::with_prefix("APP").separator("_"))
            
            .build()?;
        
        config.try_deserialize()
    }
}
```

---

## 🧪 テスト戦略

### 統合テスト

```rust
// tests/integration/cli_tests.rs
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("my-cli-tool").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A CLI tool built with Rust"));
}

#[test]
fn test_config_file_processing() {
    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("test.toml");
    
    fs::write(&config_file, r#"
        [server]
        host = "0.0.0.0"
        port = 8080
    "#).unwrap();
    
    let mut cmd = Command::cargo_bin("my-cli-tool").unwrap();
    cmd.arg("--config")
        .arg(config_file)
        .arg("config")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.0.0.0:8080"));
}
```

---

## 🚀 パフォーマンス最適化

### コンパイル時最適化

```toml
# Cargo.toml
[profile.release]
lto = true                    # Link Time Optimization
codegen-units = 1            # 単一コード生成単位
strip = true                 # デバッグシンボル削除
panic = "abort"              # パニック時にアボート

[profile.release-small]
inherits = "release"
opt-level = "z"              # サイズ優先最適化
lto = true
codegen-units = 1
strip = true
```

### 依存関係の最適化

```toml
# 機能フラグを活用
[dependencies]
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
serde = { version = "1.0", features = ["derive"], default-features = false }
clap = { version = "4.4", features = ["derive"], default-features = false }

# 不要なデフォルト機能を無効化
reqwest = { version = "0.11", default-features = false, features = ["json", "rustls-tls"] }
```

---

## 🎨 ユーザビリティ

### プログレス表示

```rust
use indicatif::{ProgressBar, ProgressStyle};

pub struct ProgressReporter {
    pb: ProgressBar,
}

impl ProgressReporter {
    pub fn new(total: u64, message: &str) -> Self {
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb.set_message(message.to_string());
        
        Self { pb }
    }
    
    pub fn inc(&self, delta: u64) {
        self.pb.inc(delta);
    }
    
    pub fn finish_with_message(&self, message: &str) {
        self.pb.finish_with_message(message.to_string());
    }
}
```

### ユーザーフレンドリーなエラーメッセージ

```rust
use console::style;

pub fn display_error(err: &CliError) {
    eprintln!("{} {}", 
        style("エラー:").red().bold(),
        err
    );
    
    // 解決策の提案
    match err {
        CliError::ConfigNotFound { path } => {
            eprintln!("{} 以下のコマンドで設定ファイルを作成できます:", 
                style("ヒント:").yellow().bold());
            eprintln!("  {} --init {}", 
                style("my-cli-tool").cyan(),
                path
            );
        }
        CliError::InvalidInput { .. } => {
            eprintln!("{} --help でヘルプを表示", 
                style("ヒント:").yellow().bold());
        }
        _ => {}
    }
}
```

### 対話的な設定

```rust
use inquire::{Text, Confirm, Select};

pub fn interactive_setup() -> Result<AppConfig> {
    println!("{}", style("初期設定を開始します").blue().bold());
    
    let host = Text::new("サーバーホスト:")
        .with_default("localhost")
        .prompt()?;
    
    let port_str = Text::new("ポート番号:")
        .with_default("3000")
        .with_validator(|input: &str| {
            match input.parse::<u16>() {
                Ok(_) => Ok(inquire::validator::Validation::Valid),
                Err(_) => Ok(inquire::validator::Validation::Invalid(
                    "有効なポート番号を入力してください".into()
                )),
            }
        })
        .prompt()?;
    
    let log_level = Select::new(
        "ログレベル:",
        vec!["error", "warn", "info", "debug", "trace"]
    )
    .with_default(2) // "info"
    .prompt()?;
    
    let save_config = Confirm::new("設定をファイルに保存しますか?")
        .with_default(true)
        .prompt()?;
    
    let config = AppConfig {
        server: ServerConfig {
            host,
            port: port_str.parse().unwrap(),
        },
        log: LogConfig {
            level: log_level.to_string(),
            file: None,
        },
        database: DatabaseConfig::default(),
    };
    
    if save_config {
        save_config_file(&config)?;
        println!("{}", style("設定ファイルを保存しました").green());
    }
    
    Ok(config)
}
```

---

## 📦 配布・デプロイ

### GitHub Actions での CI/CD

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta]
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@master
      with:
        toolchain: ${{ matrix.rust }}
    
    - name: Cache Cargo
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Run tests
      run: cargo test --verbose
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
    
    - name: Check formatting
      run: cargo fmt -- --check

  release:
    name: Release
    runs-on: ubuntu-latest
    if: startsWith(github.ref, 'refs/tags/')
    needs: test
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Build release
      run: cargo build --release
    
    - name: Create Release
      uses: softprops/action-gh-release@v1
      with:
        files: target/release/my-cli-tool*
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### クロスコンパイル設定

```toml
# .cargo/config.toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"

[target.x86_64-apple-darwin]
linker = "x86_64-apple-darwin-clang"

[target.aarch64-apple-darwin]
linker = "aarch64-apple-darwin-clang"
```

---

## 🔒 セキュリティ

### セキュアな設定管理

```rust
// パスワードや秘密鍵は環境変数から読み取り
use std::env;

fn load_sensitive_config() -> Result<String> {
    env::var("DATABASE_PASSWORD")
        .map_err(|_| CliError::InvalidInput {
            message: "DATABASE_PASSWORD環境変数が設定されていません".to_string()
        })
}

// 設定ファイルにはプレースホルダーを使用
let config_template = r#"
[database]
host = "localhost"
port = 5432
password = "${DATABASE_PASSWORD}"
"#;
```

### 入力検証

```rust
use regex::Regex;

fn validate_email(email: &str) -> Result<()> {
    let email_regex = Regex::new(r"^[^@]+@[^@]+\.[^@]+$").unwrap();
    
    if email_regex.is_match(email) {
        Ok(())
    } else {
        Err(CliError::InvalidInput {
            message: "無効なメールアドレス形式です".to_string()
        })
    }
}

fn validate_path(path: &str) -> Result<()> {
    // パストラバーサル攻撃を防ぐ
    if path.contains("..") {
        return Err(CliError::InvalidInput {
            message: "相対パス指定は許可されていません".to_string()
        });
    }
    
    Ok(())
}
```

---

## 📋 チェックリスト

### 開発時
- [ ] エラーハンドリングが適切に実装されている
- [ ] ログが適切に出力されている
- [ ] 設定ファイルが正しく読み込まれる
- [ ] コマンドライン引数が正しく解析される
- [ ] ユニットテストが書かれている
- [ ] セキュリティ検証が実装されている

### リリース前
- [ ] 統合テストがすべて通る
- [ ] ドキュメントが最新
- [ ] パフォーマンステストを実施
- [ ] 異なるプラットフォームでテスト
- [ ] セキュリティチェック
- [ ] 依存関係の脆弱性チェック

### ユーザビリティ
- [ ] ヘルプメッセージが分かりやすい
- [ ] エラーメッセージに解決策が含まれる
- [ ] プログレス表示がある（長時間処理）
- [ ] 設定例やドキュメントが充実
- [ ] インストール手順が明確

### パフォーマンス
- [ ] 起動時間が適切（< 100ms）
- [ ] メモリ使用量が適切
- [ ] 大容量ファイル処理対応
- [ ] 並列処理の活用
- [ ] バイナリサイズの最適化

---

## 📚 追加リソース

### 推奨学習リソース
- [The Rust CLI Book](https://rust-cli.github.io/book/)
- [clap documentation](https://docs.rs/clap/)
- [tracing documentation](https://docs.rs/tracing/)
- [ratatui tutorial](https://ratatui.rs/tutorial/)

### コミュニティ
- [Rust CLI Working Group](https://github.com/rust-cli)
- [Rust Discord #cli-and-tui](https://discord.gg/rust-lang)
- [r/rust CLI discussions](https://reddit.com/r/rust)

### ツール
- [cargo-audit](https://crates.io/crates/cargo-audit) - セキュリティ監査
- [cargo-watch](https://crates.io/crates/cargo-watch) - ホットリロード
- [cargo-expand](https://crates.io/crates/cargo-expand) - マクロ展開
- [cargo-bloat](https://crates.io/crates/cargo-bloat) - バイナリサイズ解析

---

**関連ドキュメント**:
- [CLI-Crates.md](./CLI-Crates.md) - CLI開発用クレート
- [TUI-Crates.md](./TUI-Crates.md) - TUI開発用クレート  
- [Utility-Crates.md](./Utility-Crates.md) - ユーティリティクレート
