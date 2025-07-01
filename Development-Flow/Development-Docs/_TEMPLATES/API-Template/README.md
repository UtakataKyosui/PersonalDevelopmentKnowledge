# API プロジェクト: [API名]

## API概要

### APIタイプ
- [ ] REST API
- [ ] GraphQL API
- [ ] gRPC API
- [ ] WebSocket API
- [ ] Webhook API
- [ ] マイクロサービス

### サービス種別
- [ ] 認証・認可サービス
- [ ] データ処理・変換API
- [ ] 外部連携API
- [ ] 通知・メッセージングAPI
- [ ] ファイル処理API
- [ ] 分析・レポートAPI

## 技術スタック

### バックエンド
- **言語**: Node.js/Python/Go/Rust/Java
- **フレームワーク**: Express/FastAPI/Gin/Axum/Spring Boot
- **データベース**: PostgreSQL/MongoDB/Redis
- **ORM**: Prisma/SQLAlchemy/GORM/SeaORM

### インフラ・デプロイ
- **コンテナ**: Docker/Podman
- **オーケストレーション**: Kubernetes/Docker Compose
- **クラウド**: AWS/GCP/Azure
- **API Gateway**: AWS API Gateway/Kong/Nginx

### 監視・ログ
- **ログ**: Winston/Loguru/Zap/tracing
- **メトリクス**: Prometheus/DataDog
- **トレーシング**: Jaeger/OpenTelemetry
- **ヘルスチェック**: /health エンドポイント

## API設計

### 基本情報
- **ベースURL**: `https://api.example.com/v1`
- **認証方式**: JWT/API Key/OAuth 2.0
- **レート制限**: 1000 requests/hour
- **レスポンス形式**: JSON

### エンドポイント設計

#### 認証関連
```
POST   /auth/login           # ログイン
POST   /auth/logout          # ログアウト
POST   /auth/refresh         # トークン更新
GET    /auth/me              # ユーザー情報取得
```

#### リソース操作（RESTful）
```
GET    /users                # ユーザー一覧
GET    /users/:id            # ユーザー詳細
POST   /users                # ユーザー作成
PUT    /users/:id            # ユーザー更新
DELETE /users/:id            # ユーザー削除

GET    /users/:id/posts      # ユーザーの投稿一覧
POST   /users/:id/posts      # 投稿作成
```

#### その他
```
GET    /health               # ヘルスチェック
GET    /metrics              # メトリクス
GET    /docs                 # API ドキュメント
```

### レスポンス仕様

#### 成功レスポンス
```json
{
  "success": true,
  "data": {
    "id": "123",
    "name": "Example User",
    "email": "user@example.com"
  },
  "metadata": {
    "timestamp": "2025-06-18T10:00:00Z",
    "version": "v1"
  }
}
```

#### エラーレスポンス
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid input data",
    "details": [
      {
        "field": "email",
        "message": "Invalid email format"
      }
    ]
  },
  "metadata": {
    "timestamp": "2025-06-18T10:00:00Z",
    "requestId": "req_123456"
  }
}
```

## データモデル設計

### ユーザーモデル
```typescript
interface User {
  id: string
  email: string
  name: string
  avatar?: string
  role: 'admin' | 'user'
  createdAt: Date
  updatedAt: Date
  lastLoginAt?: Date
}
```

### 投稿モデル
```typescript
interface Post {
  id: string
  title: string
  content: string
  authorId: string
  status: 'draft' | 'published' | 'archived'
  tags: string[]
  createdAt: Date
  updatedAt: Date
}
```

### リレーション
```
User (1) ←→ (many) Post
User (many) ←→ (many) Role
```

## 認証・認可

### JWT実装
```typescript
// JWT ペイロード
interface JWTPayload {
  sub: string          // ユーザーID
  email: string        // メールアドレス
  role: string         // ロール
  iat: number          // 発行時刻
  exp: number          // 有効期限
}

// 認証ミドルウェア
async function authenticate(req: Request): Promise<User> {
  const token = extractToken(req)
  const payload = verifyJWT(token)
  return await getUserById(payload.sub)
}
```

### 認可制御
```typescript
// ロールベースアクセス制御
enum Permission {
  READ_USER = 'read:user',
  WRITE_USER = 'write:user',
  DELETE_USER = 'delete:user'
}

// 認可チェック
function authorize(permission: Permission) {
  return (req: Request, res: Response, next: NextFunction) => {
    if (hasPermission(req.user, permission)) {
      next()
    } else {
      res.status(403).json({ error: 'Forbidden' })
    }
  }
}
```

## バリデーション

### リクエスト検証
```typescript
// ユーザー作成リクエスト
interface CreateUserRequest {
  email: string       // 必須、メール形式
  name: string        // 必須、2-50文字
  password: string    // 必須、8文字以上
}

// バリデーションスキーマ（Zod例）
const createUserSchema = z.object({
  email: z.string().email(),
  name: z.string().min(2).max(50),
  password: z.string().min(8)
})
```

### データベース制約
```sql
-- ユーザーテーブル
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email VARCHAR(255) UNIQUE NOT NULL,
  name VARCHAR(100) NOT NULL,
  password_hash VARCHAR(255) NOT NULL,
  role VARCHAR(20) DEFAULT 'user',
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

-- インデックス
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at);
```

## エラーハンドリング

### エラーコード体系
```typescript
enum ErrorCode {
  // 4xx クライアントエラー
  VALIDATION_ERROR = 'VALIDATION_ERROR',
  AUTHENTICATION_REQUIRED = 'AUTHENTICATION_REQUIRED',
  FORBIDDEN = 'FORBIDDEN',
  NOT_FOUND = 'NOT_FOUND',
  
  // 5xx サーバーエラー
  INTERNAL_ERROR = 'INTERNAL_ERROR',
  DATABASE_ERROR = 'DATABASE_ERROR',
  EXTERNAL_SERVICE_ERROR = 'EXTERNAL_SERVICE_ERROR'
}
```

### グローバルエラーハンドラー
```typescript
function errorHandler(err: Error, req: Request, res: Response, next: NextFunction) {
  const errorResponse = {
    success: false,
    error: {
      code: err.code || 'INTERNAL_ERROR',
      message: err.message,
      requestId: req.id
    },
    metadata: {
      timestamp: new Date().toISOString()
    }
  }
  
  // ログ出力
  logger.error('API Error', { error: err, requestId: req.id })
  
  res.status(err.statusCode || 500).json(errorResponse)
}
```

## セキュリティ

### 基本的なセキュリティ対策
```typescript
// セキュリティヘッダー
app.use(helmet())

// CORS設定
app.use(cors({
  origin: ['https://example.com'],
  credentials: true
}))

// レート制限
app.use(rateLimit({
  windowMs: 15 * 60 * 1000, // 15分
  max: 100 // 最大100リクエスト
}))

// リクエストサイズ制限
app.use(express.json({ limit: '10mb' }))
```

### 入力検証・サニタイゼーション
```typescript
// SQLインジェクション対策
const user = await db.user.findUnique({
  where: { id: userId } // パラメータ化クエリ
})

// XSS対策
const sanitizedContent = DOMPurify.sanitize(userInput)
```

## テスト

### テスト戦略
- **単体テスト**: 個別関数・メソッドのテスト
- **統合テスト**: エンドポイント単位のテスト
- **E2Eテスト**: シナリオベースのテスト
- **負荷テスト**: パフォーマンステスト

### テスト実装例
```typescript
// 統合テスト例
describe('POST /users', () => {
  it('should create a new user', async () => {
    const userData = {
      email: 'test@example.com',
      name: 'Test User',
      password: 'password123'
    }
    
    const response = await request(app)
      .post('/users')
      .send(userData)
      .expect(201)
    
    expect(response.body.success).toBe(true)
    expect(response.body.data.email).toBe(userData.email)
  })
  
  it('should return validation error for invalid email', async () => {
    const invalidData = {
      email: 'invalid-email',
      name: 'Test',
      password: 'pass'
    }
    
    await request(app)
      .post('/users')
      .send(invalidData)
      .expect(400)
  })
})
```

## パフォーマンス最適化

### データベース最適化
```sql
-- 適切なインデックス
CREATE INDEX idx_posts_author_status ON posts(author_id, status);
CREATE INDEX idx_posts_created_at ON posts(created_at DESC);

-- クエリ最適化
EXPLAIN ANALYZE 
SELECT p.*, u.name as author_name 
FROM posts p 
JOIN users u ON p.author_id = u.id 
WHERE p.status = 'published'
ORDER BY p.created_at DESC 
LIMIT 20;
```

### キャッシュ戦略
```typescript
// Redis キャッシュ
async function getUserWithCache(userId: string): Promise<User> {
  const cacheKey = `user:${userId}`
  
  // キャッシュから取得試行
  const cached = await redis.get(cacheKey)
  if (cached) {
    return JSON.parse(cached)
  }
  
  // データベースから取得
  const user = await db.user.findUnique({ where: { id: userId } })
  
  // キャッシュに保存（1時間）
  await redis.setEx(cacheKey, 3600, JSON.stringify(user))
  
  return user
}
```

## ドキュメンテーション

### OpenAPI/Swagger
```yaml
openapi: 3.0.0
info:
  title: Example API
  version: 1.0.0
  description: API for example service

paths:
  /users:
    get:
      summary: Get all users
      responses:
        '200':
          description: Successful response
          content:
            application/json:
              schema:
                type: object
                properties:
                  success:
                    type: boolean
                  data:
                    type: array
                    items:
                      $ref: '#/components/schemas/User'

components:
  schemas:
    User:
      type: object
      properties:
        id:
          type: string
        email:
          type: string
        name:
          type: string
```

## デプロイメント

### Docker設定
```dockerfile
FROM node:18-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --only=production

COPY . .
RUN npm run build

EXPOSE 3000

USER node

CMD ["npm", "start"]
```

### 環境設定
```bash
# 本番環境
NODE_ENV=production
PORT=3000
DATABASE_URL=postgresql://user:pass@host:5432/db
JWT_SECRET=your-secret-key
REDIS_URL=redis://host:6379
```

## 監視・運用

### ヘルスチェック
```typescript
// /health エンドポイント
app.get('/health', async (req, res) => {
  const health = {
    status: 'healthy',
    timestamp: new Date().toISOString(),
    services: {
      database: await checkDatabase(),
      redis: await checkRedis(),
      externalAPI: await checkExternalAPI()
    }
  }
  
  const allHealthy = Object.values(health.services).every(status => status === 'healthy')
  res.status(allHealthy ? 200 : 503).json(health)
})
```

### メトリクス収集
```typescript
// Prometheus メトリクス
const promClient = require('prom-client')

const httpRequestDuration = new promClient.Histogram({
  name: 'http_request_duration_seconds',
  help: 'Duration of HTTP requests in seconds',
  labelNames: ['method', 'route', 'status_code']
})

// メトリクス記録ミドルウェア
app.use((req, res, next) => {
  const start = Date.now()
  
  res.on('finish', () => {
    const duration = (Date.now() - start) / 1000
    httpRequestDuration
      .labels(req.method, req.route?.path || req.path, res.statusCode.toString())
      .observe(duration)
  })
  
  next()
})
```

### ログ設定
```typescript
import winston from 'winston'

const logger = winston.createLogger({
  level: 'info',
  format: winston.format.combine(
    winston.format.timestamp(),
    winston.format.errors({ stack: true }),
    winston.format.json()
  ),
  transports: [
    new winston.transports.File({ filename: 'error.log', level: 'error' }),
    new winston.transports.File({ filename: 'combined.log' }),
    new winston.transports.Console({
      format: winston.format.simple()
    })
  ]
})
```

## スケーラビリティ

### 水平スケーリング
```yaml
# Kubernetes Deployment
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-deployment
spec:
  replicas: 3
  selector:
    matchLabels:
      app: api
  template:
    metadata:
      labels:
        app: api
    spec:
      containers:
      - name: api
        image: api:latest
        ports:
        - containerPort: 3000
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: api-secrets
              key: database-url
```

### 負荷分散
```nginx
# Nginx 設定
upstream api_backend {
    server api1:3000;
    server api2:3000;
    server api3:3000;
}

server {
    listen 80;
    
    location /api/ {
        proxy_pass http://api_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## トラブルシューティング

### よくある問題

#### 1. データベース接続エラー
```typescript
// 対策：接続プール設定
const pool = new Pool({
  connectionString: process.env.DATABASE_URL,
  max: 20,
  idleTimeoutMillis: 30000,
  connectionTimeoutMillis: 2000,
})
```

#### 2. メモリリーク
```typescript
// 対策：適切なリソース管理
process.on('SIGTERM', async () => {
  await db.destroy()
  await redis.quit()
  process.exit(0)
})
```

#### 3. レスポンス遅延
```typescript
// 対策：タイムアウト設定
app.use((req, res, next) => {
  res.setTimeout(30000, () => {
    res.status(408).json({ error: 'Request timeout' })
  })
  next()
})
```

## 関連ドキュメント

- **要件定義**: [[Requirements]]
- **データベース設計**: [[Database-Design]]
- **セキュリティ**: [[Security-Guidelines]]
- **デプロイ手順**: [[Deployment]]
- **運用手順**: [[Operations]]
- **API仕様書**: [[API-Specification]]

---

**作成日**: {{date}}  
**最終更新**: {{date}}
