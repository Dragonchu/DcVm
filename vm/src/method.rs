use std::{cell::{Cell, Ref, RefCell}, fmt::format};

use reader::{attribute_info::AttributeInfo, constant_pool::{ConstantPool, CpInfo}, method_info::MethodInfo, types::{U1, U2, U4}};

use crate::instructions::Instruction;
#[derive(Debug, Clone)]
struct ExceptionEntry {
    start_pc: U2,
    end_pc: U2,
    handler_pc: U2,
    catch_type: U2
}

#[derive(Debug, Clone)]
pub struct ByteCodes(Vec<U1>);

impl ByteCodes {
    pub fn iter(&self) -> ByteCodesInterator {
        ByteCodesInterator {
            byte_codes: self,
            index: 0
        }
    }

    fn read_u1(&self, index: usize) -> Option<U1> {
        self.0.get(index).cloned()
    }
}

pub struct ByteCodesInterator<'a> {
    byte_codes: &'a ByteCodes,
    index: usize
}

impl<'a> ByteCodesInterator<'a> {
    fn read_u1(&mut self) -> U1 {
        let op_byte = self.byte_codes.read_u1(self.index).expect("no more code");
        self.index += 1;
        op_byte
    }

    fn read_u2(&mut self) -> U2 {
        let byte1 = self.read_u1();
        let byte2 = self.read_u1();
        ((byte1 as u16) << 8) | (byte2 as u16)
    }

    fn read_u4(&mut self) -> U4 {
        let byte1 = self.read_u1();
        let byte2 = self.read_u1();
        let byte3 = self.read_u1();
        let byte4 = self.read_u1();
        ((byte1 as u32) << 24) | ((byte2 as u32) << 16) | ((byte3 as u32) << 8) | (byte4 as u32)
    }
}

impl<'a> Iterator for ByteCodesInterator<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.byte_codes.0.len() {
            return None;
        }
        let op_byte = self.read_u1();
        match op_byte {
            0x32 => Some(Instruction::Aaload),
            0x53 => Some(Instruction::Aastore),
            0x1 => Some(Instruction::Aconst_null),
            0x19 => Some(Instruction::Aload(self.read_u1())),
            0x2a => Some(Instruction::Aload_0),
            0x2b => Some(Instruction::Aload_1),
            0x2c => Some(Instruction::Aload_2),
            0x2d => Some(Instruction::Aload_3),
            0xbd => Some(Instruction::Anewarray(self.read_u2())),
            0xb0 => Some(Instruction::Areturn),
            0xbe => Some(Instruction::Arraylength),
            0x3a => Some(Instruction::Astore(self.read_u1())),
            0x4b => Some(Instruction::Astore_0),
            0x4c => Some(Instruction::Astore_1),
            0x4d => Some(Instruction::Astore_2),
            0x4e => Some(Instruction::Astore_3),
            0xbf => Some(Instruction::Athrow),
            0x33 => Some(Instruction::Baload),
            0x54 => Some(Instruction::Bastore),
            0x10 => Some(Instruction::Bipush(self.read_u1())),
            0x34 => Some(Instruction::Caload),
            0x55 => Some(Instruction::Castore),
            0xc0 => Some(Instruction::Checkcast(self.read_u2())),
            0x90 => Some(Instruction::D2f),
            0x8e => Some(Instruction::D2i),
            0x8f => Some(Instruction::D2l),
            0x63 => Some(Instruction::Dadd),
            0x31 => Some(Instruction::Daload),
            0x52 => Some(Instruction::Dastore),
            0x98 => Some(Instruction::Dcmpg),
            0x97 => Some(Instruction::Dcmpl),
            0xe => Some(Instruction::Dconst_0),
            0xf => Some(Instruction::Dconst_1),
            0x6f => Some(Instruction::Ddiv),
            0x18 => Some(Instruction::Dload(self.read_u1())),
            0x26 => Some(Instruction::Dload_0),
            0x27 => Some(Instruction::Dload_1),
            0x28 => Some(Instruction::Dload_2),
            0x29 => Some(Instruction::Dload_3),
            0x6b => Some(Instruction::Dmul),
            0x77 => Some(Instruction::Dneg),
            0x73 => Some(Instruction::Drem),
            0xaf => Some(Instruction::Dreturn),
            0x39 => Some(Instruction::Dstore(self.read_u1())),
            0x47 => Some(Instruction::Dstore_0),
            0x48 => Some(Instruction::Dstore_1),
            0x49 => Some(Instruction::Dstore_2),
            0x4a => Some(Instruction::Dstore_3),
            0x67 => Some(Instruction::Dsub),
            0x59 => Some(Instruction::Dup),
            0x5a => Some(Instruction::Dup_x1),
            0x5b => Some(Instruction::Dup_x2),
            0x5c => Some(Instruction::Dup2),
            0x5d => Some(Instruction::Dup2_x1),
            0x5e => Some(Instruction::Dup2_x2),
            0x8d => Some(Instruction::F2d),
            0x8b => Some(Instruction::F2i),
            0x8c => Some(Instruction::F2l),
            0x62 => Some(Instruction::Fadd),
            0x30 => Some(Instruction::Faload),
            0x51 => Some(Instruction::Fastore),
            0x96 => Some(Instruction::Fcmpg),
            0x95 => Some(Instruction::Fcmpl),
            0xb => Some(Instruction::Fconst_0),
            0xc => Some(Instruction::Fconst_1),
            0xd => Some(Instruction::Fconst_2),
            0x6e => Some(Instruction::Fdiv),
            0x17 => Some(Instruction::Fload(self.read_u1())),
            0x22 => Some(Instruction::Fload_0),
            0x23 => Some(Instruction::Fload_1),
            0x24 => Some(Instruction::Fload_2),
            0x25 => Some(Instruction::Fload_3),
            0x6a => Some(Instruction::Fmul),
            0x76 => Some(Instruction::Fneg),
            0x72 => Some(Instruction::Frem),
            0xae => Some(Instruction::Freturn),
            0x38 => Some(Instruction::Fstore(self.read_u1())),
            0x43 => Some(Instruction::Fstore_0),
            0x44 => Some(Instruction::Fstore_1),
            0x45 => Some(Instruction::Fstore_2),
            0x46 => Some(Instruction::Fstore_3),
            0x66 => Some(Instruction::Fsub),
            0xb4 => Some(Instruction::Getfield(self.read_u2())),
            0xb2 => Some(Instruction::Getstatic(self.read_u2())),
            0xa7 => Some(Instruction::Goto(self.read_u2())),
            0xc8 => Some(Instruction::Goto_w(self.read_u4())),
            0x91 => Some(Instruction::I2b),
            0x92 => Some(Instruction::I2c),
            0x87 => Some(Instruction::I2d),
            0x86 => Some(Instruction::I2f),
            0x85 => Some(Instruction::I2l),
            0x93 => Some(Instruction::I2s),
            0x60 => Some(Instruction::Iadd),
            0x2e => Some(Instruction::Iaload),
            0x7e => Some(Instruction::Iand),
            0x4f => Some(Instruction::Iastore),
            0x2 => Some(Instruction::Iconst_m1),
            0x3 => Some(Instruction::Iconst_0),
            0x4 => Some(Instruction::Iconst_1),
            0x5 => Some(Instruction::Iconst_2),
            0x6 => Some(Instruction::Iconst_3),
            0x7 => Some(Instruction::Iconst_4),
            0x8 => Some(Instruction::Iconst_5),
            0x6c => Some(Instruction::Idiv),
            0xa5 => Some(Instruction::If_acmpeq(self.read_u2())),
            0xa6 => Some(Instruction::If_acmpne(self.read_u2())),
            0x9f => Some(Instruction::If_icmpeq(self.read_u2())),
            0xa0 => Some(Instruction::If_icmpne(self.read_u2())),
            0xa1 => Some(Instruction::If_icmplt(self.read_u2())),
            0xa2 => Some(Instruction::If_icmpge(self.read_u2())),
            0xa3 => Some(Instruction::If_icmpgt(self.read_u2())),
            0xa4 => Some(Instruction::If_icmple(self.read_u2())),
            0x99 => Some(Instruction::Ifeq(self.read_u2())),
            0x9a => Some(Instruction::Ifne(self.read_u2())),
            0x9b => Some(Instruction::Iflt(self.read_u2())),
            0x9c => Some(Instruction::Ifge(self.read_u2())),
            0x9d => Some(Instruction::Ifgt(self.read_u2())),
            0x9e => Some(Instruction::Ifle(self.read_u2())),
            0xc7 => Some(Instruction::Ifnonnull(self.read_u2())),
            0xc6 => Some(Instruction::Ifnull(self.read_u2())),
            0x84 => Some(Instruction::Iinc(self.read_u1(), self.read_u1() as i8)),
            0x15 => Some(Instruction::Iload(self.read_u1())),
            0x1a => Some(Instruction::Iload_0),
            0x1b => Some(Instruction::Iload_1),
            0x1c => Some(Instruction::Iload_2),
            0x1d => Some(Instruction::Iload_3),
            0x68 => Some(Instruction::Imul),
            0x74 => Some(Instruction::Ineg),
            0xc1 => Some(Instruction::Instanceof(self.read_u2())),
            0xba => {
                let index = self.read_u2();
                let _ = self.read_u2();
                Some(Instruction::Invokedynamic(index))
            },
            0xb9 => {
                let index = self.read_u2();
                let cnt = self.read_u1();
                let _ = self.read_u1();
                Some(Instruction::Invokeinterface(index, cnt))
            },
            0xb7 => Some(Instruction::Invokespecial(self.read_u2())),
            0xb8 => Some(Instruction::Invokestatic(self.read_u2())),
            0xb6 => Some(Instruction::Invokevirtual(self.read_u2())),
            0x80 => Some(Instruction::Ior),
            0x70 => Some(Instruction::Irem),
            0xac => Some(Instruction::Ireturn),
            0x78 => Some(Instruction::Ishl),
            0x7a => Some(Instruction::Ishr),
            0x36 => Some(Instruction::Istore(self.read_u1())),
            0x3b => Some(Instruction::Istore_0),
            0x3c => Some(Instruction::Istore_1),
            0x3d => Some(Instruction::Istore_2),
            0x3e => Some(Instruction::Istore_3),
            0x64 => Some(Instruction::Isub),
            0x7c => Some(Instruction::Iushr),
            0x82 => Some(Instruction::Ixor),
            0xa8 => Some(Instruction::Jsr(self.read_u2())),
            0xc9 => Some(Instruction::Jsr_w(self.read_u4())),
            0x8a => Some(Instruction::L2d),
            0x89 => Some(Instruction::L2f),
            0x88 => Some(Instruction::L2i),
            0x61 => Some(Instruction::Ladd),
            0x2f => Some(Instruction::Laload),
            0x7f => Some(Instruction::Land),
            0x50 => Some(Instruction::Lastore),
            0x94 => Some(Instruction::Lcmp),
            0x9 => Some(Instruction::Lconst_0),
            0xa => Some(Instruction::Lconst_1),
            0x12 => Some(Instruction::Ldc(self.read_u1())),
            0x13 => Some(Instruction::Ldc_w(self.read_u2())),
            0x14 => Some(Instruction::Ldc2_w(self.read_u2())),
            0x6d => Some(Instruction::Ldiv),
            0x16 => Some(Instruction::Lload(self.read_u1())),
            0x1e => Some(Instruction::Lload_0),
            0x1f => Some(Instruction::Lload_1),
            0x20 => Some(Instruction::Lload_2),
            0x21 => Some(Instruction::Lload_3),
            0x69 => Some(Instruction::Imul),
            0x75 => Some(Instruction::Ineg),
            0xab => {
                for _ in 0..3 {
                    let _ =self.read_u1();
                }
                let default = self.read_u4();
                let npairs = self.read_u4();
                let mut pairs = Vec::with_capacity(npairs as usize);
                for _ in 0..npairs {
                    let int_match = self.read_u4();
                    let offset = self.read_u4() as i32;
                    pairs.push((int_match, offset));
                }
                Some(Instruction::Lookupswitch(default, npairs, pairs))
            },
            0x81 => Some(Instruction::Lor),
            0x71 => Some(Instruction::Irem),
            0xad => Some(Instruction::Ireturn),
            0x79 => Some(Instruction::Ishl),
            0x7b => Some(Instruction::Ishr),
            0x37 => Some(Instruction::Istore(self.read_u1())),
            0x3f => Some(Instruction::Istore_0),
            0x40 => Some(Instruction::Istore_1),
            0x41 => Some(Instruction::Istore_2),
            0x42 => Some(Instruction::Istore_3),
            0x65 => Some(Instruction::Isub),
            0x7d => Some(Instruction::Lushr),
            0x83 => Some(Instruction::Lxor),
            0xc2 => Some(Instruction::Monitorenter),
            0xc3 => Some(Instruction::Monitorexit),
            0xc5 => Some(Instruction::Multianewarray(self.read_u2(), self.read_u1())),
            0xbb => Some(Instruction::New(self.read_u2())),
            0xbc => Some(Instruction::Newarray(self.read_u1())),//TODO use enum ArrayType
            0x0 => Some(Instruction::Nop),
            0x57 => Some(Instruction::Pop),
            0x58 => Some(Instruction::Pop2),
            0xb5 => Some(Instruction::Putfield(self.read_u2())),
            0xb3 => Some(Instruction::Putstatic(self.read_u2())),
            0xa9 => Some(Instruction::Ret(self.read_u1())),
            0xb1 => Some(Instruction::Return),
            0x35 => Some(Instruction::Saload),
            0x56 => Some(Instruction::Sastore),
            0x11 => Some(Instruction::Sipush(self.read_u2())),
            0x5f => Some(Instruction::Swap),
            0xaa => {
                for _ in 0..3 {
                    self.read_u1();
                }
                let default = self.read_u4();
                let low = self.read_u4();
                let high = self.read_u4();
                let size = high - low + 1;
                let mut offsets = Vec::with_capacity(size as usize);
                for _ in 0..size {
                    offsets.push(self.read_u4() as i32);
                }
                Some(Instruction::Tableswitch(default, low, high, offsets))
            },
            0xc4 => Some(Instruction::Wide),
            _ =>None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Code {
    pub max_stack: U2,
    pub max_locals: U2,
    pub byte_codes: ByteCodes,
    exception_table_length: U2,
    exception_table: Vec<ExceptionEntry>
}
#[derive(Debug, Clone)]
pub struct Method {
    access_flags: U2,
    name: String,
    descriptor: String,
    code: Option<Code>,
}

pub fn link_code(method_info: &MethodInfo) -> Option<Code>{
    for attribute_info in &method_info.attributes {
        match attribute_info {
            AttributeInfo::Code { attribute_name_index, attribute_length, max_stack, max_locals, code_length, code, exception_table_length, exception_table, attributes_count, attributes } => {
                let mut rt_exception_table = Vec::new();
                for exception_entry in exception_table {
                    rt_exception_table.push(
                        ExceptionEntry{
                            start_pc: exception_entry.0,
                            end_pc: exception_entry.1,
                            handler_pc: exception_entry.2,
                            catch_type: exception_entry.3
                        }
                    )
                }
                return Some(Code {
                    max_stack: max_stack.clone(),
                    max_locals: max_locals.clone(),
                    byte_codes: ByteCodes(code.clone()),
                    exception_table_length: exception_table_length.clone(),
                    exception_table: rt_exception_table
                });
            }
            _ => {
            }
        }
    }
return None;
}

impl Method {
    pub fn new(method_info: &MethodInfo, cp_pool: & dyn ConstantPool) -> Method {
        Method {
            access_flags: method_info.access_flags,
            name:  cp_pool.get_utf8_string(method_info.name_index),
            descriptor: cp_pool.get_utf8_string(method_info.descriptor_index),
            code: link_code(method_info)
        }
    }

    pub fn get_unique_key(&self) -> String {
        let name = self.name.clone();
        let descriptor = self.descriptor.clone();
        format!("{name} {descriptor}")
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_descriptor(&self) -> String {
        self.descriptor.clone()
    }

    pub fn get_code(&self) -> Code {
        self.code.clone().unwrap()
    }

}