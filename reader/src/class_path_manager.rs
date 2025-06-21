use std::{
    fs::{self},
    io::BufReader,
};

use crate::{class_file::ClassFile, class_file_parse::ClassFileParser};

enum ClassPathEntry {
    DIR { path: String },
    JAR { path: String },
}

#[derive(Debug, Clone)]
pub struct ClassNotFoundError;

impl std::fmt::Display for ClassNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Class not found")
    }
}

pub struct ClassPathManager {
    run_time_class_path: Vec<ClassPathEntry>,
}

impl ClassPathManager {
    pub fn new() -> ClassPathManager {
        ClassPathManager {
            run_time_class_path: Vec::new(),
        }
    }

    pub fn add_class_path(&mut self, path: &str) {
        let abs_path = std::fs::canonicalize(path).expect("Invalid class path");
        println!("[ClassPathManager] 添加类路径: {}", abs_path.display());
        let md = fs::metadata(&abs_path).expect("Invalid class path");
        let source = if md.is_dir() {
            ClassPathEntry::DIR {
                path: abs_path.display().to_string(),
            }
        } else if md.is_file() && abs_path.to_string_lossy().ends_with(".jar") {
            ClassPathEntry::JAR {
                path: abs_path.display().to_string(),
            }
        } else {
            panic!("Invalid class path: {}", abs_path.display());
        };
        self.run_time_class_path.push(source);
    }

    pub fn add_class_paths(&mut self, paths: &str) {
        for path in paths.split(':').filter(|p| !p.is_empty()) {
            self.add_class_path(path);
        }
    }

    pub fn search_class(&self, class_name: &str) -> Result<ClassFile, ClassNotFoundError> {
        let file_name = class_name
            .trim_start_matches('L')
            .trim_end_matches(';')
            .replace("/", std::path::MAIN_SEPARATOR_STR)
            .replace(".", std::path::MAIN_SEPARATOR_STR)
            + ".class";
        
        println!("[ClassPathManager] 搜索类: {} (文件名: {})", class_name, file_name);
        println!("[ClassPathManager] 类路径条目数量: {}", self.run_time_class_path.len());
        
        for (i, entry) in self.run_time_class_path.iter().enumerate() {
            match entry {
                ClassPathEntry::DIR { path } => {
                    println!("[ClassPathManager] 条目 {}: 目录 {}", i, path);
                    let fname = std::path::Path::new(&path).join(&file_name);
                    println!("[ClassPathManager] 查找类文件: {}", fname.display());
                    match fs::File::open(&fname) {
                        Ok(file) => {
                            let reader = BufReader::new(file);
                            let class_file = ClassFileParser::file(reader).parse();
                            return Ok(class_file);
                        }
                        Err(_) => continue,
                    }
                }
                ClassPathEntry::JAR { path } => {
                    println!("[ClassPathManager] 条目 {}: JAR {}", i, path);
                    let fname = std::path::Path::new(&path);
                    println!("[ClassPathManager] 查找JAR文件: {}，类: {}", fname.display(), file_name);
                    if let Ok(file) = fs::File::open(fname) {
                        let reader = BufReader::new(file);
                        if let Ok(mut archive) = zip::ZipArchive::new(reader) {
                            if let Ok(class_file) = archive.by_name(&file_name) {
                                if class_file.is_file() {
                                    return Ok(ClassFileParser::zip(class_file).parse());
                                }
                            } else {
                                println!("[ClassPathManager] JAR中未找到类: {}", file_name);
                            }
                        } else {
                            println!("[ClassPathManager] 无法解析JAR文件: {}", path);
                        }
                    } else {
                        println!("[ClassPathManager] 无法打开JAR文件: {}", path);
                    }
                }
            }
        }
        println!("[ClassPathManager] 在所有类路径中未找到类: {}", class_name);
        Err(ClassNotFoundError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_class_path_manager() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");
        let mut class_path_manager = ClassPathManager::new();
        class_path_manager.add_class_paths(d.display().to_string().as_str());
        let class_file = class_path_manager.search_class("Main").unwrap();
        print!("{:?}", class_file);
    }

    // #[test]
    // fn test_class_path_manager_jar() {
    //     let mut class_path_manager = ClassPathManager::new();
    //     class_path_manager.add_class_paths("/home/codespace/java/current/jre/lib/rt.jar");
    //     let class_file = class_path_manager
    //         .search_class("java/lang/Boolean")
    //         .unwrap();
    //     print!("{:?}", class_file);
    // }
}
