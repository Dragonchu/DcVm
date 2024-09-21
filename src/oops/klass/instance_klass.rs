use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    classfile::{
        attribute_info::{
            BootStrapMethodsAttribute, EnclosingMethodAttribute, InnerClassesAttribute,
        },
        class_file::ClassFile,
        constant_pool::{require_constant, CpInfo},
        types::U2,
    },
    classpath::class_loader::ClassLoader,
    common::{
        ACC_ABSTRACT, ACC_FINAL, ACC_INTERFACE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC,
    },
    oops::{
        field::Field,
        oop::{MirrorOop, MirrorOopDesc, Oop},
        reflection::{FieldId, MethodId},
    },
    runtime::constant_pool::RuntimeConstantPool,
};

use super::{
    klass::{ClassState, ClassType, Klass, KlassMeta},
    mark_oop::MarkOop,
};

pub struct InstanceKlass(Rc<RefCell<InstanceKlassInner>>);

struct InstanceKlassInner {
    _state: ClassState,
    _access_flag: U2,
    _name: String,
    _type: ClassType,
    _java_mirror: MirrorOop,
    _super_klass: InstanceKlass,

    _class_loader: Rc<ClassLoader>,
    _java_loader: MirrorOop,
    _class_file: Rc<ClassFile>,
    _source_file: String,
    _signature: String,
    _inner_class_attr: InnerClassesAttribute,
    _enclosing_method_attr: EnclosingMethodAttribute,
    _bootstrap_method_attr: BootStrapMethodsAttribute,
    _runtime_pool: RuntimeConstantPool,
    /**
     * all methods in this class.
     * map<name + " " + descriptor, method>
     */
    _all_methods: HashMap<String, MethodId>,
    /**
     * virtual methods (public or protected methods).
     * map<name + " " + descriptor, method>
     */
    _vtable: HashMap<String, MethodId>,
    /**
     * static fields.
     * map<className + " " + name + " " + descriptor, <vector-offset, Field*>>
     */
    _static_fields: HashMap<String, FieldId>,
    /**
     * instance fields.
     * map<className + " " + name + " " + descriptor, <vector-offset, Field*>>
     */
    _instance_fields: HashMap<String, FieldId>,
    /**
     * static fields' values.
     */
    _static_field_values: Vec<Oop>,
    /**
     * interfaces
     * map<interface-name, class>
     */
    _interfaces: HashMap<String, InstanceKlass>,
}

impl InstanceKlass {
    pub fn new(class_file: Box<ClassFile>, class_loader: Box<ClassLoader>, java_loader: MirrorOop, class_type: ClassType) -> Self {
        Self {
            _runtime_pool: RuntimeConstantPool::new(self),
        }
    }

    pub fn get_java_mirror(&self) -> &MirrorOop {
        &self.0.borrow()._java_mirror
    }

    pub fn get_class_state(&self) -> ClassState {
        self.0.borrow()._state
    }

    pub fn get_access_flag(&self) -> U2 {
        self.0.borrow()._access_flag
    }

    pub fn get_name(&self) -> &str {
        &self.0.borrow()._name
    }

    pub fn get_class_type(&self) -> ClassType {
        self.0.borrow()._type
    }

    pub fn get_super_klass(&self) -> &InstanceKlass {
        &self.0.borrow()._super_klass
    }

    pub fn is_public(&self) -> bool {
        self.get_access_flag() & ACC_PUBLIC == ACC_PUBLIC
    }

    pub fn is_private(&self) -> bool {
        self.get_access_flag() & ACC_PRIVATE == ACC_PRIVATE
    }

    pub fn is_protected(&self) -> bool {
        self.get_access_flag() & ACC_PROTECTED == ACC_PROTECTED
    }

    pub fn is_final(&self) -> bool {
        self.get_access_flag() & ACC_FINAL == ACC_FINAL
    }

    pub fn is_static(&self) -> bool {
        self.get_access_flag() & ACC_STATIC == ACC_STATIC
    }

    pub fn is_abstract(&self) -> bool {
        self.get_access_flag() & ACC_ABSTRACT == ACC_ABSTRACT
    }

    pub fn is_interface(&self) -> bool {
        self.get_access_flag() & ACC_INTERFACE == ACC_INTERFACE
    }

    pub fn get_class_loader(&self) -> Rc<ClassLoader> {
        self.0.borrow()._class_loader.clone()
    }

    pub fn get_source_file(&self) -> &str {
        &self.0.borrow()._source_file
    }

    pub fn get_signature(&self) -> &str {
        &self.0.borrow()._signature
    }

    pub fn get_runtime_pool(&self) -> &RuntimeConstantPool {
        &self.0.borrow()._runtime_pool
    }

    pub fn get_static_fields(&self) -> &HashMap<String, FieldId> {
        &self.0.borrow()._static_fields
    }

    pub fn get_instance_fields(&self) -> &HashMap<String, FieldId> {
        &self.0.borrow()._instance_fields
    }

    pub fn get_declared_methods(&self) -> &HashMap<String, MethodId> {
        &self.0.borrow()._all_methods
    }

    pub fn get_interfaces(&self) -> &HashMap<String, InstanceKlass> {
        &self.0.borrow()._interfaces
    }

    pub fn get_bootstrap_methods(&self) -> &BootStrapMethodsAttribute {
        &self.0.borrow()._bootstrap_method_attr
    }

    pub fn get_enclosing_method(&self) -> &EnclosingMethodAttribute {
        &self.0.borrow()._enclosing_method_attr
    }

    pub fn get_inner_classes(&self) -> &InnerClassesAttribute {
        &self.0.borrow()._inner_class_attr
    }

    pub fn get_declared_methods_by_offset(&self, offset: usize) -> Option<&MethodId> {
        self.0
            .borrow()
            ._all_methods
            .values()
            .find(|method| method.get_offset() == offset)
    }

    pub fn get_this_class_field(&self, name: &str, descriptor: &str) -> Option<FieldId> {
        let key = format!("{} {} {}", self.get_name(), name, descriptor);
        self.0.borrow()._instance_fields.get(&key).cloned()
    }

    pub fn get_static_field_offset(&self, name: &str, descriptor: &str) -> Option<usize> {
        let key = format!("{} {} {}", self.get_name(), name, descriptor);
        self.0.borrow()._static_fields.get(&key).map(|field| field.get_offset())
    }

    /**
   * Get static field info.
   * @param className Where the wanted field belongs to
   * @param name Field name
   * @param descriptor Field descriptor
   * @return FieldID if found, otherwise {@code FieldID(-1, nullptr)}
   */
    pub fn get_static_field_Info(&self, class_name: &str, name: &str, descriptor: &str) -> Option<Field> {
        let key = format!("{} {} {}", class_name, name, descriptor);
        self.0.borrow()._static_fields.get(&key).map(|field| field.clone())
    }
}
