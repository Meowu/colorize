# colorize

`colorize` 是一个简单且高效的命令行颜色转换工具。它支持多种颜色格式互相转换，包括 RGB、RGBA、HEX 和 HSL。

## 功能描述

- 支持从 RGB、RGBA、HEX 和 HSL 格式的字符串解析颜色。
- 将解析后的颜色输出为 RGB、RGBA、HEX 和 HSL 格式。

## 使用方法

首先，确保你已经安装了 Rust 开发环境。

1. 克隆项目：

   ```bash
   git clone git@github.com:Meowu/colorize.git
   cd colorize
   ```

2. 运行项目：

   你可以通过以下命令运行程序：

   ```bash
   cargo run -- "<color>"
   ```

   其中 `<color>` 可以是以下格式之一：

   - HEX： `#RRGGBB` 或 `#RRGGBBAA`
   - RGB： `rgb(r, g, b)`，例 `rgb(255, 0, 0)`
   - RGBA： `rgba(r, g, b, a)`，例 `rgba(255, 0, 0, 0.5)`
   - HSL： `hsl(h, s%, l%)`，例 `hsl(120, 100%, 50%)`

3. 示例：

   ```bash
   cargo run -- "#FF5733"
   ```

## 贡献

欢迎提交 Bug 报告和功能请求。如果您想要贡献代码，请 Fork 项目并发送 Pull Request。
