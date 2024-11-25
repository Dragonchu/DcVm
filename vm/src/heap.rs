use reader::class_file::ClassFile;

enum Oop<'a> {
    InstanceOop(&'a InstanceOopDesc<'a>),
    ObjArrayOop(&'a ObjArrayOopDesc<'a>),
    TypeArrayOop(&'a TypeArrayOopDesc<'a>)
}

struct InstanceKlassDesc;
struct ObjArrayKlassDesc;
struct TypeArrayKlassDesc;



struct InstanceOopDesc<'a> {
    fields: Vec<Oop<'a>>,
    klass: InstanceKlassDesc
}

struct ObjArrayOopDesc<'a> {
    elements: Vec<Oop<'a>>,
    klass: ObjArrayKlassDesc
}

struct TypeArrayOopDesc<'a> {
    elements: Vec<Oop<'a>>,
    klass: TypeArrayKlassDesc
}

impl<'a> InstanceOopDesc<'a> {
    fn set_field_value(&mut self, class_name: &str, field_name: &str, field_descriptor: &str) {
        todo!()
    }

    fn get_field_value(&self, class_name: &str, field_name: &str, field_descriptor: &str) -> &'a Oop {
        todo!()
    }
}

trait ArrayOop<'a> {
    fn get_dimension(&self) -> usize;
    fn get_length(&self) -> usize;
    fn get_element_at(&'a self, position: usize) -> &'a Oop;
    fn set_element_at(&mut self, element: Oop);
}

impl<'a> ArrayOop<'a> for ObjArrayOopDesc<'a> {
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

impl<'a> ArrayOop<'a> for TypeArrayOopDesc<'a> {
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

pub struct Heap;