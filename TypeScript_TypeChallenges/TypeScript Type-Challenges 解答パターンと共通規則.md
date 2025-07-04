# TypeScript Type-Challenges 解答パターンと共通規則

## 概要

TypeScript Type-Challengesは、TypeScriptの型システムのみを使用して解決する課題集です。ランタイムコードは使用せず、型レベルでの操作のみを行います。

## 難易度レベル

- **Warm Up**: 基本的な型定義の理解
- **Easy**: 基本的なユーティリティ型の実装
- **Medium**: 複雑な型操作と条件分岐
- **Hard**: 高度な型システムの機能を活用
- **Extreme**: 最も複雑な型システムの問題

## 共通する解答パターン

### 1. Mapped Types（マップ型）

**基本パターン:**
```typescript
type MyType<T> = {
  [K in keyof T]: T[K]
}
```

**応用例 - Pick:**
```typescript
type MyPick<T, K extends keyof T> = {
  [P in K]: T[P]
}
```

**応用例 - Readonly:**
```typescript
type MyReadonly<T> = {
  readonly [P in keyof T]: T[P]
}
```

### 2. Conditional Types（条件型）

**基本パターン:**
```typescript
type MyType<T> = T extends SomeType ? TrueType : FalseType
```

**応用例 - If:**
```typescript
type If<C extends boolean, T, F> = C extends true ? T : F
```

**応用例 - First:**
```typescript
type First<T extends any[]> = T extends [infer H, ...any] ? H : never
```

### 3. infer キーワード

**関数の戻り値型を取得:**
```typescript
type MyReturnType<T> = T extends (...args: any) => infer R ? R : never
```

**配列の最初の要素を取得:**
```typescript
type First<T extends any[]> = T extends [infer H, ...any] ? H : never
```

**配列の最後の要素を取得:**
```typescript
type Last<T extends any[]> = T extends [...any, infer L] ? L : never
```

### 4. Template Literal Types（テンプレートリテラル型）

**基本パターン:**
```typescript
type Template<T extends string> = `prefix_${T}_suffix`
```

**文字列の置換:**
```typescript
type Replace<S extends string, From extends string, To extends string> = 
  S extends `${infer Prefix}${From}${infer Suffix}` 
    ? `${Prefix}${To}${Suffix}` 
    : S
```

**文字列の分割:**
```typescript
type Split<S extends string, SEP extends string> = 
  S extends `${infer H}${SEP}${infer T}` 
    ? [H, ...Split<T, SEP>] 
    : [S]
```

### 5. Indexed Access Types（インデックスアクセス型）

**配列からユニオン型を作成:**
```typescript
type TupleToUnion<T extends readonly any[]> = T[number]
```

**TupleToObject:**
```typescript
type TupleToObject<T extends readonly PropertyKey[]> = {
  [K in T[number]]: K
}
```

**配列の長さを取得:**
```typescript
type Length<T extends readonly any[]> = T['length']
```

### 6. 再帰的な型定義

**DeepReadonly:**
```typescript
type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends Record<string, any> 
    ? DeepReadonly<T[P]> 
    : T[P]
}
```

**Flatten（配列のフラット化）:**
```typescript
type Flatten<T extends any[]> = T extends [infer H, ...infer R]
  ? H extends any[]
    ? [...Flatten<H>, ...Flatten<R>]
    : [H, ...Flatten<R>]
  : []
```

## 重要な制約とキーワード

### extends 制約

```typescript
// 基本的な制約
type MyType<T extends string> = T

// keyof を使用した制約
type MyPick<T, K extends keyof T> = {
  [P in K]: T[P]
}

// readonly配列の制約
type MyType<T extends readonly any[]> = T['length']

// PropertyKey の制約（string | number | symbol）
type TupleToObject<T extends readonly PropertyKey[]> = {
  [K in T[number]]: K
}
```

### never 型の活用

```typescript
// 条件に合わない場合は never を返す
type Exclude<T, U> = T extends U ? never : T

// Key Remapping で特定のプロパティを除外
type MyOmit<T, K extends keyof T> = {
  [P in keyof T as P extends K ? never : P]: T[P]
}
```

## よく使用される組み合わせパターン

### 1. Pick + Exclude の組み合わせ（Omit の実装）

```typescript
type MyOmit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>
```

### 2. Mapped Types + Conditional Types

```typescript
type PickByType<T, U> = {
  [K in keyof T as T[K] extends U ? K : never]: T[K]
}
```

### 3. Template Literal + infer

```typescript
type Capitalize<S extends string> = S extends `${infer F}${infer R}` 
  ? `${Uppercase<F>}${R}` 
  : S
```

## 解答時の考え方

### 1. 段階的アプローチ
1. 入力型と期待される出力型を明確にする
2. 必要な型操作を特定する
3. 基本的なパターンから始めて徐々に複雑にする

### 2. デバッグ手法
- `// ^?` コメントでTypeScript Playgroundで型をチェック
- 段階的に型を構築して途中結果を確認
- テストケースを参考に期待される動作を理解

### 3. よくある失敗パターン
- `extends` 制約を忘れる
- `infer` の位置を間違える
- 再帰の終了条件を考慮しない
- Template Literal Typesの構文エラー

## 高度なテクニック

### 1. Distributive Conditional Types

```typescript
// T がユニオン型の場合、各メンバーに対して条件型が適用される
type ToArray<T> = T extends any ? T[] : never
type Result = ToArray<string | number> // string[] | number[]
```

### 2. Key Remapping

```typescript
type Getters<T> = {
  [K in keyof T as `get${Capitalize<string & K>}`]: () => T[K]
}
```

### 3. Helper Types の活用

```typescript
// 多くの解答で共通して使用される
type Flatten<T> = { [K in keyof T]: T[K] }
type Copy<T> = { [K in keyof T]: T[K] }
```

## 学習の進め方

1. **Easy レベルから開始**: 基本的なパターンを理解
2. **解答例を参考にする**: 複数の解法を比較検討
3. **段階的に複雑化**: Medium → Hard → Extreme の順に進む
4. **実際のコードで応用**: 学んだパターンを実際のプロジェクトで活用

## 参考リソース

- [Type Challenges GitHub](https://github.com/type-challenges/type-challenges)
- [Type Challenges Solutions](https://ghaiklor.github.io/type-challenges-solutions/en/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [TypeScript Playground](https://www.typescriptlang.org/play)

---

このガイドは TypeScript の型システムをより深く理解し、複雑な型操作を習得するための実践的な参考資料として活用してください。