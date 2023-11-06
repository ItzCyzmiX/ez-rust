pub fn read_file(path: &str) -> String {
    use std::fs;

    let content = fs::read_to_string(path)
                                            .expect("Error while reading file!");

    return content.to_string();
}

pub fn write_file(path: &str, content: &str) -> bool {
    use std::fs;

    match fs::write(path, content) {
        Err(err) => {
            println!("{err}");
            return true;
        }

        Ok(_) => {
            return false;
        }
    }
       
}

pub fn append_file(path: &str, content: &str) -> bool {
    let mut init_file = read_file(path);
    init_file.push_str(content);

    let err: bool = write_file(path, &init_file);

    return err
}

pub fn create_file(path: &str) -> bool {
    use std::fs;

    match fs::File::create(path) {
        Ok(_) => {
            return false;
        }
        Err(err) => {
            println!("{err}");
            return true;
        }

    }
}

pub fn create_file_write(path: &str, content: &str) -> bool {
    let err1 = create_file(path);
    if err1 {
        return true;
    } 
    let err2: bool = write_file(path, content);

    return err2
}   