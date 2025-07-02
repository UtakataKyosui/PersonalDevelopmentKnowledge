# Rust C FFI

## メタデータ
- **作成日時**: 2025-04-14T10:57:00.000Z
- **最終更新**: 2025-07-01T19:58:00.000Z
- **タグ**: #Rust #C #FFI #Interop
- **移植元**: Notion
- **移植日**: 2025-07-02

## 概要

Rust と C言語 間の Foreign Function Interface (FFI) について記載したページです。

## 主要トピック

### C から Rust を呼び出す方法
- Rustライブラリのコンパイル設定
- エクスポート関数の定義
- C言語での関数呼び出し

### Rust から C を呼び出す方法
- unsafe ブロックの使用方法
- C言語ライブラリのリンク
- 型変換とメモリ管理

### 注意点
- メモリ安全性の確保
- 文字列の取り扱い
- エラーハンドリング

## コード例

```rust
// Rust側のエクスポート関数例
#[no_mangle]
pub extern "C" fn rust_function(x: i32) -> i32 {
    x * 2
}

// C言語ライブラリの呼び出し例
extern "C" {
    fn c_function(x: i32) -> i32;
}

pub fn call_c_function(x: i32) -> i32 {
    unsafe {
        c_function(x)
    }
}
```

## 関連リンク
- [[Rust-DevContainer-WebAssembly開発]] - DevContainer環境でのRust開発
- [[Rust-Development-Tips]] - Rust開発のコツ

## ノート
- このページはNotionから移植されました
- FFIを使用する際は特にメモリ安全性に注意が必要です
