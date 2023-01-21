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

    // dbg!(result.unwrap());
    let mut symbols = collections::radix_tree::RadixTree::new();
    symbols.insert("test".as_bytes().to_vec(), 1);
    symbols.insert("testa".as_bytes().to_vec(), 1);
    symbols.insert("tesalo".as_bytes().to_vec(), 1);
    symbols.insert("testament".as_bytes().to_vec(), 1);
    symbols.insert("testalogy".as_bytes().to_vec(), 1);
    symbols.insert("testoterone".as_bytes().to_vec(), 1);
    symbols.insert("title".as_bytes().to_vec(), 1);
    symbols.insert("tian".as_bytes().to_vec(), 1);
    symbols.insert("tesalonika".as_bytes().to_vec(), 1);
    symbols.insert("teslo".as_bytes().to_vec(), 1);
    symbols.insert("tomboy".as_bytes().to_vec(), 1);
    symbols.insert("tomato".as_bytes().to_vec(), 1);
    symbols.insert("tomatoes".as_bytes().to_vec(), 1);
    symbols.insert("tomatos".as_bytes().to_vec(), 1);
    symbols.insert("toes".as_bytes().to_vec(), 1);
    symbols.insert("cuteygendut".as_bytes().to_vec(), 1);
    symbols.insert("cuut".as_bytes().to_vec(), 1);
    symbols.insert("cut nyak dien".as_bytes().to_vec(), 1);
    dbg!(&symbols);
}
