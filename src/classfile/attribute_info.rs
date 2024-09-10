use crate::classfile::types::{U1, U2, U4};

#[derive(Debug)]
pub enum AttributeInfo {
    ConstantValue(ConstantValueAttribute),
    Code(CodeAttribute),
    StackMapTable(StackMapTableAttribute),
    Exceptions(ExceptionsAttribute),
    InnerClasses(InnerClassesAttribute),
    EnclosingMethod(EnclosingMethodAttribute),
    Synthetic(SyntheticAttribute),
    Signature(SignatureAttribute),
    SourceFile(SourceFileAttribute),
    SourceDebugExtension(SourceDebugExtensionAttribute),
    LineNumberTable(LineNumberTableAttribute),
    LocalVariableTable(LocalVariableTableAttribute),
    LocalVariableTypeTable(LocalVariableTypeTableAttribute),
    Deprecated(DeprecatedAttribute),
    RuntimeVisibleAnnotations(RuntimeVisibleAnnotationsAttribute),
    RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotationsAttribute),
    RuntimeVisibleParameterAnnotations(RuntimeVisibleParameterAnnotationsAttribute),
    RuntimeInvisibleParameterAnnotations(RuntimeInvisibleParameterAnnotationsAttribute),
    RuntimeVisibleTypeAnnotations(RuntimeVisibleTypeAnnotationsAttribute),
    RuntimeInvisibleTypeAnnotations(RuntimeInvisibleTypeAnnotationsAttribute),
    AnnotationDefault(AnnotationDefaultAttribute),
    BootstrapMethods(BootstrapMethodsAttribute),
    MethodParameters(MethodParametersAttribute),
}

#[derive(Debug)]
struct ConstantValueAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    constant_value_index: U2,
}

#[derive(Debug)]
struct ExceptionTableEntry {
    start_pc: U2,
    end_pc: U2,
    handler_pc: U2,
    catch_type: U2,
}

#[derive(Debug)]
struct CodeAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    max_stack: U2,
    max_locals: U2,
    code_length: U4,
    code: Vec<U1>,
    exception_table_length: U2,
    exception_table: Vec<ExceptionTableEntry>,
    attributes_count: U2,
    attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
struct StackMapTableAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    number_of_entries: U2,
    entries: Vec<StackMapFrame>,
}

#[derive(Debug)]
enum VerificationTypeInfo {
    TopVariable(TopVariableInfo),
    IntegerVariable(IntegerVariableInfo),
    FloatVariable(FloatVariableInfo),
    LongVariable(LongVariableInfo),
    DoubleVariable(DoubleVariableInfo),
    NullVariable(NullVariableInfo),
    UninitializedThisVariable(UninitializedThisVariableInfo),
    ObjectVariable(ObjectVariableInfo),
    UninitializedVariable(UninitializedVariableInfo),
}

#[derive(Debug)]
struct TopVariableInfo {
    tag: U1,
}

#[derive(Debug)]
struct IntegerVariableInfo {
    tag: U1,
}

#[derive(Debug)]
struct FloatVariableInfo {
    tag: U1,
}

#[derive(Debug)]
struct NullVariableInfo {
    tag: U1,
}

#[derive(Debug)]
struct UninitializedThisVariableInfo {
    tag: U1,
}

#[derive(Debug)]
struct ObjectVariableInfo {
    tag: U1,
    cpool_index: U2,
}

#[derive(Debug)]
struct UninitializedVariableInfo {
    tag: U1,
    offset: U2,
}

#[derive(Debug)]
struct LongVariableInfo {
    tag: U1,
}

#[derive(Debug)]
struct DoubleVariableInfo {
    tag: U1,
}

#[derive(Debug)]
enum StackMapFrame {
    SameFrame(SameFrame),
    SameLocals1StackItemFrame(SameLocals1StackItemFrame),
    SameLocals1StackItemFrameExtended(SameLocals1StackItemFrameExtended),
    ChopFrame(ChopFrame),
    SameFrameExtended(SameFrameExtended),
    AppendFrame(AppendFrame),
    FullFrame(FullFrame),
}

#[derive(Debug)]
struct SameFrame {
    //0-63
    frame_type: U1,
}

#[derive(Debug)]
struct SameLocals1StackItemFrame {
    //64-127
    frame_type: U1,
    stack: VerificationTypeInfo,
}

#[derive(Debug)]
struct SameLocals1StackItemFrameExtended {
    //247
    frame_type: U1,
    offset_delta: U2,
    stack: VerificationTypeInfo,
}

#[derive(Debug)]
struct ChopFrame {
    //248-250
    frame_type: U1,
    offset_delta: U2,
}

#[derive(Debug)]
struct SameFrameExtended {
    //251
    frame_type: U1,
    offset_delta: U2,
}

#[derive(Debug)]
struct AppendFrame {
    //252-254
    frame_type: U1,
    offset_delta: U2,
    locals: Vec<VerificationTypeInfo>,
}

#[derive(Debug)]
struct FullFrame {
    //255
    frame_type: U1,
    offset_delta: U2,
    number_of_locals: U2,
    locals: Vec<VerificationTypeInfo>,
    number_of_stack_items: U2,
    stack: Vec<VerificationTypeInfo>,
}

#[derive(Debug)]
struct ExceptionsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    number_of_exceptions: U2,
    exception_index_table: Vec<U2>,
}

#[derive(Debug)]
struct InnerClassesAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    number_of_classes: U2,
    classes: Vec<InnerClassInfo>,
}