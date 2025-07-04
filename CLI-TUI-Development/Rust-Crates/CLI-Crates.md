# Rust CLI Development - 引数パーサー・プロンプトクレート

## 🎯 CLI 引数パーサー

### clap - フル機能引数パーサー

**基本情報**
- **人気度**: ⭐⭐⭐⭐⭐ (最も人気)
- **特徴**: 最も包括的で高機能な引数パーサー
- **MSRV**: Rust 1.54+
- **公式**: https://github.com/clap-rs/clap

**主な特徴**
```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }
```

#### Builder API vs Derive API

**Builder API**
```rust
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("myapp")
        .version("1.0")
        .author("Your Name")
        .about("Does awesome things")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Sets the input file to use"))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(clap::ArgAction::Count)
            .help("Sets the level of verbosity"))
        .get_matches();
        
    if let Some(input) = matches.get_one::<String>("input") {
        println!("Using input file: {}", input);
    }
    
    let verbose_level = matches.get_count("verbose");
    println!("Verbosity level: {}", verbose_level);
}
```

**Derive API (推奨)**
```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "myapp")]
#[command(version = "1.0")]
#[command(about = "A simple CLI tool", long_about = None)]
struct Cli {
    /// Input file to process
    #[arg(short, long, value_name = "FILE")]
    input: Option<String>,
    
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    
    /// Configuration file
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.input {
        Some(input) => println!("Processing file: {}", input),
        None => println!("No input file specified"),
    }
    
    println!("Verbosity level: {}", cli.verbose);
    println!("Config file: {}", cli.config);
}
```

#### サブコマンド

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "git-like")]
#[command(about = "A git-like CLI tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add files to the staging area
    Add {
        /// Files to add
        files: Vec<String>,
        /// Add all files
        #[arg(short, long)]
        all: bool,
    },
    /// Commit changes
    Commit {
        /// Commit message
        #[arg(short, long)]
        message: String,
    },
    /// Show status
    Status,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Add { files, all } => {
            if all {
                println!("Adding all files");
            } else {
                println!("Adding files: {:?}", files);
            }
        }
        Commands::Commit { message } => {
            println!("Committing with message: {}", message);
        }
        Commands::Status => {
            println!("Showing status");
        }
    }
}
```

#### 高度な機能

**カスタムバリデーション**
```rust
use clap::Parser;

fn validate_port(s: &str) -> Result<u16, String> {
    let port: u16 = s.parse()
        .map_err(|_| format!("'{}' is not a valid port number", s))?;
    
    if port < 1024 {
        Err("Port must be >= 1024".to_string())
    } else {
        Ok(port)
    }
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_parser = validate_port)]
    port: u16,
}
```

**値の列挙型**
```rust
use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_enum, default_value_t = LogLevel::Info)]
    log_level: LogLevel,
}
```

**長所**
- 包括的な機能セット（ヘルプ生成、補完、カラー出力）
- 優れたエラーメッセージ
- Unix conventions完全サポート
- 豊富なカスタマイゼーション

**短所**
- コンパイル時間が長い
- バイナリサイズが大きくなる
- 学習コストが高い

---

### argh - 軽量引数パーサー

**基本情報**
- **人気度**: ⭐⭐⭐
- **特徴**: Google製の軽量パーサー
- **ターゲット**: Fuchsia OS conventions
- **制限**: Unix conventionsを完全サポートしない

```toml
[dependencies]
argh = "0.1"
```

**基本使用例**
```rust
use argh::FromArgs;

#[derive(FromArgs)]
/// Reach new heights.
struct GoUp {
    /// whether or not to jump
    #[argh(switch, short = 'j')]
    jump: bool,

    /// how high to go
    #[argh(option)]
    height: usize,

    /// an optional nickname for the pilot
    #[argh(option)]
    pilot_nickname: Option<String>,
}

fn main() {
    let up: GoUp = argh::from_env();
    println!("Going up {} units", up.height);
    if up.jump {
        println!("Jumping!");
    }
}
```

**長所**
- 軽量でコンパイルが早い
- 最小限の依存関係
- 直感的なAPI

**短所**
- Unix conventions非対応
- 機能が限定的
- ヘルプ生成が基本的

---

### pico-args - ミニマム引数パーサー

**基本情報**
- **人気度**: ⭐⭐⭐
- **特徴**: ゼロ依存、最小バイナリサイズ
- **制限**: ヘルプ生成なし、derive APIなし

```toml
[dependencies]
pico-args = "0.5"
```

**基本使用例**
```rust
use pico_args::Arguments;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pargs = Arguments::from_env();
    
    // フラグ
    let help = pargs.contains("--help");
    let verbose = pargs.contains(["-v", "--verbose"]);
    
    // オプション引数
    let config: Option<String> = pargs.opt_value_from_str("--config")?;
    let count: u32 = pargs.value_from_str("--count").unwrap_or(1);
    
    // 位置引数
    let remaining = pargs.finish();
    
    if help {
        println!("Usage: myapp [OPTIONS] [FILES...]");
        return Ok(());
    }
    
    println!("Verbose: {}", verbose);
    println!("Config: {:?}", config);
    println!("Count: {}", count);
    println!("Files: {:?}", remaining);
    
    Ok(())
}
```

**長所**
- 最小限の依存関係（ゼロ）
- 非常に軽量
- 高速コンパイル

**短所**
- 手動でのヘルプ実装が必要
- エラーメッセージが基本的
- derive APIなし

---

## 🗣️ 対話プロンプト

### inquire - モダンプロンプトライブラリ

**基本情報**
- **人気度**: ⭐⭐⭐⭐
- **特徴**: 豊富なUI要素、美しいインターフェース
- **公式**: https://github.com/mikaelmello/inquire

```toml
[dependencies]
inquire = "0.7"
```

**基本的なプロンプト**
```rust
use inquire::{Text, Password, Confirm, Select, MultiSelect, CustomType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // テキスト入力
    let name = Text::new("What's your name?")
        .with_help_message("Enter your full name")
        .prompt()?;
    
    // パスワード入力
    let password = Password::new("Password:")
        .with_display_mode(inquire::PasswordDisplayMode::Masked)
        .prompt()?;
    
    // 確認
    let confirmed = Confirm::new("Do you want to continue?")
        .with_default(true)
        .prompt()?;
    
    // 単一選択
    let language = Select::new("Choose a programming language:", 
        vec!["Rust", "Go", "Python", "JavaScript"])
        .prompt()?;
    
    // 複数選択
    let frameworks = MultiSelect::new("Select frameworks:",
        vec!["Actix", "Warp", "Rocket", "Axum"])
        .prompt()?;
    
    // カスタム型
    let age: u32 = CustomType::new("What's your age?")
        .with_error_message("Please type a valid number")
        .prompt()?;
    
    println!("Name: {}", name);
    println!("Language: {}", language);
    println!("Frameworks: {:?}", frameworks);
    println!("Age: {}", age);
    
    Ok(())
}
```

**高度な機能**
```rust
use inquire::{Text, validator::Validation};

// バリデーター
let email = Text::new("Email:")
    .with_validator(|input: &str| {
        if input.contains('@') && input.contains('.') {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Please enter a valid email".into()))
        }
    })
    .prompt()?;

// オートコンプリート
let command = Text::new("Command:")
    .with_autocomplete(&|input: &str| {
        let commands = vec!["build", "test", "run", "clean"];
        Ok(commands.into_iter()
            .filter(|cmd| cmd.starts_with(input))
            .collect())
    })
    .prompt()?;
```

**長所**
- 豊富なプロンプト種類
- 美しいUI
- バリデーション・オートコンプリート機能
- 高度なカスタマイゼーション

**短所**
- 比較的重い
- 学習コストがやや高い

---

### dialoguer - シンプルダイアログ

**基本情報**
- **人気度**: ⭐⭐⭐⭐
- **特徴**: シンプルで使いやすいプロンプト
- **公式**: https://github.com/console-rs/dialoguer

```toml
[dependencies]
dialoguer = "0.11"
```

**基本使用例**
```rust
use dialoguer::{Input, Password, Confirm, Select, MultiSelect, theme::ColorfulTheme};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // テーマ設定
    let theme = &ColorfulTheme::default();
    
    // 入力
    let name: String = Input::with_theme(theme)
        .with_prompt("Your name")
        .interact_text()?;
    
    // パスワード
    let password = Password::with_theme(theme)
        .with_prompt("Password")
        .interact()?;
    
    // 確認
    let confirmed = Confirm::with_theme(theme)
        .with_prompt("Do you want to continue?")
        .interact()?;
    
    // 選択
    let selection = Select::with_theme(theme)
        .with_prompt("Pick your favorite language")
        .default(0)
        .items(&["Rust", "Go", "Python", "JavaScript"])
        .interact()?;
    
    // 複数選択
    let selections = MultiSelect::with_theme(theme)
        .with_prompt("Pick your tools")
        .items(&["Git", "Docker", "Kubernetes", "CI/CD"])
        .interact()?;
    
    println!("Hello {}", name);
    println!("You chose: {}", ["Rust", "Go", "Python", "JavaScript"][selection]);
    
    Ok(())
}
```

**プログレスバー**
```rust
use dialoguer::ProgressBar;
use std::thread;
use std::time::Duration;

let pb = ProgressBar::new(100);
for i in 0..100 {
    thread::sleep(Duration::from_millis(50));
    pb.inc(1);
}
pb.finish_with_message("done");
```

**長所**
- シンプルで直感的
- テーマ機能
- プログレスバー
- 軽量

**短所**
- inquireほど高機能ではない
- カスタマイゼーションが限定的

---

## 📊 引数パーサー比較表

| クレート | バイナリサイズ | コンパイル時間 | 機能数 | Unix対応 | 学習コスト |
|----------|--------------|--------------|--------|----------|-----------|
| **clap** | 大 (2-5MB) | 長い | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | 高 |
| **argh** | 小 (500KB-1MB) | 短い | ⭐⭐⭐ | ⭐⭐ | 低 |
| **pico-args** | 最小 (<500KB) | 最短 | ⭐⭐ | ⭐⭐⭐ | 中 |

## 🎯 使い分けガイド

### clap を選ぶべき場合
- 本格的なCLIツールを作成
- ヘルプ生成・補完が必要
- サブコマンドが必要
- Unix conventions準拠が重要

### argh を選ぶべき場合
- シンプルなツール
- コンパイル時間を重視
- Fuchsia OS向け開発

### pico-args を選ぶべき場合
- 最小限のバイナリサイズが必要
- 依存関係を避けたい
- シンプルな引数処理のみ

### inquire を選ぶべき場合
- リッチなユーザーインタラクション
- バリデーション・オートコンプリートが必要
- 美しいUI

### dialoguer を選ぶべき場合
- シンプルなプロンプト
- プログレスバー
- 軽量なインタラクション

---

**関連ドキュメント**:
- [TUI-Crates.md](./TUI-Crates.md) - TUI開発用クレート
- [Utility-Crates.md](./Utility-Crates.md) - ユーティリティクレート
- [Best-Practices.md](./Best-Practices.md) - ベストプラクティス
