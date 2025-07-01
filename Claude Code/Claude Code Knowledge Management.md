
```bash
project-root/
├── CLAUDE.md                    # Claude Codeのメイン設定
└── .claude/
    ├── context.md              # プロジェクトの背景・制約
    ├── project-knowledge.md    # 技術的知見・パターン
    ├── project-improvements.md # 改善履歴・教訓
    ├── common-patterns.md      # 頻用コマンドパターン
    ├── debug-log.md           # 重要なデバッグ記録
    └── debug/                 # 一時的なデバッグファイル
        ├── sessions/          # セッション別ログ
        ├── temp-logs/         # 作業中の一時ファイル
        └── archive/           # 解決済み問題の保管
```


## `CLAUDE.md` メイン設定

- プロジェクトの概要
- `Claude`の指示を記載
```md
# プロジェクト概要
このプロジェクトは...

## 知見管理システム
このプロジェクトでは以下のファイルで知見を体系的に管理しています：

### `.claude/context.md`
- プロジェクトの背景、目的、制約条件
- 技術スタック選定理由
- ビジネス要件や技術的制約

### `.claude/project-knowledge.md`
- 実装パターンや設計決定の知見
- アーキテクチャの選択理由
- 避けるべきパターンやアンチパターン

### `.claude/project-improvements.md`
- 過去の試行錯誤の記録
- 失敗した実装とその原因
- 改善プロセスと結果

### `.claude/common-patterns.md`
- 頻繁に使用するコマンドパターン
- 定型的な実装テンプレート

**重要**: 新しい実装や重要な決定を行った際は、該当するファイルを更新してください。
```

## `.claude/context.md` プロジェクトコンテキスト

```md
# プロジェクトコンテキスト

## 概要
- プロジェクト名: ECサイト管理システム
- 技術スタック: React + TypeScript + Node.js
- 目標: 中小企業向けの使いやすいECサイト構築

## 制約条件
- 既存のPostgreSQLデータベースとの互換性維持
- レスポンス時間は200ms以内
- IE11サポートは不要（モダンブラウザのみ）

## 技術選定理由
- React: チームの習熟度が高い
- TypeScript: 型安全性とメンテナンス性重視
- Tailwind CSS: 迅速なUI開発のため
```

## `.claude/project-knowledge.md` 技術的知見
```md
# プロジェクト知見集

## アーキテクチャ決定
### 認証システム
- JWT + RefreshTokenパターンを採用
- 理由: スケーラビリティとセキュリティのバランス
- 学習: 当初Sessionベースだったがマイクロサービス化を見据え変更

## 実装パターン
### エラーハンドリング
- 共通のErrorBoundaryコンポーネントを使用
- APIエラーは `src/utils/errorHandler.ts` で統一処理
- 学習: 各コンポーネントでの個別処理は重複が多く非効率

## 避けるべきパターン
- useStateの過度な分割（パフォーマンス問題）
- CSS-in-JSの深いネスト（バンドルサイズ増大）

## ライブラリ選定
- 状態管理: Zustand（Reduxより軽量、学習コスト低）
- テスト: Jest + React Testing Library（チーム習熟度重視）
```

## `.claude/project-improvements.md` 改善履歴

```md
# 改善履歴

## 2024-12-15: ページ表示速度の改善
**問題**: 商品一覧ページの初期表示に3秒以上かかる

**試行錯誤**:
- ❌ 画像の遅延読み込み → 効果限定的
- ❌ SQLクエリの最適化 → 10%程度の改善
- ✅ 商品データの段階的読み込み → 60%の改善

**最終解決策**:
1. 初期表示は20件のみ
2. 無限スクロールで追加読み込み
3. 画像のWebP変換とCDN配信

**教訓**: UXの観点から全件表示よりも段階的読み込みが効果的
```

## `.claude/common-patterns.md` 頻用パターン
```md
# よく使用するパターン

## コンポーネント生成
```bash
# 新しいReactコンポーネントの作成
claude create component [ComponentName] with TypeScript and styled-components
```

## Commandによる知見更新
`.claude/commands/learnings.md`

```md
新たな知見が得られたら、次のファイルを更新してください

- `.claude/context.md`: 背景・制約・技術選定理由
- `.claude/project-knowledge.md`: 実装パターン・設計決定
- `.claude/project-improvements.md`: 試行錯誤・改善記録
- `.claude/common-patterns.md`: 定型実装・コマンドパターン
```

