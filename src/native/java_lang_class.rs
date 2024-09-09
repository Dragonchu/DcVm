pub mod java {
    pub mod lang {
        use crate::oop::mirror_oop::MirrorOopDesc;
        use std::collections::HashMap;

        enum ClassMirrorState {
            Fixed,
            NotFixed,
        }
        pub struct Class<'a> {
            _mirrors: Vec<&'a str>,
            _primitive_type_mirrors: HashMap<String, &'a MirrorOopDesc>,
            _class_mirror_state: ClassMirrorState,
        }
        
        impl<'a> Class<'a> {
            pub fn new() -> Self {
                Class {
                    _mirrors: vec!["I", "Z", "B", "C", "S", "F", "J", "D", "V", "[I", "[Z", "[B", "[C", "[S", "[F", "[J", "[D"],
                    _primitive_type_mirrors: Default::default(),
                    _class_mirror_state: ClassMirrorState::NotFixed,
                }
            }
        }
        
    }
}
