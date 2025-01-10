# Translate CLI

一个使用 Rust 编写的命令行翻译工具，支持自动语言检测和多语言翻译。基于 Google Translate API 实现。

[English Documentation](./README.md)

## 功能特点

- 无需 API 密钥
- 自动语言检测
- 支持 100+ 种语言

## 安装

### 从源码安装

```bash
git clone https://github.com/yourusername/translate.git
cd translate
cargo install --path .
```

## 示例

```bash
# 英译中
translate "Hello, World!"
> 你好，世界！

# 中译英
translate "你好，世界！"
> Hello, World!

# 日译中
translate --source ja --target zh-CN "こんにちは世界"
> 你好世界

# 自动检测并翻译成德语
translate --target de "Hello World"
> Hallo Welt
```

### 命令行选项

```
选项:
  -s, --source <SOURCE>  源语言 (默认: auto)
  -t, --target <TARGET>  目标语言 (默认: auto)
  -l, --list            显示支持的语言列表
  -h, --help           显示帮助信息
  -V, --version        显示版本信息
```

### 常用语言代码

- `auto` - 自动检测
- `en` - English (英语)
- `zh-CN` - Chinese Simplified (简体中文)
- `ja` - Japanese (日语)
- `ko` - Korean (韩语)
- `fr` - French (法语)
- `de` - German (德语)

使用 `translate --list` 查看完整的语言列表。

## 注意事项

1. 该工具使用非官方的 Google Translate API，可能存在不稳定性
2. 为避免 IP 被限制，建议控制请求频率
3. 某些语言可能需要特定的字符编码支持
4. 自动检测功能可能不总是 100% 准确

## 贡献

欢迎提交 Issue 和 Pull Request！

## 致谢

- Google Translate API
- [clap-rs](https://github.com/clap-rs/clap) - 命令行解析
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP 客户端
- [alfred-translate-workflow](https://github.com/meshchaninov/alfred-translate-workflow)

## 许可证

MIT
