```typescript
// Push型を定義してください
// 例えば、Push<[1, 2], '3'>は[1, 2, '3']を返します

type Result = Push<[1, 2], "3">; // [1, 2, '3']

type Push<A extends any[],P extends any> = [...A,P]
```