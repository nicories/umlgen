use crate::uml::*;
use tree_sitter::{Node, Query, QueryCursor};

pub const METHOD_ARGS_QUERY: &str = "
[(parameter_declaration
  type: (_)@function.parameter.type
  declarator: (_)@function.parameter.name
  )
(optional_parameter_declaration
  type: (_)@function.parameter.type
  declarator: (_)@function.parameter.name
  )]
";
pub const METHOD_QUERY: &str = "
[
(declaration
  declarator: (function_declarator
    declarator: (_)@function.name
    parameters: (_)@function.parameters
))
(field_declaration type: (_) @function.return_type
  declarator: (function_declarator
    declarator: (_)@function.name
    parameters: (_)@function.parameters
    ))
]
";
pub const CLASS_FIELDS_QUERY: &str = "
(field_declaration
  type: (primitive_type) @class.field.type
  [(field_identifier)(array_declarator)] @class.field.name
)
";

pub const CLASS_ASSOCIATIONS_QUERY: &str = "
(field_declaration
  type: (type_identifier) @class.association.to
  [(field_identifier)(array_declarator)] @class.association.to_title
)
";

pub const CLASS_EXTENDS_QUERY: &str = "
(field_declaration
  type: (type_identifier) @class.association.to
  [(field_identifier)(array_declarator)] @class.association.to_title
)
";
pub const CLASS_QUERY: &str = "
(class_specifier
  name: (_)@class.name
  body: (field_declaration_list) @class.fields) @class
";

pub struct CppParser {
    ts_parser: tree_sitter::Parser,
    classes: Vec<UmlClass>,
    enums: Vec<UmlEnum>,
    associations: Vec<UmlAssociation>,
    method_query: Query,
    method_args_query: Query,
    class_query: Query,
    class_associations_query: Query,
    class_fields_query: Query,
}
impl crate::parser::LangParser for CppParser {
    fn parse(&mut self, buffer: &Vec<u8>) {
        let tree = self
            .ts_parser
            .parse(buffer, None)
            .expect("Error Parsing root node!");
        self.classes
            .append(self.parse_classes(tree.root_node(), buffer).as_mut());
        // self.classes = self.parse_classes(tree.root_node(), buffer);
    }

    fn classes(&self) -> &Vec<UmlClass> {
        &self.classes
    }

    fn enums(&self) -> &Vec<UmlEnum> {
        &self.enums
    }

    fn assocations(&self) -> &Vec<UmlAssociation> {
        &self.associations
    }
}
impl CppParser {
    pub fn new() -> Self {
        extern "C" {
            fn tree_sitter_cpp() -> tree_sitter::Language;
        }
        let language = unsafe { tree_sitter_cpp() };
        let mut ts_parser = tree_sitter::Parser::new();
        let _ = ts_parser.set_language(language);
        CppParser {
            ts_parser,
            method_args_query: Query::new(language, METHOD_ARGS_QUERY).unwrap(),
            method_query: Query::new(language, METHOD_QUERY).unwrap(),
            class_query: Query::new(language, CLASS_QUERY).unwrap(),
            class_associations_query: Query::new(language, CLASS_ASSOCIATIONS_QUERY).unwrap(),
            class_fields_query: Query::new(language, CLASS_FIELDS_QUERY).unwrap(),
            classes: vec![],
            enums: vec![],
            associations: vec![],
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
        for m in
            QueryCursor::new().matches(&self.method_query, node, |x| x.utf8_text(buffer).unwrap())
        {
            let mut return_type: Option<String> = None;
            let mut name: Option<String> = None;
            let mut params: Vec<UmlParameter> = vec![];
            for c in m.captures.iter() {
                match self.method_query.capture_names()[c.index as usize].as_str() {
                    "function.name" => name = Some(c.node.utf8_text(&buffer).unwrap().to_owned()),
                    "function.return_type" => {
                        return_type = Some(c.node.utf8_text(&buffer).unwrap().to_owned())
                    }
                    "function.parameters" => {
                        params = self.parse_method_args(c.node, buffer);
                    }
                    _ => {}
                }
            }
            v.push(UmlMethod {
                name: name.as_ref().unwrap().to_string(),
                visibility: UmlVisibility::Public,
                parameters: params,
                return_type: return_type,
            });
        }
        v
    }

    fn parse_associations(&self, node: Node, buffer: &Vec<u8>) -> Vec<UmlAssociation> {
        let mut v = vec![];
        for m in QueryCursor::new().matches(&self.class_associations_query, node, |x| {
            x.utf8_text(buffer).unwrap()
        }) {
            let mut to: Option<String> = None;
            let mut to_title: Option<String> = None;
            for c in m.captures.iter() {
                match self.class_associations_query.capture_names()[c.index as usize].as_str() {
                    "class.association.to" => {
                        to = Some(c.node.utf8_text(buffer).unwrap().to_owned())
                    }
                    "class.association.to_title" => {
                        to_title = Some(c.node.utf8_text(buffer).unwrap().to_owned())
                    }
                    _ => panic!("{}", c.node.utf8_text(buffer).unwrap().to_owned()),
                }
            }
            v.push(UmlAssociation {
                from_title: to_title.as_ref().unwrap().to_string(),
                to: to.as_ref().unwrap().to_string(),
                to_title: to_title.as_ref().unwrap().to_string(),
            });
        }
        v
    }
    fn parse_class_fields(&self, node: Node, buffer: &Vec<u8>) -> Vec<UmlField> {
        let mut v = vec![];
        for m in QueryCursor::new().matches(&self.class_fields_query, node, |x| {
            x.utf8_text(buffer).unwrap()
        }) {
            let mut data_type: Option<String> = None;
            let mut name: Option<String> = None;
            for c in m.captures.iter() {
                match self.class_fields_query.capture_names()[c.index as usize].as_str() {
                    "class.field.name" => name = Some(c.node.utf8_text(buffer).unwrap().to_owned()),
                    "class.field.type" => {
                        data_type = Some(c.node.utf8_text(buffer).unwrap().to_owned())
                    }
                    _ => {} // _ => panic!("{}", c.node.utf8_text(buffer).unwrap().to_owned()),
                }
            }
            v.push(UmlField {
                name: name.as_ref().unwrap().to_string(),
                data_type: data_type.as_ref().unwrap().to_string(),
                visibility: UmlVisibility::Public,
            });
        }
        v
    }
    fn parse_extensions(&self, node: Node, buffer: &Vec<u8>) -> Vec<String> {
        let mut v = vec![];
        let query = &Query::new(
            self.ts_parser.language().unwrap(),
            "(base_class_clause (type_identifier) @class.extends)",
        )
        .unwrap();
        for m in QueryCursor::new().matches(query, node, |x| x.utf8_text(buffer).unwrap()) {
            for c in m.captures.iter() {
                match query.capture_names()[c.index as usize].as_str() {
                    "class.extends" => v.push(c.node.utf8_text(buffer).unwrap().to_owned()),
                    _ => {} // panic!("{}", c.node.utf8_text(buffer).unwrap().to_owned()),
                }
            }
        }
        v
    }
    pub fn parse_classes(&self, node: Node, buffer: &Vec<u8>) -> Vec<UmlClass> {
        // struct + impl => class
        // classes
        let mut classes: Vec<UmlClass> = vec![];

        for m in
            QueryCursor::new().matches(&self.class_query, node, |x| x.utf8_text(buffer).unwrap())
        {
            let mut name: Option<String> = None;
            let mut fields: Vec<UmlField> = vec![];
            let mut methods: Vec<UmlMethod> = vec![];
            let mut associations: Vec<UmlAssociation> = vec![];
            let mut extends: Vec<String> = vec![];
            let mut implements: Vec<String> = vec![];
            for c in m.captures.iter() {
                match self.class_query.capture_names()[c.index as usize].as_str() {
                    "class" => extends = self.parse_extensions(c.node, buffer),
                    "class.fields" => {
                        // TODO get visibility here
                        fields = self.parse_class_fields(c.node, buffer);
                        methods = self.parse_methods(c.node, buffer);
                        associations = self.parse_associations(c.node, buffer);
                    }
                    "class.name" => name = Some(c.node.utf8_text(buffer).unwrap().to_owned()),
                    _ => {}
                }
            }
            classes.push(UmlClass {
                name: name.expect("No class name found"),
                fields: fields,
                methods: methods,
                modifier: None,
                visibility: UmlVisibility::Public,
                extends,
                implements,
                associations,
            })
        }

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
    fn test_cpp_parse_methods() {
        let mut p = CppParser::new();
        let source_code = "
class Device
{

    public:
        Device();
        static Display display;
        char host_ip[15];
        string ip_to_number(char a, char b, char c);
}
";
        let tree = p.ts_parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();

        let methods = p.parse_methods(root_node, &source_code.as_bytes().to_vec());
        println!("{}", methods.get(0).unwrap().name);
        println!("{}", methods.get(1).unwrap().name);
        assert!(methods.get(0).unwrap().name == "Device");
        assert!(methods.get(1).unwrap().name == "ip_to_number");
        // assert!(methods.get(0).unwrap().return_type.unwrap() == "i32");
        // assert!(matches!(
        //     methods.get(0).unwrap().visibility,
        //     UmlVisibility::Public
        // ));
    }

    #[test]
    fn test_cpp_parse_method_args() {
        let mut p = CppParser::new();
        let source_code = "
class Device
{

    public:
        Device();
        static Display display;
        char host_ip[15];
        string ip_to_number(char a, char b, char c);
}
";
        let tree = p.ts_parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();

        let args = p.parse_method_args(root_node, &source_code.as_bytes().to_vec());
        assert!(args.get(0).unwrap().name == "a");
        assert!(args.get(0).unwrap().data_type == "char");
        assert!(args.get(1).unwrap().name == "b");
        assert!(args.get(1).unwrap().data_type == "char");
    }

    #[test]
    fn test_cpp_parse_classes() {
        let mut p = CppParser::new();
        let source_code = "
class Device: X
{

    public:
        Device();
        static Display display;
        char host_ip[15];
        string ip_to_number(char a, char b, char c);
}
";
        let tree = p.ts_parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();

        let classes = p.parse_classes(root_node, &source_code.as_bytes().to_vec());

        assert!(classes.get(0).unwrap().name == "Device");
        assert!(classes.get(0).unwrap().extends.get(0).unwrap() == "X");

        assert!(classes.get(0).unwrap().associations.get(0).unwrap().to == "Display");
        assert!(
            classes
                .get(0)
                .unwrap()
                .associations
                .get(0)
                .unwrap()
                .to_title
                == "display"
        );

        println!("{}", classes.get(0).unwrap().methods.get(1).unwrap().name);
        assert!(classes.get(0).unwrap().methods.get(0).unwrap().name == "Device");
        assert!(classes.get(0).unwrap().methods.get(1).unwrap().name == "ip_to_number");
    }
}
