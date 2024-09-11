use std::{
    fs::File,
    io::{BufReader, Read},
};

use zip::read::ZipFile;

use crate::classfile::attribute_info::{Annotation, ElementValueItem};

use super::{
    attribute_info::{
        AttributeInfo, ElementValue, StackMapFrame, TargetInfo, TypeAnnotation, TypePath,
        VerificationTypeInfo,
    },
    class_file::{ClassFile, ConstantInfoTag, CpInfo, FieldInfo, MethodInfo},
    types::{U1, U2, U4},
};

enum ClassFileStream<'a> {
    File(BufReader<File>),
    Zip(ZipFile<'a>),
}

impl<'a> ClassReader for ClassFileStream<'a> {
    fn read_u1(&mut self) -> U1 {
        match self {
            ClassFileStream::File(file) => file.read_u1(),
            ClassFileStream::Zip(zip) => zip.read_u1(),
        }
    }

    fn read_u2(&mut self) -> U2 {
        match self {
            ClassFileStream::File(file) => file.read_u2(),
            ClassFileStream::Zip(zip) => zip.read_u2(),
        }
    }

    fn read_u4(&mut self) -> U4 {
        match self {
            ClassFileStream::File(file) => file.read_u4(),
            ClassFileStream::Zip(zip) => zip.read_u4(),
        }
    }

    fn read_n(&mut self, size: usize) -> Vec<u8> {
        match self {
            ClassFileStream::File(file) => file.read_n(size),
            ClassFileStream::Zip(zip) => zip.read_n(size),
        }
    }
}

pub struct ClassFileParser<'a> {
    pub class_file_stream: ClassFileStream<'a>,
}

trait ClassReader {
    fn read_u1(&mut self) -> u8;
    fn read_u2(&mut self) -> u16;
    fn read_u4(&mut self) -> u32;
    fn read_n(&mut self, size: usize) -> Vec<u8>;
}

impl ClassReader for BufReader<File> {
    fn read_u1(&mut self) -> U1 {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer).expect("Failed to read u1");
        buffer[0]
    }

    fn read_u2(&mut self) -> U2 {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer).expect("Failed to read u2");
        ((buffer[0] as u16) << 8) | buffer[1] as u16
    }

    fn read_u4(&mut self) -> U4 {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer).expect("Failed to read u4");
        ((buffer[0] as u32) << 24)
            | ((buffer[1] as u32) << 16)
            | ((buffer[2] as u32) << 8)
            | buffer[3] as u32
    }

    fn read_n(&mut self, size: usize) -> Vec<u8> {
        let mut buffer = vec![0; size];
        self.read_exact(&mut buffer).expect("Failed to read");
        buffer
    }
}

impl ClassReader for ZipFile<'_> {
    fn read_u1(&mut self) -> U1 {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer).expect("Failed to read u1");
        buffer[0]
    }

    fn read_u2(&mut self) -> U2 {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer).expect("Failed to read u2");
        ((buffer[0] as u16) << 8) | buffer[1] as u16
    }

    fn read_u4(&mut self) -> U4 {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer).expect("Failed to read u4");
        ((buffer[0] as u32) << 24)
            | ((buffer[1] as u32) << 16)
            | ((buffer[2] as u32) << 8)
            | buffer[3] as u32
    }

    fn read_n(&mut self, size: usize) -> Vec<u8> {
        let mut buffer = vec![0; size];
        self.read_exact(&mut buffer).expect("Failed to read");
        buffer
    }
}

impl<'a> ClassFileParser<'a> {
    pub fn file(class_file_stream: BufReader<File>) -> Self {
        Self {
            class_file_stream: ClassFileStream::File(class_file_stream),
        }
    }

    pub fn zip(zip_file: ZipFile<'a>) -> Self {
        Self {
            class_file_stream: ClassFileStream::Zip(zip_file),
        }
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
        let fields = self.parse_fields(fields_count, &constant_pool);
        let methods_count = self.class_file_stream.read_u2();
        let methods = self.parse_methods(methods_count, &constant_pool);
        let attributes_count = self.class_file_stream.read_u2();
        let attributes = self.parse_attributes(attributes_count, &constant_pool);
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

    fn parse_constant_pool(&mut self, constant_pool_count: U2) -> Vec<CpInfo> {
        let mut constant_pool: Vec<CpInfo> = Vec::new();
        let mut i = 0;
        while i < constant_pool_count - 1 {
            let tag = match self.class_file_stream.read_u1().try_into() {
                Ok(tag) => tag,
                Err(_) => {
                    println!("parsed constant_pool count: \n{:?}", constant_pool.len());
                    for (i, cp) in constant_pool.iter().enumerate() {
                        println!("{}: {:?}", i+1, cp);
                    }
                    panic!("Invalid tag")
                },
            };
            match tag {
                ConstantInfoTag::ConstantUtf8 => {
                    let length = self.class_file_stream.read_u2();
                    let bytes = self.class_file_stream.read_n(length as usize);
                    constant_pool.push(CpInfo::Utf8 {
                        tag: tag as u8,
                        length,
                        bytes,
                    });
                }
                ConstantInfoTag::ConstantInteger => {
                    let bytes = self.class_file_stream.read_u4();
                    constant_pool.push(CpInfo::Integer {
                        tag: tag as u8,
                        bytes,
                    });
                }
                ConstantInfoTag::ConstantFloat => {
                    let bytes = self.class_file_stream.read_u4();
                    constant_pool.push(CpInfo::Float {
                        tag: tag as u8,
                        bytes,
                    });
                }
                ConstantInfoTag::ConstantLong => {
                    let high_bytes = self.class_file_stream.read_u4();
                    let low_bytes = self.class_file_stream.read_u4();
                    constant_pool.push(CpInfo::Long {
                        tag: tag as u8,
                        high_bytes,
                        low_bytes,
                    });
                    constant_pool.push(CpInfo::Padding);
                    i += 1;
                }
                ConstantInfoTag::ConstantDouble => {
                    let high_bytes = self.class_file_stream.read_u4();
                    let low_bytes = self.class_file_stream.read_u4();
                    constant_pool.push(CpInfo::Double {
                        tag: tag as u8,
                        high_bytes,
                        low_bytes,
                    });
                    constant_pool.push(CpInfo::Padding);
                    i += 1;
                }
                ConstantInfoTag::ConstantClass => {
                    let name_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::Class {
                        tag: tag as u8,
                        name_index,
                    });
                }
                ConstantInfoTag::ConstantString => {
                    let string_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::String {
                        tag: tag as u8,
                        string_index,
                    });
                }
                ConstantInfoTag::ConstantFieldref => {
                    let class_index = self.class_file_stream.read_u2();
                    let name_and_type_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::FieldRef {
                        tag: tag as u8,
                        class_index,
                        name_and_type_index,
                    });
                }
                ConstantInfoTag::ConstantMethodref => {
                    let class_index = self.class_file_stream.read_u2();
                    let name_and_type_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::MethodRef {
                        tag: tag as u8,
                        class_index,
                        name_and_type_index,
                    });
                }
                ConstantInfoTag::ConstantInterfaceMethodref => {
                    let class_index = self.class_file_stream.read_u2();
                    let name_and_type_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::InterfaceMethodRef {
                        tag: tag as u8,
                        class_index,
                        name_and_type_index,
                    });
                }
                ConstantInfoTag::ConstantNameAndType => {
                    let name_index = self.class_file_stream.read_u2();
                    let descriptor_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::NameAndType {
                        tag: tag as u8,
                        name_index,
                        descriptor_index,
                    });
                }
                ConstantInfoTag::ConstantMethodHandle => {
                    let reference_kind = self.class_file_stream.read_u1();
                    let reference_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::MethodHandle {
                        tag: tag as u8,
                        reference_kind,
                        reference_index,
                    });
                }
                ConstantInfoTag::ConstantMethodType => {
                    let descriptor_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::MethodType {
                        tag: tag as u8,
                        descriptor_index,
                    });
                }
                ConstantInfoTag::ConstantInvokeDynamic => {
                    let bootstrap_method_attr_index = self.class_file_stream.read_u2();
                    let name_and_type_index = self.class_file_stream.read_u2();
                    constant_pool.push(CpInfo::InvokeDynamic {
                        tag: tag as u8,
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    });
                }
            }
            i += 1;
        }
        constant_pool
    }

    fn parse_interfaces(&mut self, interfaces_count: U2) -> Vec<U2> {
        let mut interfaces: Vec<U2> = Vec::new();
        for _ in 0..interfaces_count {
            interfaces.push(self.class_file_stream.read_u2());
        }
        interfaces
    }

    fn parse_fields(&mut self, fields_count: U2, const_pool: &Vec<CpInfo>) -> Vec<FieldInfo> {
        let mut fields: Vec<FieldInfo> = Vec::new();
        for _ in 0..fields_count {
            let access_flags = self.class_file_stream.read_u2();
            let name_index = self.class_file_stream.read_u2();
            let descriptor_index = self.class_file_stream.read_u2();
            let attributes_count = self.class_file_stream.read_u2();
            let attributes = self.parse_attributes(attributes_count, const_pool);
            fields.push(FieldInfo::new(
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            ));
        }
        fields
    }

    fn parse_methods(&mut self, methods_count: U2, const_pool: &Vec<CpInfo>) -> Vec<MethodInfo> {
        let mut methods: Vec<MethodInfo> = Vec::new();
        for _ in 0..methods_count {
            let access_flags = self.class_file_stream.read_u2();
            let name_index = self.class_file_stream.read_u2();
            let descriptor_index = self.class_file_stream.read_u2();
            let attributes_count = self.class_file_stream.read_u2();
            let attributes = self.parse_attributes(attributes_count, const_pool);
            methods.push(MethodInfo::new(
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            ));
        }
        methods
    }

    fn parse_attributes(
        &mut self,
        attributes_count: U2,
        constant_pool: &Vec<CpInfo>,
    ) -> Vec<AttributeInfo> {
        let mut attributes: Vec<AttributeInfo> = Vec::new();
        for _ in 0..attributes_count {
            let attribute_name_index = self.class_file_stream.read_u2();
            let attribute_length = self.class_file_stream.read_u4();
            let attribute_name = &constant_pool[(attribute_name_index - 1) as usize];
            match attribute_name {
                CpInfo::Utf8 {
                    tag: _,
                    length: _,
                    bytes,
                } => {
                    let attribute_name = String::from_utf8(bytes.clone()).unwrap();
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
                            let stack_map_table = self.parse_stack_map_table(number_of_entries);
                            attributes.push(AttributeInfo::StackMapTable {
                                attribute_name_index,
                                attribute_length,
                                number_of_entries,
                                entries: stack_map_table,
                            });
                        }
                        "Exceptions" => {
                            let number_of_exceptions = self.class_file_stream.read_u2();
                            let mut exception_index_table = Vec::new();
                            for _ in 0..number_of_exceptions {
                                exception_index_table.push(self.class_file_stream.read_u2());
                            }
                            attributes.push(AttributeInfo::Exceptions {
                                attribute_name_index,
                                attribute_length,
                                number_of_exceptions,
                                exception_index_table,
                            });
                        }
                        "InnerClasses" => {
                            let number_of_classes = self.class_file_stream.read_u2();
                            let mut classes = Vec::new();
                            for _ in 0..number_of_classes {
                                let inner_class_info_index = self.class_file_stream.read_u2();
                                let outer_class_info_index = self.class_file_stream.read_u2();
                                let inner_name_index = self.class_file_stream.read_u2();
                                let inner_class_access_flags = self.class_file_stream.read_u2();
                                classes.push((
                                    inner_class_info_index,
                                    outer_class_info_index,
                                    inner_name_index,
                                    inner_class_access_flags,
                                ));
                            }
                            attributes.push(AttributeInfo::InnerClasses {
                                attribute_name_index,
                                attribute_length,
                                number_of_classes,
                                classes,
                            });
                        }
                        "EnclosingMethod" => {
                            let class_index = self.class_file_stream.read_u2();
                            let method_index = self.class_file_stream.read_u2();
                            attributes.push(AttributeInfo::EnclosingMethod {
                                attribute_name_index,
                                attribute_length,
                                class_index,
                                method_index,
                            });
                        }
                        "Synthetic" => {
                            attributes.push(AttributeInfo::Synthetic {
                                attribute_name_index,
                                attribute_length,
                            });
                        }
                        "Signature" => {
                            let signature_index = self.class_file_stream.read_u2();
                            attributes.push(AttributeInfo::Signature {
                                attribute_name_index,
                                attribute_length,
                                signature_index,
                            });
                        }
                        "SourceFile" => {
                            let sourcefile_index = self.class_file_stream.read_u2();
                            attributes.push(AttributeInfo::SourceFile {
                                attribute_name_index,
                                attribute_length,
                                sourcefile_index,
                            });
                        }
                        "SourceDebugExtension" => {
                            let debug_extension =
                                self.class_file_stream.read_n(attribute_length as usize);
                            attributes.push(AttributeInfo::SourceDebugExtension {
                                attribute_name_index,
                                attribute_length,
                                debug_extension,
                            });
                        }
                        "LineNumberTable" => {
                            let line_number_table_length = self.class_file_stream.read_u2();
                            let mut line_number_table = Vec::new();
                            for _ in 0..line_number_table_length {
                                let start_pc = self.class_file_stream.read_u2();
                                let line_number = self.class_file_stream.read_u2();
                                line_number_table.push((start_pc, line_number));
                            }
                            attributes.push(AttributeInfo::LineNumberTable {
                                attribute_name_index,
                                attribute_length,
                                line_number_table_length,
                                line_number_table,
                            });
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
                                local_variable_table.push((
                                    start_pc,
                                    length,
                                    name_index,
                                    descriptor_index,
                                    index,
                                ));
                            }
                            attributes.push(AttributeInfo::LocalVariableTable {
                                attribute_name_index,
                                attribute_length,
                                local_variable_table_length,
                                local_variable_table,
                            });
                        }
                        "LocalVariableTypeTable" => {
                            let local_variable_table_length = self.class_file_stream.read_u2();
                            let mut local_variable_table = Vec::new();
                            for _ in 0..local_variable_table_length {
                                let start_pc = self.class_file_stream.read_u2();
                                let length = self.class_file_stream.read_u2();
                                let name_index = self.class_file_stream.read_u2();
                                let signature_index = self.class_file_stream.read_u2();
                                let index = self.class_file_stream.read_u2();
                                local_variable_table.push((
                                    start_pc,
                                    length,
                                    name_index,
                                    signature_index,
                                    index,
                                ));
                            }
                            attributes.push(AttributeInfo::LocalVariableTable {
                                attribute_name_index,
                                attribute_length,
                                local_variable_table_length,
                                local_variable_table,
                            });
                        }
                        "Deprecated" => {
                            attributes.push(AttributeInfo::Deprecated {
                                attribute_name_index,
                                attribute_length,
                            });
                        }
                        "RuntimeVisibleAnnotations" => {
                            let num_annotations = self.class_file_stream.read_u2();
                            let mut annotations = Vec::new();
                            for _ in 0..num_annotations {
                                annotations.push(self.parse_annotation());
                            }
                            attributes.push(AttributeInfo::RuntimeVisibleAnnotations {
                                attribute_name_index,
                                attribute_length,
                                num_annotations,
                                annotations,
                            });
                        }
                        "RuntimeInvisibleAnnotations" => {
                            let num_annotations = self.class_file_stream.read_u2();
                            let mut annotations = Vec::new();
                            for _ in 0..num_annotations {
                                annotations.push(self.parse_annotation());
                            }
                            attributes.push(AttributeInfo::RuntimeInvisibleAnnotations {
                                attribute_name_index,
                                attribute_length,
                                num_annotations,
                                annotations,
                            });
                        }
                        "RuntimeVisibleParameterAnnotations" => {
                            let num_parameters = self.class_file_stream.read_u1();
                            let mut parameter_annotations = Vec::new();
                            for _ in 0..num_parameters {
                                let num_annotations = self.class_file_stream.read_u2();
                                let mut annotations = Vec::new();
                                for _ in 0..num_annotations {
                                    annotations.push(self.parse_annotation());
                                }
                                parameter_annotations.push((num_annotations, annotations));
                            }
                            attributes.push(AttributeInfo::RuntimeVisibleParameterAnnotations {
                                attribute_name_index,
                                attribute_length,
                                num_parameters,
                                parameter_annotations,
                            });
                        }
                        "RuntimeInvisibleParameterAnnotations" => {
                            let num_parameters = self.class_file_stream.read_u1();
                            let mut parameter_annotations = Vec::new();
                            for _ in 0..num_parameters {
                                let num_annotations = self.class_file_stream.read_u2();
                                let mut annotations = Vec::new();
                                for _ in 0..num_annotations {
                                    annotations.push(self.parse_annotation());
                                }
                                parameter_annotations.push((num_annotations, annotations));
                            }
                            attributes.push(AttributeInfo::RuntimeInvisibleParameterAnnotations {
                                attribute_name_index,
                                attribute_length,
                                num_parameters,
                                parameter_annotations,
                            });
                        }
                        "RuntimeVisibleTypeAnnotations" => {
                            let num_annotations = self.class_file_stream.read_u2();
                            let mut annotations = Vec::new();
                            for _ in 0..num_annotations {
                                annotations.push(self.parse_type_annotation());
                            }
                            attributes.push(AttributeInfo::RuntimeVisibleTypeAnnotations {
                                attribute_name_index,
                                attribute_length,
                                num_annotations,
                                annotations,
                            });
                        }
                        "AnnotationDefault" => {
                            let default_value = self.parse_element_value();
                            attributes.push(AttributeInfo::AnnotationDefault {
                                attribute_name_index,
                                attribute_length,
                                default_value,
                            });
                        }
                        "BootstrapMethods" => {
                            let num_bootstrap_methods = self.class_file_stream.read_u2();
                            let mut bootstrap_methods = Vec::new();
                            for _ in 0..num_bootstrap_methods {
                                let bootstrap_method_ref = self.class_file_stream.read_u2();
                                let num_bootstrap_arguments = self.class_file_stream.read_u2();
                                let mut bootstrap_arguments = Vec::new();
                                for _ in 0..num_bootstrap_arguments {
                                    bootstrap_arguments.push(self.class_file_stream.read_u2());
                                }
                                bootstrap_methods.push((
                                    bootstrap_method_ref,
                                    num_bootstrap_arguments,
                                    bootstrap_arguments,
                                ));
                            }
                            attributes.push(AttributeInfo::BootstrapMethods {
                                attribute_name_index,
                                attribute_length,
                                num_bootstrap_methods,
                                bootstrap_methods,
                            });
                        }
                        "MethodParameters" => {
                            let parameters_count = self.class_file_stream.read_u1();
                            let mut parameters = Vec::new();
                            for _ in 0..parameters_count {
                                let name_index = self.class_file_stream.read_u2();
                                let access_flags = self.class_file_stream.read_u2();
                                parameters.push((name_index, access_flags));
                            }
                            attributes.push(AttributeInfo::MethodParameters {
                                attribute_name_index,
                                attribute_length,
                                parameters_count,
                                parameters,
                            });
                        }
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
        AttributeInfo::ConstantValue {
            attribute_name_index,
            attribute_length,
            constant_value_index,
        }
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
        let code = self.class_file_stream.read_n(code_length as usize);
        let exception_table_length = self.class_file_stream.read_u2();
        let exception_table = self.parse_exception_table(exception_table_length);
        let attributes_count = self.class_file_stream.read_u2();
        let attributes = self.parse_attributes(attributes_count, constant_pool);
        AttributeInfo::Code {
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
        }
    }

    fn parse_exception_table(&mut self, exception_table_length: U2) -> Vec<(U2, U2, U2, U2)> {
        let mut exception_table = Vec::new();
        for _ in 0..exception_table_length {
            let start_pc = self.class_file_stream.read_u2();
            let end_pc = self.class_file_stream.read_u2();
            let handler_pc = self.class_file_stream.read_u2();
            let catch_type = self.class_file_stream.read_u2();
            exception_table.push((start_pc, end_pc, handler_pc, catch_type));
        }
        exception_table
    }

    fn parse_stack_map_table(&mut self, number_of_entries: U2) -> Vec<StackMapFrame> {
        let mut stack_map_table: Vec<StackMapFrame> = Vec::new();
        for _ in 0..number_of_entries {
            let frame_type = self.class_file_stream.read_u1();
            if frame_type >= 0 && frame_type <= 63 {
                // SAME
                stack_map_table.push(StackMapFrame::SameFrame { frame_type });
            } else if frame_type >= 64 && frame_type <= 127 {
                // SAME_LOCALS_1_STACK_ITEM
                let tag = self.class_file_stream.read_u1();
                let verify_type_info = self.parse_verification_type_info(tag);
                stack_map_table.push(StackMapFrame::SameLocals1StackItemFrame {
                    frame_type,
                    stack: [verify_type_info],
                })
            } else if frame_type == 247 {
                // SAME_LOCALS_1_STACK_ITEM_EXTENDED
                let offset_delta = self.class_file_stream.read_u2();
                let tag = self.class_file_stream.read_u1();
                let verify_type_info = self.parse_verification_type_info(tag);
                stack_map_table.push(StackMapFrame::SameLocals1StackItemFrameExtended {
                    frame_type,
                    offset_delta,
                    stack: [verify_type_info],
                })
            } else if frame_type >= 248 && frame_type <= 250 {
                // CHOP
                let offset_delta = self.class_file_stream.read_u2();
                stack_map_table.push(StackMapFrame::ChopFrame {
                    frame_type,
                    offset_delta,
                })
            } else if frame_type == 251 {
                // SAME_FRAME_EXTENDED
                let offset_delta = self.class_file_stream.read_u2();
                stack_map_table.push(StackMapFrame::SameFrameExtended {
                    frame_type,
                    offset_delta,
                })
            } else if frame_type >= 252 && frame_type <= 254 {
                // APPEND
                let offset_delta = self.class_file_stream.read_u2();
                let mut locals = Vec::new();
                for _ in 0..frame_type - 251 {
                    let tag = self.class_file_stream.read_u1();
                    let verify_type_info = self.parse_verification_type_info(tag);
                    locals.push(verify_type_info);
                }
                stack_map_table.push(StackMapFrame::AppendFrame {
                    frame_type,
                    offset_delta,
                    locals,
                })
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
                stack_map_table.push(StackMapFrame::FullFrame {
                    frame_type,
                    offset_delta,
                    number_of_locals,
                    locals,
                    number_of_stack_items,
                    stack,
                })
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

    fn parse_element_value_pairs(
        &mut self,
        num_element_value_pairs: U2,
    ) -> Vec<(U2, ElementValue)> {
        let mut element_value_pairs = Vec::new();
        for _ in 0..num_element_value_pairs {
            let element_name_index = self.class_file_stream.read_u2();
            let element_value = self.parse_element_value();
            element_value_pairs.push((element_name_index, element_value));
        }
        element_value_pairs
    }

    fn parse_element_value(&mut self) -> ElementValue {
        let tag = self.class_file_stream.read_u1();
        match tag {
            b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's' => {
                let const_value_index = self.class_file_stream.read_u2();
                ElementValue {
                    tag,
                    value: ElementValueItem::ConstValueIndex { const_value_index },
                }
            }
            b'e' => {
                let type_name_index = self.class_file_stream.read_u2();
                let const_name_index = self.class_file_stream.read_u2();
                ElementValue {
                    tag,
                    value: ElementValueItem::EnumConstValue {
                        type_name_index,
                        const_name_index,
                    },
                }
            }
            b'c' => {
                let class_info_index = self.class_file_stream.read_u2();
                ElementValue {
                    tag,
                    value: ElementValueItem::ClassInfoIndex { class_info_index },
                }
            }
            b'@' => {
                let annotation = self.parse_annotation();
                ElementValue {
                    tag,
                    value: ElementValueItem::AnnotationValue {
                        annotation_value: annotation,
                    },
                }
            }
            b'[' => {
                let num_values = self.class_file_stream.read_u2();
                let mut values = Vec::new();
                for _ in 0..num_values {
                    values.push(self.parse_element_value());
                }
                ElementValue {
                    tag,
                    value: ElementValueItem::ArrayValue { num_values, values },
                }
            }
            _ => panic!("Invalid tag"),
        }
    }

    fn parse_annotation(&mut self) -> Annotation {
        let type_index = self.class_file_stream.read_u2();
        let num_element_value_pairs = self.class_file_stream.read_u2();
        let element_value_pairs = self.parse_element_value_pairs(num_element_value_pairs);
        Annotation {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        }
    }

    fn parse_type_annotation(&mut self) -> TypeAnnotation {
        let target_type = self.class_file_stream.read_u1();
        let target_info = self.parse_target_info(target_type);
        let target_path = self.parse_type_path();
        let type_index = self.class_file_stream.read_u2();
        let num_element_value_pairs = self.class_file_stream.read_u2();
        let element_value_pairs = self.parse_element_value_pairs(num_element_value_pairs);
        TypeAnnotation {
            target_type,
            target_info,
            target_path,
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        }
    }

    fn parse_target_info(&mut self, target_type: U1) -> TargetInfo {
        match target_type {
            0x00 | 0x01 => {
                let type_parameter_index = self.class_file_stream.read_u1();
                TargetInfo::TypeParameterTarget {
                    type_parameter_index,
                }
            }
            0x10 => {
                let supertype_index = self.class_file_stream.read_u2();
                TargetInfo::SuperTypeTarget { supertype_index }
            }
            0x11 | 0x12 => {
                let type_parameter_index = self.class_file_stream.read_u1();
                let bound_index = self.class_file_stream.read_u1();
                TargetInfo::TypeParameterBoundTarget {
                    type_parameter_index,
                    bound_index,
                }
            }
            0x13 | 0x14 | 0x15 => TargetInfo::EmptyTarget,
            0x16 => {
                let formal_parameter_index = self.class_file_stream.read_u1();
                TargetInfo::FormalParameterTarget {
                    formal_parameter_index,
                }
            }
            0x17 => {
                let throws_type_index = self.class_file_stream.read_u2();
                TargetInfo::ThrowsTarget { throws_type_index }
            }
            0x40 | 0x41 => {
                let table_length = self.class_file_stream.read_u2();
                let mut table = Vec::new();
                for _ in 0..table_length {
                    let start_pc = self.class_file_stream.read_u2();
                    let length = self.class_file_stream.read_u2();
                    let index = self.class_file_stream.read_u2();
                    table.push((start_pc, length, index));
                }
                TargetInfo::LocalVarTarget {
                    table_length,
                    table,
                }
            }
            0x42 => {
                let exception_table_index = self.class_file_stream.read_u2();
                TargetInfo::CatchTarget {
                    exception_table_index,
                }
            }
            0x43 | 0x44 | 0x45 | 0x46 => {
                let offset = self.class_file_stream.read_u2();
                TargetInfo::OffsetTarget { offset }
            }
            0x47 | 0x48 | 0x49 | 0x4A | 0x4B => {
                let offset = self.class_file_stream.read_u2();
                let type_argument_index = self.class_file_stream.read_u1();
                TargetInfo::TypeArgumentTarget {
                    offset,
                    type_argument_index,
                }
            }
            _ => panic!("Invalid target type"),
        }
    }

    fn parse_type_path(&mut self) -> TypePath {
        let path_length = self.class_file_stream.read_u1();
        let mut path = Vec::new();
        for _ in 0..path_length {
            let type_path_kind = self.class_file_stream.read_u1();
            let type_argument_index = self.class_file_stream.read_u1();
            path.push((type_path_kind, type_argument_index));
        }
        TypePath { path_length, path }
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
        let reader = File::open(d.display().to_string()).expect("Failed to open class file");
        let mut parser = ClassFileParser::file(BufReader::new(reader));
        let cf = parser.parse();
        print!("{}", cf);
    }
}
