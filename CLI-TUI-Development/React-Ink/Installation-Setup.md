# React Ink - インストールとセットアップ

## 🚀 クイックスタート

### 新規プロジェクト作成（推奨）

```bash
# JavaScript プロジェクト
npx create-ink-app my-cli-app

# TypeScript プロジェクト
npx create-ink-app my-cli-app --typescript
```

### 手動インストール

```bash
# 基本パッケージ
npm install ink react

# TypeScript使用時
npm install --save-dev @types/react @types/node

# 開発依存関係
npm install --save-dev ink-testing-library
```

## 📁 プロジェクト構造

### 基本構造
```
my-cli-app/
├── source/
│   ├── app.tsx          # メインアプリコンポーネント
│   ├── cli.tsx          # CLIエントリーポイント
│   └── ui.tsx           # UI コンポーネント
├── package.json
├── tsconfig.json        # TypeScript使用時
└── README.md
```

### 推奨ディレクトリ構造（大規模プロジェクト）
```
src/
├── components/          # 再利用可能コンポーネント
│   ├── ui/             # UI基本コンポーネント
│   ├── forms/          # フォーム関連
│   └── layout/         # レイアウト関連
├── hooks/              # カスタムフック
├── utils/              # ユーティリティ関数
├── types/              # TypeScript型定義
├── styles/             # スタイル定義
├── app.tsx             # メインアプリ
└── cli.tsx             # エントリーポイント
```

## ⚙️ package.json 設定

### 基本設定
```json
{
  "name": "my-cli-tool",
  "version": "1.0.0",
  "license": "MIT",
  "bin": {
    "my-cli": "./dist/cli.js"
  },
  "engines": {
    "node": ">=14.0.0"
  },
  "files": [
    "dist"
  ],
  "scripts": {
    "build": "tsc",
    "dev": "tsx watch source/cli.tsx",
    "start": "node dist/cli.js",
    "test": "jest",
    "prepublishOnly": "npm run build"
  },
  "dependencies": {
    "ink": "^4.4.1",
    "react": "^18.2.0",
    "meow": "^12.1.1",
    "import-jsx": "^5.0.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.45",
    "@types/node": "^20.10.0",
    "ink-testing-library": "^3.0.0",
    "typescript": "^5.3.2",
    "tsx": "^4.6.0",
    "jest": "^29.7.0"
  }
}
```

### TypeScript設定
```json
// tsconfig.json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "lib": ["ES2020"],
    "outDir": "./dist",
    "rootDir": "./source",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "jsx": "react"
  },
  "include": ["source/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

## 🔧 基本的な実装

### CLI エントリーポイント (cli.tsx)
```typescript
#!/usr/bin/env node
import React from 'react';
import { render } from 'ink';
import meow from 'meow';
import App from './app.js';

const cli = meow(`
  Usage
    $ my-cli [options]

  Options
    --name, -n  Name to greet
    --verbose   Verbose output

  Examples
    $ my-cli --name=Jane
    Hello, Jane!
`, {
  importMeta: import.meta,
  flags: {
    name: {
      type: 'string',
      shortFlag: 'n',
      default: 'World'
    },
    verbose: {
      type: 'boolean',
      default: false
    }
  }
});

render(<App name={cli.flags.name} verbose={cli.flags.verbose} />);
```

### メインアプリ (app.tsx)
```typescript
import React, { useState, useEffect } from 'react';
import { Text, Box, useInput, useApp } from 'ink';

interface AppProps {
  name: string;
  verbose?: boolean;
}

const App: React.FC<AppProps> = ({ name, verbose = false }) => {
  const [counter, setCounter] = useState(0);
  const { exit } = useApp();

  useInput((input, key) => {
    if (input === 'q' || key.escape) {
      exit();
    }
    if (key.upArrow) {
      setCounter(c => c + 1);
    }
    if (key.downArrow) {
      setCounter(c => Math.max(0, c - 1));
    }
  });

  useEffect(() => {
    if (verbose) {
      console.log('App started in verbose mode');
    }
  }, [verbose]);

  return (
    <Box flexDirection="column">
      <Text color="green" bold>
        Hello, {name}!
      </Text>
      <Text>
        Counter: <Text color="blue">{counter}</Text>
      </Text>
      <Text dimColor>
        Use ↑/↓ to change counter, 'q' to quit
      </Text>
    </Box>
  );
};

export default App;
```

## 🧪 テスト環境セットアップ

### Jest 設定
```json
// jest.config.js
export default {
  preset: 'ts-jest',
  testEnvironment: 'node',
  setupFilesAfterEnv: ['<rootDir>/src/test-setup.ts'],
  testMatch: ['**/__tests__/**/*.test.ts', '**/*.test.ts'],
  collectCoverageFrom: [
    'src/**/*.{ts,tsx}',
    '!src/**/*.d.ts',
    '!src/cli.tsx'
  ]
};
```

### テストセットアップ
```typescript
// src/test-setup.ts
import { jest } from '@jest/globals';

// Mock console methods to avoid noise in tests
global.console = {
  ...console,
  log: jest.fn(),
  warn: jest.fn(),
  error: jest.fn(),
};
```

### 基本テスト例
```typescript
// src/app.test.tsx
import React from 'react';
import { render } from 'ink-testing-library';
import App from './app';

describe('App', () => {
  test('renders greeting message', () => {
    const { lastFrame } = render(<App name="Test" />);
    expect(lastFrame()).toContain('Hello, Test!');
  });

  test('handles key input', () => {
    const { stdin, lastFrame } = render(<App name="Test" />);
    
    // Simulate up arrow key press
    stdin.write('\u001B[A');
    expect(lastFrame()).toContain('Counter: 1');
  });

  test('exits on q key', () => {
    const mockExit = jest.fn();
    jest.mock('ink', () => ({
      ...jest.requireActual('ink'),
      useApp: () => ({ exit: mockExit })
    }));

    const { stdin } = render(<App name="Test" />);
    stdin.write('q');
    
    expect(mockExit).toHaveBeenCalled();
  });
});
```

## 🔄 開発ワークフロー

### ローカル開発
```bash
# 開発モード（ホットリロード）
npm run dev

# 一度だけ実行
npm start

# テスト実行
npm test

# テスト（ウォッチモード）
npm test -- --watch
```

### ビルドと配布
```bash
# プロダクションビルド
npm run build

# ローカルでテスト
npm link
my-cli --name="Test"

# npm公開
npm publish
```

## 🐛 デバッグ設定

### React DevTools 有効化
```bash
# DevTools起動
npm install -g react-devtools
react-devtools

# アプリをDevToolsモードで実行
DEV=true npm start
```

### ログ設定
```typescript
// デバッグモードでの詳細ログ
if (process.env.DEBUG) {
  console.log('Debug mode enabled');
  // デバッグ情報の出力
}
```

### エラーハンドリング
```typescript
import React from 'react';
import { Text, Box } from 'ink';

class ErrorBoundary extends React.Component<
  { children: React.ReactNode },
  { hasError: boolean; error?: Error }
> {
  constructor(props: { children: React.ReactNode }) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error: Error) {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    console.error('CLI Error:', error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      return (
        <Box flexDirection="column">
          <Text color="red" bold>Something went wrong:</Text>
          <Text>{this.state.error?.message}</Text>
          <Text dimColor>Check the console for more details.</Text>
        </Box>
      );
    }

    return this.props.children;
  }
}

export default ErrorBoundary;
```

## 📦 よく使用される追加パッケージ

### UI強化
```bash
npm install ink-text-input ink-select-input ink-spinner
npm install ink-gradient ink-big-text
```

### ユーティリティ
```bash
npm install chalk figures boxen
npm install ora inquirer
```

### データ処理
```bash
npm install lodash date-fns
npm install axios
```

---

**次のステップ**: [Core-Components.md](./Core-Components.md) で基本コンポーネントを学習

**関連ドキュメント**:
- [React-Ink-Overview.md](./React-Ink-Overview.md)
- [Hooks-API.md](./Hooks-API.md)
- [Best-Practices.md](./Best-Practices.md)