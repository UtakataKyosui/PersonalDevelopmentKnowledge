```typescript
type If<B extends boolean,T extends string,P extends string> = B extends true ? T : P
```