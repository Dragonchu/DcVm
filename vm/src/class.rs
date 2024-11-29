use std::{cell::{Ref, RefCell}, collections::HashMap};

use reader::{class_file::ClassFile, types::U2};

use crate::{method::Method, runtime_constant_pool::RunTimeConstantPool};

#[derive(Debug)]
enum Oop<'memory> {
    InstanceOop(&'memory InstanceOopDesc<'memory>)
}

#[derive(Debug)]
enum ComponentType<'memory> {
    Object(InstanceKlassRef<'memory>),
    Array(ArrayKlassRef<'memory>),
}

pub enum Klass<'a> {
    Instance(&'a InstanceKlassDesc<'a>),
}

#[derive(Debug,Clone, Copy)]
enum ClassState {
    Allocated
}

#[derive(Clone, Copy)]
enum ClassType {

}

trait KlassAbility<'a> {
    fn get_class_state(&self) -> ClassState;
    fn set_class_state(&mut self, class_state: ClassState);
    fn get_access_flag(&self) -> U2;
    fn set_access_flag(&mut self, access_flag: U2);
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: &str);
    fn get_class_type(&self) -> ClassType;
    fn set_class_type(&mut self, class_type: ClassType);
    fn get_super_class(&'a self) -> Option<&'a InstanceKlassDesc>;
    fn set_super_class(&'a mut self, super_class: &'a InstanceKlassDesc);
    fn is_public(&self);
    fn is_private(&self);
    fn is_protected(&self);
    fn is_final(&self);
    fn is_static(&self);
    fn is_abstract(&self);
    fn is_interface(&self);
}

struct CoreKlassDesc<'a> {
    class_state: ClassState,
    access_flag: U2,
    name: String,
    class_type: ClassType,
    super_class: Option<&'a InstanceKlassDesc<'a>>
}
impl<'a> KlassAbility<'a> for CoreKlassDesc<'a> {
    fn get_class_state(&self) -> ClassState {
        self.class_state
    }

    fn set_class_state(&mut self, class_state: ClassState) {
        self.class_state = class_state;
    }

    fn get_access_flag(&self) -> U2 {
        self.access_flag
    }

    fn set_access_flag(&mut self, access_flag: U2) {
        self.access_flag = access_flag;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: &str) {
        self.name = String::from(name)
    }

    fn get_class_type(&self) -> ClassType {
        self.class_type
    }

    fn set_class_type(&mut self, class_type: ClassType) {
        self.class_type = class_type
    }

    fn get_super_class(&'a self) -> Option<&'a InstanceKlassDesc> {
        self.super_class
    }

    fn set_super_class(&'a mut self, super_class: &'a InstanceKlassDesc) {
        self.super_class = Some(super_class)
    }

    fn is_public(&self) {
        todo!()
    }

    fn is_private(&self) {
        todo!()
    }

    fn is_protected(&self) {
        todo!()
    }

    fn is_final(&self) {
        todo!()
    }

    fn is_static(&self) {
        todo!()
    }

    fn is_abstract(&self) {
        todo!()
    }

    fn is_interface(&self) {
        todo!()
    }
}

pub type InstanceKlassRef<'memory> = &'memory InstanceKlassDesc<'memory>;
pub type InstanceOopRef<'memory> = &'memory InstanceOopDesc<'memory>;
pub type ArrayKlassRef<'memory> = &'memory ArrayKlassDesc<'memory>;

#[derive(Debug)]
pub struct InstanceKlassDesc<'metaspace>{
    class_state: ClassState,
    super_class: Option<&'metaspace InstanceKlassDesc<'metaspace>>,
    fields_count: usize,
    vtable: RefCell<HashMap<String, Method>>,
    methods: RefCell<HashMap<String, Method>>,
    class_file: &'metaspace ClassFile,
}

fn gen_method_key(name: &str, descriptor: &str) -> String {
    format!("{name} {descriptor}")
}

impl<'metaspace> InstanceKlassDesc<'metaspace> {
    pub fn new(class_file: &'metaspace ClassFile) -> InstanceKlassDesc<'metaspace> {
        InstanceKlassDesc {
            class_state: ClassState::Allocated,
            super_class: None,
            fields_count: class_file.fields_count as usize,
            vtable: RefCell::new(HashMap::new()),
            methods: RefCell::new(HashMap::new()),
            class_file: class_file
        }
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
       self.methods.borrow().get(&unique_key).expect("No method found").clone()
    }
}
#[derive(Debug)]
pub struct ArrayKlassDesc<'memory> {
    dimension: usize,
    component_type: ComponentType<'memory>,
}
impl<'memory> ArrayKlassDesc<'memory> {
    fn multi_dimentsion(dimension: usize, down_array: ArrayKlassRef<'memory>) -> ArrayKlassDesc {
        ArrayKlassDesc {
            dimension,
            component_type: ComponentType::Array(down_array)
        }
    }

    fn object_array(component_klass: InstanceKlassRef<'memory>) -> ArrayKlassDesc {
        ArrayKlassDesc {
            dimension: 1,
            component_type: ComponentType::Object(component_klass)
        }
    }
}

#[derive(Debug)]
pub struct InstanceOopDesc<'memory> {
    fields: Vec<Oop<'memory>>,
    klass: InstanceKlassRef<'memory>
}
impl<'memory> InstanceOopDesc<'memory> {
    pub fn new(klass: InstanceKlassRef<'memory>) -> InstanceOopDesc<'memory> {
        InstanceOopDesc {
            fields: Vec::with_capacity(klass.fields_count),
            klass: klass
        }
    }
    pub fn get_klass(&self) -> InstanceKlassRef {
        self.klass
    }
    fn set_field_value(&mut self, class_name: &str, field_name: &str, field_descriptor: &str) {
        todo!()
    }

    fn get_field_value(&self, class_name: &str, field_name: &str, field_descriptor: &str) -> &'memory Oop {
        todo!()
    }
}

struct ArrayOopDesc<'heap, 'metaspace> {
    elements: Vec<Oop<'heap>>,
    klass: ArrayKlassRef<'metaspace>
}
impl<'heap, 'metaspace> ArrayOopDesc<'heap, 'metaspace> {
    fn get_dimension(&self) -> usize {
        todo!()
    }

    fn get_length(&self) -> usize {
        todo!()
    }

    fn get_element_at(&self, position: usize) -> &Oop {
        todo!()
    }

    fn set_element_at(&mut self, element: Oop) {
        todo!()
    }
}