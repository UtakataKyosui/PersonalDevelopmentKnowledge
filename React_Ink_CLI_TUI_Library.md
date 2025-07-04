# React Ink - CLI/TUIライブラリ完全ガイド

## 📋 目次
1. [概要](#概要)
2. [特徴](#特徴)
3. [インストールとセットアップ](#インストールとセットアップ)
4. [基本コンポーネント](#基本コンポーネント)
5. [フック一覧](#フック一覧)
6. [Ink UI コンポーネントライブラリ](#ink-ui-コンポーネントライブラリ)
7. [実装例](#実装例)
8. [ベストプラクティス](#ベストプラクティス)
9. [本番環境での活用事例](#本番環境での活用事例)
10. [追加情報・Tips](#追加情報tips)
11. [参考資料](#参考資料)

---

## 概要

**React Ink**は、Reactを使ってインタラクティブなコマンドラインアプリケーション（CLI）やターミナルユーザーインターフェース（TUI）を構築するためのJavaScriptライブラリです。

### 核心概念
- **React Renderer**: DOMの代わりにターミナルにReactコンポーネントをレンダリング
- **Component-Based**: Reactの宣言的UIパラダイムをCLIに適用
- **Flexbox Layout**: YogaレイアウトエンジンによるFlexboxスタイリング
- **React Ecosystem**: 既存のReact知識とエコシステムを活用

### なぜReact Inkが注目されているのか
1. **学習コストの低さ**: React開発者が既存の知識でCLI開発可能
2. **コンポーネント再利用**: UIコンポーネントの再利用とモジュール化
3. **開発者体験**: React DevToolsサポートによる優れたデバッグ体験
4. **豊富なエコシステム**: React Hooks、Suspense、テストライブラリとの統合

---

## 特徴

### 🚀 主要機能
- **React完全対応**: すべてのReact機能（Hooks、Suspense、Error Boundaries等）が利用可能
- **TypeScript対応**: 完全なTypeScriptサポート
- **柔軟なレイアウト**: FlexboxベースのレイアウトシステムYoga使用
- **入力処理**: キーボード入力、フォーカス管理の高度なサポート
- **ストリーム管理**: stdin、stdout、stderrの直接制御
- **テスト対応**: `ink-testing-library`による包括的テスト環境

### 🎨 スタイリング機能
- **色とスタイル**: Chalkライブラリベースのテキストスタイリング
- **ボックスモデル**: margin、padding、border、dimensionsサポート
- **レスポンシブ**: ターミナルサイズに応じた動的レイアウト

---

## インストールとセットアップ

### 新規プロジェクト作成
### 💼 ファイルマネージャー（Cursive）

```rust
use cursive::views::{SelectView, TextView, LinearLayout, Panel, Dialog, EditView};
use cursive::{Cursive, CursiveExt};
use std::{fs, path::PathBuf};

struct FileManager {
    current_path: PathBuf,
}

impl FileManager {
    fn new() -> Self {
        Self {
            current_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
        }
    }

    fn create_file_list(&self) -> SelectView<PathBuf> {
        let mut select = SelectView::new();
        
        // 親ディレクトリへのエントリ
        if let Some(parent) = self.current_path.parent() {
            select.add_item("..", parent.to_path_buf());
        }
        
        if let Ok(entries) = fs::read_dir(&self.current_path) {
            let mut dirs = Vec::new();
            let mut files = Vec::new();
            
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                
                if path.is_dir() {
                    dirs.push((format!("📁 {}", name), path));
                } else {
                    files.push((format!("📄 {}", name), path));
                }
            }
            
            // ディレクトリを先に表示
            dirs.sort_by(|a, b| a.0.cmp(&b.0));
            files.sort_by(|a, b| a.0.cmp(&b.0));
            
            for (display_name, path) in dirs.into_iter().chain(files.into_iter()) {
                select.add_item(display_name, path);
            }
        }
        
        select.set_on_submit(handle_file_selection);
        select
    }
}

fn main() {
    let mut siv = cursive::default();
    let file_manager = FileManager::new();
    
    siv.set_user_data(file_manager);
    
    create_main_layout(&mut siv);
    
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('r', refresh_view);
    siv.add_global_callback('n', create_new_file);
    siv.add_global_callback('d', create_new_directory);
    
    siv.run();
}

fn create_main_layout(siv: &mut Cursive) {
    let file_manager = siv.user_data::<FileManager>().unwrap();
    let current_path = file_manager.current_path.display().to_string();
    
    let file_list = file_manager.create_file_list().with_name("file_list");
    
    let layout = LinearLayout::vertical()
        .child(TextView::new(format!("Current: {}", current_path)).with_name("current_path"))
        .child(Panel::new(file_list).title("Files"))
        .child(TextView::new("Commands: q=quit, r=refresh, n=new file, d=new dir"));
    
    siv.add_layer(layout);
}

fn handle_file_selection(siv: &mut Cursive, path: &PathBuf) {
    if path.is_dir() {
        // ディレクトリの場合は移動
        siv.with_user_data(|file_manager: &mut FileManager| {
            file_manager.current_path = path.clone();
        });
        refresh_view(siv);
    } else {
        // ファイルの場合は内容を表示
        show_file_content(siv, path);
    }
}

fn show_file_content(siv: &mut Cursive, path: &PathBuf) {
    let content = match fs::read_to_string(path) {
        Ok(content) => {
            if content.len() > 1000 {
                format!("{}\n\n... (truncated, {} bytes total)", 
                    &content[..1000], content.len())
            } else {
                content
            }
        }
        Err(e) => format!("Error reading file: {}", e),
    };
    
    siv.add_layer(
        Dialog::around(TextView::new(content).scrollable())
            .title(path.file_name().unwrap().to_string_lossy())
            .button("Close", |s| { s.pop_layer(); })
            .max_width(80)
            .max_height(25),
    );
}

fn refresh_view(siv: &mut Cursive) {
    let (current_path, file_list) = siv.with_user_data(|file_manager: &mut FileManager| {
        (file_manager.current_path.display().to_string(),
         file_manager.create_file_list())
    }).unwrap();
    
    siv.call_on_name("current_path", |view: &mut TextView| {
        view.set_content(format!("Current: {}", current_path));
    });
    
    siv.call_on_name("file_list", |view: &mut SelectView<PathBuf>| {
        *view = file_list;
    });
}

fn create_new_file(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("New File")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("File name:"))
                    .child(EditView::new().with_name("filename"))
            )
            .button("Create", |s| {
                let filename = s.call_on_name("filename", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
                
                if !filename.is_empty() {
                    let current_path = s.user_data::<FileManager>().unwrap().current_path.clone();
                    let file_path = current_path.join(filename.as_str());
                    
                    match fs::write(&file_path, "") {
                        Ok(()) => {
                            s.pop_layer();
                            refresh_view(s);
                        }
                        Err(e) => {
                            s.add_layer(
                                Dialog::around(TextView::new(format!("Error: {}", e)))
                                    .title("Error")
                                    .button("OK", |s| { s.pop_layer(); })
                            );
                        }
                    }
                }
            })
            .button("Cancel", |s| { s.pop_layer(); })
    );
}

fn create_new_directory(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("New Directory")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Directory name:"))
                    .child(EditView::new().with_name("dirname"))
            )
            .button("Create", |s| {
                let dirname = s.call_on_name("dirname", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
                
                if !dirname.is_empty() {
                    let current_path = s.user_data::<FileManager>().unwrap().current_path.clone();
                    let dir_path = current_path.join(dirname.as_str());
                    
                    match fs::create_dir(&dir_path) {
                        Ok(()) => {
                            s.pop_layer();
                            refresh_view(s);
                        }
                        Err(e) => {
                            s.add_layer(
                                Dialog::around(TextView::new(format!("Error: {}", e)))
                                    .title("Error")
                                    .button("OK", |s| { s.pop_layer(); })
                            );
                        }
                    }
                }
            })
            .button("Cancel", |s| { s.pop_layer(); })
    );
}
```

---

## ベストプラクティス

### 🏗️ プロジェクト構造

```
src/
├── main.rs              # エントリーポイント
├── cli/                 # CLI関連モジュール
│   ├── mod.rs
│   ├── args.rs          # 引数定義
│   └── commands.rs      # コマンド実装
├── tui/                 # TUI関連モジュール
│   ├── mod.rs
│   ├── app.rs           # アプリ状態管理
│   ├── ui.rs            # UIレンダリング
│   └── events.rs        # イベント処理
├── core/                # ビジネスロジック
│   ├── mod.rs
│   └── lib.rs
└── utils/               # ユーティリティ
    ├── mod.rs
    ├── config.rs        # 設定管理
    └── logger.rs        # ログ設定
```

### 📦 Cargo.toml設定例

```toml
[package]
name = "my-cli-tool"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A powerful CLI/TUI tool"
license = "MIT"
repository = "https://github.com/yourusername/my-cli-tool"
keywords = ["cli", "tui", "terminal"]
categories = ["command-line-utilities"]

[[bin]]
name = "my-tool"
path = "src/main.rs"

[dependencies]
# CLI
clap = { version = "4.4", features = ["derive", "env"] }
inquire = "0.7"

# TUI
ratatui = "0.25"
crossterm = "0.27"

# ユーティリティ
indicatif = "0.17"
console = "0.15"
comfy-table = "7.1"

# データ処理
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# エラーハンドリング
thiserror = "1.0"
anyhow = "1.0"

# 非同期（必要な場合）
tokio = { version = "1.0", features = ["full"], optional = true }

# ログ
tracing = "0.1"
tracing-subscriber = "0.3"

# システム情報
sysinfo = "0.30"

[features]
default = []
async = ["tokio"]

[profile.release]
lto = true
codegen-units = 1
panic = "abort"
```

### 🔧 エラーハンドリング

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    
    #[error("Configuration error: {message}")]
    Config { message: String },
    
    #[error("Invalid input: {input}")]
    InvalidInput { input: String },
}

pub type Result<T> = std::result::Result<T, AppError>;

// 使用例
fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}
```

### 📊 ログ設定

```rust
use tracing::{info, error, debug};
use tracing_subscriber::{EnvFilter, fmt};

pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();
}

// 使用例
fn main() -> Result<()> {
    init_logging();
    
    info!("Starting application");
    
    match run_app() {
        Ok(()) => {
            info!("Application finished successfully");
            Ok(())
        }
        Err(e) => {
            error!("Application failed: {}", e);
            Err(e)
        }
    }
}
```

### ⚙️ 設定管理

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub theme: Theme,
    pub keybindings: KeyBindings,
    pub logging: LoggingConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Theme {
    pub primary_color: String,
    pub secondary_color: String,
    pub background_color: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyBindings {
    pub quit: char,
    pub refresh: char,
    pub help: char,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Theme {
                primary_color: "blue".to_string(),
                secondary_color: "green".to_string(),
                background_color: "black".to_string(),
            },
            keybindings: KeyBindings {
                quit: 'q',
                refresh: 'r',
                help: '?',
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }
    
    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| AppError::Config {
                message: "Could not find config directory".to_string(),
            })?;
        Ok(config_dir.join("my-tool").join("config.toml"))
    }
}
```

### 🧪 テスト戦略

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_config_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        
        let original_config = Config::default();
        
        // 保存
        let content = toml::to_string_pretty(&original_config).unwrap();
        std::fs::write(&config_path, content).unwrap();
        
        // 読み込み
        let content = std::fs::read_to_string(&config_path).unwrap();
        let loaded_config: Config = toml::from_str(&content).unwrap();
        
        assert_eq!(original_config.theme.primary_color, loaded_config.theme.primary_color);
    }
    
    #[test]
    fn test_cli_parsing() {
        use clap::Parser;
        
        let args = ["my-tool", "--name", "test", "--count", "5"];
        let parsed = Args::try_parse_from(&args).unwrap();
        
        assert_eq!(parsed.name, "test");
        assert_eq!(parsed.count, 5);
    }
}

// 統合テスト用のヘルパー
#[cfg(test)]
mod integration_tests {
    use std::process::Command;
    
    #[test]
    fn test_cli_help() {
        let output = Command::new("cargo")
            .args(["run", "--", "--help"])
            .output()
            .expect("Failed to execute command");
        
        assert!(output.status.success());
        assert!(String::from_utf8(output.stdout)
            .unwrap()
            .contains("Usage:"));
    }
}
```

---

## React Inkとの比較

### 📊 機能比較表

| 特徴 | React Ink | Rust Crates |
|------|-----------|-------------|
| **学習コストの低さ** | ⭐⭐⭐⭐⭐ (React知識があれば) | ⭐⭐⭐ (Rustの学習が必要) |
| **パフォーマンス** | ⭐⭐⭐ (Node.js) | ⭐⭐⭐⭐⭐ (ネイティブ) |
| **メモリ使用量** | ⭐⭐ (V8エンジン) | ⭐⭐⭐⭐⭐ (最小限) |
| **バイナリサイズ** | ⭐⭐ (Node.js必要) | ⭐⭐⭐⭐ (単体実行ファイル) |
| **開発速度** | ⭐⭐⭐⭐⭐ (高速プロトタイピング) | ⭐⭐⭐ (コンパイル時間) |
| **クロスプラットフォーム** | ⭐⭐⭐⭐⭐ (Node.js対応環境) | ⭐⭐⭐⭐ (ネイティブコンパイル) |
| **エコシステム** | ⭐⭐⭐⭐ (npm) | ⭐⭐⭐⭐⭐ (crates.io) |
| **型安全性** | ⭐⭐⭐ (TypeScript使用時) | ⭐⭐⭐⭐⭐ (コンパイル時保証) |

### 🔄 類似機能の対応

#### React Ink vs Ratatui
```javascript
// React Ink
const App = () => (
  <Box flexDirection="column">
    <Text color="green" bold>Hello World</Text>
    <Text>Press 'q' to quit</Text>
  </Box>
);
```

```rust
// Ratatui
let paragraph = Paragraph::new("Hello World")
    .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    .block(Block::default().borders(Borders::ALL));

let help_text = Paragraph::new("Press 'q' to quit")
    .block(Block::default());
```

#### 状態管理の比較
```javascript
// React Ink
const [count, setCount] = useState(0);

useInput((input, key) => {
  if (input === '+') {
    setCount(count + 1);
  }
});
```

```rust
// Rust
struct App {
    count: i32,
}

impl App {
    fn increment(&mut self) {
        self.count += 1;
    }
}

// イベントループ内
if let KeyCode::Char('+') = key.code {
    app.increment();
}
```

### 🎯 選択基準

#### React Inkを選ぶべき場合
- **React/JavaScript チームでの開発**
- **高速プロトタイピングが必要**
- **npm エコシステムを活用したい**
- **Node.js 環境が利用可能**

#### Rust を選ぶべき場合
- **最大限のパフォーマンスが必要**
- **メモリ効率を重視**
- **単体実行ファイルを配布したい**
- **システムレベルの操作が必要**
- **長期的な保守性を重視**

---

## 参考資料

### 📚 公式ドキュメント
- [Ratatui](https://ratatui.rs/) - 最新のTUIライブラリ
- [clap](https://docs.rs/clap/) - CLI引数パーサー
- [Cursive](https://docs.rs/cursive/) - 高レベルTUIライブラリ
- [crossterm](https://docs.rs/crossterm/) - クロスプラットフォーム端末操作
- [indicatif](https://docs.rs/indicatif/) - プログレスバー・スピナー

### 🛠️ ツールとユーティリティ
- [console](https://docs.rs/console/) - ターミナル色・スタイル
- [inquire](https://docs.rs/inquire/) - 対話的プロンプト
- [dialoguer](https://docs.rs/dialoguer/) - ダイアログUIコンポーネント
- [comfy-table](https://docs.rs/comfy-table/) - テーブル作成
- [tracing](https://docs.rs/tracing/) - 構造化ログ

### 📖 学習リソース
- [Command Line Applications in Rust](https://rust-cli.github.io/book/) - 公式CLIガイド
- [Ratatui Tutorials](https://ratatui.rs/tutorials/) - Ratatui公式チュートリアル
- [Awesome TUI](https://github.com/rothgar/awesome-tuis) - TUIアプリケーション集
- [Rust CLI Working Group](https://github.com/rust-cli) - CLIツール開発グループ

### 🏆 注目プロジェクト例
- [bat](https://github.com/sharkdp/bat) - catの強化版
- [exa](https://github.com/ogham/exa) - lsの代替
- [fd](https://github.com/sharkdp/fd) - findの代替
- [ripgrep](https://github.com/BurntSushi/ripgrep) - grepの高速版
- [gitui](https://github.com/extrawurst/gitui) - Git TUI
- [bottom](https://github.com/ClementTsang/bottom) - システムモニター
- [zoxide](https://github.com/ajeetdsouza/zoxide) - スマートcd
- [starship](https://github.com/starship/starship) - クロスシェルプロンプト

### 📊 プロジェクトテンプレート
- [Ratatui Templates](https://github.com/ratatui/templates) - Ratatui公式テンプレート
- [CLI Template](https://github.com/rust-cli/cli-template) - CLI作成テンプレート

---

**作成日**: 2025年7月3日  
**最終更新**: 2025年7月3日  
**対応Rustバージョン**: 1.75+  
**主要クレートバージョン**: Ratatui 0.25+, clap 4.4+, Cursive 0.20+

**関連タグ**: #Rust #CLI #TUI #Terminal #CommandLine #Ratatui #clap #Cursive #crossterm #Performance

**次回更新予定**: 主要クレートのメジャーバージョンアップ時、または新しい注目クレートの登場時bash
# JavaScript
npx create-ink-app my-cli-app

# TypeScript
npx create-ink-app my-cli-app --typescript
```

### 手動インストール
```bash
npm install ink react
npm install --save-dev @types/react  # TypeScript使用時
```

### プロジェクト構造
```
my-cli-app/
├── source/
│   ├── app.tsx        # メインアプリコンポーネント
│   └── cli.tsx        # CLIエントリーポイント
├── package.json
└── tsconfig.json      # TypeScript使用時
```

---

## 基本コンポーネント

### Text - テキスト表示
```jsx
import React from 'react';
import {Text} from 'ink';

const App = () => (
  <Text color="green" bold>
    Hello, World!
  </Text>
);
```

**Props**:
- `color`: テキスト色 (`'black' | 'red' | 'green' | 'yellow' | 'blue' | 'magenta' | 'cyan' | 'white' | 'gray'`)
- `backgroundColor`: 背景色
- `bold`, `italic`, `underline`, `strikethrough`: テキストスタイル
- `dimColor`: 薄い色で表示

### Box - レイアウト管理
```jsx
import React from 'react';
import {Box, Text} from 'ink';

const App = () => (
  <Box flexDirection="column" padding={1} borderStyle="round">
    <Text>Header</Text>
    <Box justifyContent="space-between" marginTop={1}>
      <Text>Left</Text>
      <Text>Right</Text>
    </Box>
  </Box>
);
```

**主要Props**:
- **Layout**: `flexDirection`, `justifyContent`, `alignItems`, `flexWrap`
- **Spacing**: `margin`, `marginTop`, `padding`, `paddingLeft`など
- **Size**: `width`, `height`, `minWidth`, `maxHeight`
- **Border**: `borderStyle`, `borderColor`
- **Position**: `position`, `top`, `left`

### Newline & Spacer
```jsx
import React from 'react';
import {Box, Text, Newline, Spacer} from 'ink';

const App = () => (
  <Box flexDirection="column">
    <Text>First line</Text>
    <Newline />
    <Text>Second line after empty line</Text>
    
    <Box>
      <Text>Left</Text>
      <Spacer />  {/* 利用可能スペースを埋める */}
      <Text>Right</Text>
    </Box>
  </Box>
);
```

---

**作成日**: 2025年7月3日  
**最終更新**: 2025年7月3日  
**React Ink バージョン**: v5.x (最新)  
**Ink UI バージョン**: v2.x (最新)

**関連タグ**: #React #CLI #TUI #JavaScript #TypeScript #TerminalUI #CommandLine #DeveloperTools

**次回更新予定**: React Ink v6リリース時、または新機能追加時