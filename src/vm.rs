use crate::{loader::class_loader::BootStrapLoader, system_dic::{self, MetaSpace}};

pub struct VM {
    meta_space: MetaSpace,
    boot_strap_class_loader: BootStrapLoader,
}

impl VM {
    pub fn new(class_paths: &str) -> Self {
        let meta_space = MetaSpace::new();
        let boot_strap_class_loader = BootStrapLoader::new(class_paths);
        Self {
            meta_space,
            boot_strap_class_loader,
        }
    }
    pub fn run(&self, main_class_name: &str) {

    }

    pub fn require_class(&self, class_name: &str) {

    }
}