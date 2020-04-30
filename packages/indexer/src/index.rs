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

// ------- OLD CODE ---------
// fn visit_dirs(dir: &Path) -> io::Result<()> {
//     if dir.is_dir() {
//         for entry in fs::read_dir(dir)? {
//             let entry = entry?;
//             let path = entry.path();
//             match path.is_dir() {
//                 true => visit_dirs(&path)?,
//                 false => println!("{}", index_file(entry)?.information()),
//             }
//         }
//     }
//     Ok(())
// }

// fn index_file(entry: fs::DirEntry) -> io::Result<Indexable<String>> {
//     let file_name = String::from(entry.file_name().to_str().unwrap());
//     let modified = entry.metadata()?.created()?;
//     let index = Indexable {
//         name: file_name,
//         modified: modified,
//         is_file: entry.file_type()?.is_file(),
//         is_dir: entry.file_type()?.is_dir(),
//         is_symlink: entry.file_type()?.is_symlink(),
//     };

//     return Ok(index);
// }
