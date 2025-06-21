# JVM日志控制功能说明

## 概述

本项目新增了JVM运行日志控制功能，允许用户选择是否输出JVM内部的调试日志信息。

## 功能特性

### 1. 日志控制选项

- **正常模式**：输出完整的JVM调试日志，包括字节码执行、类加载、方法调用等详细信息
- **静默模式**：只输出Java程序的执行结果，不显示JVM内部调试信息

### 2. 使用方法

#### 使用脚本运行（推荐）

```bash
# 正常模式（显示所有日志）
./run_tests.sh -f SimplePrintTest

# 静默模式（只显示程序输出）
./run_tests.sh -f SimplePrintTest -q

# 查看帮助信息
./run_tests.sh -h
```

#### 直接运行JVM

```bash
# 正常模式
cargo run --bin vm -- test/SimplePrintTest.class test:/path/to/rt.jar

# 静默模式
cargo run --bin vm -- test/SimplePrintTest.class test:/path/to/rt.jar --quiet
```

### 3. 脚本选项说明

- `-f, --file <文件名>`: 指定要运行的Java文件（不含.java扩展名）
- `-l, --list`: 列出所有可用的测试文件
- `-c, --compile`: 强制重新编译Java文件
- `-q, --quiet`: 静默模式，不输出JVM调试日志
- `-h, --help`: 显示帮助信息

## 日志类型

### 正常模式下的日志输出

1. **JVM启动日志**
   - 类加载信息
   - 类路径信息

2. **类加载器日志**
   - 类路径搜索过程
   - 类文件查找信息

3. **字节码执行日志**
   - 指令执行过程（getstatic, invokevirtual等）
   - 方法调用信息
   - 对象创建信息

4. **Native方法日志**
   - System.out.println调用信息
   - 参数类型和值

### 静默模式下的输出

- 只显示Java程序的`System.out.println`输出
- 不显示任何JVM内部调试信息

## 示例对比

### 正常模式输出示例

```
[JVM] 加载类: LSimplePrintTest;
[JVM] 类路径: /path/to/test:/path/to/rt.jar
[ClassPathManager] 添加类路径: /path/to/test
[ClassPathManager] 搜索类: SimplePrintTest (文件名: SimplePrintTest.class)
getstatic 2
Getting static field: java/lang/System.out
[Pushed System.out object]
ldc string: Hello, JVM!
invokevirtual 4
Calling virtual method: java/io/PrintStream.println
[Native] System.out.println called with 1 arguments
[Native] Printing object reference: RawPtr(0x140008068)
[Object: RawPtr(0x140008068)]
[Native method call successful]
Hello, JVM!
```

### 静默模式输出示例

```
Hello, JVM!
42
测试中文输出
3.14
true
A
```

## 技术实现

### 1. 日志控制模块

- 位置：`vm/src/logger.rs`
- 功能：提供全局日志开关控制
- 使用原子操作确保线程安全

### 2. 日志宏

- `jvm_log!`: 条件打印JVM调试日志
- `jvm_debug!`: 条件打印调试信息

### 3. 模块集成

- VM模块：使用`jvm_log!`宏替换`println!`
- Reader模块：使用简单的日志控制函数
- 主程序：解析`--quiet`参数并设置日志模式

## 注意事项

1. **Java程序输出**：无论是否启用静默模式，Java程序的`System.out.println`输出都会正常显示
2. **错误信息**：重要的错误信息仍然会输出，不受静默模式影响
3. **性能影响**：静默模式可以略微提升性能，减少字符串格式化开销

## 扩展建议

1. **日志级别**：可以进一步细化日志级别（DEBUG, INFO, WARN, ERROR）
2. **日志文件**：支持将日志输出到文件
3. **配置文件**：通过配置文件控制日志行为 