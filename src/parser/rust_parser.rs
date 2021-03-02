use crate::uml::*;
use tree_sitter::{Node, Query, QueryCursor};
pub const METHOD_ARGS_QUERY: &str = "
(parameter pattern: (identifier) @function.parameter.name
  type: (_) @function.parameter.type)
";
pub const METHOD_QUERY: &str = "
(function_item
  (visibility_modifier)? @function.visibility
    name: (identifier) @function.name
    parameters: (parameters) @function.parameters
    return_type: (_)? @function.return_type)
";
pub const CLASS_FIELDS_QUERY: &str = "
(field_declaration
  (visibility_modifier)? @class.field.visibility
  name: (field_identifier) @class.field.name
  type: (_) @class.field.type)
";
pub const CLASS_QUERY: &str = "
(
  (struct_item
    name: (type_identifier) @struct.name
    body: (field_declaration_list ) @class.fields)
  (impl_item
    type: (type_identifier) @class.name
    body: (declaration_list) @class.functions)+
(#eq? @struct.name @class.name))";

pub struct RustParser {
    ts_parser: tree_sitter::Parser,
    classes: Vec<UmlClass>,
    enums: Vec<UmlEnum>,
    method_query: Query,
    method_args_query: Query,
    class_query: Query,
    class_fields_query: Query,
}
impl crate::parser::LangParser for RustParser {
    fn parse(&mut self, buffer: &Vec<u8>) {
        let tree = self
            .ts_parser
            .parse(buffer, None)
            .expect("Error Parsing root node!");
        self.classes.append(&mut self.parse_classes(tree.root_node(), buffer));
    }

    fn classes(&self) -> &Vec<UmlClass> {
        &self.classes
    }

    fn enums(&self) -> &Vec<UmlEnum> {
        &self.enums
    }

    fn assocations(&self) -> &Vec<UmlAssociation> {
        todo!()
    }
}
impl RustParser {
    pub fn new() -> Self {
        extern "C" {
            fn tree_sitter_rust() -> tree_sitter::Language;
        }
        let language = unsafe { tree_sitter_rust() };
        let mut ts_parser = tree_sitter::Parser::new();
        let _ = ts_parser.set_language(language);
        RustParser {
            ts_parser,
            method_args_query: Query::new(language, METHOD_ARGS_QUERY).unwrap(),
            method_query: Query::new(language, METHOD_QUERY).unwrap(),
            class_query: Query::new(language, CLASS_QUERY).unwrap(),
            class_fields_query: Query::new(language, CLASS_FIELDS_QUERY).unwrap(),
            classes: vec![],
            enums: vec![],
        }
    }
    pub fn parse_method_args(&self, node: Node, buffer: &Vec<u8>) -> Vec<UmlParameter> {
        let mut v = vec![];

        QueryCursor::new()
            .matches(&self.method_args_query, node, |x| {
                x.utf8_text(&buffer).unwrap()
            })
            .for_each(|m| {
                let mut data_type: Option<String> = None;
                let mut name: Option<String> = None;
                m.captures.iter().for_each(|c| {
                    match self.method_args_query.capture_names()[c.index as usize].as_str() {
                        "function.parameter.name" => {
                            name = Some(c.node.utf8_text(&buffer).unwrap().to_owned())
                        }
                        "function.parameter.type" => {
                            data_type = Some(c.node.utf8_text(&buffer).unwrap().to_owned())
                        }
                        _ => {
                            panic!("{}", c.node.utf8_text(&buffer).unwrap().to_owned());
                        }
                    }
                });
                match (name, data_type) {
                    (Some(name), Some(data_type)) => v.push(UmlParameter { data_type, name }),
                    _ => {
                        panic!("{}", node.utf8_text(&buffer).unwrap().to_owned());
                    }
                }
            });
        v
    }
    pub fn parse_methods(&self, node: Node, buffer: &Vec<u8>) -> Vec<UmlMethod> {
        let mut v = vec![];
        // let buffer = buffer.clone();
        QueryCursor::new()
            .matches(&self.method_query, node, |x| x.utf8_text(buffer).unwrap())
            .for_each(|m| {
                let mut return_type: Option<String> = None;
                let mut name: Option<String> = None;
                let mut parameters: Vec<UmlParameter> = vec![];
                let mut visibility = UmlVisibility::Private;
                for c in m.captures.iter() {
                    match self.method_query.capture_names()[c.index as usize].as_str() {
                        "function.name" => {
                            name = Some(c.node.utf8_text(&buffer).unwrap().to_owned())
                        }
                        "function.return_type" => {
                            return_type = Some(c.node.utf8_text(&buffer).unwrap().to_owned())
                        }
                        "function.parameters" => {
                            parameters = self.parse_method_args(c.node, buffer);
                        }
                        "function.visibility" => {
                            if c.node.utf8_text(&buffer).unwrap() == "pub" {
                                visibility = UmlVisibility::Public;
                            }
                        }

                        _ => {}
                    }
                }
                v.push(UmlMethod {
                    name: name.as_ref().unwrap().to_string(),
                    visibility,
                    parameters,
                    return_type,
                });
            });
        v
    }

    fn parse_class_fields(&self, node: Node, buffer: &Vec<u8>) -> Vec<UmlField> {
        let mut v = vec![];
        QueryCursor::new()
            .matches(&&self.class_fields_query, node, |x| {
                x.utf8_text(buffer).unwrap()
            })
            .for_each(|m| {
                let mut data_type: Option<String> = None;
                let mut name: Option<String> = None;
                let mut visibility = UmlVisibility::Private;
                for c in m.captures.iter() {
                    match self.class_fields_query.capture_names()[c.index as usize].as_str() {
                        "class.field.name" => {
                            name = Some(c.node.utf8_text(buffer).unwrap().to_owned())
                        }
                        "class.field.type" => {
                            data_type = Some(c.node.utf8_text(buffer).unwrap().to_owned())
                        }
                        "class.field.visibility" => {
                            if c.node.utf8_text(&buffer).unwrap() == "pub" {
                                visibility = UmlVisibility::Public;
                            }
                        }
                        _ => panic!("{}", c.node.utf8_text(buffer).unwrap().to_owned()),
                    }
                }
                v.push(UmlField {
                    name: name.as_ref().unwrap().to_string(),
                    data_type: data_type.as_ref().unwrap().to_string(),
                    visibility: visibility,
                });
            });
        v
    }
    pub fn parse_classes(&self, node: Node, buffer: &Vec<u8>) -> Vec<UmlClass> {
        // struct + impl => class
        // classes
        let mut classes: Vec<UmlClass> = vec![];
        QueryCursor::new()
            .matches(&self.class_query, node, |x| x.utf8_text(buffer).unwrap())
            .for_each(|m| {
                let mut name: Option<String> = None;
                let mut fields: Vec<UmlField> = vec![];
                let mut methods: Vec<UmlMethod> = vec![];
                let mut associations: Vec<UmlAssociation> = vec![];
                let mut extends: Vec<String> = vec![];
                let mut implements: Vec<String> = vec![];
                for c in m.captures.iter() {
                    match self.class_query.capture_names()[c.index as usize].as_str() {
                        "class.fields" => {
                            fields = self.parse_class_fields(c.node, buffer);
                        }
                        "class.name" => name = Some(c.node.utf8_text(buffer).unwrap().to_owned()),
                        "class.functions" => {
                            methods = self.parse_methods(c.node, buffer);
                        }
                        _ => {}
                    }
                }
                classes.push(UmlClass {
                    name: name.expect("No class name found"),
                    fields,
                    methods,
                    modifier: None,
                    visibility: UmlVisibility::Public,
                    extends,
                    implements,
                    associations,
                })
            });

        classes
    }
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::parser::*;
    use crate::uml::*;

    #[test]
    fn test_rust_parse_methods() {
        let mut p = RustParser::new();
        let source_code = "
impl X {
pub fn func1() -> i32 {}
fn func2() -> i32 {}
}
";
        let tree = p.ts_parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();

        let methods = p.parse_methods(root_node, &source_code.as_bytes().to_vec());
        let m0 = methods.get(0).unwrap();
        assert!(m0.name == "func1");
        assert!(m0.return_type.as_ref().unwrap() == "i32");
        assert!(matches!(m0.visibility, UmlVisibility::Public));

        let m1 = methods.get(1).unwrap();
        assert!(matches!(m1.visibility, UmlVisibility::Private));
    }

    #[test]
    fn test_rust_parse_method_args() {
        let mut p = RustParser::new();
        let source_code = "
impl X {
pub fn func1(arg1: i32, arg2: i32) -> i32 {}
}
";
        let tree = p.ts_parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();

        let args = p.parse_method_args(root_node, &source_code.as_bytes().to_vec());
        assert!(args.get(0).unwrap().name == "arg1");
        assert!(args.get(0).unwrap().data_type == "i32");
        assert!(args.get(1).unwrap().name == "arg2");
        assert!(args.get(1).unwrap().data_type == "i32");

        assert!(args.get(2).is_none());
    }

    #[test]
    fn test_rust_parse_classes_2_impl() {
        let mut p = RustParser::new();
        let source_code = "
struct X {
}
impl X {
pub fn func1() -> i32 {}
}
impl A for X {
pub fn func2() -> i32 {}
}
";
        let tree = p.ts_parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();

        let classes = p.parse_classes(root_node, &source_code.as_bytes().to_vec());
        assert!(classes.get(0).unwrap().name == "X");
        assert!(classes.get(1).is_none());
    }
    #[test]
    fn test_rust_parse_classes() {
        let mut p = RustParser::new();
        let source_code = "
struct X {
pub a: i32,
b: i32,
}
impl X {
pub fn func1() -> i32 {}
}

struct Y {
a: i32,
b: i32,
}
impl Y {
pub fn func1() -> i32 {}
}
";
        let tree = p.ts_parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();

        let classes = p.parse_classes(root_node, &source_code.as_bytes().to_vec());

        assert!(classes.get(0).unwrap().name == "X");
        assert!(classes.get(0).unwrap().fields.get(0).unwrap().name == "a");
        assert!(classes.get(0).unwrap().fields.get(0).unwrap().data_type == "i32");
        assert!(matches!(
            classes.get(0).unwrap().fields.get(0).unwrap().visibility,
            UmlVisibility::Public
        ));
        assert!(classes.get(0).unwrap().fields.get(1).unwrap().name == "b");
        assert!(classes.get(0).unwrap().fields.get(1).unwrap().data_type == "i32");
        assert!(classes.get(0).unwrap().methods.get(0).unwrap().name == "func1");
        assert!(matches!(
            classes.get(0).unwrap().methods.get(0).unwrap().visibility,
            UmlVisibility::Public
        ));

        assert!(classes.get(1).unwrap().name == "Y");
        assert!(classes.get(1).unwrap().fields.get(0).unwrap().name == "a");
        assert!(classes.get(1).unwrap().fields.get(0).unwrap().data_type == "i32");
        assert!(matches!(
            classes.get(1).unwrap().fields.get(0).unwrap().visibility,
            UmlVisibility::Private
        ));
        assert!(classes.get(1).unwrap().fields.get(1).unwrap().name == "b");
        assert!(classes.get(1).unwrap().fields.get(1).unwrap().data_type == "i32");
        assert!(classes.get(1).unwrap().methods.get(0).unwrap().name == "func1");

        assert!(classes.get(2).is_none());
    }
}
