use crate::parser::LangParser;
use crate::uml::*;
use brotli2::read::BrotliEncoder;

pub fn plantuml_encode(s: String) -> String {
    let encoded_string = String::new();
    encoded_string
}

// impl<T: PlantUml> PlantUml for Vec<T> {
//     fn to_plantuml(&self) -> String {
//         let vec: Vec<String> = self.iter().map(|x| x.to_plantuml()).collect();
//         vec.join("\n")
//     }
// }
impl<T: LangParser> PlantUml for T {
    fn to_plantuml(&self) -> String {
        format!(
            "@startuml
{}
@enduml
",
            self.classes().to_plantuml()
        )
    }
}
impl PlantUml for Vec<UmlMethod> {
    fn to_plantuml(&self) -> String {
        let vec: Vec<String> = self.iter().map(|x| x.to_plantuml()).collect();
        vec.join("\n")
    }
}
impl PlantUml for Vec<UmlField> {
    fn to_plantuml(&self) -> String {
        let vec: Vec<String> = self.iter().map(|x| x.to_plantuml()).collect();
        vec.join("\n")
    }
}
impl PlantUml for Vec<UmlClass> {
    fn to_plantuml(&self) -> String {
        let vec: Vec<String> = self.iter().map(|x| x.to_plantuml()).collect();
        vec.join("\n")
    }
}
impl PlantUml for Vec<UmlParameter> {
    fn to_plantuml(&self) -> String {
        let vec: Vec<String> = self.iter().map(|x| x.to_plantuml()).collect();
        vec.join(", ")
    }
}
impl PlantUml for UmlParameter {
    fn to_plantuml(&self) -> String {
        format!("{name} : {type}",name = self.name,type = self.data_type)
    }
}
impl PlantUml for UmlVisibility {
    fn to_plantuml(&self) -> String {
        match self {
            UmlVisibility::Public => "+".to_owned(),
            UmlVisibility::Private => "-".to_owned(),
        }
    }
}
impl PlantUml for UmlField {
    fn to_plantuml(&self) -> String {
        format!(
            "{visibility}{name} : {type}",
            visibility = self.visibility.to_plantuml(),
            name = self.name,
            type = self.data_type,
        )
    }
}
impl PlantUml for UmlMethod {
    fn to_plantuml(&self) -> String {
        match &self.return_type {
            Some(return_type) => format!(
                "{visibility}{name}({parameters}): {return_type}",
                visibility = self.visibility.to_plantuml(),
                name = self.name,
                parameters = self.parameters.to_plantuml(),
                return_type = return_type
            ),
            None => format!(
                "{visibility}{name}({parameters})",
                visibility = self.visibility.to_plantuml(),
                name = self.name,
                parameters = self.parameters.to_plantuml()
            ),
        }
    }
}
impl PlantUml for UmlInterface {
    fn to_plantuml(&self) -> String {
        return format!(
            "class {name} {{
{methods}
}}",
            name = self.name,
            methods = self.methods.to_plantuml()
        );
    }
}
impl PlantUml for UmlClass {
    fn to_plantuml(&self) -> String {
        let assocs: Vec<String> = self
            .associations
            .iter()
            .map(|x| format!("{} --> {} : {}", self.name, x.to, x.to_title))
            .collect();
        let extends: Vec<String> = self
            .extends
            .iter()
            .map(|x| format!("{} --|> {} ", self.name, x))
            .collect();
        return format!(
            "
class {name} {{
{fields}
{methods}
}}
{assocs}
{extends}
",
            name = self.name,
            fields = self.fields.to_plantuml(),
            methods = self.methods.to_plantuml(),
            assocs = assocs.join("\n"),
            extends = extends.join("\n"),
        );
    }
}
pub trait PlantUml {
    fn to_plantuml(&self) -> String;
    // fn render_dependencies(&self, source: Vec<String>) -> String;
}
