#[cfg(test)]
mod compliance {
    use std::{collections::HashSet, fs::File, io::BufReader, path::PathBuf};

    use coffer::{Class, ReadWrite};

    #[test]
    fn classes() {
        read_folder(PathBuf::from("classes"), &mut 0, &mut HashSet::new());
    }

    fn read_folder(current: PathBuf, count: &mut usize, failed: &mut HashSet<PathBuf>) {
        for i in current.read_dir().unwrap().map(|x| x.unwrap()) {
            let path = i.path();
            if path.is_dir() {
                read_folder(path, count, failed);
            } else if i.file_name().to_string_lossy().ends_with(".class") {
                *count += 1;
                if !try_compile_file(&path) {
                    failed.insert(path);
                }
            }
        }
    }

    fn try_compile_file(current: &PathBuf) -> bool {
        println!("Testing compliance for {}", current.to_str().unwrap());
        let bytes = std::fs::read(current).unwrap();
        let f = File::open(current).unwrap();
        let mut reader = BufReader::new(f);
        let class: Class = ReadWrite::read_from(&mut reader).unwrap();
        let mut f1 = Vec::new();
        class.write_to(&mut f1).unwrap();
        bytes == f1
    }
}
