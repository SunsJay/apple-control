use std::fs::File;
use std::io;
use std::io::Read;

pub fn read(filename: &str) -> Result<String, io::Error> {
    let mut contents = String::new();

    let res = File::open(filename);

    match res {
        Ok(mut file) => {
            let res = file.read_to_string(&mut contents);
            match res {
                Ok(_) => delete(filename)?,
                Err(e) => match e.kind() {
                    io::ErrorKind::NotFound => {}
                    _ => return Err(e),
                },
            }
        }
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {}
            _ => return Err(e),
        },
    }

    Ok(contents)
}

pub fn delete(filename: &str) -> Result<(), io::Error> {
    std::fs::remove_file(filename)?;
    Ok(())
}