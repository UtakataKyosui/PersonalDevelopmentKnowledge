- [Zenn記事](https://zenn.dev/ficilcom/articles/t3-turbo-domain-cqrs)

## 記事要約

### 概要
Full-Stack TypeScript開発の事例として、T3-Turboアーキテクチャを導入した実際の開発体験についての記事。TSKaigi Advent Calendar 2024の記事として、実際の開発プロジェクトでの振り返りと改善案を紹介。

### 主な内容

#### T3-Turboのディレクトリ構成
```
t3-turbo
├─ apps
│  ├─ mobile (Expo)
│  ├─ web (NextJS)
│  └─ api (NestJS)
└─ packages
   ├─ tsconfig (TypeScript)
   ├─ database (Prisma)
   ├─ ui (shadcn/ui)
   ├─ graphql (GraphQL)
   ├─ trpc (tRPC)
   └─ core (DDD)
```

#### CQRS（Command Query Responsibility Segregation）の導入
- **Command**: NestJSでドメイン駆動設計（DDD）を実装
- **Query**: PostGraphileを使用したGraphQLによる自動生成API
  - PostgreSQLに接続すると自動でGraphQLエンドポイント生成
  - CRUDのコードを大量に書く必要がなくなる革新的なアプローチ
  - 自動生成で対応できない部分はtRPCでカバー

#### 技術選択の理由と実装
- **Prisma**: Turborepoの公式推奨、豊富な採用実績
- **prisma-zod-generator**: 最小限の工数でValidation実装
- **shadcn/ui**: コンポーネント数が豊富（執筆時点で50種類）、v0の登場で扱いやすさ向上
- **nestjs-trpc**: 2024年に正式リリースされた統合ライブラリ

#### フロントエンド・バックエンド連携
- NextJSとExpoで.graphqlファイル共有
- graphql-codegenで生成したTSファイルをpackages内に配置
- どのモジュールからも型情報を呼び出し可能
- フロント/バックで TypeScript の型を共有し、最上の開発体験を実現

#### PostGraphileの革新性
- PostgreSQLに接続するだけでGraphQLエンドポイントが自動生成
- 従来のCRUD実装コストを劇的に削減
- 「今までCRUDのコードをたくさん書いたのはなんだったのか」という開発者の感想

### 開発で得られた知見
- **monorepo管理による密結合の解消**
- **packages内モジュール化による最小限の依存関係**
- **フロントエンド/バックエンド間での型共有による開発体験向上**
- **PostGraphileによるCRUD操作の大幅な効率化**
- **tRPCとGraphQLの使い分けによる柔軟なAPI設計**

### フルスタックTypeScript開発の到達点
T3-Turboアーキテクチャは、個人開発レベルを超えた本格的なプロダクト開発において、TypeScriptの型安全性を最大限活用できるアーキテクチャとして位置づけられている。特にCQRSパターンとPostGraphileの組み合わせは、従来のAPI開発の常識を覆す革新的なアプローチとして注目される。