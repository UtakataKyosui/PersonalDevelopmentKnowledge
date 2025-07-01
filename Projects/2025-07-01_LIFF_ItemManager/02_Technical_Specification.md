# 技術仕様 - LIFF物品管理アプリ

## 🏗️ システムアーキテクチャ

### 全体構成
```
LINE App
└── LIFF App (React + TypeScript)
    ├── LIFF SDK v2.x
    ├── Camera API (物品撮影)
    ├── 画像処理・圧縮
    └── REST API
        └── Cloudflare Workers (Hono)
            ├── LINE認証
            ├── Cloudflare D1 Database
            ├── Cloudflare R2 Storage (画像)
            ├── Cloudflare AI (画像認識)
            ├── Cloudflare Cron Triggers
            └── LINE Bot (通知のみ)
```

### データフロー
1. **認証**: LIFF → LINE認証 → Cloudflare Workers
2. **物品登録**: 写真撮影 → 圧縮・最適化 → R2アップロード → D1保存
3. **AI処理**: 画像 → Cloudflare AI → カテゴリ自動判定
4. **通知**: Cloudflare Cron → 処分提案 → LINE Bot → Push Message

## 💻 フロントエンド技術スタック

### 基盤技術
- **Framework**: React 19
- **Language**: TypeScript
- **Build Tool**: Vite
- **Package Manager**: npm

### UI/UXライブラリ
- **Component Library**: Radix UI
- **Styling**: Tailwind CSS v4
- **Icons**: Lucide React + 物品カテゴリ専用アイコン
- **Charts**: Recharts (統計表示)
- **Image**: Next.js Image component (画像最適化)

### 状態管理・データ
- **Server State**: TanStack Query
- **Client State**: Jotai
- **Forms**: React Hook Form + Zod validation
- **Image Processing**: Canvas API (リサイズ・圧縮)

### LIFF・カメラ統合
- **SDK**: @line/liff v2.x
- **Camera**: getUserMedia API
- **File Upload**: FormData with progress tracking
- **Image Preview**: Canvas-based image preview

### 開発・テスト
- **Testing**: Vitest + Testing Library + Playwright
- **Linting**: ESLint + Prettier
- **Type Checking**: TypeScript strict mode

## 🔧 バックエンド技術スタック

### API設計 (Hono + Cloudflare Workers)
```typescript
// src/index.ts
import { Hono } from 'hono'
import { cors } from 'hono/cors'
import { authMiddleware } from './middleware/auth'
import { itemsRoutes } from './routes/items'
import { locationsRoutes } from './routes/locations'
import { householdsRoutes } from './routes/households'
import { imagesRoutes } from './routes/images'
import { disposalRoutes } from './routes/disposal'

type Bindings = {
  DB: D1Database
  R2_BUCKET: R2Bucket
  AI: Ai
  LINE_CHANNEL_ACCESS_TOKEN: string
  LINE_CHANNEL_SECRET: string
  JWT_SECRET: string
  LIFF_ID: string
}

const app = new Hono<{ Bindings: Bindings }>()

// CORS設定
app.use('*', cors({
  origin: ['https://your-liff-app.vercel.app'],
  allowMethods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
  allowHeaders: ['Content-Type', 'Authorization']
}))

// ルート設定
app.route('/api/items', itemsRoutes)
app.route('/api/locations', locationsRoutes)
app.route('/api/households', householdsRoutes)
app.route('/api/images', imagesRoutes)
app.route('/api/disposal', disposalRoutes)

export default app
```

### データベース・ストレージ
- **Database**: Cloudflare D1 (SQLite)
- **Image Storage**: Cloudflare R2
- **ORM**: Drizzle ORM
- **Migration**: Drizzle Kit

### AI・画像処理
- **Image Recognition**: Cloudflare AI Workers
- **Image Optimization**: Cloudflare Images
- **OCR**: Tesseract.js (クライアントサイド)
- **Similarity Search**: Vector embeddings (将来実装)

### 認証・セキュリティ
- **LINE認証**: LINE Login API
- **JWT**: 自前JWT実装
- **画像権限**: 署名付きURL (R2 Presigned URLs)
- **レート制限**: Cloudflare Rate Limiting

### 通知・スケジューラ
- **LINE Bot API**: プッシュメッセージ
- **Cron Jobs**: Cloudflare Cron Triggers
- **バックグラウンド処理**: Cloudflare Queues

## 🗄️ データベース設計

### Drizzle ORM スキーマ
```typescript
// src/db/schema.ts
import { sqliteTable, text, integer, real, blob } from 'drizzle-orm/sqlite-core'
import { relations } from 'drizzle-orm'

// ユーザー・世帯管理
export const households = sqliteTable('households', {
  id: text('id').primaryKey(),
  name: text('name').notNull(),
  settings: text('settings'), // JSON
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull(),
  updatedAt: integer('updated_at', { mode: 'timestamp' }).notNull()
})

export const users = sqliteTable('users', {
  id: text('id').primaryKey(), // LINE User ID
  householdId: text('household_id').references(() => households.id),
  displayName: text('display_name').notNull(),
  pictureUrl: text('picture_url'),
  role: text('role').default('member'), // admin, member, viewer
  notificationEnabled: integer('notification_enabled', { mode: 'boolean' }).default(true),
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull(),
  updatedAt: integer('updated_at', { mode: 'timestamp' }).notNull()
})

// 場所管理
export const locations = sqliteTable('locations', {
  id: text('id').primaryKey(),
  householdId: text('household_id').notNull().references(() => households.id),
  name: text('name').notNull(),
  type: text('type').notNull(), // room, furniture, container
  parentId: text('parent_id').references(() => locations.id),
  description: text('description'),
  imageUrl: text('image_url'),
  capacity: text('capacity'), // JSON {width, height, depth, unit}
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull(),
  updatedAt: integer('updated_at', { mode: 'timestamp' }).notNull()
})

// 物品管理
export const items = sqliteTable('items', {
  id: text('id').primaryKey(),
  householdId: text('household_id').notNull().references(() => households.id),
  ownerId: text('owner_id').notNull().references(() => users.id),
  locationId: text('location_id').references(() => locations.id),
  name: text('name').notNull(),
  description: text('description'),
  category: text('category').notNull(),
  subcategory: text('subcategory'),
  tags: text('tags'), // JSON array
  condition: text('condition').default('good'), // excellent, good, fair, poor
  purchaseDate: integer('purchase_date', { mode: 'timestamp' }),
  purchasePrice: real('purchase_price'),
  lastUsedDate: integer('last_used_date', { mode: 'timestamp' }),
  usageFrequency: integer('usage_frequency').default(0), // per year
  isShared: integer('is_shared', { mode: 'boolean' }).default(false),
  isDisposed: integer('is_disposed', { mode: 'boolean' }).default(false),
  disposedAt: integer('disposed_at', { mode: 'timestamp' }),
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull(),
  updatedAt: integer('updated_at', { mode: 'timestamp' }).notNull()
})

// 画像管理
export const itemImages = sqliteTable('item_images', {
  id: text('id').primaryKey(),
  itemId: text('item_id').notNull().references(() => items.id, { onDelete: 'cascade' }),
  url: text('url').notNull(), // R2 URL
  thumbnailUrl: text('thumbnail_url').notNull(),
  alt: text('alt'),
  isPrimary: integer('is_primary', { mode: 'boolean' }).default(false),
  metadata: text('metadata'), // JSON {size, width, height, format}
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull()
})

// 使用履歴
export const usageLogs = sqliteTable('usage_logs', {
  id: text('id').primaryKey(),
  itemId: text('item_id').notNull().references(() => items.id, { onDelete: 'cascade' }),
  userId: text('user_id').notNull().references(() => users.id),
  action: text('action').notNull(), // used, moved, lent, borrowed
  notes: text('notes'),
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull()
})

// 処分提案
export const disposalSuggestions = sqliteTable('disposal_suggestions', {
  id: text('id').primaryKey(),
  itemId: text('item_id').notNull().references(() => items.id, { onDelete: 'cascade' }),
  reason: text('reason').notNull(), // unused, duplicate, damaged, outgrown, expired
  score: integer('score').notNull(), // 0-100
  lastUsedDays: integer('last_used_days').notNull(),
  suggestedAction: text('suggested_action').notNull(), // donate, sell, recycle, trash
  isRead: integer('is_read', { mode: 'boolean' }).default(false),
  isActioned: integer('is_actioned', { mode: 'boolean' }).default(false),
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull(),
  actionedAt: integer('actioned_at', { mode: 'timestamp' })
})
```

### リレーション定義
```typescript
export const householdsRelations = relations(households, ({ many }) => ({
  users: many(users),
  locations: many(locations),
  items: many(items)
}))

export const usersRelations = relations(users, ({ one, many }) => ({
  household: one(households, {
    fields: [users.householdId],
    references: [households.id]
  }),
  items: many(items),
  usageLogs: many(usageLogs)
}))

export const itemsRelations = relations(items, ({ one, many }) => ({
  household: one(households, {
    fields: [items.householdId],
    references: [households.id]
  }),
  owner: one(users, {
    fields: [items.ownerId],
    references: [users.id]
  }),
  location: one(locations, {
    fields: [items.locationId],
    references: [locations.id]
  }),
  images: many(itemImages),
  usageLogs: many(usageLogs),
  disposalSuggestions: many(disposalSuggestions)
}))
```

## 📸 画像処理・ストレージ

### 画像アップロードフロー
```typescript
// フロントエンド: 画像圧縮・アップロード
export async function uploadItemImage(file: File, itemId: string): Promise<ItemImage> {
  // 1. クライアントサイド圧縮
  const compressedFile = await compressImage(file, {
    maxWidth: 1200,
    maxHeight: 1200,
    quality: 0.8
  })
  
  // 2. サムネイル生成
  const thumbnail = await generateThumbnail(compressedFile, {
    width: 300,
    height: 300
  })
  
  // 3. アップロード用署名URL取得
  const { uploadUrl, imageId } = await apiClient.getUploadUrl(itemId)
  
  // 4. R2への直接アップロード
  await uploadToR2(uploadUrl, compressedFile)
  await uploadToR2(`${uploadUrl}_thumb`, thumbnail)
  
  // 5. データベース登録
  return apiClient.createItemImage({
    id: imageId,
    itemId,
    url: `https://r2.example.com/${imageId}`,
    thumbnailUrl: `https://r2.example.com/${imageId}_thumb`,
    metadata: {
      size: compressedFile.size,
      width: 1200,
      height: 1200,
      format: 'jpeg'
    }
  })
}
```

### Cloudflare R2 設定
```typescript
// バックエンド: 署名付きURL生成
export async function generateUploadUrl(itemId: string): Promise<UploadUrlResponse> {
  const imageId = generateId()
  const key = `items/${itemId}/${imageId}`
  
  const uploadUrl = await env.R2_BUCKET.createPresignedUrl('PUT', key, {
    expiresIn: 3600, // 1時間
    httpMetadata: {
      contentType: 'image/jpeg'
    }
  })
  
  return {
    uploadUrl,
    imageId,
    publicUrl: `https://your-domain.com/${key}`
  }
}
```

## 🤖 AI・画像認識

### Cloudflare AI 活用
```typescript
// 物品カテゴリ自動判定
export async function classifyItem(imageUrl: string, env: Env): Promise<ItemClassification> {
  const response = await env.AI.run('@cf/microsoft/resnet-50', {
    image: await fetchImageAsArrayBuffer(imageUrl)
  })
  
  // ResNet-50の結果をアプリ用カテゴリにマッピング
  const appCategory = mapToAppCategory(response.label)
  
  return {
    category: appCategory.primary,
    subcategory: appCategory.secondary,
    confidence: response.score,
    suggestedTags: generateTags(response.label)
  }
}

// カテゴリマッピング
function mapToAppCategory(aiLabel: string): CategoryMapping {
  const categoryMap = {
    'clothing': { primary: '衣類', secondary: '一般衣類' },
    'book': { primary: '本・メディア', secondary: '書籍' },
    'electronic device': { primary: '電子機器', secondary: 'その他電子機器' },
    'furniture': { primary: '家具', secondary: 'その他家具' },
    // ... more mappings
  }
  
  return categoryMap[aiLabel] || { primary: 'その他', secondary: null }
}
```

### 重複検出
```typescript
// 類似画像検出（将来実装）
export async function findSimilarItems(imageUrl: string): Promise<SimilarItem[]> {
  // 画像の特徴ベクトル抽出
  const features = await env.AI.run('@cf/baai/bge-base-en-v1.5', {
    text: await analyzeImageContent(imageUrl)
  })
  
  // ベクトル類似検索
  const similarItems = await vectorSearch(features.data[0], {
    threshold: 0.8,
    limit: 5
  })
  
  return similarItems
}
```

## 🔔 通知・自動化システム

### 処分提案ロジック
```typescript
// Cron Job: 処分候補分析
export async function analyzeDisposalCandidates(env: Env) {
  const db = drizzle(env.DB)
  
  // 長期未使用物品の抽出
  const unusedItems = await db
    .select()
    .from(items)
    .where(and(
      eq(items.isDisposed, false),
      lt(items.lastUsedDate, subMonths(new Date(), 6)) // 6ヶ月未使用
    ))
  
  for (const item of unusedItems) {
    const score = calculateDisposalScore(item)
    
    if (score >= 70) { // 閾値以上で提案
      await db.insert(disposalSuggestions).values({
        id: generateId(),
        itemId: item.id,
        reason: determineDisposalReason(item),
        score,
        lastUsedDays: daysSince(item.lastUsedDate),
        suggestedAction: suggestAction(item),
        createdAt: new Date()
      })
    }
  }
}

// 処分スコア計算
function calculateDisposalScore(item: Item): number {
  const daysSinceUsed = daysSince(item.lastUsedDate)
  const usageScore = Math.min(daysSinceUsed / 365 * 100, 100) // 1年で100点
  
  const conditionScore = {
    'poor': 30,
    'fair': 10,
    'good': 0,
    'excellent': -10
  }[item.condition] || 0
  
  const categoryScore = {
    '衣類': 0,
    '電子機器': 20, // 陳腐化しやすい
    '本・メディア': 10,
    'その他': 0
  }[item.category] || 0
  
  return Math.max(0, Math.min(100, usageScore + conditionScore + categoryScore))
}
```

### LINE Bot 通知
```typescript
// 処分提案通知
export async function sendDisposalNotifications(env: Env) {
  const recentSuggestions = await getUnreadDisposalSuggestions()
  
  for (const user of recentSuggestions) {
    const message = createDisposalSuggestionMessage(user.suggestions)
    
    await sendLineMessage(env.LINE_CHANNEL_ACCESS_TOKEN, user.lineUserId, message)
  }
}

function createDisposalSuggestionMessage(suggestions: DisposalSuggestion[]) {
  return {
    type: 'flex',
    altText: `🧹 ${suggestions.length}個のアイテムの整理を提案します`,
    contents: {
      type: 'bubble',
      header: {
        type: 'box',
        layout: 'vertical',
        contents: [
          {
            type: 'text',
            text: '🧹 整理の提案',
            weight: 'bold',
            size: 'lg',
            color: '#ffffff'
          }
        ],
        backgroundColor: '#2563eb'
      },
      body: {
        type: 'box',
        layout: 'vertical',
        contents: [
          {
            type: 'text',
            text: `${suggestions.length}個のアイテムを整理することをお勧めします`,
            wrap: true
          },
          ...suggestions.slice(0, 3).map(suggestion => ({
            type: 'box',
            layout: 'horizontal',
            contents: [
              {
                type: 'text',
                text: suggestion.itemName,
                flex: 1,
                size: 'sm'
              },
              {
                type: 'text',
                text: `${suggestion.lastUsedDays}日未使用`,
                size: 'xs',
                color: '#999999'
              }
            ],
            margin: 'md'
          }))
        ]
      },
      footer: {
        type: 'box',
        layout: 'vertical',
        contents: [
          {
            type: 'button',
            action: {
              type: 'uri',
              label: '📱 アプリで確認',
              uri: `https://liff.line.me/${env.LIFF_ID}/disposal`
            },
            style: 'primary'
          }
        ]
      }
    }
  }
}
```

## 🚀 パフォーマンス最適化

### フロントエンド最適化
```typescript
// 画像の遅延読み込み
export function ItemGrid({ items }: { items: Item[] }) {
  return (
    <VirtualizedGrid
      items={items}
      itemHeight={200}
      renderItem={({ item, isVisible }) => (
        <ItemCard 
          item={item}
          imageUrl={isVisible ? item.thumbnailUrl : undefined}
          onVisible={() => preloadImage(item.imageUrl)}
        />
      )}
    />
  )
}

// 画像プリロード
const imageCache = new Map<string, HTMLImageElement>()

export function preloadImage(url: string): Promise<void> {
  if (imageCache.has(url)) return Promise.resolve()
  
  return new Promise((resolve, reject) => {
    const img = new Image()
    img.onload = () => {
      imageCache.set(url, img)
      resolve()
    }
    img.onerror = reject
    img.src = url
  })
}
```

### バックエンド最適化
```typescript
// データベースクエリ最適化
export async function getItemsWithImages(householdId: string, options: QueryOptions) {
  const db = drizzle(env.DB)
  
  // 1クエリで必要なデータを取得
  const results = await db
    .select({
      item: items,
      primaryImage: itemImages,
      location: locations
    })
    .from(items)
    .leftJoin(itemImages, and(
      eq(itemImages.itemId, items.id),
      eq(itemImages.isPrimary, true)
    ))
    .leftJoin(locations, eq(locations.id, items.locationId))
    .where(eq(items.householdId, householdId))
    .limit(options.limit)
    .offset(options.offset)
  
  return results
}

// キャッシュ戦略
export class ItemService {
  private cache = new Map<string, CacheEntry>()
  
  async getItem(id: string): Promise<Item> {
    const cached = this.cache.get(id)
    if (cached && !this.isExpired(cached)) {
      return cached.data
    }
    
    const item = await this.fetchItem(id)
    this.cache.set(id, {
      data: item,
      timestamp: Date.now(),
      ttl: 5 * 60 * 1000 // 5分
    })
    
    return item
  }
}
```

## 🔒 セキュリティ・プライバシー

### 画像アクセス制御
```typescript
// 署名付きURL生成（時間制限付き）
export async function getSecureImageUrl(imageId: string, userId: string): Promise<string> {
  // アクセス権限チェック
  const hasAccess = await checkImageAccess(imageId, userId)
  if (!hasAccess) {
    throw new Error('Access denied')
  }
  
  // 時間制限付きURL生成（1時間有効）
  return await env.R2_BUCKET.createPresignedUrl('GET', `images/${imageId}`, {
    expiresIn: 3600
  })
}

// 家族間データ分離
export async function checkHouseholdAccess(userId: string, householdId: string): Promise<boolean> {
  const user = await getUserWithHousehold(userId)
  return user?.householdId === householdId
}
```

### データ暗号化
```typescript
// 機密情報の暗号化（購入価格など）
export function encryptSensitiveData(data: string): string {
  return encrypt(data, env.ENCRYPTION_KEY)
}

export function decryptSensitiveData(encryptedData: string): string {
  return decrypt(encryptedData, env.ENCRYPTION_KEY)
}
```

---

前のドキュメント: [[01_Requirements_Definition]]
次のドキュメント: [[03_Development_Plan]]