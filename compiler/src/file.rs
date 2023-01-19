use std::{fs, io};

pub fn read_file(path: &String) -> io::Result<io::BufReader<fs::File>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file))
}
