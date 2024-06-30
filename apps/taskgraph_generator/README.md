1. `src` 以下に `dependencygraph.json` という名前でファイルを配置
1. `input.txt` に以下の内容を記述

```
${修正対象のファイル数}
${ファイル1のパス} ${ファイル2のパス} ...
${修正対象のファイルの修正工数} ${依存関係で見つかったファイルの修正工数}
```

1. `cat input.txt | cargo run -- --quiet >> component.taskgraph.json`
