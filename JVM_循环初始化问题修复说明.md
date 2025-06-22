# JVM 类初始化死循环/栈溢出问题修复说明

## 问题现象

在执行 `run_tests.sh` 运行 TestProgram 时，JVM 出现了如下问题：
- 控制台不断打印 `<clinit>` 和 `registerNatives` 的查找与调用日志。
- 最终导致 `thread 'main' has overflowed its stack`，即栈溢出。

## 问题原因分析

1. **<clinit> 方法递归调用**
   - 当类的 `<clinit>` 方法（静态初始化块）被触发时，JVM 会尝试查找并执行该方法。
   - 如果 `<clinit>` 方法中存在对其他静态方法（如 `registerNatives`）的调用，而这些方法又会间接触发本类或父类的 `<clinit>`，就会导致递归初始化。

2. **初始化状态未提前标记**
   - 原有实现中，只有在 `<clinit>` 执行完毕后，才将类的状态标记为 `Initialized`。
   - 这样如果 `<clinit>` 执行过程中再次触发本类的初始化，JVM 认为还未初始化，于是再次执行 `<clinit>`，形成死循环，最终栈溢出。

## JVM 规范要求

- JVM 规范要求：**在执行 `<clinit>` 之前，必须先将类的状态标记为"已初始化"或"正在初始化"**，这样递归触发时不会重复执行。

## 修复方法

1. **提前设置初始化状态**
   - 在 `initialize_class` 方法中，在执行 `<clinit>` 之前，先将类的状态设置为 `Initialized`。
   - 这样即使 `<clinit>` 过程中有递归初始化，也会被正确拦截，不会重复执行。

2. **核心修复代码片段**

```rust
pub fn initialize_class(&self, class_name: &str, heap: &mut Heap, vm: *mut crate::vm::Vm) -> Result<(), JvmError> {
    let class_info = self.get_or_create_class_info(class_name);

    // 先判断状态，避免递归 borrow
    let state = {
        let info = class_info.borrow();
        info.state.clone()
    };

    if state != ClassLoadingState::Prepared {
        return Ok(());
    }

    // 先标记为已初始化，防止递归
    {
        let mut info = class_info.borrow_mut();
        info.state = ClassLoadingState::Initialized;
    }

    // 执行<clinit>
    let klass = {
        let info = class_info.borrow();
        info.klass.as_ref().unwrap().clone()
    };
    if let Klass::Instance(instance) = &klass {
        if let Some(clinit) = instance.get_method("<clinit>", "()V") {
            let mut thread = crate::jvm_thread::JvmThread::new(1024, 128);
            unsafe {
                thread.execute(clinit, heap, Some(&mut *vm))?;
            }
        }
    }
    Ok(())
}
```

## 总结

- 本次问题的根本原因是 `<clinit>` 初始化流程未提前设置初始化状态，导致递归初始化死循环。
- 修复方法是严格遵循JVM规范，在执行 `<clinit>` 之前就将类状态标记为已初始化。
- 这样可以彻底避免类似的递归死循环和栈溢出问题。 