use std::{cell::RefCell, collections::HashMap, fmt};

use reader::{
    class_file::ClassFile,
    constant_pool::{ConstantPool, CpInfo},
    types::U2,
};

use crate::{
    field::{Field, FieldId},
    method::Method,
};

use gc::{Finalize, Gc, GcCell, Trace};

#[derive(Debug, Clone, Trace, Finalize)]
pub enum Oop {
    Instance(InstanceOopRef),
    Array(ArrayOopRef),
    Int(i32),
    Uninitialized,
}

#[derive(Clone, Debug)]
pub enum Klass {
    Instance(InstanceKlassRef),
    Array(ArrayKlassRef),
}

pub type InstanceKlassRef = Gc<GcCell<InstanceKlassDesc>>;
pub type ArrayKlassRef = Gc<GcCell<ArrayKlassDesc>>;
pub type InstanceOopRef = Gc<InstanceOopDesc>;
pub type ArrayOopRef = Gc<ArrayOopDesc>;
pub type ClassFileRef = Box<ClassFile>;

impl Oop {
    pub fn get_klass(&self) -> Klass {
        match self {
            Oop::Instance(oop_desc) => Klass::Instance(oop_desc.get_klass()),
            Oop::Array(oop_desc) => Klass::Array(oop_desc.get_klass()),
            _ => {
                panic!("primitive value do not have class")
            }
        }
    }
}

#[derive(Debug)]
pub enum ComponentType {
    Object(InstanceKlassRef),
    Array(ArrayKlassRef),
    Byte,
    Boolean,
    Short,
    Char,
    Int,
    Long,
    Float,
    Double,
    Void,
}

#[derive(Debug, Clone, Trace, Finalize)]
enum ClassState {
    Allocated,
}

impl Klass {
    pub fn get_method(&self, method_name: &str, descriptor: &str) -> Method {
        match self {
            Klass::Instance(klass) => klass.borrow().get_method(method_name, descriptor),
            _ => {
                panic!("Do not support")
            }
        }
    }
}

#[derive(Trace, Finalize)]
pub struct InstanceKlassDesc {
    class_name: String,
    class_state: ClassState,
    super_class: Option<InstanceKlassRef>,
    fields_count: usize,
    vtable: RefCell<HashMap<String, Method>>,
    methods: RefCell<HashMap<String, Method>>,
    static_fields: RefCell<HashMap<String, FieldId>>,
    static_values: RefCell<HashMap<String, Oop>>,
    instance_fields: RefCell<HashMap<String, FieldId>>,
    class_file: ClassFileRef,
}
impl fmt::Debug for InstanceKlassDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InstanceKlassDesc")
            .field("class_name", &self.class_name)
            .field("class_state", &self.class_state)
            .field("super_class", &self.super_class)
            .field("fields_count", &self.fields_count)
            .field("vtable", &self.vtable)
            .field("methods", &self.methods)
            .field("static_fields", &self.static_fields)
            .field("instance_fields", &self.instance_fields)
            .finish()
    }
}

fn gen_method_key(name: &str, descriptor: &str) -> String {
    format!("{name} {descriptor}")
}

impl InstanceKlassDesc {
    pub fn new(class_file: ClassFileRef) -> InstanceKlassDesc {
        let class_name =
            Self::get_this_class_name(class_file.this_class, &class_file.constant_pool);
        InstanceKlassDesc {
            class_name: class_name.clone(),
            class_state: ClassState::Allocated,
            super_class: None,
            fields_count: class_file.fields_count as usize,
            vtable: RefCell::new(HashMap::new()),
            methods: RefCell::new(HashMap::new()),
            static_fields: RefCell::new(HashMap::new()),
            static_values: RefCell::new(HashMap::new()),
            instance_fields: RefCell::new(HashMap::new()),
            class_file,
        }
    }

    pub fn get_class_name(&self) -> String {
        self.class_name.clone()
    }

    pub fn get_instance_field_info(
        &self,
        class_name: &str,
        field_name: &str,
        descriptor: &str,
    ) -> FieldId {
        let unique_name = Self::gen_field_unique_name(class_name, field_name, descriptor);
        self.instance_fields
            .borrow()
            .get(&unique_name)
            .cloned()
            .expect("unknown filed")
    }

    pub fn set_instance_field_info(
        &self,
        receiver: InstanceOopRef,
        class_name: &str,
        field_name: &str,
        descriptor: &str,
        value: Oop,
    ) {
    }

    pub fn link(&self) {
        self.link_method();
        self.link_fields();
    }

    fn get_this_class_name(this_class: U2, cp_pool: &Vec<CpInfo>) -> String {
        if let CpInfo::Class { tag: _, name_index } = cp_pool
            .get((this_class - 1) as usize)
            .expect("Unknown class")
        {
            return cp_pool
                .get((name_index - 1) as usize)
                .expect("Unknow class name")
                .to_utf8_string();
        } else {
            panic!("Unknown class");
        }
    }

    pub fn get_utf8_string(&self, index: usize) -> String {
        return self
            .class_file
            .constant_pool
            .get((index - 1) as usize)
            .expect("Unknow class name")
            .to_utf8_string();
    }

    pub fn get_field_name(&self, field_index: usize) -> String {
        if let CpInfo::FieldRef {
            tag,
            class_index,
            name_and_type_index,
        } = self
            .class_file
            .constant_pool
            .get((field_index - 1) as usize)
            .expect("Unknow field")
        {
            return self.get_utf8_string(name_and_type_index.clone() as usize);
        } else {
            panic!("Not field")
        }
    }

    pub fn get_field_info(&self, field_index: U2) -> (String, String, String) {
        return self.class_file.constant_pool.get_field_info(field_index);
    }

    pub fn link_fields(&self) {
        if let Some(super_class_desc) = self.super_class {
            let super_class_fields = super_class_desc.instance_fields.borrow();
            for (key, value) in super_class_fields.iter() {
                self.instance_fields
                    .borrow_mut()
                    .insert(key.clone(), value.clone());
            }
        }
        let mut instance_field_index = self.instance_fields.borrow().len();
        let mut static_field_index = 0;
        let cp_pool = &self.class_file.constant_pool;
        for field_info in &self.class_file.fields {
            let field = Field::new(field_info, cp_pool);
            let field_name = field.get_name();
            let field_descriptor = field.get_descriptor();
            let unique_name =
                Self::gen_field_unique_name(&self.class_name, &field_name, &field_descriptor);
            if field.is_static() {
                let field_id = FieldId::new(static_field_index, field);
                self.static_fields
                    .borrow_mut()
                    .insert(unique_name, field_id);
                static_field_index += 1;
            } else {
                let field_id = FieldId::new(instance_field_index, field);
                self.instance_field
                    .borrow_mut()
                    .insert(unique_name, field_id);
                instance_field_index += 1;
            }
        }
    }

    pub fn gen_field_unique_name(class_name: &str, field_name: &str, descriptor: &str) -> String {
        format!("{class_name} {field_name} {descriptor}")
    }

    pub fn link_method(&self) {
        let cp_pool = &self.class_file.constant_pool;
        for method_info in &self.class_file.methods {
            let method = Method::new(&method_info, cp_pool);
            let unique_key = gen_method_key(&method.get_name(), &method.get_descriptor());
            self.methods.borrow_mut().insert(unique_key, method);
        }
    }

    pub fn get_method(&self, method_name: &str, descriptor: &str) -> Method {
        let unique_key = gen_method_key(method_name, descriptor);
        self.methods
            .borrow()
            .get(&unique_key)
            .expect("No method found")
            .clone()
    }

    pub fn new_instance(&self) -> InstanceOopDesc {
        InstanceOopDesc::new(self)
    }
}

#[derive(Debug)]
pub struct ArrayKlassDesc {
    dimension: usize,
    component_type: ComponentType,
}

impl ArrayKlassDesc {
    pub fn new(dimension: usize, component_type: ComponentType) -> ArrayKlassDesc {
        ArrayKlassDesc {
            dimension,
            component_type,
        }
    }

    pub fn get_dimension(&self) -> usize {
        self.dimension
    }

    pub fn new_instance(&self, length: usize) -> ArrayOopDesc {
        ArrayOopDesc::new(self, length)
    }
}

#[derive(Debug, Trace, Finalize)]
pub struct InstanceOopDesc {
    fields: Vec<Oop>,
    klass: InstanceKlassRef,
}

impl InstanceOopDesc {
    pub fn new(klass: InstanceKlassRef) -> InstanceOopDesc {
        let fields_count = klass.clone().borrow().fields_count;
        InstanceOopDesc {
            fields: vec![Oop::Uninitialized; fields_count],
            klass,
        }
    }

    pub fn get_klass(&self) -> InstanceKlassRef {
        self.klass.clone()
    }

    pub fn set_field_value(
        &mut self,
        class_name: &str,
        field_name: &str,
        field_descriptor: &str,
        value: Oop,
    ) {
        let field_id = self
            .klass
            .borrow()
            .get_instance_field_info(class_name, field_name, field_descriptor)
            .clone();
        self.fields.insert(field_id.offset, value);
    }

    fn get_field_value(&self, class_name: &str, field_name: &str, field_descriptor: &str) -> Oop {
        todo!()
    }
}

#[derive(Debug, Trace, Finalize)]
pub struct ArrayOopDesc {
    elements: Vec<Oop>,
    klass: ArrayKlassRef,
}

impl ArrayOopDesc {
    pub fn new(klass: ArrayKlassRef, length: usize) -> ArrayOopDesc {
        ArrayOopDesc {
            elements: vec![Oop::Uninitialized; length],
            klass,
        }
    }

    pub fn get_klass(&self) -> ArrayKlassRef {
        self.klass.clone()
    }

    pub fn set_element_at(&mut self, position: usize, element: Oop) {
        self.elements.insert(position, element);
    }
}
