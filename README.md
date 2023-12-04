![snowlhive_banner](https://github.com/ystk-kai/snowlhive/assets/1179522/92dfff27-1a3a-48ed-b003-ec959f99c681)

# SnowlHive

「SnowlHive」は、ドキュメント情報のデータを一つのファイルに結合するプロジェクトです。

## What is SnowlHive?

Open AI の GPTs などで Knowledge として利用するドキュメントテキストを作成するのが目的です。

## Why SnowlHive?

- 「Snowl」はシロフクロウ（Snowly Owl）の造語で、知恵と美しい見た目を連想するように結合されたドキュメントファイルは綺麗な構造を目指します。
- 「Hive」は蜂の巣の様にドキュメント毎に区切られた（別ファイル）となるように、ドキュメントを分割して管理します。

## How to use SnowlHive?

SnowlHive は Docker イメージとして提供します。

簡単な利用方法としては、以下のようにディレクトリ内に入力ディレクトリを配置します。

```
.
└── input_dir
```

そして、以下のコマンドを実行します。

```bash
docker run --rm -v $(pwd):/work ystkkai/snowlhive /work/input_dir
```

入力ディレクトリと同じ階層にファイルが出力されます。

```
.
├── input_dir
└── input_dir.txt
```

## CLI Options

```bash
snowlhive [OPTIONS] INPUT_DIR
```

| Option               | Description                                                           | Default              |
| -------------------- | --------------------------------------------------------------------- | -------------------- |
| (input_dir)          | 使用する入力ディレクトリを指定します。                                | (必須)               |
| -d, --output-dir     | 出力先ディレクトリを指定します。                                      | ./output             |
| -x, --extensions     | 出力するファイルの拡張子をカンマ区切りで指定します。                  | _.md,_.mdx           |
| -e, --exclude        | 出力しないファイルのパターンを指定します。                            | (なし)               |
| -n, --output-name    | 出力ファイル名を指定します。                                          | (入力ディレクトリ名) |
| --verbose            | 詳細な出力を有効にします。                                            | false                |
| --remove-codeblock   | Markdown のコードブロックを削除します。                               | false                |
| --remove-url         | URL を削除します。                                                    | false                |
| --disabled-gitignore | .gitignore ファイルで指定されたファイルを無視する設定を無効化します。 | false                |
