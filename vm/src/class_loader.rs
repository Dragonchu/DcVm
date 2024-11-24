use std::collections::HashMap;

use reader::class_path_manager::ClassPathManager;
use typed_arena::Arena;

use crate::class::Class;

struct BootstrapClassLoader {
    class_path_manger: ClassPathManager,
    classes: HashMap<String, Class>,
    allocator: Arena<Class>
}

impl BootstrapClassLoader {
    fn load(&mut self, class_name: &str) -> &Class {
        let class_file = self.class_path_manger.search_class(class_name).expect("msg");
        let class = Class::new(class_file);
        self.allocator.alloc(class)
    }
}