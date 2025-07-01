```typescript
type MyPick<T, K extends keyof T | string | number | symbol> = {
  [P in K]: P extends keyof T ? T[P] : never
}
```
まず、`T`型と、`T`のキーか`string`か`number`か`symbol`型に限定した`P`型を型引数として作成する
そして、`Conditional Types`を使って**条件による型変更**ができる機能を使う。**三項演算子を使って条件分岐して型変更**する

`never`型は、**発生しない値の型として、この型を使って存在しない型をないものとして扱う**ことができる
