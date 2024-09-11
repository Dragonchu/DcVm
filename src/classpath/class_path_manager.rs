use std::{
    fs::{self, File},
    io::{BufReader, Read},
};

use zip::read::ZipFile;

use crate::classfile::{self, class_file::ClassFile, class_file_parse::ClassFileParser};

enum ClassPathEntry {
    DIR { path: String },
    JAR { path: String },
}

#[derive(Debug, Clone)]
struct ClassNotFoundError;

impl std::fmt::Display for ClassNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Class not found")
    }
}

pub struct ClassPathManager {
    run_time_class_path: Vec<ClassPathEntry>,
}

impl ClassPathManager {
    fn new() -> ClassPathManager {
        ClassPathManager {
            run_time_class_path: Vec::new(),
        }
    }

    fn add_class_path(&mut self, path: &str) {
        let md = fs::metadata(&path).expect("Invalid class path");
        let source = if md.is_dir() {
            ClassPathEntry::DIR {
                path: String::from(path),
            }
        } else if md.is_file() && path.ends_with(".jar") {
            ClassPathEntry::JAR {
                path: String::from(path),
            }
        } else {
            panic!("Invalid class path");
        };
        self.run_time_class_path.push(source);
    }

    pub fn add_class_paths(&mut self, paths: &str) {
        for path in paths.split(':') {
            self.add_class_path(path);
        }
    }

    pub fn search_class(&mut self, class_name: &str) -> Result<ClassFile, ClassNotFoundError> {
        let file_name = class_name
            .replace("/", std::path::MAIN_SEPARATOR_STR)
            .replace(".", std::path::MAIN_SEPARATOR_STR)
            + ".class";
        for entry in &self.run_time_class_path {
            match entry {
                ClassPathEntry::DIR { path } => {
                    let fname = std::path::Path::new(&path).join(&file_name);
                    match fs::File::open(fname) {
                        Ok(file) => {
                            let reader = BufReader::new(file);
                            let class_file = ClassFileParser::file(reader).parse();
                            return Ok(class_file);
                        }
                        Err(_) => continue,
                    }
                }
                ClassPathEntry::JAR { path } => {
                    let fname = std::path::Path::new(&path);
                    match fs::File::open(fname) {
                        Ok(file) => {
                            let reader = BufReader::new(file);
                            let mut archive = zip::ZipArchive::new(reader).unwrap();
                            for i in 0..archive.len() {
                                let file = archive.by_index(i).unwrap();
                                if file.name() != file_name {
                                    continue;
                                }
                                if file.is_file() {
                                    let class_file = ClassFileParser::zip(file).parse();
                                    return Ok(class_file);
                                }
                            }
                        }
                        Err(_) => continue,
                    }
                }
            }
        }
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
}