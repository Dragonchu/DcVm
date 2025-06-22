# JVM Pointer High Bits Lost Bug Fix 说明

## 问题背景

在 macOS (arm64) 64 位环境下，运行 JVM 项目时，执行 `TestProgram` 会出现段错误（Segmentation fault）。日志显示对象分配时指针地址正常，但后续写字段时指针高位丢失，导致写入非法内存。

## 现象描述

- 对象分配时地址如：`RawPtr(0x0000000160008058)`
- 写字段时地址变成：`RawPtr(0x0000000060008058)`（高位丢失）
- 最终导致段错误，程序崩溃

## 问题根因

本地变量（LocalVars）实现如下：
```rust
pub struct LocalVars {
    max_locals: usize,
    values: Vec<i32>,
}
```
- set_obj_ref 时用 `i32` 存储指针低位，get_obj_ref 时用 `i32` 还原指针。
- 在 64 位系统下，指针高位全部丢失，导致后续访问非法内存。

## 排查过程

1. 通过日志发现分配和写字段时指针高位不一致。
2. 检查 RawPtr、JvmValue、栈和本地变量的实现。
3. 发现 LocalVars 用 i32 存储指针，导致高位丢失。

## 修复方法

- 将 LocalVars 的实现改为：
  - 用 `Vec<JvmValue>` 存储所有本地变量。
  - set/get 方法直接操作 JvmValue，不做任何类型截断。

修复后代码示例：
```rust
use crate::JvmValue;

pub struct LocalVars {
    max_locals: usize,
    values: Vec<JvmValue>,
}

impl LocalVars {
    pub fn new(max_locals: usize) -> Self {
        LocalVars {
            max_locals,
            values: vec![JvmValue::Null; max_locals],
        }
    }
    // ... 其它 get/set 方法 ...
}
```

## 结果验证

- 修复后，所有对象指针传递和访问都正常，JVM 稳定运行，测试全部通过。

## 总结

本次问题的根本原因是 64 位指针被错误地截断为 32 位，导致高位丢失。修复方式是所有与指针相关的数据结构都要用 64 位安全类型（如 RawPtr/JvmValue）存储，绝不能用 i32/u32 等类型保存指针。

如遇类似问题，优先排查所有指针与整数类型的转换和存储方式。 