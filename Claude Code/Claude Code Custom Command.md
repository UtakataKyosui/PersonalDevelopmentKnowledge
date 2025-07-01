## Custom Commandの作成
```bash
mkdir -p .claude/commands # カスタムコマンド設定用のリポジトリを作成
echo "Analyze the performance of this code and suggest three specific optimizations:" > .claude/commands/optimize.md
# 最適化を指示するコマンドを作成する
```

## 引数を与えられるCustom Commandの作成
```bash
echo "Find and fix issue #$ARGUMENTS. Follow these steps: 1.
Understand the issue described in the ticket 2. Locate the relevant code in
our codebase 3. Implement a solution that addresses the root cause 4. Add
appropriate tests 5. Prepare a concise PR description" >
.claude/commands/fix-issue.md

/project:fix-issue 123
```

`$ARGUMENTS`を設定すると、引数が渡せる
