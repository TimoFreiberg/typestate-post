pub mod simple_types {
    use std::{collections::HashMap, error::Error, fs, fs::File, path::Path};

    pub fn list_dir(path: &Path) -> Result<Vec<File>, Box<dyn Error>> {
        let mut files = Vec::new();
        for dir_entry in fs::read_dir(path)? {
            files.push(File::open(dir_entry?.path())?)
        }
        Ok(files)
    }

    pub fn open_config(path: &Path) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let mut config = HashMap::new();
        for line in contents.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let line: Vec<_> = line.splitn(2, '=').collect();
            match line.as_slice() {
                [key, value] => {
                    config.insert(key.to_string(), value.to_string());
                }
                _ => {}
            }
        }
        Ok(config)
    }
}

pub mod type_state {
    use std::{collections::HashMap, error::Error, fs, marker::PhantomData, path};

    pub fn list_dir<B>(path: &Path<B, Directory>) -> Result<Vec<fs::File>, Box<dyn Error>> {
        super::simple_types::list_dir(&path.path)
    }

    pub fn open_config<B>(path: &Path<B, File>) -> Result<HashMap<String, String>, Box<dyn Error>> {
        super::simple_types::open_config(&path.path)
    }

    pub struct Path<B, T> {
        path: path::PathBuf,
        base: PhantomData<B>,
        target: PhantomData<T>,
    }

    pub struct Directory;
    pub struct File;

    pub struct Absolute;
    pub struct Relative;

    impl<B> Path<B, Directory> {
        pub fn join<T>(self, path: Path<Relative, T>) -> Path<B, T> {
            Path {
                path: self.path.join(path.path),
                base: PhantomData,
                target: PhantomData,
            }
        }
    }

    impl<T> Path<Absolute, T> {
        pub fn parent(self) -> Option<Path<Absolute, Directory>> {
            Some(Path {
                path: self.path.parent()?.to_owned(),
                base: PhantomData,
                target: PhantomData,
            })
        }
    }

    impl<B> Path<B, File> {
        pub fn filename(self) -> Path<Relative, File> {
            Path {
                path: self.path.file_name().unwrap().to_owned().into(),
                base: PhantomData,
                target: PhantomData,
            }
        }
    }
}
