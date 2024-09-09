use super::{
    class_file::{
        AttributeInfo, ClassFile, ConstantClassInfo, ConstantDoubleInfo, ConstantFieldrefInfo,
        ConstantFloatInfo, ConstantInfoTag, ConstantIntegerInfo, ConstantInterfaceMethodrefInfo,
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

    pub fn parse_attributes(&mut self, attributes_count: U2) -> Vec<AttributeInfo> {
        let mut attributes: Vec<AttributeInfo> = Vec::new();
        for _ in 0..attributes_count {
            let attribute_name_index = self.class_file_stream.read_u2();
            let attribute_length = self.class_file_stream.read_u4();
            let info = self
                .class_file_stream
                .drain(0..attribute_length as usize)
                .collect();
            attributes.push(AttributeInfo::new(
                attribute_name_index,
                attribute_length,
                info,
            ));
        }
        attributes
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
