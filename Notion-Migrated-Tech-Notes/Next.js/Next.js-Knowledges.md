# Next.js Knowledges

## メタデータ
- **作成日時**: 2025-03-10T13:55:00.000Z
- **最終更新**: 2025-07-01T20:03:00.000Z
- **タグ**: #Next.js #React #Frontend #WebDevelopment
- **移植元**: Notion
- **移植日**: 2025-07-02

## 概要

Next.jsに関する知識やテクニックをまとめたページです。

## Next.js 関連リソース

### 主要な学習書籍
- [実践Next.js——App Routerで進化するWebアプリ開発](./Next.js-App-Router.md)
- [実践Next.js](./Next.js-Practice.md)

### 認証・認可
- [Next.js App Router + Auth.js (Next Auth v5)](./Next.js-Auth.js.md)

### 関連技術
- [JSで作るいまどきのブラウザ拡張(WebExtensions)](https://techbookfest.org/product/kbzK73N5HjUjLPkZdaAAey)

## App Router（Next.js 13+）

### 主要な変更点
- Pages RouterからApp Routerへの移行
- Server Componentsの活用
- 新しいファイルベースルーティング
- メタデータAPIの改善

### ディレクトリ構造
```
app/
├── layout.tsx      # ルートレイアウト
├── page.tsx        # ホームページ
├── about/
│   └── page.tsx    # /about ページ
└── blog/
    ├── page.tsx    # /blog ページ
    └── [slug]/
        └── page.tsx # 動的ルート
```

### Server Components vs Client Components
- **Server Components**: サーバーサイドで実行、デフォルト
- **Client Components**: クライアントサイドで実行、`"use client"` 指定

## パフォーマンス最適化

### 画像最適化
```tsx
import Image from 'next/image'

<Image
  src="/hero.jpg"
  alt="Hero image"
  width={500}
  height={300}
  priority // LCPの改善
/>
```

### 動的インポート
```tsx
import dynamic from 'next/dynamic'

const DynamicComponent = dynamic(() => import('./Component'), {
  loading: () => <p>Loading...</p>,
  ssr: false // SSRを無効化
})
```

## データフェッチング

### Server Components でのデータフェッチ
```tsx
// app/posts/page.tsx
async function Posts() {
  const posts = await fetch('https://api.example.com/posts')
    .then(res => res.json())
  
  return (
    <div>
      {posts.map(post => (
        <article key={post.id}>{post.title}</article>
      ))}
    </div>
  )
}
```

### Client Components でのデータフェッチ
```tsx
'use client'
import { useState, useEffect } from 'react'

function PostsClient() {
  const [posts, setPosts] = useState([])
  
  useEffect(() => {
    fetch('/api/posts')
      .then(res => res.json())
      .then(setPosts)
  }, [])
  
  return <div>{/* posts を表示 */}</div>
}
```

## API Routes

### App Router でのAPI
```tsx
// app/api/users/route.ts
export async function GET() {
  const users = await getUsers()
  return Response.json(users)
}

export async function POST(request: Request) {
  const data = await request.json()
  const user = await createUser(data)
  return Response.json(user)
}
```

## デプロイメント

### Vercel
- 最も簡単なデプロイ方法
- 自動でビルド・デプロイ
- Edge Functionsサポート

### 自己ホスティング
```dockerfile
FROM node:18-alpine
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build
EXPOSE 3000
CMD ["npm", "start"]
```

## 開発tips

### 環境変数
```bash
# .env.local
NEXTAUTH_URL=http://localhost:3000
NEXTAUTH_SECRET=your-secret-key
DATABASE_URL=postgresql://...
```

### TypeScript設定
```json
// tsconfig.json
{
  "compilerOptions": {
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "paths": {
      "@/*": ["./src/*"]
    }
  }
}
```

## 関連リンク
- [[Next.js-App-Router]] - App Routerでの開発
- [[Next.js-Auth.js]] - 認証の実装
- [[DevContainer-CLI]] - 開発環境構築

## ノート
- このページはNotionから移植されました
- Next.jsは活発に開発されているため、最新のドキュメントを参照することを推奨します
- App Routerは安定版となったため、新規プロジェクトではApp Routerの使用を推奨
