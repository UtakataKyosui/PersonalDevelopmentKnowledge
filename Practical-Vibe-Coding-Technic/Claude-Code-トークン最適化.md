# Claude Codeトークン使用量削減アプローチ

> **原文**: [Claude Codeでトークン使用量を削減するアプローチ](https://zenn.dev/driller/articles/ff6a50ae228b2b)

## 概要

Claude Codeのプロプランにはレート制限があるため、トークン使用量を意識した効率的な使い方で快適な開発体験を実現する方法論。実際の開発で使える具体的な削減テクニックを体系化。

## 1. コード生成の最適化

### 具体的な指示を出す

#### ❌ 悪い例
```
"ログイン機能を作って"
```

#### ✅ 良い例
```
"FastAPIでJWT認証を使ったログイン機能を実装して、
- Pydanticでリクエスト/レスポンスモデル定義
- bcryptでパスワードハッシュ化
- 入力バリデーション（email形式、パスワード8文字以上）
- HTTPExceptionでエラーハンドリング（401, 422ステータス）を含めて"
```

### 段階的な開発
大きな機能を小さな単位に分割：

1. **Phase 1**: ユーザーモデルの定義（SQLAlchemy）
2. **Phase 2**: Pydanticスキーマの作成
3. **Phase 3**: JWTトークン生成/検証ユーティリティ
4. **Phase 4**: 認証エンドポイントの実装
5. **Phase 5**: 認証デコレータの作成

## 2. コンテキスト管理の最適化

### 必要最小限のファイル共有
- ❌ プロジェクト全体を対象に含める
- ✅ 修正対象のファイルと直接的な依存関係のみ

### repomixツールの活用
```bash
# 最適な設定でのrepomix実行
npx repomix --style xml --compress --remove-comments --remove-empty-lines
```

#### repomixの効果：
- プロジェクト構造の明確な可視化
- 不要なコメントや空行の除去によるトークン削減
- XML形式でのClaude最適化された出力

### 部分的更新の活用
```
"models/user.py の User クラスに email_verified フィールドを追加し、
対応するPydanticスキーマも schemas/user.py で更新してください"
```

## 3. 事前設計によるトークン節約

### Claude Codeの限界（2025年6月時点）
- 難しいタスクでのループ：同じアプローチを繰り返し
- 遠回りな解決方法：最適ではない実装を継続
- 設計レベルの議論が困難：アーキテクチャ検討に向かない
- レート制限の制約：無駄な試行錯誤を避ける必要

### 効果的な役割分担
```
【通常のClaude】設計相談・アーキテクチャ検討・問題解決戦略
                ↓
【Claude Code】明確な指示による一発実装
```

### 他のAIサービス併用
ChatGPTやGeminiを併用してレート制限を分散し、多角的な視点を獲得。

### Claude Codeの知見活用
```
Claude Code で以下の問題が発生しています：
[エラーログやスタックトレース]
[試行した解決方法]
より効率的な解決アプローチを提案してください。
その後、Claude Code に明確な指示を出したいと思います。
```

## 4. 効率的なコミュニケーション

### 関連タスクのまとめ
```
以下の3つの修正を一度に実行してください:
1. models/user.py にcreated_at, updated_atフィールド追加
2. schemas/user.py の対応するPydanticモデル更新
3. alembicマイグレーションファイルの生成
```

### エラー対応の効率化
```
以下のPydanticValidationErrorが発生しています:
ValidationError: email field required

関連するコード:
[schemas/user.py の該当部分]

期待する動作:
emailフィールドが必須でない場合のオプショナル設定
```

## 5. 実践的なワークフロー

### 推奨開発フロー例：FastAPIユーザー管理機能

#### Step 1: 設計相談（通常のClaude）
- 要件の整理
- SQLAlchemyモデル設計
- Pydanticスキーマ設計
- FastAPIエンドポイント設計
- エラーハンドリング方針

#### Step 2: 実装（Claude Code）
```
以下の仕様でFastAPIユーザー管理機能を実装してください:
[SQLAlchemyモデル定義]
[Pydanticスキーマ定義]
[FastAPIエンドポイント仕様]
[JWT認証の実装詳細]
[pytestテストケース]
```

#### Step 3: 問題発生時の対処
```
通常のClaudeへ相談:
「Claude Code で以下のSQLAlchemyエラーが発生し、
複数のアプローチを試しましたが解決しません。
[エラー詳細とログ]
効率的な解決戦略を教えてください」

↓ 解決策を得た後

Claude Code へ明確な修正指示:
「以下の手順でSQLAlchemyの設定を修正してください」
```

## 6. Python特有の最適化テクニック

### 型ヒントの活用
```python
from typing import Optional, List
from pydantic import BaseModel

def get_users(
    skip: int = 0,
    limit: int = 100,
    search: Optional[str] = None
) -> List[UserResponse]:
    # 実装を依頼
```

### pyproject.tomlと依存関係の明確化
```toml
[tool.poetry.dependencies]
python = "^3.11"
fastapi = "^0.104.1"
sqlalchemy = "^2.0.23"
pydantic = "^2.5.0"
python-jose = {extras = ["cryptography"], version = "^3.3.0"}
passlib = {extras = ["bcrypt"], version = "^1.7.4"}
```

### pytestテストの同時生成
```
以下の関数を実装し、対応するpytestテストも同時に生成してください:
- 正常系テスト
- 異常系テスト（バリデーションエラー）
- モックを使った外部依存のテスト
```

## 7. 構造化された知見管理システムの活用

### 効率的なプロンプト例
```
.claude/context.md の技術スタック（FastAPI + SQLAlchemy + Pydantic）と制約条件を考慮し、
.claude/project-knowledge.md の実装パターンに従って、
User管理APIを実装してください。
```

プロジェクト固有の制約や技術選定理由、過去の決定事項を毎回説明する必要がなくなり、劇的なトークン削減効果を実現。

## まとめ

最も重要なのは**明確で具体的な指示**。特に効果的な戦略：

1. **事前設計**: 通常のClaudeで設計を固めてからClaude Codeで実装
2. **段階的開発**: 大きな機能を小さな単位に分割
3. **具体的指示**: 曖昧な要求を避け、詳細な仕様を提供
4. **効率的なコンテキスト管理**: 必要最小限のファイルのみを対象に含める

これらの戦略を組み合わせることで、開発効率を維持しながらトークン使用量を大幅に削減可能。