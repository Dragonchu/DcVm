use std::ptr::NonNull;

use reader::{class_file::ClassFile, types::U2};

#[derive(Copy, Clone)]
pub union Klass {
    instance: NonNull<InstanceKlassDesc>,
    array: NonNull<ArrayKlassDesc>
}

#[derive(Copy, Clone)]
pub union Oop {
    instance: NonNull<InstanceOopDesc>,
    array: NonNull<ArrayOopDesc>
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

pub struct InstanceKlassDesc{
    class_state: ClassState,
    super_class: Option<Klass>,
    class_file: NonNull<ClassFile>,
}

pub struct ArrayKlassDesc {
    down_dimension_type: Option<Klass>
}
impl ArrayKlassDesc {
    fn get_dimension(&self) -> usize {
        todo!()
    }
    fn is_object_array(&self) -> bool {
        todo!()
    }
}

pub struct InstanceOopDesc {
    fields: Vec<Oop>,
    klass: Klass
}

pub struct ArrayOopDesc {
    elements: Vec<Oop>,
    klass: Klass 
}
