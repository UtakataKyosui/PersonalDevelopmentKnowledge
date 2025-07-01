# 要件定義 - LIFF物品管理アプリ

## 🎯 ユーザーストーリー

### 基本的な物品管理
> 「家にある物を写真付きで記録して、いつでも確認できるようにしたい」

- ユーザーとして、物品の写真を撮影して登録できる
- ユーザーとして、物品の名前・カテゴリ・保管場所を記録できる
- ユーザーとして、登録した物品を一覧で確認できる
- ユーザーとして、物品の詳細情報を編集・削除できる

### 場所・収納管理
> 「どの部屋のどの収納場所に何があるかを把握したい」

- ユーザーとして、部屋と収納場所を階層的に管理できる
- ユーザーとして、物品を具体的な保管場所に関連付けできる
- ユーザーとして、場所別に物品を一覧表示できる
- ユーザーとして、収納場所の使用状況を確認できる

### 検索・発見
> 「欲しい物がどこにあるかをすぐに見つけたい」

- ユーザーとして、物品名で検索できる
- ユーザーとして、カテゴリ・場所・タグで絞り込み検索できる
- ユーザーとして、写真から視覚的に物品を探せる
- ユーザーとして、最近使用した物品を確認できる

### 処分判断支援
> 「どの物を処分すべきかを科学的に判断したい」

- ユーザーとして、物品の使用頻度を記録できる
- ユーザーとして、最後に使用した日を確認できる
- ユーザーとして、処分候補の物品を提案される
- ユーザーとして、処分の理由と根拠を確認できる

### 家族共有
> 「家族全員で物品情報を共有したい」

- ユーザーとして、家族メンバーを招待できる
- ユーザーとして、家族の誰が何を登録したかを確認できる
- ユーザーとして、家族メンバーの変更を通知で受け取れる
- ユーザーとして、物品の貸し借り状況を管理できる

## 📋 機能要件

### 1. 物品管理機能
- [ ] 物品登録（写真、名前、説明、カテゴリ、タグ）
- [ ] 物品編集・削除
- [ ] 物品一覧表示（グリッド・リスト切り替え）
- [ ] 物品詳細表示
- [ ] 複数写真対応（角度違い、使用状況など）
- [ ] バーコード読み取り（将来機能）

### 2. 場所・収納管理機能
- [ ] 部屋の登録・管理
- [ ] 収納場所の階層管理（部屋 > 家具 > 引き出し）
- [ ] 場所別物品一覧
- [ ] 収納容量管理
- [ ] 場所の写真登録
- [ ] 3D間取り表示（将来機能）

### 3. 検索・フィルタ機能
- [ ] テキスト検索（部分一致、曖昧検索）
- [ ] カテゴリ別絞り込み
- [ ] 場所別絞り込み
- [ ] タグ検索
- [ ] 日付範囲検索（登録日、最終使用日）
- [ ] 画像類似検索（AI活用）

### 4. 処分判断機能
- [ ] 使用頻度記録
- [ ] 最終使用日追跡
- [ ] 処分候補自動提案
- [ ] 処分理由表示（未使用期間、重複など）
- [ ] 処分予定リスト
- [ ] 処分実行記録

### 5. 家族共有機能
- [ ] 家族グループ作成・管理
- [ ] メンバー招待（LINEアカウント連携）
- [ ] 権限管理（閲覧のみ、編集可能）
- [ ] 変更通知（新規登録、編集、削除）
- [ ] 貸し借り管理
- [ ] 個人・共有物品の区別

### 6. 統計・分析機能
- [ ] 物品数統計（カテゴリ別、場所別）
- [ ] 処分実績統計
- [ ] 新規登録推移
- [ ] 使用頻度分析
- [ ] 収納効率分析
- [ ] 月次・年次レポート

### 7. 通知機能
- [ ] 処分提案通知
- [ ] 長期未使用物品通知
- [ ] 家族メンバーの活動通知
- [ ] 定期整理リマインダー
- [ ] 季節物品の出し入れ提案

## 🔧 非機能要件

### パフォーマンス
- [ ] 3秒以内の画面表示
- [ ] 1秒以内の検索結果表示
- [ ] 画像アップロード進捗表示
- [ ] オフライン対応（基本機能のみ）
- [ ] 画像の自動圧縮・最適化

### セキュリティ
- [ ] LINE認証による安全なログイン
- [ ] 画像の適切な権限管理
- [ ] 家族間データの分離
- [ ] HTTPS通信の必須化
- [ ] 個人情報の暗号化

### ユーザビリティ
- [ ] モバイルファーストデザイン
- [ ] 直感的な写真撮影・アップロード
- [ ] ワンタップでの物品登録
- [ ] 音声入力対応
- [ ] アクセシビリティ対応

### 互換性
- [ ] iOS Safari対応
- [ ] Android Chrome対応
- [ ] LINE内ブラウザ対応
- [ ] カメラ機能対応
- [ ] 画像ファイル形式対応（JPEG、PNG、WebP）

## 📊 データ要件

### 物品データ
```typescript
interface Item {
  id: string
  name: string              // 物品名 (例: "黒いワンピース")
  description?: string      // 説明 (例: "フォーマル用、結婚式で着用")
  category: string          // カテゴリ (例: "衣類")
  subcategory?: string      // サブカテゴリ (例: "ワンピース")
  tags: string[]           // タグ (例: ["フォーマル", "黒", "Lサイズ"])
  images: ItemImage[]      // 写真リスト
  location: Location       // 保管場所
  purchaseDate?: Date      // 購入日
  purchasePrice?: number   // 購入価格
  lastUsedDate?: Date      // 最終使用日
  usageFrequency: number   // 使用頻度（年間回数）
  condition: 'excellent' | 'good' | 'fair' | 'poor'  // 状態
  isShared: boolean        // 家族共有物かどうか
  ownerId: string          // 所有者ID
  createdAt: Date
  updatedAt: Date
}
```

### 画像データ
```typescript
interface ItemImage {
  id: string
  itemId: string
  url: string              // Cloudflare R2 URL
  thumbnailUrl: string     // サムネイル URL
  alt: string             // 代替テキスト
  isPrimary: boolean      // メイン画像かどうか
  metadata: {
    size: number          // ファイルサイズ
    width: number         // 幅
    height: number        // 高さ
    format: string        // ファイル形式
  }
  createdAt: Date
}
```

### 場所データ
```typescript
interface Location {
  id: string
  name: string            // 場所名 (例: "リビング クローゼット 上段")
  type: 'room' | 'furniture' | 'container'
  parentId?: string       // 親場所ID（階層構造）
  description?: string    // 説明
  image?: string         // 場所の写真
  capacity?: {           // 収納容量
    width: number
    height: number  
    depth: number
    unit: 'cm' | 'inch'
  }
  householdId: string    // 世帯ID
  createdAt: Date
  updatedAt: Date
}
```

### 家族・世帯データ
```typescript
interface Household {
  id: string
  name: string           // 世帯名 (例: "田中家")
  members: HouseholdMember[]
  settings: {
    autoDisposeThreshold: number    // 自動処分提案の閾値（月）
    shareByDefault: boolean         // デフォルトで家族共有
    notificationEnabled: boolean    // 通知設定
  }
  createdAt: Date
  updatedAt: Date
}

interface HouseholdMember {
  userId: string         // LINE User ID
  role: 'admin' | 'member' | 'viewer'
  displayName: string
  pictureUrl?: string
  joinedAt: Date
}
```

### 使用履歴データ
```typescript
interface UsageLog {
  id: string
  itemId: string
  userId: string
  action: 'used' | 'moved' | 'lent' | 'borrowed'
  notes?: string
  createdAt: Date
}
```

### 処分データ
```typescript
interface DisposalSuggestion {
  id: string
  itemId: string
  reason: 'unused' | 'duplicate' | 'damaged' | 'outgrown' | 'expired'
  score: number          // 処分スコア (0-100)
  lastUsedDays: number   // 最終使用からの日数
  suggestedAction: 'donate' | 'sell' | 'recycle' | 'trash'
  createdAt: Date
  disposedAt?: Date      // 実際に処分した日
}
```

## 🎨 UI/UX要件

### デザインシステム
- **カラーパレット**: 清潔感のあるブルー・グリーン系
- **タイポグラフィ**: 読みやすく現代的なフォント
- **アイコン**: 物品カテゴリに対応した豊富なアイコンセット
- **レスポンシブ**: モバイルファーストのレスポンシブデザイン

### 画面構成
1. **ダッシュボード**: 物品統計、最近の活動、処分提案
2. **物品一覧**: グリッド・リスト表示、検索・フィルタ
3. **物品詳細**: 写真、情報、使用履歴、関連物品
4. **物品登録**: カメラ撮影、情報入力、場所選択
5. **場所管理**: 階層構造表示、場所別物品一覧
6. **検索**: 高度な検索・フィルタ機能
7. **処分管理**: 処分候補一覧、処分理由、実行記録
8. **家族管理**: メンバー一覧、権限設定、招待
9. **統計**: 各種分析グラフ、レポート表示
10. **設定**: 通知設定、プライバシー設定

### インタラクション
- **写真撮影**: ワンタップでカメラ起動・撮影
- **物品登録**: 写真撮影後の流れるような登録フロー
- **検索**: リアルタイム検索候補表示
- **スワイプ操作**: 物品一覧でのスワイプアクション
- **長押し**: 複数選択・一括操作
- **ドラッグ&ドロップ**: 場所移動、並び替え

### アクセシビリティ
- **音声読み上げ**: スクリーンリーダー対応
- **大きな文字**: 文字サイズ調整対応
- **高コントラスト**: 色覚多様性対応
- **音声入力**: 物品名・説明の音声入力
- **簡単モード**: 高齢者向けの簡素化UI

## 🔗 外部連携要件

### LINE連携
- **認証**: LINE Login API
- **通知**: LINE Bot API（プッシュメッセージ）
- **共有**: LINE Share API（物品情報の共有）

### 画像処理・AI
- **画像圧縮**: Cloudflare Images
- **物品認識**: Cloudflare AI（物品カテゴリの自動判定）
- **OCR**: テキスト読み取り（レシート、ラベル）
- **類似画像検索**: 重複物品の検出

### 外部サービス（将来）
- **バーコードAPI**: 商品情報の自動取得
- **価格API**: 中古価格・相場情報
- **リサイクル業者API**: 処分・買取サービス連携
- **EC連携**: 購入履歴の自動インポート

---

前のドキュメント: [[00_PROJECT-OVERVIEW]]
次のドキュメント: [[02_Technical_Specification]]