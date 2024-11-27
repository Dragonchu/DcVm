use reader::{class_file::ClassFile, types::U2};

use crate::runtime_constant_pool::RunTimeConstantPool;

#[derive(Debug)]
enum Oop<'a> {
    InstanceOop(&'a InstanceOopDesc<'a>),
    ArrayKlassDesc(&'a ArrayKlassDesc<'a>)
}

#[derive(Debug)]
enum ComponentType {
    Object
}

pub enum Klass<'a> {
    Instance(&'a InstanceKlassDesc<'a>),
    Array(&'a ArrayKlassDesc<'a>) 
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

pub type InstanceKlassRef<'a> = &'a InstanceKlassDesc<'a>;
pub type InstanceOopRef<'a> = &'a InstanceOopDesc<'a>;

#[derive(Debug)]
pub struct InstanceKlassDesc<'a>{
    class_state: ClassState,
    super_class: Option<&'a InstanceKlassDesc<'a>>,
    fields_count: usize,
    class_file: &'a ClassFile,
}

impl<'a> InstanceKlassDesc<'a> {
    pub fn new(class_file: &'a ClassFile) -> InstanceKlassDesc<'a> {
        InstanceKlassDesc {
            class_state: ClassState::Allocated,
            super_class: None,
            fields_count: class_file.fields_count as usize,
            class_file: class_file
        }
    }
}

#[derive(Debug)]
struct ArrayKlassDesc<'a> {
    component_type: ComponentType,
    down_dimension_type: Option<&'a ArrayKlassDesc<'a>>
}
impl<'a> ArrayKlassDesc<'a> {
    fn get_dimension(&self) -> usize {
        todo!()
    }
    fn is_object_array(&self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub struct InstanceOopDesc<'a> {
    fields: Vec<Oop<'a>>,
    klass: InstanceKlassRef<'a>
}
impl<'a> InstanceOopDesc<'a> {
    pub fn new(klass: InstanceKlassRef<'a>) -> InstanceOopDesc<'a> {
        InstanceOopDesc {
            fields: Vec::with_capacity(klass.fields_count),
            klass: klass
        }
    }
    fn set_field_value(&mut self, class_name: &str, field_name: &str, field_descriptor: &str) {
        todo!()
    }

    fn get_field_value(&self, class_name: &str, field_name: &str, field_descriptor: &str) -> &'a Oop {
        todo!()
    }
}

struct ArrayOopDesc<'a> {
    elements: Vec<Oop<'a>>,
    klass: ArrayKlassDesc<'a>
}
impl<'a> ArrayOopDesc<'a> {
    fn get_dimension(&self) -> usize {
        todo!()
    }

    fn get_length(&self) -> usize {
        todo!()
    }

    fn get_element_at(&'a self, position: usize) -> &'a Oop {
        todo!()
    }

    fn set_element_at(&mut self, element: Oop) {
        todo!()
    }
}