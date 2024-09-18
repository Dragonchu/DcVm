use super::class_file::CpInfo;

pub fn require_constant(pool: &Vec<CpInfo>, index: usize) -> &CpInfo {
    let cp_info = pool.get(index).expect("Invalid constant pool index");
    cp_info
}
