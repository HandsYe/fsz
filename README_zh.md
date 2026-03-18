# fsz

> 快速文件大小分析器 - 命令行磁盘使用分析工具

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**[English Documentation](README.md)**

## 功能特点

- ⚡ **快速** - 使用 Rust 编写，原生性能
- 📊 **目录分析** - 按目录深度分组显示
- 🎯 **大小过滤** - 按最小文件大小筛选
- 📋 **美观输出** - 漂亮的命令行格式化输出

## 安装

### 从源码安装

```bash
cargo install --git https://github.com/HandsYe/fsz.git
```

### 本地编译

```bash
git clone https://github.com/HandsYe/fsz.git
cd fsz
cargo build --release
```

## 使用方法

```bash
# 分析当前目录
fsz

# 分析指定路径
fsz /path/to/dir

# 按最小大小过滤
fsz -s 1M /path

# 按子目录分组 (深度 1)
fsz -d 1 /path

# 显示前 20 个
fsz -n 20 /path
```

## 命令行选项

| 选项 | 说明 | 默认值 |
|------|------|--------|
| `[PATH]` | 要分析的路径 | `.` |
| `-s, --min-size` | 最小文件大小 (如 `1K`, `1M`, `1G`) | `0` |
| `-d, --depth` | 目录分组深度 | `0` |
| `-n, --top` | 显示前 N 个项目 | `10` |
| `-h, --help` | 显示帮助 | - |
| `-V, --version` | 显示版本 | - |

## 使用示例

### 列出最大的文件

```bash
$ fsz -s 1M ~/Downloads

══════════════════════════════════════════════════════════════════
  📊 FSZ - 文件大小分析器
──────────────────────────────────────────────────────────────────
  路径：/home/user/Downloads
  最小：1.0 MB
  耗时：15.2ms
  文件：42
  目录：5
  总计：523.4 MB
══════════════════════════════════════════════════════════════════

  Top 10 最大文件
  ─────────────────────
    1.   256.0 MB   [FILE]   /home/user/Downloads/video.mp4
    2.   128.0 MB   [FILE]   /home/user/Downloads/archive.zip
    3.    64.0 MB   [FILE]   /home/user/Downloads/backup.tar
```

### 按子目录分组

```bash
$ fsz -d 1 -s 10M /home/user

══════════════════════════════════════════════════════════════════
  📊 FSZ - 文件大小分析器
──────────────────────────────────────────────────────────────────
  路径：/home/user
  最小：10.0 MB
  深度：1
  耗时：25.8ms
  文件：156
  目录：24
  总计：2.3 GB
══════════════════════════════════════════════════════════════════

  Top 10 目录 (深度 1)
  ─────────────────────────────
    1.     1.2 GB   [DIR 45]   /home/user/Videos
    2.   512.0 MB   [DIR 32]   /home/user/Documents
    3.   256.0 MB   [DIR 28]   /home/user/Pictures
```

### 常用命令

```bash
# 查找大于 100MB 的文件
fsz -s 100M /home

# 查看第一级目录大小
fsz -d 1 /home

# 查看前三级目录，显示前 20 个
fsz -d 3 -n 20 /home

# 分析当前目录，显示前 5 个
fsz -n 5
```

## 输出说明

- `[FILE]` - 单个文件
- `[DIR N]` - 目录，包含 N 个文件

## 技术栈

- **Rust** - 系统级编程语言，提供原生性能
- **walkdir** - 高效的目录遍历
- **clap** - 命令行参数解析

## 性能

- 二进制大小：~800KB
- 扫描速度：~500K 文件/秒

## 许可证

MIT 许可证 - 详见 [LICENSE](LICENSE) 文件
