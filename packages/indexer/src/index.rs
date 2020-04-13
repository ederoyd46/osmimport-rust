#![allow(dead_code)]
pub mod data_types {
    pub struct Indexable<T> {
        pub name: T,
        pub modified: std::time::SystemTime,
        pub is_file: bool,
        pub is_dir: bool,
        pub is_symlink: bool
    }

    impl Indexable<String> {
        pub fn information(&self) -> String {
            format!("Name: {} Modified: {:?} FileType: {}", self.name, self.modified, self.file_type())
        }

        fn file_type(&self) -> String {
            if self.is_file {
                return String::from("File");
            } else if self.is_dir {
                return String::from("Directory");   
            } else {
                return String::from("SymLink");   
            }
        }
    
        pub fn copy(&self) -> Indexable<String> {
            return Indexable {
                name: self.name.clone(),
                modified: self.modified,
                is_file: self.is_file,
                is_dir: self.is_dir,
                is_symlink: self.is_symlink
            };
        }
    }
}