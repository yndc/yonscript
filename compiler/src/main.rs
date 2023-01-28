use crate::lexer::Lexer;
use std;

mod collections;
mod dictionary;
mod lexer;
mod token;

fn main() {
    let mut lexer = Lexer::new();

    // let args: Vec<String> = std::env::args().collect();
    // let result = lexer.scan_file(&args[1]);

    let result = lexer.scan_file(&"../docs/examples/hello-events/script.ys".to_string());
    dbg!(result.unwrap());
    // let mut symbols = collections::radix_tree::RadixTree::new();
    // symbols.insert("test".as_bytes().to_vec(), 1);
    // symbols.insert("testa".as_bytes().to_vec(), 2);
    // symbols.insert("tesalo".as_bytes().to_vec(), 3);
    // symbols.insert("testament".as_bytes().to_vec(), 4);
    // symbols.insert("testalogy".as_bytes().to_vec(), 5);
    // symbols.insert("testoterone".as_bytes().to_vec(), 6);
    // symbols.insert("title".as_bytes().to_vec(), 7);
    // symbols.insert("tian".as_bytes().to_vec(), 8);
    // symbols.insert("tesalonika".as_bytes().to_vec(), 9);
    // symbols.insert("teslo".as_bytes().to_vec(), 10);
    // symbols.insert("tomboy".as_bytes().to_vec(), 11);
    // symbols.insert("tomato".as_bytes().to_vec(), 12);
    // symbols.insert("tomatoes".as_bytes().to_vec(), 13);
    // symbols.insert("tomatos".as_bytes().to_vec(), 14);
    // symbols.insert("toes".as_bytes().to_vec(), 15);
    // symbols.insert("cuteygendut".as_bytes().to_vec(), 16);
    // symbols.insert("cuut".as_bytes().to_vec(), 17);
    // symbols.insert("cut nyak dien".as_bytes().to_vec(), 18);
    // dbg!(&symbols);
}
