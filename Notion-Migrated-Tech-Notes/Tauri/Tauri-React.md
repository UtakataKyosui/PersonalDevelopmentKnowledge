# Tauri × Reactをうまいことやりたい

## メタデータ
- **作成日時**: 2025-04-14T01:05:00.000Z
- **最終更新**: 2025-07-01T19:58:00.000Z
- **タグ**: #Tauri #React #Frontend #Desktop
- **移植元**: Notion
- **移植日**: 2025-07-02

## 概要

TauriとReactを組み合わせてデスクトップアプリケーションを開発する際のベストプラクティスや注意点をまとめたページです。

## Tauri + React 環境構築

### プロジェクト初期化
```bash
# Tauriプロジェクトの作成
npm create tauri-app@latest

# または手動でReactプロジェクトにTauriを追加
npx create-react-app my-app
cd my-app
npm install --save-dev @tauri-apps/cli
npx tauri init
```

### 依存関係の設定
```json
{
  "devDependencies": {
    "@tauri-apps/cli": "^1.5.0"
  },
  "dependencies": {
    "@tauri-apps/api": "^1.5.0"
  }
}
```

## React コンポーネントでのTauri API活用

### ファイルシステム操作
```tsx
import { open, save } from '@tauri-apps/api/dialog';
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';

function FileManager() {
  const openFile = async () => {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Text Files',
        extensions: ['txt', 'md']
      }]
    });
    
    if (selected) {
      const contents = await readTextFile(selected as string);
      console.log(contents);
    }
  };

  const saveFile = async (content: string) => {
    const path = await save({
      filters: [{
        name: 'Text Files',
        extensions: ['txt']
      }]
    });
    
    if (path) {
      await writeTextFile(path, content);
    }
  };

  return (
    <div>
      <button onClick={openFile}>Open File</button>
      <button onClick={() => saveFile('Hello World!')}>Save File</button>
    </div>
  );
}
```

### ウィンドウ操作
```tsx
import { appWindow } from '@tauri-apps/api/window';
import { useState, useEffect } from 'react';

function WindowControls() {
  const [isMaximized, setIsMaximized] = useState(false);

  useEffect(() => {
    const checkMaximized = async () => {
      setIsMaximized(await appWindow.isMaximized());
    };
    
    checkMaximized();
    
    const unlisten = appWindow.listen('tauri://resize', checkMaximized);
    return () => {
      unlisten.then(f => f());
    };
  }, []);

  const toggleMaximize = async () => {
    await appWindow.toggleMaximize();
  };

  const minimize = async () => {
    await appWindow.minimize();
  };

  const close = async () => {
    await appWindow.close();
  };

  return (
    <div className="titlebar">
      <button onClick={minimize}>−</button>
      <button onClick={toggleMaximize}>
        {isMaximized ? '⧉' : '⬜'}
      </button>
      <button onClick={close}>×</button>
    </div>
  );
}
```

## カスタムフック活用

### useTauriCommand hook
```tsx
import { invoke } from '@tauri-apps/api/tauri';
import { useState, useCallback } from 'react';

function useTauriCommand<T>(command: string) {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const execute = useCallback(async (args?: any) => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await invoke<T>(command, args);
      setData(result);
      return result;
    } catch (err) {
      setError(err as string);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [command]);

  return { data, loading, error, execute };
}

// 使用例
function App() {
  const { data, loading, error, execute } = useTauriCommand<string>('greet');

  const handleGreet = () => {
    execute({ name: 'World' });
  };

  return (
    <div>
      <button onClick={handleGreet} disabled={loading}>
        {loading ? 'Loading...' : 'Greet'}
      </button>
      {error && <p>Error: {error}</p>}
      {data && <p>Response: {data}</p>}
    </div>
  );
}
```

## 状態管理

### Context APIでのTauri状態管理
```tsx
import React, { createContext, useContext, useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface AppState {
  user: User | null;
  settings: Settings;
  isLoading: boolean;
}

const AppContext = createContext<{
  state: AppState;
  actions: {
    loadUser: () => Promise<void>;
    updateSettings: (settings: Settings) => Promise<void>;
  };
}>({} as any);

export function AppProvider({ children }: { children: React.ReactNode }) {
  const [state, setState] = useState<AppState>({
    user: null,
    settings: defaultSettings,
    isLoading: true
  });

  const loadUser = async () => {
    try {
      const user = await invoke<User>('get_current_user');
      setState(prev => ({ ...prev, user, isLoading: false }));
    } catch (error) {
      setState(prev => ({ ...prev, isLoading: false }));
    }
  };

  const updateSettings = async (settings: Settings) => {
    await invoke('update_settings', { settings });
    setState(prev => ({ ...prev, settings }));
  };

  useEffect(() => {
    loadUser();
  }, []);

  return (
    <AppContext.Provider value={{
      state,
      actions: { loadUser, updateSettings }
    }}>
      {children}
    </AppContext.Provider>
  );
}

export const useApp = () => useContext(AppContext);
```

## パフォーマンス最適化

### イベントリスナーの適切な管理
```tsx
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';

function useAppEvents() {
  const [status, setStatus] = useState('idle');

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setupListener = async () => {
      unlisten = await listen('app-status-change', (event) => {
        setStatus(event.payload as string);
      });
    };

    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  return status;
}
```

### メモリ使用量の最適化
```tsx
import { memo, useMemo } from 'react';

const ExpensiveComponent = memo(({ data }: { data: any[] }) => {
  const processedData = useMemo(() => {
    // 重い処理
    return data.map(item => ({
      ...item,
      processed: true
    }));
  }, [data]);

  return (
    <div>
      {processedData.map(item => (
        <div key={item.id}>{item.name}</div>
      ))}
    </div>
  );
});
```

## デザインシステムとの統合

### Tailwind CSS
```tsx
// カスタムタイトルバー
function CustomTitleBar() {
  return (
    <div 
      data-tauri-drag-region 
      className="h-8 bg-gray-800 flex justify-between items-center px-4 select-none"
    >
      <div className="text-white text-sm">My App</div>
      <WindowControls />
    </div>
  );
}
```

### Material-UI / MUI
```tsx
import { ThemeProvider, createTheme } from '@mui/material/styles';
import { CssBaseline } from '@mui/material';

const theme = createTheme({
  palette: {
    mode: 'dark',
  },
});

function App() {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <CustomTitleBar />
      <MainContent />
    </ThemeProvider>
  );
}
```

## ビルドとデプロイ

### ビルド設定
```json
// tauri.conf.json
{
  "build": {
    "beforeBuildCommand": "npm run build",
    "distDir": "../build"
  },
  "tauri": {
    "bundle": {
      "active": true,
      "targets": ["deb", "appimage", "msi", "app", "dmg"]
    }
  }
}
```

### GitHub Actions
```yaml
name: Build Tauri App

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        platform: [macos-latest, ubuntu-latest, windows-latest]
    
    runs-on: ${{ matrix.platform }}
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18
          
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Install dependencies
        run: npm ci
        
      - name: Build Tauri app
        run: npm run tauri build
```

## トラブルシューティング

### よくある問題
1. **CORS エラー**: 開発時にAPIを呼び出す際の問題
2. **ファイルアクセス権限**: ファイルシステム操作時の権限不足
3. **ビルドサイズ**: バンドルサイズが大きくなる問題

### 解決策
```tsx
// CORS問題の解決
const fetchWithTauri = async (url: string) => {
  try {
    const response = await fetch(url, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });
    return await response.json();
  } catch (error) {
    // Tauriのhttpプラグインを使用
    const { Body, fetch: tauriFetch } = await import('@tauri-apps/api/http');
    return await tauriFetch(url);
  }
};
```

## 関連リンク
- [[Tauri-Development-Environment]] - Tauri開発環境の構築
- [[DevContainer-CLI]] - DevContainer環境での開発
- [[Next.js-Knowledges]] - React系フレームワークの知識

## ノート
- このページはNotionから移植されました
- TauriとReactの組み合わせは非常に強力ですが、デスクトップアプリ特有の考慮事項があります
- パフォーマンスとセキュリティのバランスを取ることが重要です
