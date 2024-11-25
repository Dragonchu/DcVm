use reader::types::{U1, U2, U4};
struct ExceptionEntry {
    start_pc: U2,
    end_pc: U2,
    handler_pc: U2,
    catch_type: U2
}
struct Code {
    max_stack: U2,
    max_locals: U2,
    code: Vec<U1>,
    exception_table_length: U2,
    exception_table: Vec<ExceptionEntry>
}
struct Method {
    access_flags: U2,
    name: String,
    descriptor: String,
    code: Code,
}