use std::sync::Arc;

use crate::common::types::{U1, U2, U4};

#[derive(Debug)]
pub enum AttributeInfo {
    ConstantValue {
        attribute_name_index: U2,
        attribute_length: U4,
        constant_value_index: U2,
    },
    Code(CodeAtrribute),
    StackMapTable {
        attribute_name_index: U2,
        attribute_length: U4,
        number_of_entries: U2,
        entries: Vec<StackMapFrame>,
    },
    Exceptions(ExceptionsAttribute),
    InnerClasses(InnerClassesAttribute),
    EnclosingMethod(EnclosingMethodAttribute),
    Synthetic {
        attribute_name_index: U2,
        attribute_length: U4,
    },
    Signature {
        attribute_name_index: U2,
        attribute_length: U4,
        signature_index: U2,
    },
    SourceFile {
        attribute_name_index: U2,
        attribute_length: U4,
        sourcefile_index: U2,
    },
    SourceDebugExtension {
        attribute_name_index: U2,
        attribute_length: U4,
        debug_extension: Vec<U1>,
    },
    LineNumberTable {
        attribute_name_index: U2,
        attribute_length: U4,
        line_number_table_length: U2,
        //start_pc, line_number
        line_number_table: Vec<(U2, U2)>,
    },
    LocalVariableTable {
        attribute_name_index: U2,
        attribute_length: U4,
        local_variable_table_length: U2,
        //start_pc, length, name_index, descriptor_index, index
        local_variable_table: Vec<(U2, U2, U2, U2, U2)>,
    },
    LocalVariableTypeTable {
        attribute_name_index: U2,
        attribute_length: U4,
        local_variable_type_table_length: U2,
        //start_pc, length, name_index, signature_index, index
        local_variable_type_table: Vec<(U2, U2, U2, U2, U2)>,
    },
    Deprecated {
        attribute_name_index: U2,
        attribute_length: U4,
    },
    RuntimeVisibleAnnotations {
        attribute_name_index: U2,
        attribute_length: U4,
        num_annotations: U2,
        annotations: Vec<Annotation>,
    },
    RuntimeInvisibleAnnotations {
        attribute_name_index: U2,
        attribute_length: U4,
        num_annotations: U2,
        annotations: Vec<Annotation>,
    },
    RuntimeVisibleParameterAnnotations {
        attribute_name_index: U2,
        attribute_length: U4,
        num_parameters: U1,
        //num_annotations, annotations
        parameter_annotations: Vec<(U2, Vec<Annotation>)>,
    },
    RuntimeInvisibleParameterAnnotations {
        attribute_name_index: U2,
        attribute_length: U4,
        num_parameters: U1,
        //num_annotations, annotations
        parameter_annotations: Vec<(U2, Vec<Annotation>)>,
    },
    RuntimeVisibleTypeAnnotations {
        attribute_name_index: U2,
        attribute_length: U4,
        num_annotations: U2,
        annotations: Vec<TypeAnnotation>,
    },
    RuntimeInvisibleTypeAnnotations {
        attribute_name_index: U2,
        attribute_length: U4,
        num_annotations: U2,
        annotations: Vec<TypeAnnotation>,
    },
    AnnotationDefault {
        attribute_name_index: U2,
        attribute_length: U4,
        default_value: ElementValue,
    },
    BootstrapMethods(BootstrapMethodsAttribute),
    MethodParameters {
        attribute_name_index: U2,
        attribute_length: U4,
        parameters_count: U1,
        //name_index, access_flags
        parameters: Vec<(U2, U2)>,
    },
}

type InnerClassesAttributeRef = Option<Arc<InnerClassesAttribute>>;
#[derive(Debug)]
pub struct InnerClassesAttribute {
    pub attribute_name_index: U2,
    pub attribute_length: U4,
    pub number_of_classes: U2,
    //inner_class_info_index, outer_class_info_index, inner_name_index, inner_class_access_flags
    pub classes: Vec<(U2, U2, U2, U2)>,
}

#[derive(Debug)]
pub struct EnclosingMethodAttribute {
    pub attribute_name_index: U2,
    pub attribute_length: U4,
    pub class_index: U2,
    pub method_index: U2,
}

#[derive(Debug)]
pub struct BootstrapMethodsAttribute {
    pub attribute_name_index: U2,
    pub attribute_length: U4,
    pub num_bootstrap_methods: U2,
    //bootstrap_method_ref, num_bootstrap_arguments, bootstrap_arguments
    pub bootstrap_methods: Vec<(U2, U2, Vec<U2>)>,
}

#[derive(Debug)]
pub struct ExceptionsAttribute {
    pub attribute_name_index: U2,
    pub attribute_length: U4,
    pub number_of_exceptions: U2,
    pub exception_index_table: Vec<U2>,
}

#[derive(Debug)]
pub struct CodeAtrribute {
    pub attribute_name_index: U2,
    pub attribute_length: U4,
    pub max_stack: U2,
    pub max_locals: U2,
    pub code_length: U4,
    pub code: Vec<U1>,
    pub exception_table_length: U2,
    //start_pc, end_pc, handler_pc, catch_type
    pub exception_table: Vec<(U2, U2, U2, U2)>,
    pub attributes_count: U2,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub enum StackMapFrame {
    SameFrame {
        frame_type: U1,
    },
    SameLocals1StackItemFrame {
        frame_type: U1,
        stack: [VerificationTypeInfo; 1],
    },
    SameLocals1StackItemFrameExtended {
        frame_type: U1,
        offset_delta: U2,
        stack: [VerificationTypeInfo; 1],
    },
    ChopFrame {
        frame_type: U1,
        offset_delta: U2,
    },
    SameFrameExtended {
        frame_type: U1,
        offset_delta: U2,
    },
    AppendFrame {
        frame_type: U1,
        offset_delta: U2,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame {
        frame_type: U1,
        offset_delta: U2,
        number_of_locals: U2,
        locals: Vec<VerificationTypeInfo>,
        number_of_stack_items: U2,
        stack: Vec<VerificationTypeInfo>,
    },
}

#[derive(Debug)]
pub enum VerificationTypeInfo {
    TopVariable { tag: U1 },
    IntegerVariable { tag: U1 },
    FloatVariable { tag: U1 },
    LongVariable { tag: U1 },
    DoubleVariable { tag: U1 },
    NullVariable { tag: U1 },
    UninitializedThisVariable { tag: U1 },
    ObjectVariable { tag: U1, cpool_index: U2 },
    UninitializedVariable { tag: U1, offset: U2 },
}

#[derive(Debug)]
pub struct Annotation {
    pub type_index: U2,
    pub num_element_value_pairs: U2,
    pub element_value_pairs: Vec<(U2, ElementValue)>,
}

#[derive(Debug)]
pub struct ElementValue {
    pub tag: U1,
    pub value: ElementValueItem,
}

#[derive(Debug)]
pub enum ElementValueItem {
    ConstValueIndex {
        const_value_index: U2,
    },
    EnumConstValue {
        type_name_index: U2,
        const_name_index: U2,
    },
    ClassInfoIndex {
        class_info_index: U2,
    },
    AnnotationValue {
        annotation_value: Annotation,
    },
    ArrayValue {
        num_values: U2,
        values: Vec<ElementValue>,
    },
}

#[derive(Debug)]
pub struct ParameterAnnotation {
    num_annotations: U2,
    annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct TypeAnnotation {
    pub target_type: U1,
    pub target_info: TargetInfo,
    pub target_path: TypePath,
    pub type_index: U2,
    pub num_element_value_pairs: U2,
    pub element_value_pairs: Vec<(U2, ElementValue)>,
}

#[derive(Debug)]
pub enum TargetInfo {
    TypeParameterTarget {
        type_parameter_index: U1,
    },
    SuperTypeTarget {
        supertype_index: U2,
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
        //start_pc, length, index
        table: Vec<(U2, U2, U2)>,
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
pub struct TypePath {
    pub path_length: U1,
    //type_path_kind, type_argument_index
    pub path: Vec<(U1, U1)>,
}
