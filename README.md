# 🚀 Rust JVM Implementation

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Java](https://img.shields.io/badge/Java-1.8-blue.svg)](https://www.oracle.com/java/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Cursor](https://img.shields.io/badge/Built%20with-Cursor-purple.svg)](https://cursor.sh/)

> A Java 1.8 compatible JVM implementation written in Rust, featuring comprehensive test automation and modern development practices.

## 🌟 Features

- **Java 1.8 Compatibility**: Full support for Java 1.8 bytecode and language features
- **Comprehensive Testing**: Automated test suite with multiple test scenarios
- **Modern Architecture**: Clean, modular Rust implementation
- **Easy Testing**: One-command test execution with intelligent compilation
- **Cross-platform**: Works on macOS, Linux, and Windows

## 🛠️ Technology Stack

- **Language**: [Rust](https://www.rust-lang.org/) (90%+ code generated with [Cursor](https://cursor.sh/))
- **Target**: Java 1.8 JVM Specification
- **Build System**: Cargo
- **Testing**: Custom test automation scripts

## 📦 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or higher)
- [Java JDK 8](https://www.oracle.com/java/technologies/javase/javase8-archive-downloads.html) (for compilation and rt.jar)

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-jvm.git
cd rust-jvm

# Build the project
cargo build --release
```

## 🧪 Testing

### Quick Start

```bash
# List all available test cases
./run_tests.sh -l

# Run a specific test (auto-compiles if needed)
./run_tests.sh -f TestProgram

# Force recompile and run
./run_tests.sh -f TestProgram -c

# Interactive test selection
./run_tests.sh
```

### Available Test Cases

- **TestProgram**: Comprehensive JVM feature testing
- **SimpleStaticTest**: Static field and method testing
- **SimplePrintTest**: Basic output functionality

### Test Script Options

| Option | Description |
|--------|-------------|
| `-f, --file <name>` | Run specific Java file (without .java extension) |
| `-l, --list` | List all available test files |
| `-c, --compile` | Force recompile Java files |
| `-q, --quiet` | Silent mode (no JVM debug output) |
| `-h, --help` | Show help information |

## 🏗️ Project Structure

```
rust-jvm/
├── vm/                    # Core JVM implementation
├── cli/                   # Command-line interface
├── reader/                # Class file reader
├── test/                  # Test cases directory
│   ├── TestProgram.java   # Main test program
│   ├── SimpleStaticTest.java
│   └── SimplePrintTest.java
├── run_tests.sh          # Test automation script
└── resources/            # JVM resources
```

## 🔧 Development

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run specific component
cargo run --bin vm
```

### Adding New Tests

1. Create a new Java file in the `test/` directory
2. Ensure it only uses Java 1.8 features
3. Run with: `./run_tests.sh -f YourTestName`

## 🤝 Contributing

We welcome contributions! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Guidelines

- Follow Rust coding conventions
- Add tests for new features
- Ensure Java 1.8 compatibility
- Update documentation as needed

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Cursor](https://cursor.sh/) - The AI-first code editor
- Inspired by the Java Virtual Machine Specification
- Community contributors and testers

## 🔗 Links

- [Cursor Editor](https://cursor.sh/) - AI-powered code editor used for development
- [Rust Programming Language](https://www.rust-lang.org/)
- [Java 8 Documentation](https://docs.oracle.com/javase/8/)
- [JVM Specification](https://docs.oracle.com/javase/specs/jvms/se8/html/)

---

# 🚀 Rust JVM 实现

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Java](https://img.shields.io/badge/Java-1.8-blue.svg)](https://www.oracle.com/java/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Cursor](https://img.shields.io/badge/Built%20with-Cursor-purple.svg)](https://cursor.sh/)

> 使用 Rust 编写的 Java 1.8 兼容 JVM 实现，具备全面的测试自动化和现代开发实践。

## 🌟 特性

- **Java 1.8 兼容性**: 完整支持 Java 1.8 字节码和语言特性
- **全面测试**: 包含多种测试场景的自动化测试套件
- **现代架构**: 清晰、模块化的 Rust 实现
- **便捷测试**: 一键测试执行，智能编译
- **跨平台**: 支持 macOS、Linux 和 Windows

## 🛠️ 技术栈

- **语言**: [Rust](https://www.rust-lang.org/) (90%+ 代码由 [Cursor](https://cursor.sh/) 生成)
- **目标**: Java 1.8 JVM 规范
- **构建系统**: Cargo
- **测试**: 自定义测试自动化脚本

## 📦 安装

### 前置要求

- [Rust](https://www.rust-lang.org/tools/install) (1.70 或更高版本)
- [Java JDK 8](https://www.oracle.com/java/technologies/javase/javase8-archive-downloads.html) (用于编译和 rt.jar)

### 设置

```bash
# 克隆仓库
git clone https://github.com/yourusername/rust-jvm.git
cd rust-jvm

# 构建项目
cargo build --release
```

## 🧪 测试

### 快速开始

```bash
# 列出所有可用测试用例
./run_tests.sh -l

# 运行指定测试（自动编译）
./run_tests.sh -f TestProgram

# 强制重新编译并运行
./run_tests.sh -f TestProgram -c

# 交互式测试选择
./run_tests.sh
```

### 可用测试用例

- **TestProgram**: 全面的 JVM 功能测试
- **SimpleStaticTest**: 静态字段和方法测试
- **SimplePrintTest**: 基本输出功能测试

### 测试脚本选项

| 选项 | 描述 |
|------|------|
| `-f, --file <名称>` | 运行指定的 Java 文件（不含 .java 扩展名） |
| `-l, --list` | 列出所有可用的测试文件 |
| `-c, --compile` | 强制重新编译 Java 文件 |
| `-q, --quiet` | 静默模式（无 JVM 调试输出） |
| `-h, --help` | 显示帮助信息 |

## 🏗️ 项目结构

```
rust-jvm/
├── vm/                    # 核心 JVM 实现
├── cli/                   # 命令行界面
├── reader/                # 类文件读取器
├── test/                  # 测试用例目录
│   ├── TestProgram.java   # 主测试程序
│   ├── SimpleStaticTest.java
│   └── SimplePrintTest.java
├── run_tests.sh          # 测试自动化脚本
└── resources/            # JVM 资源
```

## 🔧 开发

### 构建

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 运行指定组件
cargo run --bin vm
```

### 添加新测试

1. 在 `test/` 目录下创建新的 Java 文件
2. 确保只使用 Java 1.8 特性
3. 运行: `./run_tests.sh -f YourTestName`

## 🤝 贡献

我们欢迎贡献！请随时提交 Pull Request。对于重大更改，请先开一个 issue 来讨论您想要更改的内容。

### 开发指南

- 遵循 Rust 编码约定
- 为新功能添加测试
- 确保 Java 1.8 兼容性
- 根据需要更新文档

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- 使用 [Cursor](https://cursor.sh/) 构建 - AI 优先的代码编辑器
- 受 Java 虚拟机规范启发
- 社区贡献者和测试者

## 🔗 链接

- [Cursor 编辑器](https://cursor.sh/) - 用于开发的 AI 驱动代码编辑器
- [Rust 编程语言](https://www.rust-lang.org/)
- [Java 8 文档](https://docs.oracle.com/javase/8/)
- [JVM 规范](https://docs.oracle.com/javase/specs/jvms/se8/html/) 