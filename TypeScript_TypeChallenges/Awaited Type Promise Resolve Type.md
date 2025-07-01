```typescript
type MyAwaited<T> = T extends Promise<infer R> ? R : never
```