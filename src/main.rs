use umlgen::generator::plantuml::*;
use umlgen::parser::*;
// use crate::parser::{cpp_parser, rust_parser};
use std::io::Read;
// use umlgen::hello;


fn main() {
    // umlgen::hello();
    // let mut p = cpp_parser::CppParser::new();
    let mut p = rust_parser::RustParser::new();
    // find language
    for entry in walkdir::WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".rs") {
            let mut buf: Vec<u8> = vec![];
            let mut file = std::fs::File::open(entry.path()).unwrap();
            let _ = file.read_to_end(&mut buf);
            // match language {}
            p.parse(&buf);
        }
    }
    println!("{}", p.to_plantuml());
}
