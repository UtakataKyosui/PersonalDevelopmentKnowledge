# LIFF×Next.js構成の共通点とベストプラクティス

調査対象リポジトリと記事から抽出した、LIFF（LINE Front-end Framework）とNext.jsを組み合わせた開発における共通パターンとベストプラクティス。

## 📋 調査対象

- **nottyo/nextjs-liff** - Next.jsベーシックなLIFFアプリ例
- **jiyuujin/nextjs-liff** - TypeScript + Firebase + LIFFの構成
- **hyodoblog/liff-nextjs-template** - LIFFテンプレート
- **line/create-liff-app** - LINEオフィシャルテンプレート
- **hideokamoto/use-line-liff** - カスタムフックライブラリ
- **公式Qiita記事** - 実装ガイドライン

## 🔧 技術スタック共通点

### フロントエンド
- **Next.js** (App Router / Page Router)
- **TypeScript** (ほぼ全プロジェクトで採用)
- **React 18+**
- **@line/liff** パッケージ

### 開発・デプロイ環境
- **ngrok** (ローカル開発でのHTTPS環境)
- **Vercel / Netlify / Heroku** (本番デプロイ)
- **Firebase Hosting** (一部プロジェクト)

### バックエンド連携
- **Firebase Authentication** (認証)
- **Firebase Firestore** (データベース)
- **AWS AppRunner** (コンテナデプロイ)

## 🏗️ アーキテクチャパターン

### 1. Context Provider パターン（最も一般的）

```typescript
// LiffProvider.tsx
'use client';
import React, { createContext, useContext, useEffect, useState } from 'react';
import { Liff } from '@line/liff';

const LiffContext = createContext<{
  liff: Liff | null;
  liffError: string | null;
}>({ liff: null, liffError: null });

export const useLiff = () => useContext(LiffContext);

export const LiffProvider = ({ children, liffId }) => {
  const [liff, setLiff] = useState<Liff | null>(null);
  const [liffError, setLiffError] = useState<string | null>(null);

  useEffect(() => {
    const initLiff = async () => {
      try {
        const liffModule = await import('@line/liff');
        const liff = liffModule.default;
        await liff.init({ liffId });
        setLiff(liff);
      } catch (error) {
        setLiffError(error.toString());
      }
    };
    initLiff();
  }, [liffId]);

  return (
    <LiffContext.Provider value={{ liff, liffError }}>
      {children}
    </LiffContext.Provider>
  );
};
```

### 2. pageProps注入パターン（Page Router）

```typescript
// _app.tsx (Page Router)
function MyApp({ Component, pageProps }) {
  const [liffObject, setLiffObject] = useState(null);
  const [liffError, setLiffError] = useState(null);

  useEffect(() => {
    import('@line/liff')
      .then((liff) => liff.default)
      .then((liff) => {
        liff.init({ liffId: process.env.NEXT_PUBLIC_LIFF_ID })
          .then(() => setLiffObject(liff))
          .catch((error) => setLiffError(error.toString()));
      });
  }, []);

  pageProps.liff = liffObject;
  pageProps.liffError = liffError;

  return <Component {...pageProps} />;
}
```

## 🎯 ベストプラクティス

### 1. LIFF初期化の基本原則

#### ✅ **動的インポートの必須使用**
```typescript
// ❌ 直接インポートは避ける
import liff from '@line/liff';

// ✅ 動的インポートでSSRエラー回避
const liffModule = await import('@line/liff');
const liff = liffModule.default;
```

**理由**: LIFF SDKは`window`オブジェクトに依存するため、SSR環境では実行時エラーが発生

#### ✅ **App Router対応**
```typescript
// App Router使用時は必ず'use client'ディレクティブ
'use client';
import { useEffect } from 'react';
```

#### ✅ **エラーハンドリングの徹底**
```typescript
const initLiff = useCallback(async () => {
  try {
    const liffModule = await import('@line/liff');
    const liff = liffModule.default;
    await liff.init({ liffId });
    setLiff(liff);
  } catch (error) {
    console.error('LIFF init failed:', error);
    setLiffError(error.toString());
  }
}, [liffId]);
```

### 2. 環境変数管理

#### ✅ **命名規則の統一**
```bash
# .env.local
NEXT_PUBLIC_LIFF_ID=your_liff_id_here
```

**重要**: Next.jsでクライアントサイドからアクセスする環境変数は`NEXT_PUBLIC_`プレフィックスが必須

#### ✅ **開発環境の分離**
```bash
# 開発環境
NEXT_PUBLIC_LIFF_ID=dev_liff_id

# 本番環境
NEXT_PUBLIC_LIFF_ID=prod_liff_id
```

### 3. 開発環境構築

#### ✅ **ngrokによるHTTPS環境**
```bash
# ngrokインストール・設定
npm install -g ngrok
ngrok http 3000

# 生成されたHTTPS URLをLIFF設定に登録
https://your-subdomain.ngrok-free.app
```

**理由**: LIFFアプリは本番・開発環境問わずHTTPS必須

#### ✅ **デプロイ戦略**
- **開発**: ngrok + npm run dev
- **ステージング**: Vercel/Netlify preview
- **本番**: Vercel/Netlify production / AWS AppRunner

### 4. カスタムフック活用

#### ✅ **use-line-liffライブラリ**
```typescript
// より簡潔な実装
import { LiffProvider, useLiff } from 'use-line-liff';

function App() {
  return (
    <LiffProvider liffId={process.env.NEXT_PUBLIC_LIFF_ID}>
      <HomePage />
    </LiffProvider>
  );
}

function HomePage() {
  const { liff } = useLiff();
  
  useEffect(() => {
    if (!liff) return;
    if (!liff.isLoggedIn()) {
      liff.login();
    }
  }, [liff]);
}
```

### 5. テスト・デバッグ支援

#### ✅ **LIFFモック機能**
```typescript
<LiffProvider
  liffId={process.env.NEXT_PUBLIC_LIFF_ID}
  mock={{
    enable: process.env.NODE_ENV === 'development',
    mockDidLoaded: (liff) => ({
      ...liff,
      getProfile: {
        userId: 'test-user-id',
        displayName: 'Test User'
      }
    })
  }}
>
```

## 🛡️ セキュリティ考慮事項

### 1. **AccessToken管理**
- LIFFで取得したユーザー情報を直接サーバーに送信しない
- 必要に応じてLINE API経由でサーバーサイド検証を実施

### 2. **環境変数の適切な管理**
- LIFF IDは機密情報として扱う
- .env.localを.gitignoreに必ず追加

### 3. **HTTPS通信の徹底**
- 開発・本番環境問わずHTTPS必須
- 証明書エラーが発生した場合の対処法を事前に準備

## 📈 パフォーマンス最適化

### 1. **LIFF初期化の最適化**
- useCallbackでの初期化関数メモ化
- 重複初期化の防止
- エラー時のリトライ機構

### 2. **バンドルサイズ最適化**
- 動的インポートによるコード分割
- 必要なLIFF機能のみの読み込み

## 🔄 状態管理パターン

### Context API（推奨）
- グローバルなLIFF状態管理
- TypeScript対応
- エラーハンドリング一元化

### Zustand/Redux（大規模アプリ）
- 複雑な状態ロジック
- ミドルウェア活用
- DevTools連携

## 📱 プラットフォーム対応

### モバイル最適化
- Tailwind CSS / Emotion / styled-components
- レスポンシブデザイン
- LIFFサイズ設定（Full, Tall, Compact）

### ブラウザ対応
- モダンブラウザ対応
- iOS Safari / Android Chrome最適化
- LINE in-app browser考慮

---

**結論**: LIFF×Next.js開発では、**Context APIによる状態管理**、**動的インポート**、**HTTPS環境構築**、**適切な環境変数管理**が成功の鍵となる。特にSSRエラー回避とApp Router対応が現代的な開発では重要。