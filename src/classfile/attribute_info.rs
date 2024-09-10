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
pub struct ConstantValueAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    constant_value_index: U2,
}

impl ConstantValueAttribute {
    pub fn new(attribute_name_index: U2, attribute_length: U4, constant_value_index: U2) -> Self {
        Self {
            attribute_name_index,
            attribute_length,
            constant_value_index,
        }
    }
}

#[derive(Debug)]
pub struct ExceptionTableEntry {
    start_pc: U2,
    end_pc: U2,
    handler_pc: U2,
    catch_type: U2,
}

impl ExceptionTableEntry {
    pub fn new(start_pc: U2, end_pc: U2, handler_pc: U2, catch_type: U2) -> Self {
        Self {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        }
    }
}

#[derive(Debug)]
pub struct CodeAttribute {
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

impl CodeAttribute {
    pub fn new(
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
    ) -> Self {
        Self {
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
}

#[derive(Debug)]
pub struct StackMapTableAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    number_of_entries: U2,
    entries: Vec<StackMapFrame>,
}

#[derive(Debug)]
pub enum VerificationTypeInfo {
    TopVariable{tag: U1},
    IntegerVariable{tag: U1},
    FloatVariable{tag: U1},
    LongVariable{tag: U1},
    DoubleVariable{tag: U1},
    NullVariable{tag: U1},
    UninitializedThisVariable{tag: U1},
    ObjectVariable{tag: U1, cpool_index: U2},
    UninitializedVariable{tag: U1, offset: U2},
}

#[derive(Debug)]
pub enum StackMapFrame {
    SameFrame(SameFrame),
    SameLocals1StackItemFrame(SameLocals1StackItemFrame),
    SameLocals1StackItemFrameExtended(SameLocals1StackItemFrameExtended),
    ChopFrame(ChopFrame),
    SameFrameExtended(SameFrameExtended),
    AppendFrame(AppendFrame),
    FullFrame(FullFrame),
}

#[derive(Debug)]
pub struct SameFrame {
    //0-63
    frame_type: U1,
}

impl SameFrame {
    pub fn new(frame_type: U1) -> Self {
        Self {
            frame_type,
        }
    }
}

#[derive(Debug)]
pub struct SameLocals1StackItemFrame {
    //64-127
    frame_type: U1,
    stack: VerificationTypeInfo,
}

impl SameLocals1StackItemFrame {
    pub fn new(frame_type: U1, stack: VerificationTypeInfo) -> Self {
        Self {
            frame_type,
            stack,
        }
    }
}

#[derive(Debug)]
pub struct SameLocals1StackItemFrameExtended {
    //247
    frame_type: U1,
    offset_delta: U2,
    stack: VerificationTypeInfo,
}

impl SameLocals1StackItemFrameExtended {
    pub fn new(frame_type: U1, offset_delta: U2, stack: VerificationTypeInfo) -> Self {
        Self {
            frame_type,
            offset_delta,
            stack,
        }
    }
}

#[derive(Debug)]
pub struct ChopFrame {
    //248-250
    frame_type: U1,
    offset_delta: U2,
}

impl ChopFrame {
    pub fn new(frame_type: U1, offset_delta: U2) -> Self {
        Self {
            frame_type,
            offset_delta,
        }
    }
}

#[derive(Debug)]
pub struct SameFrameExtended {
    //251
    frame_type: U1,
    offset_delta: U2,
}

impl SameFrameExtended {
    pub fn new(frame_type: U1, offset_delta: U2) -> Self {
        Self {
            frame_type,
            offset_delta,
        }
    }
}

#[derive(Debug)]
pub struct AppendFrame {
    //252-254
    frame_type: U1,
    offset_delta: U2,
    locals: Vec<VerificationTypeInfo>,
}

impl AppendFrame {
    pub fn new(frame_type: U1, offset_delta: U2, locals: Vec<VerificationTypeInfo>) -> Self {
        Self {
            frame_type,
            offset_delta,
            locals,
        }
    }
}

#[derive(Debug)]
pub struct FullFrame {
    //255
    frame_type: U1,
    offset_delta: U2,
    number_of_locals: U2,
    locals: Vec<VerificationTypeInfo>,
    number_of_stack_items: U2,
    stack: Vec<VerificationTypeInfo>,
}

impl FullFrame {
    pub fn new(
        frame_type: U1,
        offset_delta: U2,
        number_of_locals: U2,
        locals: Vec<VerificationTypeInfo>,
        number_of_stack_items: U2,
        stack: Vec<VerificationTypeInfo>,
    ) -> Self {
        Self {
            frame_type,
            offset_delta,
            number_of_locals,
            locals,
            number_of_stack_items,
            stack,
        }
    }
}

#[derive(Debug)]
pub struct ExceptionsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    number_of_exceptions: U2,
    exception_index_table: Vec<U2>,
}

impl ExceptionsAttribute {
    pub fn new(
        attribute_name_index: U2,
        attribute_length: U4,
        number_of_exceptions: U2,
        exception_index_table: Vec<U2>,
    ) -> Self {
        Self {
            attribute_name_index,
            attribute_length,
            number_of_exceptions,
            exception_index_table,
        }
    }
}

#[derive(Debug)]
pub struct InnerClassesAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    number_of_classes: U2,
    classes: Vec<InnerClassInfo>,
}

impl InnerClassesAttribute {
    pub fn new(
        attribute_name_index: U2,
        attribute_length: U4,
        number_of_classes: U2,
        classes: Vec<InnerClassInfo>,
    ) -> Self {
        Self {
            attribute_name_index,
            attribute_length,
            number_of_classes,
            classes,
        }
    }
}

#[derive(Debug)]
pub struct InnerClassInfo {
    inner_class_info_index: U2,
    outer_class_info_index: U2,
    inner_name_index: U2,
    inner_class_access_flags: U2,
}

impl InnerClassInfo {
    pub fn new(
        inner_class_info_index: U2,
        outer_class_info_index: U2,
        inner_name_index: U2,
        inner_class_access_flags: U2,
    ) -> Self {
        Self {
            inner_class_info_index,
            outer_class_info_index,
            inner_name_index,
            inner_class_access_flags,
        }
    }
}

#[derive(Debug)]
pub struct EnclosingMethodAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    class_index: U2,
    method_index: U2,
}

impl EnclosingMethodAttribute {
    pub fn new(
        attribute_name_index: U2,
        attribute_length: U4,
        class_index: U2,
        method_index: U2,
    ) -> Self {
        Self {
            attribute_name_index,
            attribute_length,
            class_index,
            method_index,
        }
    }
}

#[derive(Debug)]
pub struct SyntheticAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
}

impl SyntheticAttribute {
    pub fn new(attribute_name_index: U2, attribute_length: U4) -> Self {
        Self {
            attribute_name_index,
            attribute_length,
        }
    }
}

#[derive(Debug)]
pub struct SignatureAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    signature_index: U2,
}

impl SignatureAttribute {
    pub fn new(attribute_name_index: U2, attribute_length: U4, signature_index: U2) -> Self {
        Self {
            attribute_name_index,
            attribute_length,
            signature_index,
        }
    }
}

#[derive(Debug)]
pub struct SourceFileAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    sourcefile_index: U2,
}

impl SourceFileAttribute {
    pub fn new(attribute_name_index: U2, attribute_length: U4, sourcefile_index: U2) -> Self {
        Self {
            attribute_name_index,
            attribute_length,
            sourcefile_index,
        }
    }
}

#[derive(Debug)]
pub struct SourceDebugExtensionAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    debug_extension: Vec<U1>,
}

impl SourceDebugExtensionAttribute {
    pub fn new(attribute_name_index: U2, attribute_length: U4, debug_extension: Vec<U1>) -> Self {
        Self {
            attribute_name_index,
            attribute_length,
            debug_extension,
        }
    }
}

#[derive(Debug)]
pub struct LineNumberTableAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    line_number_table_length: U2,
    line_number_table: Vec<LineNumberTableEntry>,
}

impl LineNumberTableAttribute {
    pub fn new(
        attribute_name_index: U2,
        attribute_length: U4,
        line_number_table_length: U2,
        line_number_table: Vec<LineNumberTableEntry>,
    ) -> Self {
        Self {
            attribute_name_index,
            attribute_length,
            line_number_table_length,
            line_number_table,
        }
    }
}

#[derive(Debug)]
pub struct LineNumberTableEntry {
    start_pc: U2,
    line_number: U2,
}

impl LineNumberTableEntry {
    pub fn new(start_pc: U2, line_number: U2) -> Self {
        Self { start_pc, line_number }
    }
}

#[derive(Debug)]
pub struct LocalVariableTableAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    local_variable_table_length: U2,
    local_variable_table: Vec<LocalVariableTableEntry>,
}

impl LocalVariableTableAttribute {
    pub fn new(
        attribute_name_index: U2,
        attribute_length: U4,
        local_variable_table_length: U2,
        local_variable_table: Vec<LocalVariableTableEntry>,
    ) -> Self {
        Self {
            attribute_name_index,
            attribute_length,
            local_variable_table_length,
            local_variable_table,
        }
    }
}

#[derive(Debug)]
pub struct LocalVariableTableEntry {
    start_pc: U2,
    length: U2,
    name_index: U2,
    descriptor_index: U2,
    index: U2,
}

impl LocalVariableTableEntry {
    pub fn new(
        start_pc: U2,
        length: U2,
        name_index: U2,
        descriptor_index: U2,
        index: U2,
    ) -> Self {
        Self {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        }
    }
}

#[derive(Debug)]
pub struct LocalVariableTypeTableAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    local_variable_type_table_length: U2,
    local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
}

impl LocalVariableTypeTableAttribute {
    pub fn new(
        attribute_name_index: U2,
        attribute_length: U4,
        local_variable_type_table_length: U2,
        local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
    ) -> Self {
        Self {
            attribute_name_index,
            attribute_length,
            local_variable_type_table_length,
            local_variable_type_table,
        }
    }
}

#[derive(Debug)]
pub struct LocalVariableTypeTableEntry {
    start_pc: U2,
    length: U2,
    name_index: U2,
    signature_index: U2,
    index: U2,
}

impl LocalVariableTypeTableEntry {
    pub fn new(
        start_pc: U2,
        length: U2,
        name_index: U2,
        signature_index: U2,
        index: U2,
    ) -> Self {
        Self {
            start_pc,
            length,
            name_index,
            signature_index,
            index,
        }
    }
}

#[derive(Debug)]
pub struct DeprecatedAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
}

impl DeprecatedAttribute {
    pub fn new(attribute_name_index: U2, attribute_length: U4) -> Self {
        Self {
            attribute_name_index,
            attribute_length,
        }
    }
}

#[derive(Debug)]
pub struct RuntimeVisibleAnnotationsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    num_annotations: U2,
    annotations: Vec<Annotation>,
}

impl RuntimeVisibleAnnotationsAttribute {
    pub fn new(
        attribute_name_index: U2,
        attribute_length: U4,
        num_annotations: U2,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            attribute_name_index,
            attribute_length,
            num_annotations,
            annotations,
        }
    }
}

#[derive(Debug)]
pub struct Annotation {
    type_index: U2,
    num_element_value_pairs: U2,
    element_value_pairs: Vec<ElementValuePair>,
}

impl Annotation {
    pub fn new(
        type_index: U2,
        num_element_value_pairs: U2,
        element_value_pairs: Vec<ElementValuePair>,
    ) -> Self {
        Self {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        }
    }
}

#[derive(Debug)]
pub struct ElementValuePair {
    element_name_index: U2,
    value: ElementValue,
}

impl ElementValuePair {
    pub fn new(element_name_index: U2, value: ElementValue) -> Self {
        Self {
            element_name_index,
            value,
        }
    }
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
pub struct ConstValueIndexElementValue {
    const_value_index: U2,
}

#[derive(Debug)]
pub struct EnumConstValueElementValue {
    type_name_index: U2,
    const_name_index: U2,
}

#[derive(Debug)]
pub struct ClassInfoIndexElementValue {
    class_info_index: U2,
}

#[derive(Debug)]
pub struct AnnotationValueElementValue {
    annotation_value: Annotation,
}

#[derive(Debug)]
pub struct ArrayValueElementValue {
    num_values: U2,
    values: Vec<ElementValue>,
}

#[derive(Debug)]
pub struct RuntimeInvisibleAnnotationsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    num_annotations: U2,
    annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct RuntimeVisibleParameterAnnotationsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    num_parameters: U1,
    parameter_annotations: Vec<ParameterAnnotation>,
}

#[derive(Debug)]
pub struct ParameterAnnotation {
    num_annotations: U2,
    annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct RuntimeInvisibleParameterAnnotationsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    num_parameters: U1,
    parameter_annotations: Vec<ParameterAnnotation>,
}

#[derive(Debug)]
pub struct RuntimeVisibleTypeAnnotationsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    num_annotations: U2,
    annotations: Vec<TypeAnnotation>,
}

#[derive(Debug)]
pub struct TypeAnnotation {
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
pub struct LocalVarTargetTable {
    start_pc: U2,
    length: U2,
    index: U2,
}

#[derive(Debug)]
pub struct TypePath {
    path_length: U1,
    path: Vec<TypePathKind>,
}

#[derive(Debug)]
pub struct TypePathKind {
    type_path_kind: U1,
    type_argument_index: U1,
}

#[derive(Debug)]
pub struct RuntimeInvisibleTypeAnnotationsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    num_annotations: U2,
    annotations: Vec<TypeAnnotation>,
}

#[derive(Debug)]
pub struct AnnotationDefaultAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    default_value: ElementValue,
}

#[derive(Debug)]
pub struct BootstrapMethodsAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    num_bootstrap_methods: U2,
    bootstrap_methods: Vec<BootstrapMethod>,
}

#[derive(Debug)]
pub struct BootstrapMethod {
    bootstrap_method_ref: U2,
    num_bootstrap_arguments: U2,
    bootstrap_arguments: Vec<U2>,
}

#[derive(Debug)]
pub struct MethodParametersAttribute {
    attribute_name_index: U2,
    attribute_length: U4,
    parameters_count: U1,
    parameters: Vec<MethodParameter>,
}

#[derive(Debug)]
pub struct MethodParameter {
    name_index: U2,
    access_flags: U2,
}
