# Rust JVM 测试与运行说明

本项目包含一个简易JVM实现及自动化测试脚本，支持自动编译、选择、运行Java测试文件。

## 目录结构

```
.
├── test/                  # 测试用例目录，放置Java源文件和class文件
│   ├── TestProgram.java   # 示例测试用例
│   └── TestProgram.class  # 编译生成的class文件
├── vm/                    # Rust JVM实现主目录
├── run_tests.sh           # 一键测试脚本
└── ...
```

## 一键测试脚本用法

### 1. 列出所有可用测试用例
```bash
./run_tests.sh -l
```

### 2. 运行指定测试用例（自动编译）
```bash
./run_tests.sh -f TestProgram
```

### 3. 强制重新编译并运行
```bash
./run_tests.sh -f TestProgram -c
```

### 4. 交互式选择并运行
```bash
./run_tests.sh
```

## 说明
- 所有Java测试文件请放在 `test/` 目录下。
- 脚本会自动编译 `.java` 文件为 `.class`，并调用Rust JVM执行。
- JVM主程序支持命令行参数，自动推断类名和类路径。
- 支持自动识别、管理多个测试用例。

## 依赖
- Java JDK（需有 `javac` 命令）
- Rust（需有 `cargo` 命令）

## 进阶
- 你可以添加更多Java测试文件到 `test/` 目录，脚本会自动识别。
- 如需批量测试、支持包名、子目录等可进一步扩展脚本。

---

如有问题或建议，欢迎反馈！ 