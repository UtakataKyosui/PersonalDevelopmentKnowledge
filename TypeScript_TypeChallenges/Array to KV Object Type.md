```typescript
const tuple = ['tesla', 'model 3', 'model X', 'model Y'] as const

type TupleToObject<T extends readonly any[]> = {
  [K in T[number]]: K
}
```

型引数`T`は、読み取りのみ可能（`readonly`）な`any`型の配列であることを確定する
`[K in T[number]]`から、配列内の文字列を展開しユニオン型にして、K`に代入。それをキーと値として両方に使用