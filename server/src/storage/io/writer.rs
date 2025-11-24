use std::fs::File;

use std::io::prelude::*;
use std::path::Path;

pub struct FileWriter {
    file: File,
}

impl FileWriter {
    fn new(path: &Path) -> FileWriter {
        let file = if path.exists() {
            File::open(path).expect("File open error")
        } else {
            File::create(path).expect("File create error")
        };
        FileWriter {file}
    }
    
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.file.write(data)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;

    #[test]
    fn creates_new_file_if_not_exists() {
        let path = PathBuf::from("test_create_file.txt");
        if path.exists() {
            fs::remove_file(&path).unwrap();
        }

        let _ = FileWriter::new(&path);
        assert!(path.exists());

        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn opens_existing_file() {
        let path = PathBuf::from("test_open_file.txt");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "test content").unwrap();

        let _ = FileWriter::new(&path);
        assert!(path.exists());

        fs::remove_file(&path).unwrap();
    }

    #[test]
    #[should_panic(expected = "File open")]
    fn panics_when_opening_unreadable_file() {
        let path = PathBuf::from("test_unreadable_file.txt");
        let _ = File::create(&path).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o000)).unwrap();

        let _ = FileWriter::new(&path);

        fs::remove_file(&path).unwrap();
    }

    #[test]
    #[should_panic(expected = "File create")]
    fn panics_when_creating_file_in_unwritable_directory() {
        let path = PathBuf::from("/root/test_unwritable_file.txt");
        let _ = FileWriter::new(&path);
    }
}