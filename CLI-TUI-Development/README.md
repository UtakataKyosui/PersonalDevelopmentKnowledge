# CLI/TUI Development - 総合ガイド

## 📁 フォルダ構成

### [React-Ink/](./React-Ink/)
React を使用したCLI/TUI開発に関する情報

- **[React-Ink-Overview.md](./React-Ink/React-Ink-Overview.md)** - React Ink の概要と特徴
- **[Installation-Setup.md](./React-Ink/Installation-Setup.md)** - インストールとセットアップ
- **[Core-Components.md](./React-Ink/Core-Components.md)** - 基本コンポーネント
- **[Hooks-API.md](./React-Ink/Hooks-API.md)** - フック一覧とAPI
- **[Ink-UI-Library.md](./React-Ink/Ink-UI-Library.md)** - Ink UIコンポーネントライブラリ
- **[Best-Practices.md](./React-Ink/Best-Practices.md)** - ベストプラクティス
- **[Production-Examples.md](./React-Ink/Production-Examples.md)** - 本番環境での活用事例

### [Rust-Crates/](./Rust-Crates/)
Rust を使用したCLI/TUI開発に関する情報

- **[Rust-CLI-TUI-Overview.md](./Rust-Crates/Rust-CLI-TUI-Overview.md)** - Rust CLI/TUI開発の概要
- **[CLI-Crates.md](./Rust-Crates/CLI-Crates.md)** - CLI開発用クレート（clap, argh, inquire等）
- **[TUI-Crates.md](./Rust-Crates/TUI-Crates.md)** - TUI開発用クレート（Ratatui, Cursive等）
- **[Utility-Crates.md](./Rust-Crates/Utility-Crates.md)** - ユーティリティクレート（indicatif, console等）
- **[Best-Practices.md](./Rust-Crates/Best-Practices.md)** - Rustでのベストプラクティス
- **[Project-Structure.md](./Rust-Crates/Project-Structure.md)** - プロジェクト構造とCargo.toml設定

### [Examples/](./Examples/)
実装例とサンプルコード

- **[React-Ink-Examples/](./Examples/React-Ink-Examples/)** - React Ink実装例
- **[Rust-Examples/](./Examples/Rust-Examples/)** - Rust実装例
- **[Comparison/](./Examples/Comparison/)** - React Ink vs Rust の比較例

## 🎯 学習パス

### 初心者向け
1. [React-Ink-Overview.md](./React-Ink/React-Ink-Overview.md) または [Rust-CLI-TUI-Overview.md](./Rust-Crates/Rust-CLI-TUI-Overview.md)
2. それぞれの Installation-Setup.md
3. Examples フォルダの基本例

### 中級者向け
1. Core-Components.md と CLI-Crates.md
2. Best-Practices.md
3. 実装例での実践

### 上級者向け
1. Production-Examples.md
2. Project-Structure.md
3. Comparison フォルダでの技術選択検討

## 🔄 技術比較

| 項目 | React Ink | Rust Crates |
|------|-----------|-------------|
| 学習コスト | ⭐⭐⭐⭐⭐ (React知識活用) | ⭐⭐⭐ (Rust学習必要) |
| 開発速度 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| パフォーマンス | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| メモリ効率 | ⭐⭐ | ⭐⭐⭐⭐⭐ |
| 配布のしやすさ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |

詳細な比較は [Comparison](./Examples/Comparison/) フォルダを参照してください。

---

**作成日**: 2025年7月3日  
**最終更新**: 2025年7月3日  
**関連タグ**: #CLI #TUI #React #Rust #Terminal #CommandLine