# scls

Fork by https://github.com/estin/simple-completion-language-server

具体改动请参考 https://github.com/d1y/scls/pull/6 该 PR


| 路径提示      | 代码片段 |
| ----------- | ----------- |
| <img width="645" alt="image" src="https://github.com/d1y/scls/assets/45585937/a7c3211f-7fa8-4eac-9fe8-23d4943b25e3">      | <img width="780" alt="image" src="https://github.com/d1y/scls/assets/45585937/e02bc64f-4922-40c3-b040-fd643e871786">      |

### Install

Homebrew:

```sh
brew tap d1ylab/homebrew-tap
brew install scls
```

From GitHub:

```console
$ cargo install --git https://github.com/d1y/scls
```

From local repository:

```console
$ git clone https://github.com/d1y/scls
$ cd scls
$ cargo install --path .
```

### Configure

zed-editor:

```jsonc
"lsp": {
  "scls": {
    "initialization_options": {
      "max_completion_items": 6, // set max completion results len for each group: words, snippets, unicode-input
      "feature_words": false, // enable completion by word
      "feature_unicode_input": false, // enable "unicode input"
      "snippets_first": true, // completions will return before snippets by default
      "feature_snippets": true, // enable snippets
      "feature_paths": true // enable path completion
    }
  }
}
```

### Snippets

代码片段目录在 `~/.scls/snippets`, 示例(`~/.scls/snippets/go.toml`):

```toml
[[snippets]]
prefix = "err"
scope = [ "go" ]
body = "if err := $1; err != nil {\n\t$2\t\n}"
```

可以使用

```sh
simple-completion-language-server fetch-snippets
```

命令来自动拉取 `https://github.com/rafamadriz/friendly-snippets` 仓库代码片段(它会自动生成配置文件)