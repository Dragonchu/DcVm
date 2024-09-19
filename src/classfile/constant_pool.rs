use super::class_file::CpInfo;

pub fn require_constant(pool: &Vec<CpInfo>, index: usize) -> &CpInfo {
    let cp_info = pool.get(index).expect("Invalid constant pool index");
    cp_info
}

pub fn require_constant_utf8(pool: &Vec<CpInfo>, index: usize) -> String {
    let cp_info = require_constant(pool, index);
    if let CpInfo::Utf8 { tag: _, length: _, bytes } = cp_info {
        String::from_utf8_lossy(bytes).to_string()
    } else {
        panic!("Expected CpInfo::Utf8, found {:?}", cp_info);
    }
}
