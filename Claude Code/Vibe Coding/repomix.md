LLMに渡すコードコンテキストを1ファイルにまとめてくれるツール

## なぜ使うのかについて

- `Vibe Coding`において、実装計画を作成するのは必須
- 実装計画の作成において、質の高いコンテキストをLLMに与えられるかが重要

## どう使うのか
```sh
repomix --include src/pages/index.astro,src/constants,package.json,
```

これを入力すると、`repomix-output.yml`というファイルが生成される