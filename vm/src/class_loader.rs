use crate::class::{ArrayKlass, ComponentType, InstanceKlass, Klass};
use reader::class_path_manager::ClassPathManager;
use reader::constant_pool::ConstantPool;
use std::cell::Cell;
use std::{cell::RefCell, collections::HashMap};
use crate::class::Klass::Instance;
use crate::heap::Heap;
use std::sync::Arc;
use crate::JvmValue;
use crate::error::JvmError;

/// 类加载状态枚举
/// 表示类在加载过程中的不同阶段
#[derive(Debug, Clone, PartialEq)]
pub enum ClassLoadingState {
    /// 类尚未开始加载
    NotLoaded,
    /// 类正在加载中
    Loading,
    /// 类已加载完成
    Loaded,
    /// 类已准备完成（静态字段已分配内存并设置默认值）
    Prepared,
    /// 类已初始化完成（静态初始化块已执行）
    Initialized,
    /// 类加载失败
    Failed,
}

/// 类加载信息结构
/// 存储类加载过程中的状态和相关信息
pub struct ClassLoadingInfo {
    /// 当前加载状态
    state: ClassLoadingState,
    /// 类信息
    klass: Option<Klass>,
    /// 加载错误信息
    error: Option<String>,
    /// 静态字段值
    static_fields: Option<Vec<JvmValue>>,
}

/// 引导类加载器
/// 负责加载Java核心类库和用户类
pub struct BootstrapClassLoader {
    /// 类路径管理器，用于查找类文件
    class_path_manager: ClassPathManager,
    /// 已加载的类缓存
    classes: RefCell<HashMap<String, Arc<RefCell<ClassLoadingInfo>>>>,
    /// 类ID计数器
    nxt_id: Cell<usize>,
}

impl BootstrapClassLoader {
    /// 创建新的引导类加载器
    /// 
    /// # 参数
    /// * `paths` - 类路径字符串，多个路径用分隔符分隔
    pub fn new(paths: &str) -> BootstrapClassLoader {
        let mut class_path_manager = ClassPathManager::new();
        class_path_manager.add_class_paths(paths);
        BootstrapClassLoader {
            class_path_manager,
            classes: RefCell::new(HashMap::new()),
            nxt_id: Cell::new(0),
        }
    }

    /// 加载指定的类
    /// 
    /// # 参数
    /// * `class_name` - 要加载的类名
    /// * `heap` - 堆内存引用
    /// 
    /// # 返回
    /// * `Result<Klass, JvmError>` - 加载成功返回类信息，失败返回错误
    pub fn load(&self, class_name: &str, heap: &mut Heap) -> Result<Klass, JvmError> {
        let class_info = self.get_or_create_class_info(class_name);

        // 先判断状态，避免递归 borrow
        let state = {
            let info = class_info.borrow();
            info.state.clone()
        };

        match state {
            ClassLoadingState::NotLoaded => {
                // 先将状态设置为Loading
                {
                    let mut info = class_info.borrow_mut();
                    info.state = ClassLoadingState::Loading;
                }
                // 递归加载
                let klass = match self.do_load_class(class_name, heap) {
                    Ok(klass) => klass,
                    Err(e) => {
                        let mut info = class_info.borrow_mut();
                        info.state = ClassLoadingState::Failed;
                        info.error = Some(e.to_string());
                        return Err(JvmError::ClassNotFoundError(format!("Failed to load class {}: {}", class_name, e)));
                    }
                };
                // 加载成功后，设置klass和状态
                {
                    let mut info = class_info.borrow_mut();
                    info.klass = Some(klass.clone());
                    info.state = ClassLoadingState::Loaded;
                    self.prepare_class(&mut info, heap)?;
                }
                Ok(class_info.borrow().klass.as_ref().unwrap().clone())
            }
            ClassLoadingState::Loading => {
                Err(JvmError::IllegalStateError(format!("Circular dependency detected while loading class {}", class_name)))
            }
            ClassLoadingState::Loaded | ClassLoadingState::Prepared | ClassLoadingState::Initialized => {
                Ok(class_info.borrow().klass.as_ref().unwrap().clone())
            }
            ClassLoadingState::Failed => {
                let info = class_info.borrow();
                Err(JvmError::ClassNotFoundError(format!("Class {} failed to load: {}", class_name, info.error.as_ref().unwrap())))
            }
        }
    }

    /// 准备类
    /// 为静态字段分配内存并设置默认值
    fn prepare_class(&self, info: &mut ClassLoadingInfo, heap: &mut Heap) -> Result<(), JvmError> {
        if info.state != ClassLoadingState::Loaded {
            return Ok(());
        }

        let klass = info.klass.as_ref().unwrap();
        if let Klass::Instance(instance) = klass {
            // 为静态字段分配内存并设置默认值
            let mut static_fields = Vec::new();
            for field in instance.get_static_fields() {
                let default_value = field.get_default();
                static_fields.push(default_value);
            }
            info.static_fields = Some(static_fields);
            info.state = ClassLoadingState::Prepared;
        }
        Ok(())
    }

    /// 初始化类
    /// 执行静态初始化块
    pub fn initialize_class(&self, class_name: &str, heap: &mut Heap, vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
        let class_info = self.get_or_create_class_info(class_name);

        // 先判断状态，避免递归 borrow
        let state = {
            let info = class_info.borrow();
            info.state.clone()
        };

        if state != ClassLoadingState::Prepared {
            return Ok(());
        }

        // 递归后再 borrow_mut 修改状态
        {
            let mut info = class_info.borrow_mut();
            // 先标记为已初始化，防止递归
            info.state = ClassLoadingState::Initialized;
        }

        // 递归前不持有 borrow_mut
        let klass = {
            let info = class_info.borrow();
            info.klass.as_ref().unwrap().clone()
        };

        if let Klass::Instance(instance) = &klass {
            // 执行静态初始化块
            if let Some(clinit) = instance.get_method("<clinit>", "()V") {
                let mut thread = crate::jvm_thread::JvmThread::new(65536, 128);
                thread.execute(clinit, heap, vm)?;
            }
        }

        Ok(())
    }

    /// 获取或创建类加载信息
    fn get_or_create_class_info(&self, class_name: &str) -> Arc<RefCell<ClassLoadingInfo>> {
        let mut classes = self.classes.borrow_mut();
        if let Some(info) = classes.get(class_name) {
            return info.clone();
        }
        
        let info = Arc::new(RefCell::new(ClassLoadingInfo {
            state: ClassLoadingState::NotLoaded,
            klass: None,
            error: None,
            static_fields: None,
        }));
        classes.insert(class_name.to_string(), info.clone());
        info
    }

    fn do_load_class(&self, class_name: &str, heap: &mut Heap) -> Result<Klass, Box<dyn std::error::Error>> {
        let klass = if class_name.starts_with('[') {
            Klass::Array(self.do_load_array(class_name, heap))
        } else {
            Klass::Instance(self.do_load_instance(class_name, heap))
        };
        Ok(klass)
    }

    fn do_load_array(&self, class_name: &str, heap: &mut Heap) -> ArrayKlass {
        let dimension_size = class_name
            .chars()
            .into_iter()
            .take_while(|&ch| ch == '[')
            .count();
        let element_type = self.load_element_type(&class_name[1..], heap);
        Klass::new_array(dimension_size, element_type, self.nxt_id.get())
    }

    fn load_element_type(&self, element_type: &str, heap: &mut Heap) -> ComponentType {
        match element_type.chars().next().unwrap() {
            '[' => {
                let array_klass = self.do_load_array(element_type, heap);
                ComponentType::Array(Box::new(array_klass))
            }
            'L' => {
                let instance_klass = self.do_load_instance(element_type, heap);
                ComponentType::Object(Box::new(instance_klass))
            }
            'B' => ComponentType::Byte,
            'Z' => ComponentType::Boolean,
            'S' => ComponentType::Short,
            'C' => ComponentType::Char,
            'I' => ComponentType::Int,
            'J' => ComponentType::Long,
            'F' => ComponentType::Float,
            'D' => ComponentType::Double,
            'V' => ComponentType::Void,
            _ => panic!("Unknown element type {}", element_type),
        }
    }

    fn do_load_instance(&self, class_name: &str, heap: &mut Heap) -> InstanceKlass {
        let class_name = class_name.trim_start_matches('L').trim_end_matches(';');
        let class_file = self
            .class_path_manager
            .search_class(class_name)
            .unwrap_or_else(|_| panic!("class {} not found", class_name));
        // 递归加载父类InstanceKlass
        let super_klass = if !class_file.get_super_class_name().is_empty() {
            let super_class_name = class_file.get_super_class_name();
            let super_klass = self.do_load_instance(super_class_name.as_str(), heap);
            Some(Box::new(super_klass))
        } else {
            None
        };
        let super_klass_ref = super_klass.as_deref();
        InstanceKlass::of(&class_file, self.nxt_id.get(), heap, super_klass_ref)
    }

    /// 设置静态字段值
    pub fn set_static_field(&self, class_name: &str, field_name: &str, value: crate::JvmValue, heap: &mut crate::heap::Heap) {
        if let Some(class_info) = self.classes.borrow().get(class_name) {
            let mut info = class_info.borrow_mut();
            if let Some(Klass::Instance(ref mut instance)) = info.klass {
                let idx = instance.get_static_fields()
                    .iter()
                    .position(|field| field.get_name() == field_name);
                if let Some(i) = idx {
                    let vals = instance.get_static_field_values_mut();
                    vals[i] = value;
                }
            }
        }
    }

    /// 获取静态字段值
    pub fn get_static_field(&self, class_name: &str, field_name: &str) -> Option<crate::JvmValue> {
        if let Some(class_info) = self.classes.borrow().get(class_name) {
            let info = class_info.borrow();
            if let Some(Klass::Instance(ref instance)) = info.klass {
                let fields = instance.get_static_fields();
                let vals = instance.get_static_field_values();
                for (i, field) in fields.iter().enumerate() {
                    if field.get_name() == field_name {
                        return Some(vals[i]);
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_main_class() {
        let cl = BootstrapClassLoader::new("resources/test");
        let mut heap = Heap::with_maximum_memory(1024);
        let klass_ref = cl.load("LMain;", &mut heap).unwrap();
        println!("{:?}", klass_ref);
    }

    #[test]
    fn load_array_class() {
        let cl = BootstrapClassLoader::new("resources/test");
        let mut heap = Heap::with_maximum_memory(1024);
        let klass_ref = cl.load("[[D", &mut heap).unwrap();
        println!("{:?}", klass_ref);
    }
    
    #[test]
    fn get_main_method() {
        let cl = BootstrapClassLoader::new("resources/test");
        let mut heap = Heap::with_maximum_memory(1024);
        let klass_ref = cl.load("LMain;", &mut heap).unwrap();
        let method = klass_ref.get_method("main", "([Ljava/lang/String;)V");
        println!("{:?}", method);
    }
    
    #[test]
    fn test_static_field_initialization() {
        let cl = BootstrapClassLoader::new("resources/test");
        let mut heap = Heap::with_maximum_memory(1024);
        
        // 加载类
        let _ = cl.load("LMain;", &mut heap).unwrap();
        let class_info = cl.get_or_create_class_info("LMain;");
        
        // 验证静态字段已初始化
        assert!(class_info.borrow().static_fields.is_some());
    }
}
