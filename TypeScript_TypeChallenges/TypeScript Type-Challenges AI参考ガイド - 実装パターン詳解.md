# TypeScript Type-Challenges AI参考ガイド - 実装パターン詳解

## 概要
このガイドは、TypeScript Type-Challengeの解答生成時に参考にすべき具体的な実装パターンと判断基準を提供します。各パターンには「いつ使うか」「なぜそのパターンか」「具体的な実装手順」を明記しています。

---

## 問題分類と解法選択フローチャート

### STEP 1: 問題の種類を判定

```
入力/出力の型を確認
├─ オブジェクト型 → プロパティ操作系
├─ 配列/タプル型 → 配列操作系  
├─ 文字列型 → 文字列操作系
├─ 関数型 → 関数操作系
└─ ユニオン型 → 条件分岐系
```

### STEP 2: 操作の種類を判定

```
何をしたいか？
├─ プロパティ抽出/除外 → Mapped Types + Key Manipulation
├─ 型の条件判定 → Conditional Types + infer
├─ 配列の要素操作 → Indexed Access + Template Literal
├─ 再帰的な処理 → Recursive Types
└─ 型の合成/分解 → Union/Intersection Types
```

---

## パターン1: プロパティ操作系

### 1.1 Pick系（プロパティ抽出）

**判定条件:** 「特定のプロパティだけを選択」「〜のみを含む」

**基本テンプレート:**
```typescript
type MyPick<T, K extends keyof T> = {
  [P in K]: T[P]
}
```

**実装の考え方:**
1. `K extends keyof T` で選択可能なキーに制限
2. `[P in K]` でKに含まれるキーのみループ
3. `T[P]` で元の型の値を保持

**派生パターン:**
```typescript
// PickByType: 値の型で選択
type PickByType<T, U> = {
  [K in keyof T as T[K] extends U ? K : never]: T[K]
}

// PickByValueType: より具体的な値型指定
type PickByValueType<T, U> = {
  [K in keyof T as T[K] extends U ? K : never]: T[K]
}
```

### 1.2 Omit系（プロパティ除外）

**判定条件:** 「特定のプロパティを除外」「〜以外」「〜を削除」

**基本テンプレート:**
```typescript
// 方法1: Pick + Exclude の組み合わせ（推奨）
type MyOmit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>

// 方法2: Key Remapping使用
type MyOmit<T, K extends keyof T> = {
  [P in keyof T as P extends K ? never : P]: T[P]
}
```

**使い分け:**
- 方法1: シンプルで理解しやすい、既存のユーティリティ型活用
- 方法2: より直接的、Key Remappingの理解が必要

### 1.3 Readonly系（読み取り専用化）

**判定条件:** 「読み取り専用」「不変」「readonly」

**基本テンプレート:**
```typescript
// 浅いReadonly
type MyReadonly<T> = {
  readonly [P in keyof T]: T[P]
}

// 深いReadonly（再帰的）
type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends Record<string, any> 
    ? DeepReadonly<T[P]> 
    : T[P]
}

// 部分的Readonly
type MyReadonly2<T, K extends keyof T = keyof T> = {
  [P in keyof T]: P extends K ? readonly T[P] : T[P]
}
```

---

## パターン2: 配列/タプル操作系

### 2.1 配列要素アクセス

**判定条件:** 「最初の要素」「最後の要素」「n番目の要素」

**First（最初の要素）:**
```typescript
type First<T extends readonly any[]> = T extends readonly [infer H, ...any] ? H : never

// 空配列対応版
type First<T extends readonly any[]> = T['length'] extends 0 ? never : T[0]
```

**Last（最後の要素）:**
```typescript
type Last<T extends readonly any[]> = T extends readonly [...any, infer L] ? L : never
```

**実装のポイント:**
- `readonly` 制約で配列の不変性を保証
- `infer` で要素を抽出
- 空配列の場合は `never` を返す

### 2.2 配列操作

**Push（要素追加）:**
```typescript
type Push<T extends readonly any[], U> = [...T, U]
```

**Pop（最後削除）:**
```typescript
type Pop<T extends readonly any[]> = T extends readonly [...infer R, any] ? R : []
```

**Shift（最初削除）:**
```typescript
type Shift<T extends readonly any[]> = T extends readonly [any, ...infer R] ? R : []
```

**Unshift（最初に追加）:**
```typescript
type Unshift<T extends readonly any[], U> = [U, ...T]
```

### 2.3 配列と型の変換

**TupleToObject:**
```typescript
type TupleToObject<T extends readonly PropertyKey[]> = {
  [K in T[number]]: K
}
```

**TupleToUnion:**
```typescript
type TupleToUnion<T extends readonly any[]> = T[number]
```

**Length（長さ取得）:**
```typescript
type Length<T extends readonly any[]> = T['length']
```

---

## パターン3: 条件型とinfer

### 3.1 関数型の操作

**判定条件:** 「関数の戻り値型」「引数の型」「関数か判定」

**ReturnType（戻り値型取得）:**
```typescript
type MyReturnType<T> = T extends (...args: any) => infer R ? R : never
```

**Parameters（引数型取得）:**
```typescript
type MyParameters<T> = T extends (...args: infer P) => any ? P : never
```

**関数への引数追加:**
```typescript
type AppendArgument<Fn, A> = Fn extends (...args: infer Args) => infer R 
  ? (...args: [...Args, A]) => R 
  : never
```

### 3.2 Promise型の操作

**Awaited（Promise解決型）:**
```typescript
type MyAwaited<T> = T extends Promise<infer U> 
  ? U extends Promise<any> 
    ? MyAwaited<U>  // 再帰的にPromiseを解決
    : U 
  : T
```

### 3.3 条件判定型

**If（条件分岐）:**
```typescript
type If<C extends boolean, T, F> = C extends true ? T : F
```

**IsNever（never判定）:**
```typescript
type IsNever<T> = [T] extends [never] ? true : false
```

**IsAny（any判定）:**
```typescript
type IsAny<T> = 0 extends (1 & T) ? true : false
```

---

## パターン4: 文字列操作系

### 4.1 文字列変換

**判定条件:** 「大文字化」「小文字化」「キャメルケース」「ケバブケース」

**Capitalize（最初を大文字）:**
```typescript
type MyCapitalize<S extends string> = S extends `${infer F}${infer R}` 
  ? `${Uppercase<F>}${R}` 
  : S
```

**KebabCase（ケバブケース変換）:**
```typescript
type KebabCase<S extends string> = S extends `${infer F}${infer R}`
  ? F extends Uppercase<F>
    ? F extends Lowercase<F>
      ? `${F}${KebabCase<R>}`
      : `-${Lowercase<F>}${KebabCase<R>}`
    : `${F}${KebabCase<R>}`
  : S
```

### 4.2 文字列検索・置換

**Replace（置換）:**
```typescript
type Replace<S extends string, From extends string, To extends string> = 
  From extends '' 
    ? S 
    : S extends `${infer Prefix}${From}${infer Suffix}` 
      ? `${Prefix}${To}${Suffix}` 
      : S
```

**ReplaceAll（全置換）:**
```typescript
type ReplaceAll<S extends string, From extends string, To extends string> = 
  From extends '' 
    ? S 
    : S extends `${infer L}${From}${infer R}` 
      ? `${L}${To}${ReplaceAll<R, From, To>}` 
      : S
```

**StartsWith/EndsWith:**
```typescript
type StartsWith<T extends string, U extends string> = T extends `${U}${any}` ? true : false
type EndsWith<T extends string, U extends string> = T extends `${any}${U}` ? true : false
```

### 4.3 文字列分解

**Split（分割）:**
```typescript
type Split<S extends string, SEP extends string> = 
  string extends S 
    ? string[] 
    : S extends `${infer H}${SEP}${infer T}` 
      ? [H, ...Split<T, SEP>] 
      : [S]
```

**StringToUnion（文字列を文字のユニオンに）:**
```typescript
type StringToUnion<T extends string> = T extends `${infer F}${infer R}` 
  ? F | StringToUnion<R> 
  : never
```

---

## パターン5: 再帰的型定義

### 5.1 再帰の基本パターン

**判定条件:** 「深い」「ネスト」「再帰的」「全ての〜」

**再帰の構造:**
```typescript
type RecursiveType<T> = T extends SomeCondition 
  ? RecursiveType<TransformedT>  // 再帰呼び出し
  : BaseCase                     // 終了条件
```

### 5.2 配列の再帰処理

**Flatten（配列のフラット化）:**
```typescript
type Flatten<T extends any[]> = T extends [infer H, ...infer R]
  ? H extends any[]
    ? [...Flatten<H>, ...Flatten<R>]
    : [H, ...Flatten<R>]
  : []
```

**FlattenDepth（指定深度のフラット化）:**
```typescript
type FlattenDepth<T extends any[], N extends number = 1, C extends any[] = []> = 
  C['length'] extends N 
    ? T 
    : T extends [infer H, ...infer R]
      ? H extends any[]
        ? [...FlattenDepth<H, N, [...C, any]>, ...FlattenDepth<R, N, C>]
        : [H, ...FlattenDepth<R, N, C>]
      : []
```

### 5.3 オブジェクトの再帰処理

**DeepReadonly:**
```typescript
type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends Record<string | number | symbol, any> 
    ? DeepReadonly<T[P]> 
    : T[P]
}
```

**DeepRequired:**
```typescript
type DeepRequired<T> = {
  [P in keyof T]-?: T[P] extends Record<string | number | symbol, any> 
    ? DeepRequired<T[P]> 
    : T[P]
}
```

---

## パターン6: 高度な型操作

### 6.1 Union型の操作

**判定条件:** 「ユニオン型」「各型に対して」「分散」

**UnionToIntersection:**
```typescript
type UnionToIntersection<U> = (U extends any ? (k: U) => void : never) extends (k: infer I) => void ? I : never
```

**UnionToTuple:**
```typescript
type UnionToTuple<T> = UnionToIntersection<T extends any ? () => T : never> extends () => infer R ? [R] : []
```

### 6.2 Key Remapping

**GetKeys（特定の条件のキーのみ取得）:**
```typescript
type GetKeys<T, U> = {
  [K in keyof T]: T[K] extends U ? K : never
}[keyof T]
```

**Getters生成:**
```typescript
type Getters<T> = {
  [K in keyof T as `get${Capitalize<string & K>}`]: () => T[K]
}
```

### 6.3 Helper Types

**よく使用されるHelper Types:**
```typescript
// 型をフラット化（交差型を単純なオブジェクト型に）
type Flatten<T> = { [K in keyof T]: T[K] }

// 型をコピー（型エイリアスの作成）
type Copy<T> = { [K in keyof T]: T[K] }

// Optional/Required の条件付き適用
type ConditionalPick<T, K extends keyof T> = Pick<T, K>
type ConditionalOmit<T, K extends keyof T> = Omit<T, K>
```

---

## 実装判断フローチャート

### 問題を見たときの判断手順

```
1. 問題文のキーワードを確認
   ├─ "pick", "select", "extract" → Pick系
   ├─ "omit", "exclude", "remove" → Omit系
   ├─ "readonly", "immutable" → Readonly系
   ├─ "first", "last", "head", "tail" → 配列アクセス系
   ├─ "replace", "substitute" → 文字列置換系
   ├─ "flatten", "deep" → 再帰系
   └─ "return type", "parameters" → 関数型系

2. 入力と出力の型の関係を確認
   ├─ オブジェクト → オブジェクト: Mapped Types
   ├─ 配列 → 配列: 配列操作
   ├─ 配列 → 単一型: Indexed Access
   ├─ 型 → boolean: 条件判定
   └─ 複合型 → 分解: infer使用

3. 実装方針を決定
   ├─ 単純変換: 基本パターン適用
   ├─ 条件付き変換: Conditional Types
   ├─ 再帰必要: 再帰パターン
   └─ 複合操作: 複数パターン組み合わせ
```

---

## デバッグとテスト手法

### TypeScript Playgroundでの確認方法

```typescript
// 途中結果の確認
type Step1<T> = keyof T  // ^? をつけて型を確認
type Step2<T> = Pick<T, keyof T>  // ^?
type FinalResult<T> = MyType<T>  // ^?

// テストケースの作成
type TestCase1 = MyType<{ a: string; b: number }>  // 期待する結果を確認
type TestCase2 = MyType<[]>  // エッジケースの確認
```

### よくあるエラーとその対処

```typescript
// エラー: Type 'K' cannot be used to index type 'T'
// 対処: K extends keyof T 制約を追加

// エラー: Type instantiation is excessively deep
// 対処: 再帰の終了条件を確認、型の複雑さを軽減

// エラー: A mapped type may not declare properties or methods
// 対処: Mapped Typesの構文を確認
```

---

## 実践例: 完全な問題解決手順

### 例題: RequiredByKeys の実装

**問題:** 指定されたキーのみを必須にする型を実装せよ

**STEP 1: 問題分析**
- 入力: オブジェクト型T、キーの集合K
- 出力: Kに含まれるプロパティが必須、それ以外は元のまま
- パターン: プロパティ操作系 + 条件付き変換

**STEP 2: 実装方針**
1. Kに含まれるプロパティを必須にしたオブジェクト作成
2. Kに含まれないプロパティはそのまま
3. 交差型で結合

**STEP 3: 実装**
```typescript
type RequiredByKeys<T, K extends keyof T = keyof T> = 
  // Helper type for flattening
  { [P in keyof T]: P extends K ? T[P] : T[P] } & 
  // Make K properties required
  { [P in K]-?: T[P] }

// よりクリーンな実装
type RequiredByKeys<T, K extends keyof T = keyof T> = 
  Flatten<
    { [P in keyof T as P extends K ? never : P]: T[P] } &
    { [P in K]-?: T[P] }
  >

// Helper type
type Flatten<T> = { [K in keyof T]: T[K] }
```

**STEP 4: テスト**
```typescript
interface User {
  name?: string
  age?: number
  address?: string
}

type Result = RequiredByKeys<User, 'name' | 'age'>
// Result: { name: string; age: number; address?: string }
```

---

このガイドを参考に、TypeScript Type-Challengeの問題を体系的かつ効率的に解決できます。各パターンは実際の問題で頻繁に使用されるため、習得することで多くの問題に対応可能になります。