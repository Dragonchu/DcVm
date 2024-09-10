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

#[derive(Debug)]
struct InnerClassInfo {
    inner_class_info_index: U2,
    outer_class_info_index: U2,
    inner_name_index: U2,
    inner_class_access_flags: U2,
}

#[derive(Debug)]
struct EnclosingMethodAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    class_index: U2,
    method_index: U2,
}

#[derive(Debug)]
struct SyntheticAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
}

#[derive(Debug)]
struct SignatureAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    signature_index: U2,
}

#[derive(Debug)]
struct SourceFileAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    sourcefile_index: U2,
}

#[derive(Debug)]
struct SourceDebugExtensionAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    debug_extension: Vec<U1>,
}

#[derive(Debug)]
struct LineNumberTableAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    line_number_table_length: U2,
    line_number_table: Vec<LineNumberTableEntry>,
}

#[derive(Debug)]
struct LineNumberTableEntry {
    start_pc: U2,
    line_number: U2,
}

#[derive(Debug)]
struct LocalVariableTableAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    local_variable_table_length: U2,
    local_variable_table: Vec<LocalVariableTableEntry>,
}

#[derive(Debug)]
struct LocalVariableTableEntry {
    start_pc: U2,
    length: U2,
    name_index: U2,
    descriptor_index: U2,
    index: U2,
}

#[derive(Debug)]
struct LocalVariableTypeTableAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    local_variable_type_table_length: U2,
    local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
}

#[derive(Debug)]
struct LocalVariableTypeTableEntry {
    start_pc: U2,
    length: U2,
    name_index: U2,
    signature_index: U2,
    index: U2,
}

#[derive(Debug)]
struct DeprecatedAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
}

#[derive(Debug)]
struct RuntimeVisibleAnnotationsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    num_annotations: U2,
    annotations: Vec<Annotation>,
}

#[derive(Debug)]
struct Annotation {
    type_index: U2,
    num_element_value_pairs: U2,
    element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug)]
struct ElementValuePair {
    element_name_index: U2,
    value: ElementValue,
}

#[derive(Debug)]
enum ElementValue {
    ConstValueIndex {
        tag: U1,
        value: ConstValueIndexElementValue,
    },
    EnumConstValue {
        tag: U1,
        value: EnumConstValueElementValue,
    },
    ClassInfoIndex {
        tag: U1,
        value: ClassInfoIndexElementValue,
    },
    AnnotationValue {
        tag: U1,
        value: AnnotationValueElementValue,
    },
    ArrayValue {
        tage: U1,
        value: ArrayValueElementValue,
    },
}

#[derive(Debug)]
struct ConstValueIndexElementValue {
    const_value_index: U2,
}

#[derive(Debug)]
struct EnumConstValueElementValue {
    type_name_index: U2,
    const_name_index: U2,
}

#[derive(Debug)]
struct ClassInfoIndexElementValue {
    class_info_index: U2,
}

#[derive(Debug)]
struct AnnotationValueElementValue {
    annotation_value: Annotation,
}

#[derive(Debug)]
struct ArrayValueElementValue {
    num_values: U2,
    values: Vec<ElementValue>,
}

#[derive(Debug)]
struct RuntimeInvisibleAnnotationsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    num_annotations: U2,
    annotations: Vec<Annotation>,
}

#[derive(Debug)]
struct RuntimeVisibleParameterAnnotationsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    num_parameters: U1,
    parameter_annotations: Vec<ParameterAnnotation>,
}

#[derive(Debug)]
struct ParameterAnnotation {
    num_annotations: U2,
    annotations: Vec<Annotation>,
}

#[derive(Debug)]
struct RuntimeInvisibleParameterAnnotationsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    num_parameters: U1,
    parameter_annotations: Vec<ParameterAnnotation>,
}

#[derive(Debug)]
struct RuntimeVisibleTypeAnnotationsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    num_annotations: U2,
    annotations: Vec<TypeAnnotation>,
}

#[derive(Debug)]
struct TypeAnnotation {
    target_type: U1,
    target_info: TargetInfo,
    target_path: TypePath,
    type_index: U2,
    num_element_value_pairs: U2,
    element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug)]
enum TargetInfo {
    TypeParameterTarget {
        type_parameter_index: U1,
    },
    SuperTypeTarget {
        super_type_index: U2,
    },
    TypeParameterBoundTarget {
        type_parameter_index: U1,
        bound_index: U1,
    },
    EmptyTarget,
    FormalParameterTarget {
        formal_parameter_index: U1,
    },
    ThrowsTarget {
        throws_type_index: U2,
    },
    LocalVarTarget {
        table_length: U2,
        table: Vec<LocalVarTargetTable>,
    },
    CatchTarget {
        exception_table_index: U2,
    },
    OffsetTarget {
        offset: U2,
    },
    TypeArgumentTarget {
        offset: U2,
        type_argument_index: U1,
    },
}

#[derive(Debug)]
struct LocalVarTargetTable {
    start_pc: U2,
    length: U2,
    index: U2,
}

#[derive(Debug)]
struct TypePath {
    path_length: U1,
    path: Vec<TypePathKind>,
}

#[derive(Debug)]
struct TypePathKind {
    type_path_kind: U1,
    type_argument_index: U1,
}

#[derive(Debug)]
struct RuntimeInvisibleTypeAnnotationsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    num_annotations: U2,
    annotations: Vec<TypeAnnotation>,
}

#[derive(Debug)]
struct AnnotationDefaultAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    default_value: ElementValue,
}

#[derive(Debug)]
struct BootstrapMethodsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    num_bootstrap_methods: U2,
    bootstrap_methods: Vec<BootstrapMethod>,
}

#[derive(Debug)]
struct BootstrapMethod {
    bootstrap_method_ref: U2,
    num_bootstrap_arguments: U2,
    bootstrap_arguments: Vec<U2>,
}

#[derive(Debug)]
struct MethodParametersAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    parameters_count: U1,
    parameters: Vec<MethodParameter>,
}

#[derive(Debug)]
struct MethodParameter {
    name_index: U2,
    access_flags: U2,
}
