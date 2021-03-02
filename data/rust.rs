use std::collections::HashSet;
use std::iter::FromIterator;

use super::PlantUml;

enum Day {
    Monday,
    Tuesday,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    pub entity_type: EntityType,
    pub name: String,
    pub fields: Vec<Entity>,
}

struct Class2 {
    pub field_1: String,
    pub field_2: i32,
}

impl Class2 {
    pub fn func1(arg1: i32) -> Self {
        Class2
    }
    pub fn func2() {}
}
impl Entity {
    pub fn new(entity_type: EntityType, name: &str, fields: Vec<Entity>) -> Self {
        Entity {
            entity_type,
            name: name.to_string(),
            fields,
        }
    }
}

impl PlantUml for Entity {
    fn render(&self) -> String {
        let prefix = match self.entity_type {
            EntityType::Struct => format!("class \"{}\" {{\n", self.name),
            EntityType::Enum => format!("enum \"{}\" {{\n", self.name),
            EntityType::Field(ref name) => format!("    + {}: {}\n", name, self.name),
        };

        let body: Vec<String> = self
            .fields
            .clone()
            .into_iter()
            .map(|field| match field.entity_type {
                EntityType::Field(_) => field.render(),
                _ => "".to_string(),
            })
            .collect();
        let body = body.join("");
        format!("{}{}", prefix, body)
    }

    fn render_dependencies(&self, source: Vec<String>) -> String {
        let mut c: Vec<String> = Vec::new();
        for f in self.fields.clone() {
            for a in f.fields.clone() {
                c.push(a.render_dependencies(source.clone()))
            }

            let source_set: HashSet<String> = HashSet::from_iter(source.clone());
            let ent = make_dependencies(&f.name);

            let cnt = source_set.intersection(&ent).collect::<HashSet<_>>().len();

            if cnt >= 1 {
                c.push(format!("\"{}\" <-- \"{}\"\n", self.name, f.name))
            }
        }
        c.join("")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EntityType {
    Struct,
    Enum,
    Field(String),
}

fn make_dependencies(type_name: &str) -> HashSet<String> {
    let dependencies: Vec<&str> = type_name
        .split(|x| (x == ',') || (x == '<') || (x == '>'))
        .collect();
    let dependencies = dependencies
        .into_iter()
        .map(|x| x.to_string())
        .filter(|x| x != "")
        .map(|x| x.replace(" ", ""))
        .collect::<Vec<String>>();
    HashSet::from_iter(dependencies)
}

#[test]
fn test_make_dependencies() {
    let type_name = "String";
    let expected: HashSet<String> = HashSet::from_iter(vec!["String".to_string()]);
    assert_eq!(make_dependencies(type_name), expected);

    let type_name = "HashSet<String>";
    let expected: HashSet<String> =
        HashSet::from_iter(vec!["HashSet".to_string(), "String".to_string()]);
    assert_eq!(make_dependencies(type_name), expected);

    let type_name = "HashMap<Id, String>";
    let expected: HashSet<String> = HashSet::from_iter(vec![
        "HashMap".to_string(),
        "Id".to_string(),
        "String".to_string(),
    ]);
    assert_eq!(make_dependencies(type_name), expected);
}
