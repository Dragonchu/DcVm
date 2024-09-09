use std::any::Any;

enum SlotData {
    Jint(i32),
    Jobject(Box<dyn Any>),
}
struct Slot {
    is_object: bool,
    slot_data: SlotData,
}

struct SlotArray {
    _elements: Vec<Slot>,
}

impl SlotArray {
    fn new() -> SlotArray {
        SlotArray {
            _elements: Vec::new(),
        }
    }
}
