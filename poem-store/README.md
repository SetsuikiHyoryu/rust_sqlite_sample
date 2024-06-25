# Rust 使用 SQLx 异步操作 SQLite 数据库

## 使用方式

下文默认使用者已安装 Rust 与 SQLite。  
未安装者可从 GitHub release 页下载生成后的可执行文件（EXE 文件）尝试。

使用可执行文件请将 `cargo run --` 替换为：

- Command Prompt (CMD): `poem-store`
- PowerShell / Git Bash: `./poem-store.exe`

### 查看帮助

```shell
# 注意，如果缺少不带任何后缀的 `--`，会变成传参数给 `cargo` 而不是我们自己的程序。
cargo run -- --help
```

### 列出所有诗词

```shell
cargo run
```

### 按标题查找诗词

```shell
cargo run -- --title <TITLE>
```

### 添加诗词

```shell
cargo run -- add <TITLE> <AUTHOR> <BODY>
```

### 按编号或标题删除诗词

```shell
# 使用编号删除，编号为整数数字。
cargo run -- delete --id <ID>

# 使用标题删除。
cargo run -- delete --title <TITLE>
```
