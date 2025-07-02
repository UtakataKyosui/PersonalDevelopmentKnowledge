# WebAssemblyのメモリアロケータ

## メタデータ
- **作成日時**: 2024-08-21T11:30:00.000Z
- **最終更新**: 2024-08-21T11:30:00.000Z
- **タグ**: #WebAssembly #Memory #Allocator #Performance
- **移植元**: Notion
- **移植日**: 2025-07-02

## 概要

WebAssemblyにおけるメモリ管理とアロケータの仕組みについて解説するページです。

## WebAssemblyのメモリモデル

### 線形メモリ
WebAssemblyは線形メモリモデルを採用しており、連続したバイト配列として表現されます。

```javascript
// JavaScriptからWebAssemblyメモリにアクセス
const memory = new WebAssembly.Memory({ initial: 1 }); // 1ページ = 64KB
const buffer = new Uint8Array(memory.buffer);
```

### メモリページ
- 1ページ = 64KB (65,536バイト)
- 初期サイズと最大サイズを指定可能
- 動的にページを追加可能（grow操作）

## Rustでのメモリアロケータ

### デフォルトアロケータ
RustのWebAssemblyでは、デフォルトで`dlmalloc`が使用されます。

```rust
// Cargo.toml
[dependencies]
wee_alloc = "0.4"

// main.rs
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

### wee_alloc
軽量なアロケータで、バイナリサイズを削減できます。

```rust
use wee_alloc;

// グローバルアロケータとして設定
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// パニック時の処理
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

### lol_alloc
さらに小さなアロケータオプション：

```rust
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> = 
    LockedAllocator::new(FreeListAllocator::new());
```

## メモリ使用量の最適化

### バイナリサイズの削減
```toml
# Cargo.toml
[profile.release]
lto = true
opt-level = 's'  # サイズ最適化
debug = false
panic = 'abort'
codegen-units = 1

[dependencies]
wasm-bindgen = { version = "0.2", features = ["serde-serialize"] }
```

### 不要な機能の除去
```rust
// std機能を無効化
#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
```

## JavaScript側でのメモリ管理

### メモリビューの作成
```javascript
class WasmMemoryManager {
  constructor(wasmModule) {
    this.wasmModule = wasmModule;
    this.memory = wasmModule.memory;
  }

  // メモリビューを取得
  getUint8Array(ptr, length) {
    return new Uint8Array(this.memory.buffer, ptr, length);
  }

  getUint32Array(ptr, length) {
    return new Uint32Array(this.memory.buffer, ptr, length);
  }

  // 文字列の読み取り
  readString(ptr, length) {
    const bytes = this.getUint8Array(ptr, length);
    return new TextDecoder().decode(bytes);
  }

  // 文字列の書き込み
  writeString(str) {
    const bytes = new TextEncoder().encode(str);
    const ptr = this.wasmModule.alloc(bytes.length);
    const memory = this.getUint8Array(ptr, bytes.length);
    memory.set(bytes);
    return ptr;
  }
}
```

### メモリ成長の監視
```javascript
function watchMemoryGrowth(wasmModule) {
  let lastSize = wasmModule.memory.buffer.byteLength;
  
  setInterval(() => {
    const currentSize = wasmModule.memory.buffer.byteLength;
    if (currentSize !== lastSize) {
      console.log(`Memory grew from ${lastSize} to ${currentSize} bytes`);
      lastSize = currentSize;
    }
  }, 1000);
}
```

## パフォーマンス測定

### メモリ使用量の測定
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
}

#[wasm_bindgen]
pub fn measure_allocation_performance() {
    let start = now();
    
    // 大量のメモリ割り当て
    let mut vectors = Vec::new();
    for i in 0..1000 {
        vectors.push(vec![i; 1000]);
    }
    
    let end = now();
    web_sys::console::log_1(&format!("Allocation took {} ms", end - start).into());
}
```

### ヒープサイズの確認
```javascript
// ブラウザの開発者ツールで実行
function checkWasmMemoryUsage() {
  if (performance.measureUserAgentSpecificMemory) {
    performance.measureUserAgentSpecificMemory().then(result => {
      console.log('Memory usage:', result);
    });
  } else {
    console.log('Memory measurement not supported');
  }
}
```

## メモリリークの防止

### RAII パターンの活用
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ResourceManager {
    data: Vec<u8>,
}

#[wasm_bindgen]
impl ResourceManager {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> ResourceManager {
        ResourceManager {
            data: vec![0; size],
        }
    }

    pub fn process(&mut self) {
        // データ処理
    }
}

// Dropトレイトで自動的にクリーンアップ
impl Drop for ResourceManager {
    fn drop(&mut self) {
        web_sys::console::log_1(&"ResourceManager dropped".into());
    }
}
```

### JavaScript側でのクリーンアップ
```javascript
class WasmResourceManager {
  constructor() {
    this.resources = new Set();
  }

  allocate(size) {
    const resource = wasmModule.allocate(size);
    this.resources.add(resource);
    return resource;
  }

  deallocate(resource) {
    if (this.resources.has(resource)) {
      wasmModule.deallocate(resource);
      this.resources.delete(resource);
    }
  }

  cleanup() {
    for (const resource of this.resources) {
      wasmModule.deallocate(resource);
    }
    this.resources.clear();
  }
}
```

## 実践的な例

### 大きなデータセットの処理
```rust
use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

#[wasm_bindgen]
pub fn process_large_dataset(data: &Uint8Array) -> Vec<u8> {
    let input_data = data.to_vec();
    
    // チャンク単位での処理でメモリ使用量を制御
    let chunk_size = 1024 * 1024; // 1MB chunks
    let mut result = Vec::new();
    
    for chunk in input_data.chunks(chunk_size) {
        let processed_chunk = process_chunk(chunk);
        result.extend(processed_chunk);
    }
    
    result
}

fn process_chunk(chunk: &[u8]) -> Vec<u8> {
    // チャンクごとの処理
    chunk.iter().map(|&x| x.wrapping_mul(2)).collect()
}
```

### メモリプールの実装
```rust
use std::collections::VecDeque;

pub struct MemoryPool {
    free_blocks: VecDeque<*mut u8>,
    block_size: usize,
}

impl MemoryPool {
    pub fn new(block_size: usize, initial_blocks: usize) -> Self {
        let mut pool = MemoryPool {
            free_blocks: VecDeque::new(),
            block_size,
        };
        
        // 初期ブロックを確保
        for _ in 0..initial_blocks {
            let block = unsafe { 
                std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(
                    block_size, 
                    std::mem::align_of::<u8>()
                ))
            };
            pool.free_blocks.push_back(block);
        }
        
        pool
    }

    pub fn allocate(&mut self) -> Option<*mut u8> {
        self.free_blocks.pop_front()
    }

    pub fn deallocate(&mut self, ptr: *mut u8) {
        self.free_blocks.push_back(ptr);
    }
}
```

## ベストプラクティス

### 1. 適切なアロケータの選択
- 小さなバイナリサイズが必要: `wee_alloc`
- パフォーマンス重視: デフォルトの`dlmalloc`
- 極小サイズ: `lol_alloc`

### 2. メモリ使用量の監視
- 定期的なメモリ使用量チェック
- 大きなアロケーションの追跡
- ガベージコレクションのタイミング

### 3. 効率的なデータ構造
- 連続メモリレイアウトの活用
- 不必要なコピーの回避
- 適切なバッファリング

## トラブルシューティング

### Out of Memory エラー
```rust
// メモリ不足を検出
#[wasm_bindgen]
pub fn safe_allocation(size: usize) -> Option<Vec<u8>> {
    if size > 100 * 1024 * 1024 { // 100MB制限
        return None;
    }
    
    Some(vec![0; size])
}
```

### メモリ断片化の回避
```rust
// オブジェクトプールパターン
pub struct ObjectPool<T> {
    objects: Vec<Option<T>>,
    free_indices: Vec<usize>,
}

impl<T> ObjectPool<T> {
    pub fn new() -> Self {
        ObjectPool {
            objects: Vec::new(),
            free_indices: Vec::new(),
        }
    }

    pub fn get(&mut self) -> usize {
        if let Some(index) = self.free_indices.pop() {
            index
        } else {
            let index = self.objects.len();
            self.objects.push(None);
            index
        }
    }

    pub fn return_object(&mut self, index: usize) {
        if index < self.objects.len() {
            self.objects[index] = None;
            self.free_indices.push(index);
        }
    }
}
```

## 関連リンク
- [[WebAssembly-Links]] - WebAssembly関連リンク集
- [[WebAssembly-JSON]] - JSONデータの取り扱い
- [[Rust-DevContainer-WebAssembly開発]] - Rust開発環境

## 参考資料
- [The wasm-bindgen Guide - Memory](https://rustwasm.github.io/wasm-bindgen/)
- [WebAssembly Memory Model](https://webassembly.github.io/spec/core/syntax/modules.html#memories)
- [Rust WebAssembly Book](https://rustwasm.github.io/book/)

## ノート
- このページはNotionから移植されました
- メモリアロケータの選択はアプリケーションの要件によって決定してください
- パフォーマンス測定は実際の使用環境で行うことを推奨します
- メモリリークの早期発見のため、定期的な監視を実装することが重要です
