use std::fmt;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, ScannerError>;

#[derive(Debug, Clone)]
pub struct ScannerError;

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "scanner failure")
    }
}

pub fn scan_source<R>(reader: &mut io::BufReader<R>) -> Result<()>
where
    R: std::io::Read,
{
    let mut buf = Vec::<u8>::new();
    while match reader.read_until(b'\n', &mut buf) {
        Ok(len) => len,
        Err(err) => return Err(ScannerError),
    } != 0
    {
        let s = match String::from_utf8(buf) {
            Ok(value) => value,
            Err(err) => return Err(ScannerError),
        };
        for c in s.chars() {
            println!("Character: {}", c);
        }
        buf = s.into_bytes();
        buf.clear();
    }

    Ok(())
}
