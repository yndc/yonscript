use crate::{file::read_file, scanner::scan_source};
use std::{fs, io};

mod collections;
mod file;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_paths = &args[1..];
    let result: Result<Vec<Vec<token::Token>>, scanner::ScannerError> = file_paths
        .iter()
        .map(read_file)
        .collect::<io::Result<Vec<io::BufReader<fs::File>>>>()
        .unwrap()
        .iter_mut()
        .map(scan_source)
        .collect();

    dbg!(result.unwrap());

    let symbols = collections::radix_tree::RadixTree::new();
    symbols.insert("test".as_bytes().to_vec(), 1);
    symbols.insert("testa".as_bytes().to_vec(), 1);
    symbols.insert("testament".as_bytes().to_vec(), 1);
    symbols.insert("testalogy".as_bytes().to_vec(), 1);
    // dbg!(symbols);
}
