use super::{class::Class, class_path_manager::ClassPathManager};

pub struct BootStrapLoader {
    class_path_manager: ClassPathManager,
}

impl BootStrapLoader {
    pub fn new(class_paths: &str) -> Self {
        let mut class_path_manager = ClassPathManager::new();
        class_path_manager.add_class_paths(class_paths);
        Self {
            class_path_manager,
        }
    }
    fn load_class(&self, class_name: &str) -> Class {
        todo!()
    }
}