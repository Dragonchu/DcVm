use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    classfile::{
        class_file::ClassFile,
        constant_pool::{require_constant, CpInfo},
    },
    classpath::class_loader::ClassLoader,
    common::ACC_FINAL,
    oops::{
        field::Field,
        klass::klass::ClassState,
        oop::{MirrorOop, Oop},
        reflection::FieldId,
    },
};

use super::klass::{ClassType, Klass, KlassMeta};

#[derive(Debug, Clone)]
pub struct InstanceKlassRef {
    pub layout: Rc<RefCell<InstanceKlass>>,
}

impl InstanceKlassRef {
    fn link(&self) {
        self.link_access_flags();
        self.link_klass_name();
        self.link_super_class();
        self.link_fields();
    }

    fn link_access_flags(&self) {
        let mut klass = self.layout.borrow_mut();
        let access_flags = klass.class_file.access_flags;
        klass.klass_meta.access_flags = access_flags;
    }

    fn link_klass_name(&self) {
        let mut klass = self.layout.borrow_mut();
        let pool = &klass.class_file.constant_pool;
        let class_name = pool.get_class_name(klass.class_file.this_class as usize);
        klass.klass_meta.name = class_name;
    }

    fn link_super_class(&self) {
        let mut klass = self.layout.borrow_mut();
        if klass.class_file.super_class == 0 {
            if klass.klass_meta.name != "java/lang/Object" {
                klass.klass_meta.state = Some(ClassState::InitializationError);
                panic!("Super class not found");
            }
            klass.klass_meta.super_klass = None;
            return;
        }
        let super_class_ref = self.require_instance_class(klass.class_file.super_class);
        let super_class = super_class_ref.layout.borrow_mut();
        if super_class.is_final() {
            klass.klass_meta.state = Some(ClassState::InitializationError);
            panic!("Super class is final");
        }
        klass.klass_meta.super_klass = Some(super_class_ref.clone());
    }

    fn link_fields(&mut self) {
        let mut klass = self.layout.borrow_mut();
        let mut instance_field_index = 0;
        if let Some(super_class_ref) = klass.klass_meta.super_klass.clone() {
            let super_class = super_class_ref.layout.borrow_mut();
            let super_class_fields = super_class.instance_fields.clone();
            klass.instance_fields.extend(super_class_fields);
            instance_field_index += super_class.instance_field_nums;
        }
        let mut static_field_index = 0;
        let fields = &mut klass.class_file.fields;
        let static_fields = &mut klass.static_fields;
        let instance_fields = &mut klass.instance_fields;
        for field_info_ref in fields {
            let field_ref = Field::new(self.layout.clone(), field_info_ref.clone());
            let mut field = field_ref.borrow_mut();
            field.link_field();
            let identity = field.make_identity();
            let field_id = FieldId {
                offset: static_field_index,
                field: field_ref.clone(),
            };
            if field.is_static() {
                static_field_index += 1;
                static_fields.insert(identity, field_id);
            } else {
                instance_field_index += 1;
                instance_fields.insert(identity, field_id);
            }
        }
        klass.static_field_nums = static_field_index;
        klass.instance_field_nums = instance_field_index;
    }
}

#[derive(Debug)]
pub struct InstanceKlass {
    pub klass_meta: KlassMeta,
    class_loader: Rc<dyn ClassLoader>,
    java_loader: Option<MirrorOop>,
    pub class_file: Box<ClassFile>,
    source_file: String,
    signature: String,
    inner_class_attr: String,
    enclosing_method_attr: String,
    boot_strap_methods_attr: String,
    runtime_constant_pool: String,
    all_methods: HashMap<String, String>,
    vtable: HashMap<String, String>,
    static_fields: HashMap<String, FieldId>,
    instance_fields: HashMap<String, FieldId>,
    static_field_values: Vec<Oop>,
    interfaces: HashMap<String, Rc<InstanceKlass>>,
}

impl InstanceKlass {
    pub fn new(class_file: &ClassFile, class_loader: Rc<dyn ClassLoader>) -> Rc<RefCell<Self>> {
        let access_flags = class_file.access_flags;
        let pool = &class_file.constant_pool;
        let class_name = pool.get_class_name(class_file.this_class as usize);
        let super_klass = if class_file.super_class == 0 {
            if class_name != "java/lang/Object" {
                panic!("Super class not found");
            }
            None
        } else {
            let super_class_name = pool.get_class_name(class_file.super_class as usize);
            let class_ref = class_loader.load_class(&super_class_name);
            if let Ok(Klass::InstanceKlass(super_class)) = class_ref {
                if super_class.is_final() {
                    panic!("Super class is final");
                }
                Some(super_class)
            } else {
                panic!("Super class not found");
            }
        };
        let instance_fields = if let Some(super_klass) = super_klass {
            super_klass.instance_fields.clone()
        } else {
            HashMap::new()
        };
        for field_info in class_file.fields {
            let field = Field::new(&field_info);
            field.link_field();
            let identity = field.make_identity();
            let field_id = FieldId {
                offset: static_field_index,
                field: field_ref.clone(),
            };
            if field.is_static() {
                static_field_index += 1;
                static_fields.insert(identity, field_id);
            } else {
                instance_field_index += 1;
                instance_fields.insert(identity, field_id);
            }
        }
        klass.static_field_nums = static_field_index;
        klass.instance_field_nums = instance_field_index; 

        Rc::new(RefCell::new(Self {
            klass_meta: KlassMeta {
                state: None,
                access_flags: class_file.access_flags,
                name: String::new(),
                ktype: ClassType::InstanceKlass,
                java_mirror: None,
                super_klass: None,
            },
            class_loader: class_loader.clone(),
            java_loader: None,
            class_file: class_file,
            source_file: String::new(),
            signature: String::new(),
            inner_class_attr: String::new(),
            enclosing_method_attr: String::new(),
            boot_strap_methods_attr: String::new(),
            runtime_constant_pool: String::new(),
            static_field_nums: 0,
            instance_field_nums: 0,
            all_methods: HashMap::new(),
            vtable: HashMap::new(),
            static_fields: HashMap::new(),
            instance_fields: HashMap::new(),
            static_field_values: Vec::new(),
            interfaces: HashMap::new(),
        }))
    }

    pub fn is_final(&self) -> bool {
        self.klass_meta.access_flags & ACC_FINAL == ACC_FINAL
    }

}

    fn require_instance_class(, class_info_index: u16) -> InstanceKlassRef {
        let class_name = klass
            .class_file
            .constant_pool
            .get_class_name(class_info_index as usize);
        let class_loader = klass.class_loader.clone();
        let class_ref = class_loader.load_class(&class_name);
        match class_ref {
            Ok(klass) => match klass {
                Klass::InstanceKlass(instance_klass_ref) => instance_klass_ref,
                _ => panic!("Class not found"),
            },
            Err(_) => panic!("Class not found"),
        }
    }
