# Translate CLI

A command-line translation tool written in Rust that supports automatic language detection and multi-language translation. Based on Google Translate API.

[中文文档](./README_zh.md)

## Features

- No API key required
- Automatic language detection
- Support for 100+ languages

## Installation

### From Source

```bash
git clone https://github.com/yourusername/translate.git
cd translate
cargo install --path .
```

## Examples

```bash
# English to Chinese
translate "Hello, World!"
> 你好，世界！

# Chinese to English
translate "你好，世界！"
> Hello, World!

# Japanese to Chinese
translate --source ja --target zh-CN "こんにちは世界"
> 你好世界

# Auto-detect to German
translate --target de "Hello World"
> Hallo Welt
```

### Command-line Options

```
Options:
  -s, --source <SOURCE>  Source language (default: auto)
  -t, --target <TARGET>  Target language (default: auto)
  -l, --list            Show supported languages
  -h, --help           Show help information
  -V, --version        Show version information
```

### Common Language Codes

- `auto` - Auto detect
- `en` - English
- `zh-CN` - Chinese Simplified
- `ja` - Japanese
- `ko` - Korean
- `fr` - French
- `de` - German

Use `translate --list` to see the complete language list.

## Notes

1. This tool uses an unofficial Google Translate API and may have stability issues
2. To avoid IP restrictions, please control request frequency
3. Some languages may require specific character encoding support
4. Auto-detection may not always be 100% accurate

## Contributing

Issues and Pull Requests are welcome!

## Credits

- Google Translate API
- [clap-rs](https://github.com/clap-rs/clap) - Command line argument parser
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP Client
- [alfred-translate-workflow](https://github.com/meshchaninov/alfred-translate-workflow)

## License

MIT
