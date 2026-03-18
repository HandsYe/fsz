# fsz

> Fast file size analyzer - CLI tool to analyze disk usage

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**[中文文档](README_zh.md)**

## Features

- ⚡ **Fast** - Built with Rust for native performance
- 📊 **Directory analysis** - Group files by directory depth
- 🎯 **Size filtering** - Filter by minimum file size
- 📋 **Clean output** - Beautiful formatted CLI output

## Installation

### From Source

```bash
cargo install --git https://github.com/HandsYe/fsz.git
```

### Build Locally

```bash
git clone https://github.com/HandsYe/fsz.git
cd fsz
cargo build --release
```

## Usage

```bash
# Analyze current directory
fsz

# Analyze specific path
fsz /path/to/dir

# Filter by minimum size
fsz -s 1M /path

# Group by subdirectory (depth 1)
fsz -d 1 /path

# Show top 20 items
fsz -n 20 /path
```

## Options

| Option | Description | Default |
|--------|-------------|---------|
| `[PATH]` | Path to analyze | `.` |
| `-s, --min-size` | Minimum file size (e.g., `1K`, `1M`, `1G`) | `0` |
| `-d, --depth` | Directory depth for grouping | `0` |
| `-n, --top` | Number of top items to show | `10` |
| `-h, --help` | Show help | - |
| `-V, --version` | Show version | - |

## Examples

### List largest files

```bash
$ fsz -s 1M ~/Downloads

══════════════════════════════════════════════════════════════════
  📊 FSZ - File Size Analyzer
──────────────────────────────────────────────────────────────────
  Path:      /home/user/Downloads
  Min size:  1.0 MB
  Scan time: 15.2ms
  Files:     42
  Dirs:      5
  Total:     523.4 MB
══════════════════════════════════════════════════════════════════

  Top 10 Largest Files
  ─────────────────────
    1.   256.0 MB   [FILE]   /home/user/Downloads/video.mp4
    2.   128.0 MB   [FILE]   /home/user/Downloads/archive.zip
    3.    64.0 MB   [FILE]   /home/user/Downloads/backup.tar
```

### Group by subdirectory

```bash
$ fsz -d 1 -s 10M /home/user

══════════════════════════════════════════════════════════════════
  📊 FSZ - File Size Analyzer
──────────────────────────────────────────────────────────────────
  Path:      /home/user
  Min size:  10.0 MB
  Depth:     1
  Scan time: 25.8ms
  Files:     156
  Dirs:      24
  Total:     2.3 GB
══════════════════════════════════════════════════════════════════

  Top 10 Directories (depth 1)
  ─────────────────────────────
    1.     1.2 GB   [DIR 45]   /home/user/Videos
    2.   512.0 MB   [DIR 32]   /home/user/Documents
    3.   256.0 MB   [DIR 28]   /home/user/Pictures
```

## License

MIT License - see [LICENSE](LICENSE) for details.
