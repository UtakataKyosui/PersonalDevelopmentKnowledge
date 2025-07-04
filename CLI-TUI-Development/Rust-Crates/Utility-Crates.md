# Rust CLI Development - ユーティリティクレート

## 🎨 出力・スタイリング

### indicatif - プログレスバー

**基本情報**
- **人気度**: ⭐⭐⭐⭐⭐ (最も人気)
- **特徴**: 美しく機能的なプログレスバー
- **公式**: https://github.com/console-rs/indicatif

```toml
[dependencies]
indicatif = "0.17"
```

**基本的なプログレスバー**
```rust
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

fn main() {
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    for i in 0..100 {
        pb.set_position(i + 1);
        thread::sleep(Duration::from_millis(50));
    }
    pb.finish_with_message("完了!");
}
```

**複数のプログレスバー**
```rust
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

fn main() {
    let m = MultiProgress::new();
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap();

    let pb1 = m.add(ProgressBar::new(128));
    pb1.set_style(style.clone());
    pb1.set_message("ファイル1");

    let pb2 = m.add(ProgressBar::new(256));
    pb2.set_style(style.clone());
    pb2.set_message("ファイル2");

    let h1 = thread::spawn(move || {
        for i in 0..128 {
            pb1.set_position(i + 1);
            thread::sleep(Duration::from_millis(15));
        }
        pb1.finish_with_message("ファイル1完了");
    });

    let h2 = thread::spawn(move || {
        for i in 0..256 {
            pb2.set_position(i + 1);
            thread::sleep(Duration::from_millis(8));
        }
        pb2.finish_with_message("ファイル2完了");
    });

    h1.join().unwrap();
    h2.join().unwrap();
}
```

**長所**
- 豊富なスタイリングオプション
- マルチプログレスバー対応
- カスタマイズ可能
- 高パフォーマンス

**短所**
- 複雑な設定が必要な場合がある
- ターミナル依存

---

### console - ターミナル制御・色付け

**基本情報**
- **人気度**: ⭐⭐⭐⭐
- **特徴**: crossterm の高レベルAPI、色・スタイル制御
- **公式**: https://github.com/console-rs/console

```toml
[dependencies]
console = "0.15"
```

**基本的な色付け**
```rust
use console::{style, Term};

fn main() {
    let term = Term::stdout();
    
    // 基本的な色
    println!("{}", style("エラー").red());
    println!("{}", style("成功").green());
    println!("{}", style("警告").yellow());
    println!("{}", style("情報").blue());
    
    // スタイル組み合わせ
    println!("{}", style("太字").bold());
    println!("{}", style("下線").underlined());
    println!("{}", style("点滅").blink());
    
    // 背景色
    println!("{}", style("ハイライト").black().on_yellow());
    
    // 複合スタイル
    println!("{}", 
        style("重要なメッセージ")
            .red()
            .bold()
            .underlined()
    );
}
```

**長所**
- 使いやすい高レベルAPI
- クロスプラットフォーム対応
- 豊富なスタイリング機能
- ターミナル検出機能

**短所**
- indicatifとの重複機能
- 一部機能がcrossterm劣る

---

### colored - シンプル色付け

**基本情報**
- **人気度**: ⭐⭐⭐⭐
- **特徴**: シンプルで軽量な色付けライブラリ
- **公式**: https://github.com/colored-rs/colored

```toml
[dependencies]
colored = "2.0"
```

**基本使用例**
```rust
use colored::*;

fn main() {
    // 基本的な色
    println!("{}", "赤色テキスト".red());
    println!("{}", "緑色テキスト".green());
    println!("{}", "青色テキスト".blue());
    
    // スタイル
    println!("{}", "太字".bold());
    println!("{}", "下線".underline());
    println!("{}", "イタリック".italic());
    
    // 背景色
    println!("{}", "黄色背景".on_yellow());
    
    // 組み合わせ
    println!("{}", "赤色太字".red().bold());
    println!("{}", "青色下線".blue().underline());
    
    // RGB指定
    println!("{}", "カスタム色".truecolor(255, 100, 50));
}
```

**長所**
- 軽量で高速
- シンプルなAPI
- NO_COLOR環境変数対応
- トレイト実装が簡単

**短所**
- 機能が基本的
- ターミナル制御機能なし

---

### comfy-table - テーブル表示

**基本情報**
- **人気度**: ⭐⭐⭐
- **特徴**: 美しいテーブル作成、自動幅調整
- **公式**: https://github.com/Nukesor/comfy-table

```toml
[dependencies]
comfy-table = "7.1"
```

**基本的なテーブル**
```rust
use comfy_table::Table;

fn main() {
    let mut table = Table::new();
    table
        .set_header(vec!["名前", "年齢", "職業"])
        .add_row(vec!["Alice", "25", "エンジニア"])
        .add_row(vec!["Bob", "30", "デザイナー"])
        .add_row(vec!["Charlie", "35", "マネージャー"]);

    println!("{}", table);
}
```

**高度なスタイリング**
```rust
use comfy_table::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;

fn main() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Product", "Price", "Stock", "Description"])
        .add_row(vec![
            "Laptop",
            "$999", 
            "15",
            "High-performance laptop with SSD"
        ]);

    println!("{}", table);
}
```

**長所**
- 美しいテーブル表示
- 自動幅調整
- 豊富なスタイリングオプション
- UTF-8対応

**短所**
- 大きなデータセットには不向き
- インタラクティブ機能なし

---

## 📝 ログ・トレーシング

### tracing - 構造化ログ

**基本情報**
- **人気度**: ⭐⭐⭐⭐⭐ (モダンスタンダード)
- **特徴**: 構造化ログ、スパンベースの非同期対応
- **公式**: https://github.com/tokio-rs/tracing

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

**基本的な使用例**
```rust
use tracing::{info, warn, error, debug, trace};
use tracing_subscriber;

fn main() {
    // サブスクライバー初期化
    tracing_subscriber::fmt::init();
    
    // 基本的なログ
    trace!("非常に詳細な情報");
    debug!("デバッグ情報");
    info!("一般的な情報");
    warn!("警告メッセージ");
    error!("エラーメッセージ");
    
    // 構造化データ
    info!(user_id = 42, action = "login", "ユーザーがログインしました");
    
    // フィールド付きログ
    let user_name = "Alice";
    info!(%user_name, "処理開始");
}
```

**スパンの使用**
```rust
use tracing::{info, span, Level};
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();
    
    // スパン作成
    let span = span!(Level::INFO, "処理", user = "Alice");
    let _enter = span.enter();
    
    info!("処理中...");
    process_data();
    info!("処理完了");
    
    // スパンから抜ける（_enterがdropされる）
}

#[tracing::instrument]
fn process_data() {
    info!("データ処理中");
    // 自動的にスパンが作成される
}
```

**高度な設定**
```rust
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    
    // RUST_LOG環境変数でフィルタリング可能
    // 例: RUST_LOG=debug cargo run
}
```

**長所**
- 構造化ログ
- 非同期対応
- 強力なフィルタリング
- 豊富なエコシステム

**短所**
- 学習コストが高い
- 設定が複雑

---

### env_logger - シンプルログ

**基本情報**
- **人気度**: ⭐⭐⭐⭐
- **特徴**: 環境変数制御、シンプル設定
- **公式**: https://github.com/rust-cli/env_logger

```toml
[dependencies]
log = "0.4"
env_logger = "0.11"
```

**基本使用例**
```rust
use log::{info, warn, error, debug, trace};

fn main() {
    env_logger::init();
    
    trace!("トレースメッセージ");
    debug!("デバッグメッセージ");
    info!("情報メッセージ");
    warn!("警告メッセージ");
    error!("エラーメッセージ");
    
    // RUST_LOG=debug cargo run で実行
}
```

**カスタム設定**
```rust
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;

fn main() {
    Builder::from_default_env()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            writeln!(buf, "[{}] {}: {}",
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();
        
    log::info!("カスタムフォーマットのログ");
}
```

**長所**
- 軽量で高速
- 環境変数による簡単制御
- logクレートとの互換性
- 設定が簡単

**短所**
- 構造化ログ非対応
- 機能が基本的

---

## ⚙️ 設定・シリアライゼーション

### serde - シリアライゼーション

**基本情報**
- **人気度**: ⭐⭐⭐⭐⭐ (事実上の標準)
- **特徴**: 高性能シリアライゼーション
- **公式**: https://github.com/serde-rs/serde

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
```

**基本的な使用例**
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    port: u16,
    debug: bool,
    features: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        name: "MyApp".to_string(),
        port: 8080,
        debug: true,
        features: vec!["auth".to_string(), "logging".to_string()],
    };
    
    // JSON
    let json = serde_json::to_string_pretty(&config)?;
    println!("JSON:\n{}", json);
    
    // TOML
    let toml_str = toml::to_string(&config)?;
    println!("TOML:\n{}", toml_str);
    
    // 読み込み
    let loaded: Config = serde_json::from_str(&json)?;
    println!("Loaded: {} on port {}", loaded.name, loaded.port);
    
    Ok(())
}
```

**高度な使用例**
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct AppConfig {
    #[serde(default = "default_host")]
    host: String,
    
    #[serde(default = "default_port")]
    port: u16,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    database_url: Option<String>,
    
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

fn default_host() -> String {
    "localhost".to_string()
}

fn default_port() -> u16 {
    3000
}
```

---

### config - 設定管理

**基本情報**
- **特徴**: 複数ソースからの設定統合
- **対応形式**: JSON, TOML, YAML, 環境変数

```toml
[dependencies]
config = "0.14"
serde = { version = "1.0", features = ["derive"] }
```

**基本使用例**
```rust
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize)]
struct Settings {
    debug: bool,
    database: Database,
    server: Server,
}

#[derive(Deserialize)]
struct Database {
    url: String,
}

#[derive(Deserialize)]
struct Server {
    host: String,
    port: u16,
}

fn main() -> Result<(), ConfigError> {
    let settings = Config::builder()
        // デフォルト設定
        .set_default("debug", false)?
        .set_default("server.host", "localhost")?
        .set_default("server.port", 3000)?
        // 設定ファイル
        .add_source(File::with_name("config/default").required(false))
        .add_source(File::with_name("config/production").required(false))
        // 環境変数（APP_で始まる）
        .add_source(Environment::with_prefix("APP"))
        .build()?;
    
    let settings: Settings = settings.try_deserialize()?;
    
    println!("Debug: {}", settings.debug);
    println!("Server: {}:{}", settings.server.host, settings.server.port);
    println!("Database: {}", settings.database.url);
    
    Ok(())
}
```

---

## 🔧 ファイル・システム操作

### tempfile - 一時ファイル

**基本情報**
- **特徴**: 安全な一時ファイル・ディレクトリ作成
- **公式**: https://github.com/Stebalien/tempfile

```toml
[dependencies]
tempfile = "3.8"
```

**基本使用例**
```rust
use tempfile::{NamedTempFile, tempdir};
use std::io::Write;

fn main() -> std::io::Result<()> {
    // 一時ファイル
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "Hello, temp file!")?;
    
    println!("Temp file: {:?}", temp_file.path());
    
    // 一時ディレクトリ
    let temp_dir = tempdir()?;
    let file_path = temp_dir.path().join("test.txt");
    std::fs::write(&file_path, "Hello, temp dir!")?;
    
    println!("Temp dir: {:?}", temp_dir.path());
    
    // スコープを抜けると自動削除
    Ok(())
}
```

---

### dirs - ディレクトリ取得

**基本情報**
- **特徴**: プラットフォーム固有のディレクトリパス取得
- **公式**: https://github.com/dirs-dev/dirs-rs

```toml
[dependencies]
dirs = "5.0"
```

**使用例**
```rust
use dirs;

fn main() {
    if let Some(home) = dirs::home_dir() {
        println!("Home directory: {:?}", home);
    }
    
    if let Some(config) = dirs::config_dir() {
        println!("Config directory: {:?}", config);
    }
    
    if let Some(data) = dirs::data_dir() {
        println!("Data directory: {:?}", data);
    }
    
    if let Some(cache) = dirs::cache_dir() {
        println!("Cache directory: {:?}", cache);
    }
}
```

---

## 📊 ユーティリティクレート比較表

| カテゴリ | クレート | 人気度 | 特徴 | 学習コスト |
|----------|----------|--------|------|----------|
| **プログレスバー** | indicatif | ⭐⭐⭐⭐⭐ | 高機能 | 低 |
| **色付け** | colored | ⭐⭐⭐⭐ | シンプル | 最低 |
| **色付け** | console | ⭐⭐⭐⭐ | 高機能 | 低 |
| **テーブル** | comfy-table | ⭐⭐⭐ | 美しい | 低 |
| **ログ** | tracing | ⭐⭐⭐⭐⭐ | 構造化 | 高 |
| **ログ** | env_logger | ⭐⭐⭐⭐ | シンプル | 最低 |
| **設定** | serde | ⭐⭐⭐⭐⭐ | 標準 | 中 |
| **設定** | config | ⭐⭐⭐ | 統合型 | 中 |

## 🎯 使い分けガイド

### プログレスバー
- **indicatif**: 美しいプログレスバーが必要
- **dialoguer**: シンプルなプログレス表示

### 色付け・スタイリング
- **colored**: 軽量でシンプルな色付け
- **console**: ターミナル制御も含む高機能
- **crossterm**: 低レベル制御が必要

### ログ
- **tracing**: モダンな構造化ログ、非同期対応
- **env_logger**: シンプルで軽量
- **log**: ライブラリでの使用

### 設定管理
- **serde**: 基本的なシリアライゼーション
- **config**: 複数ソースからの設定統合
- **clap**: CLIオプションとの統合

### ファイル操作
- **tempfile**: 一時ファイル・ディレクトリ
- **dirs**: プラットフォーム固有パス取得

---

**関連ドキュメント**:
- [CLI-Crates.md](./CLI-Crates.md) - CLI開発用クレート
- [TUI-Crates.md](./TUI-Crates.md) - TUI開発用クレート
- [Best-Practices.md](./Best-Practices.md) - ベストプラクティス
