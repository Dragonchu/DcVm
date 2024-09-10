use crate::classfile::attribute_info::{Annotation, ElementValuePair};

use super::{
    attribute_info::{
        AppendFrame, AttributeInfo, ChopFrame, CodeAttribute, ConstantValueAttribute,
        DeprecatedAttribute, EnclosingMethodAttribute, ExceptionTableEntry, ExceptionsAttribute,
        FullFrame, InnerClassInfo, InnerClassesAttribute, LineNumberTableAttribute,
        LineNumberTableEntry, LocalVariableTableAttribute, LocalVariableTableEntry,
        RuntimeVisibleAnnotationsAttribute, SameFrame, SameFrameExtended,
        SameLocals1StackItemFrame, SameLocals1StackItemFrameExtended, SignatureAttribute,
        SourceDebugExtensionAttribute, SourceFileAttribute, StackMapFrame, SyntheticAttribute,
        VerificationTypeInfo,
    },
    class_file::{
        ClassFile, ConstantClassInfo, ConstantDoubleInfo, ConstantFieldrefInfo, ConstantFloatInfo,
        ConstantInfoTag, ConstantIntegerInfo, ConstantInterfaceMethodrefInfo,
        ConstantInvokeDynamicInfo, ConstantLongInfo, ConstantMethodHandleInfo,
        ConstantMethodTypeInfo, ConstantMethodrefInfo, ConstantNameAndTypeInfo, ConstantStringInfo,
        ConstantUtf8Info, CpInfo, FieldInfo, MethodInfo,
    },
    types::{U1, U2, U4, U8},
};

struct ClassFileParser {
    pub class_file_stream: Vec<u8>,
}

trait ClassReader {
    fn read_u1(&mut self) -> u8;
    fn read_u2(&mut self) -> u16;
    fn read_u4(&mut self) -> u32;
    fn read_u8(&mut self) -> u64;
}

impl ClassReader for Vec<u8> {
    fn read_u1(&mut self) -> U1 {
        self.remove(0)
    }

    fn read_u2(&mut self) -> U2 {
        let byte1 = self.remove(0) as u16;
        let byte2 = self.remove(0) as u16;
        (byte1 << 8) | byte2
    }

    fn read_u4(&mut self) -> U4 {
        let byte1 = self.remove(0) as u32;
        let byte2 = self.remove(0) as u32;
        let byte3 = self.remove(0) as u32;
        let byte4 = self.remove(0) as u32;
        (byte1 << 24) | (byte2 << 16) | (byte3 << 8) | byte4
    }

    fn read_u8(&mut self) -> U8 {
        let byte1 = self.remove(0) as u64;
        let byte2 = self.remove(0) as u64;
        let byte3 = self.remove(0) as u64;
        let byte4 = self.remove(0) as u64;
        let byte5 = self.remove(0) as u64;
        let byte6 = self.remove(0) as u64;
        let byte7 = self.remove(0) as u64;
        let byte8 = self.remove(0) as u64;
        (byte1 << 56)
            | (byte2 << 48)
            | (byte3 << 40)
            | (byte4 << 32)
            | (byte5 << 24)
            | (byte6 << 16)
            | (byte7 << 8)
            | byte8
    }
}

impl ClassFileParser {
    pub fn read(class_file_path: String) -> Self {
        let class_file_stream = std::fs::read(class_file_path).expect("Failed to read class file");
        Self { class_file_stream }
    }

    pub fn new(class_file_stream: Vec<u8>) -> Self {
        Self { class_file_stream }
    }

    pub fn parse(&mut self) -> ClassFile {
        let magic = self.class_file_stream.read_u4();
        let minor_version = self.class_file_stream.read_u2();
        let major_version = self.class_file_stream.read_u2();
        let constant_pool_count = self.class_file_stream.read_u2();
        let constant_pool = self.parse_constant_pool(constant_pool_count);
        let access_flags = self.class_file_stream.read_u2();
        let this_class = self.class_file_stream.read_u2();
        let super_class = self.class_file_stream.read_u2();
        let interfaces_count = self.class_file_stream.read_u2();
        let interfaces = self.parse_interfaces(interfaces_count);
        let fields_count = self.class_file_stream.read_u2();
        let fields = self.parse_fields(fields_count);
        let methods_count = self.class_file_stream.read_u2();
        let methods = self.parse_methods(methods_count);
        let attributes_count = self.class_file_stream.read_u2();
        let attributes = self.parse_attributes(attributes_count);
        ClassFile::new(
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes,
        )
    }

    pub fn parse_constant_pool(&mut self, constant_pool_count: U2) -> Vec<CpInfo> {
        let mut constant_pool: Vec<CpInfo> = Vec::new();
        let mut i = 1;
        for _ in 0..constant_pool_count - 1 {
            let tag = self
                .class_file_stream
                .read_u1()
                .try_into()
                .expect("Invalid tag");
            match tag {
                ConstantInfoTag::ConstantUtf8 => {
                    let length = self.class_file_stream.read_u2();
                    let bytes = self.class_file_stream.drain(0..length as usize).collect();
                    constant_pool.push(CpInfo::Utf8(ConstantUtf8Info::new(length, bytes)));
                }
                ConstantInfoTag::ConstantInteger => {
                    let value = self.class_file_stream.read_u4();
                    constant_pool.push(CpInfo::Integer(ConstantIntegerInfo::new(value)));
                }
                ConstantInfoTag::ConstantFloat => {
                    let value = self.class_file_stream.read_u4();
                    constant_pool.push(CpInfo::Float(ConstantFloatInfo::new(value)));
                }
                ConstantInfoTag::ConstantLong => {
                    let high_bytes = self.class_file_stream.read_u4();
                    let low_bytes = self.class_file_stream.read_u4();
                    constant_pool.push(CpInfo::Long(ConstantLongInfo::new(high_bytes, low_bytes)));
                    i += 1;
                }
                ConstantInfoTag::ConstantDouble => {
                    let high_bytes = self.class_file_stream.read_u4();
                    let low_bytes = self.class_file_stream.read_u4();
                    constant_pool.push(CpInfo::Double(ConstantDoubleInfo::new(
                        high_bytes, low_bytes,
                    )));
                    i += 1;
                }
                ConstantInfoTag::ConstantClass => {
                    let name_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::Class(ConstantClassInfo::new(name_index)));
                }
                ConstantInfoTag::ConstantString => {
                    let string_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::String(ConstantStringInfo::new(string_index)));
                }
                ConstantInfoTag::ConstantFieldref => {
                    let class_index = self.class_file_stream.read_u2();
                    let name_and_type_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::FieldRef(ConstantFieldrefInfo::new(
                        class_index,
                        name_and_type_index,
                    )));
                }
                ConstantInfoTag::ConstantMethodref => {
                    let class_index = self.class_file_stream.read_u2();
                    let name_and_type_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::MethodRef(ConstantMethodrefInfo::new(
                        class_index,
                        name_and_type_index,
                    )));
                }
                ConstantInfoTag::ConstantInterfaceMethodref => {
                    let class_index = self.class_file_stream.read_u2();
                    let name_and_type_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::InterfaceMethodRef(
                        ConstantInterfaceMethodrefInfo::new(class_index, name_and_type_index),
                    ));
                }
                ConstantInfoTag::ConstantNameAndType => {
                    let name_index = self.class_file_stream.read_u2();
                    let descriptor_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::NameAndType(ConstantNameAndTypeInfo::new(
                        name_index,
                        descriptor_index,
                    )));
                }
                ConstantInfoTag::ConstantMethodHandle => {
                    let reference_kind = self.class_file_stream.read_u1();
                    let reference_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::MethodHandle(ConstantMethodHandleInfo::new(
                        reference_kind,
                        reference_index,
                    )));
                }
                ConstantInfoTag::ConstantMethodType => {
                    let descriptor_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::MethodType(ConstantMethodTypeInfo::new(
                        descriptor_index,
                    )));
                }
                ConstantInfoTag::ConstantInvokeDynamic => {
                    let bootstrap_method_attr_index = self.class_file_stream.read_u2();
                    let name_and_type_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::InvokeDynamic(ConstantInvokeDynamicInfo::new(
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    )));
                }
            }
        }
        constant_pool
    }

    pub fn parse_interfaces(&mut self, interfaces_count: U2) -> Vec<U2> {
        let mut interfaces: Vec<U2> = Vec::new();
        for _ in 0..interfaces_count {
            interfaces.push(self.class_file_stream.read_u2());
        }
        interfaces
    }

    pub fn parse_fields(&mut self, fields_count: U2) -> Vec<FieldInfo> {
        let mut fields: Vec<FieldInfo> = Vec::new();
        for _ in 0..fields_count {
            let access_flags = self.class_file_stream.read_u2();
            let name_index = self.class_file_stream.read_u2();
            let descriptor_index = self.class_file_stream.read_u2();
            let attributes_count = self.class_file_stream.read_u2();
            let attributes = self.parse_attributes(attributes_count);
            fields.push(FieldInfo::new(
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            ));
        }
        fields
    }

    pub fn parse_methods(&mut self, methods_count: U2) -> Vec<MethodInfo> {
        let mut methods: Vec<MethodInfo> = Vec::new();
        for _ in 0..methods_count {
            let access_flags = self.class_file_stream.read_u2();
            let name_index = self.class_file_stream.read_u2();
            let descriptor_index = self.class_file_stream.read_u2();
            let attributes_count = self.class_file_stream.read_u2();
            let attributes = self.parse_attributes(attributes_count);
            methods.push(MethodInfo::new(
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            ));
        }
        methods
    }

    pub fn parse_attributes(
        &mut self,
        attributes_count: U2,
        constant_pool: &Vec<CpInfo>,
    ) -> Vec<AttributeInfo> {
        let mut attributes: Vec<AttributeInfo> = Vec::new();
        for _ in 0..attributes_count {
            let attribute_name_index = self.class_file_stream.read_u2();
            let attribute_length = self.class_file_stream.read_u4();
            let attribute_name = &constant_pool[attribute_name_index as usize];
            match attribute_name {
                CpInfo::Utf8(utf8_info) => {
                    let attribute_name = String::from_utf8(utf8_info.bytes.clone()).unwrap();
                    match attribute_name.as_str() {
                        "ConstantValue" => {
                            attributes.push(self.read_constant_value_attribute(
                                attribute_name_index,
                                attribute_length,
                            ));
                        }
                        "Code" => {
                            attributes.push(self.read_code_attribute(
                                attribute_name_index,
                                attribute_length,
                                constant_pool,
                            ));
                        }
                        "StackMapTable" => {
                            let number_of_entries = self.class_file_stream.read_u2();
                            let stack_map_table = self.parse_stack_map_table(
                                number_of_entries,
                                attribute_name_index,
                                attribute_length,
                            );
                            attributes.push(AttributeInfo::StackMapTable(stack_map_table));
                        }
                        "Exceptions" => {
                            let number_of_exceptions = self.class_file_stream.read_u2();
                            let mut exceptions = Vec::new();
                            for _ in 0..number_of_exceptions {
                                exceptions.push(self.class_file_stream.read_u2());
                            }
                            attributes.push(AttributeInfo::Exceptions(ExceptionsAttribute::new(
                                attribute_name_index,
                                attribute_length,
                                number_of_exceptions,
                                exceptions,
                            )));
                        }
                        "InnerClasses" => {
                            let number_of_classes = self.class_file_stream.read_u2();
                            let mut classes = Vec::new();
                            for _ in 0..number_of_classes {
                                let inner_class_info_index = self.class_file_stream.read_u2();
                                let outer_class_info_index = self.class_file_stream.read_u2();
                                let inner_name_index = self.class_file_stream.read_u2();
                                let inner_class_access_flags = self.class_file_stream.read_u2();
                                classes.push(InnerClassInfo::new(
                                    inner_class_info_index,
                                    outer_class_info_index,
                                    inner_name_index,
                                    inner_class_access_flags,
                                ));
                            }
                            attributes.push(AttributeInfo::InnerClasses(
                                InnerClassesAttribute::new(
                                    attribute_name_index,
                                    attribute_length,
                                    number_of_classes,
                                    classes,
                                ),
                            ));
                        }
                        "EnclosingMethod" => {
                            let class_index = self.class_file_stream.read_u2();
                            let method_index = self.class_file_stream.read_u2();
                            attributes.push(AttributeInfo::EnclosingMethod(
                                EnclosingMethodAttribute::new(
                                    attribute_name_index,
                                    attribute_length,
                                    class_index,
                                    method_index,
                                ),
                            ));
                        }
                        "Synthetic" => {
                            attributes.push(AttributeInfo::Synthetic(SyntheticAttribute::new(
                                attribute_name_index,
                                attribute_length,
                            )));
                        }
                        "Signature" => {
                            let signature_index = self.class_file_stream.read_u2();
                            attributes.push(AttributeInfo::Signature(SignatureAttribute::new(
                                attribute_name_index,
                                attribute_length,
                                signature_index,
                            )));
                        }
                        "SourceFile" => {
                            let source_file_index = self.class_file_stream.read_u2();
                            attributes.push(AttributeInfo::SourceFile(SourceFileAttribute::new(
                                attribute_name_index,
                                attribute_length,
                                source_file_index,
                            )));
                        }
                        "SourceDebugExtension" => {
                            let debug_extension = self
                                .class_file_stream
                                .drain(0..attribute_length as usize)
                                .collect();
                            attributes.push(AttributeInfo::SourceDebugExtension(
                                SourceDebugExtensionAttribute::new(
                                    attribute_name_index,
                                    attribute_length,
                                    debug_extension,
                                ),
                            ));
                        }
                        "LineNumberTable" => {
                            let line_number_table_length = self.class_file_stream.read_u2();
                            let mut line_number_table = Vec::new();
                            for _ in 0..line_number_table_length {
                                let start_pc = self.class_file_stream.read_u2();
                                let line_number = self.class_file_stream.read_u2();
                                line_number_table
                                    .push(LineNumberTableEntry::new(start_pc, line_number));
                            }
                            attributes.push(AttributeInfo::LineNumberTable(
                                LineNumberTableAttribute::new(
                                    attribute_name_index,
                                    attribute_length,
                                    line_number_table_length,
                                    line_number_table,
                                ),
                            ));
                        }
                        "LocalVariableTable" => {
                            let local_variable_table_length = self.class_file_stream.read_u2();
                            let mut local_variable_table = Vec::new();
                            for _ in 0..local_variable_table_length {
                                let start_pc = self.class_file_stream.read_u2();
                                let length = self.class_file_stream.read_u2();
                                let name_index = self.class_file_stream.read_u2();
                                let descriptor_index = self.class_file_stream.read_u2();
                                let index = self.class_file_stream.read_u2();
                                local_variable_table.push(LocalVariableTableEntry::new(
                                    start_pc,
                                    length,
                                    name_index,
                                    descriptor_index,
                                    index,
                                ));
                            }
                            attributes.push(AttributeInfo::LocalVariableTable(
                                LocalVariableTableAttribute::new(
                                    attribute_name_index,
                                    attribute_length,
                                    local_variable_table_length,
                                    local_variable_table,
                                ),
                            ));
                        }
                        "LocalVariableTypeTable" => {
                            let local_variable_type_table_length = self.class_file_stream.read_u2();
                            let mut local_variable_type_table = Vec::new();
                            for _ in 0..local_variable_type_table_length {
                                let start_pc = self.class_file_stream.read_u2();
                                let length = self.class_file_stream.read_u2();
                                let name_index = self.class_file_stream.read_u2();
                                let signature_index = self.class_file_stream.read_u2();
                                let index = self.class_file_stream.read_u2();
                                local_variable_type_table.push(LocalVariableTableEntry::new(
                                    start_pc,
                                    length,
                                    name_index,
                                    signature_index,
                                    index,
                                ));
                            }
                            attributes.push(AttributeInfo::LocalVariableTable(
                                LocalVariableTableAttribute::new(
                                    attribute_name_index,
                                    attribute_length,
                                    local_variable_type_table_length,
                                    local_variable_type_table,
                                ),
                            ));
                        }
                        "Deprecated" => {
                            attributes.push(AttributeInfo::Deprecated(DeprecatedAttribute::new(
                                attribute_name_index,
                                attribute_length,
                            )));
                        }
                        "RuntimeVisibleAnnotations" => {
                            let num_annotations = self.class_file_stream.read_u2();
                            let mut annotations = Vec::new();
                            for _ in 0..num_annotations {
                                let type_index = self.class_file_stream.read_u2();
                                let num_element_value_pairs = self.class_file_stream.read_u2();
                                let mut element_value_pairs = Vec::new();
                                for _ in 0..num_element_value_pairs {
                                    let element_name_index = self.class_file_stream.read_u2();
                                    let element_value = self.class_file_stream.read_u2();
                                    element_value_pairs.push(ElementValuePair::new(
                                        element_name_index,
                                        element_value,
                                    ));
                                }
                                annotations.push(Annotation::new(
                                    type_index,
                                    num_element_value_pairs,
                                    element_value_pairs,
                                ));
                            }
                            attributes.push(AttributeInfo::RuntimeVisibleAnnotations(
                                RuntimeVisibleAnnotationsAttribute::new(
                                    attribute_name_index,
                                    attribute_length,
                                    num_annotations,
                                    annotations,
                                ),
                            ));
                        }
                        "RuntimeInvisibleAnnotations" => {

                        }
                        "RuntimeVisibleParameterAnnotations" => {}
                        "RuntimeInvisibleParameterAnnotations" => {}
                        "AnnotationDefault" => {}
                        "BootstrapMethods" => {}
                        "MethodParameters" => {}
                        _ => {
                            panic!("Invalid attribute name")
                        }
                    }
                }
                _ => panic!("Invalid attribute name"),
            }
        }
        attributes
    }

    fn read_constant_value_attribute(
        &mut self,
        attribute_name_index: U2,
        attribute_length: U4,
    ) -> AttributeInfo {
        let constant_value_index = self.class_file_stream.read_u2();
        AttributeInfo::ConstantValue(ConstantValueAttribute::new(
            attribute_name_index,
            attribute_length,
            constant_value_index,
        ))
    }

    fn read_code_attribute(
        &mut self,
        attribute_name_index: U2,
        attribute_length: U4,
        constant_pool: &Vec<CpInfo>,
    ) -> AttributeInfo {
        let max_stack = self.class_file_stream.read_u2();
        let max_locals = self.class_file_stream.read_u2();
        let code_length = self.class_file_stream.read_u4();
        let code = self
            .class_file_stream
            .drain(0..code_length as usize)
            .collect();
        let exception_table_length = self.class_file_stream.read_u2();
        let exception_table = self.parse_exception_table(exception_table_length);
        let attributes_count = self.class_file_stream.read_u2();
        let attributes = self.parse_attributes(attributes_count, constant_pool);
        AttributeInfo::Code(CodeAttribute::new(
            attribute_name_index,
            attribute_length,
            max_stack,
            max_locals,
            code_length,
            code,
            exception_table_length,
            exception_table,
            attributes_count,
            attributes,
        ))
    }

    fn parse_exception_table(&mut self, exception_table_length: U2) -> Vec<ExceptionTableEntry> {
        let mut exception_table: Vec<ExceptionTableEntry> = Vec::new();
        for _ in 0..exception_table_length {
            let start_pc = self.class_file_stream.read_u2();
            let end_pc = self.class_file_stream.read_u2();
            let handler_pc = self.class_file_stream.read_u2();
            let catch_type = self.class_file_stream.read_u2();
            exception_table.push(ExceptionTableEntry::new(
                start_pc, end_pc, handler_pc, catch_type,
            ));
        }
        exception_table
    }

    fn parse_stack_map_table(
        &mut self,
        number_of_entries: U2,
        attribute_name_index: U2,
        attribute_length: U4,
    ) -> Vec<StackMapFrame> {
        let mut stack_map_table: Vec<StackMapFrame> = Vec::new();
        for _ in 0..number_of_entries {
            let frame_type = self.class_file_stream.read_u1();
            if frame_type >= 0 && frame_type <= 63 {
                // SAME
                stack_map_table.push(StackMapFrame::SameFrame(SameFrame::new(frame_type)))
            } else if frame_type >= 64 && frame_type <= 127 {
                // SAME_LOCALS_1_STACK_ITEM
                let tag = self.class_file_stream.read_u1();
                let verify_type_info = self.parse_verification_type_info(tag);
                stack_map_table.push(StackMapFrame::SameLocals1StackItemFrame(SameLocals1StackItemFrame::new(
                    frame_type,
                    verify_type_info,
                )))
            } else if frame_type == 247 {
                // SAME_LOCALS_1_STACK_ITEM_EXTENDED
                let offset_delta = self.class_file_stream.read_u2();
                let tag = self.class_file_stream.read_u1();
                let verify_type_info = self.parse_verification_type_info(tag);
                stack_map_table.push(StackMapFrame::SameLocals1StackItemFrameExtended(
                    SameLocals1StackItemFrameExtended::new(
                        frame_type,
                        offset_delta,
                        verify_type_info,
                    ),
                ))
            } else if frame_type >= 248 && frame_type <= 250 {
                // CHOP
                let offset_delta = self.class_file_stream.read_u2();
                StackMapFrame::ChopFrame(ChopFrame::new(frame_type, offset_delta))
            } else if frame_type == 251 {
                // SAME_FRAME_EXTENDED
                let offset_delta = self.class_file_stream.read_u2();
                StackMapFrame::SameFrameExtended(SameFrameExtended::new(frame_type, offset_delta))
            } else if frame_type >= 252 && frame_type <= 254 {
                // APPEND
                let offset_delta = self.class_file_stream.read_u2();
                let mut locals = Vec::new();
                for _ in 0..frame_type - 251 {
                    let tag = self.class_file_stream.read_u1();
                    let verify_type_info = self.parse_verification_type_info(tag);
                    locals.push(verify_type_info);
                }
                StackMapFrame::AppendFrame(AppendFrame::new(frame_type, offset_delta, locals))
            } else if frame_type == 255 {
                // FULL_FRAME
                let offset_delta = self.class_file_stream.read_u2();
                let number_of_locals = self.class_file_stream.read_u2();
                let mut locals = Vec::new();
                for _ in 0..number_of_locals {
                    let tag = self.class_file_stream.read_u1();
                    let verify_type_info = self.parse_verification_type_info(tag);
                    locals.push(verify_type_info);
                }
                let number_of_stack_items = self.class_file_stream.read_u2();
                let mut stack = Vec::new();
                for _ in 0..number_of_stack_items {
                    let tag = self.class_file_stream.read_u1();
                    let verify_type_info = self.parse_verification_type_info(tag);
                    stack.push(verify_type_info);
                }
                StackMapFrame::FullFrame(FullFrame::new(
                    frame_type,
                    offset_delta,
                    number_of_locals,
                    locals,
                    number_of_stack_items,
                    stack,
                ))
            }
        }
        stack_map_table
    }

    fn parse_verification_type_info(&mut self, tag: U1) -> VerificationTypeInfo {
        match tag {
            0 => VerificationTypeInfo::TopVariable { tag: 0 },
            1 => VerificationTypeInfo::IntegerVariable { tag: 1 },
            2 => VerificationTypeInfo::FloatVariable { tag: 2 },
            3 => VerificationTypeInfo::DoubleVariable { tag: 3 },
            4 => VerificationTypeInfo::LongVariable { tag: 4 },
            5 => VerificationTypeInfo::NullVariable { tag: 5 },
            6 => VerificationTypeInfo::UninitializedThisVariable { tag: 6 },
            7 => {
                let cpool_index = self.class_file_stream.read_u2();
                VerificationTypeInfo::ObjectVariable {
                    tag: 7,
                    cpool_index,
                }
            }
            8 => {
                let offset = self.class_file_stream.read_u2();
                VerificationTypeInfo::UninitializedVariable { tag: 8, offset }
            }
            _ => panic!("Invalid tag"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn parse_main_class() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/Main.class");
        let mut parser = ClassFileParser::read(d.display().to_string());
        let cf = parser.parse();
        print!("{}", cf);
    }
}
