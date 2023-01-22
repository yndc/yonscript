// use std::fmt;

// /**
//  * dictionary.rs
//  *
//  * Dictionary is a struct that contains all keyword, operator, and pattern definitions.  
//  */

// pub struct Error {
//     record: Record,
// }

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "dictionary error for record: {}", self.record)
//     }
// }

// struct Dictionary {
//     records: Vec<Record>,
// }

// impl Dictionary {
//     pub fn new() -> Dictionary {
//         return Dictionary {
//             records: Vec::new(),
//         };
//     }

//     pub fn add_record(&mut self, record: Record) -> Result<(), Error> {
//         Ok(())
//     }
// }

// /// Definition of a token inside a dictionary
// enum Record {
//     Exact(i32, String),
//     Pattern(i32, String),
// }

// impl fmt::Display for Record {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match &self {
//             Self::Exact(priority, string) => {
//                 write!(f, "Exact   ({}): {}", priority, string)
//             }
//             Self::Pattern(priority, string) => {
//                 write!(f, "Pattern ({}): {}", priority, string)
//             }
//         }
//     }
// }
