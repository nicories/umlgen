use crate::uml::*;

pub mod cpp_parser;
pub mod rust_parser;
pub enum LangChoice {
    RUST,
    CPP,
}

pub trait LangParser {
    fn parse(&mut self, buffer: &Vec<u8>);
    fn classes(&self) -> &Vec<UmlClass>;
    fn assocations(&self) -> &Vec<UmlAssociation>;
    fn enums(&self) -> &Vec<UmlEnum>;
}
