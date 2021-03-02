#[derive(Debug)]
pub enum UmlTopEntity {
    Class(UmlClass),
    Enum(UmlEnum),
}
#[derive(Debug)]
pub enum UmlVisibility {
    Private,
    Public,
}
#[derive(Debug)]
pub enum UmlClassModifier {
    Abstract,
    Interface,
}
#[derive(Debug)]
pub struct UmlEnum {
    name: String,
    variants: Vec<String>,
}
#[derive(Debug)]
pub struct UmlParameter {
    pub data_type: String,
    pub name: String,
}
#[derive(Debug)]
pub struct UmlAssociation {
    pub to: String,
    pub from_title: String,
    pub to_title: String,
}
#[derive(Debug)]
pub struct UmlMethod {
    pub name: String,
    pub visibility: UmlVisibility,
    pub parameters: Vec<UmlParameter>,
    pub return_type: Option<String>,
}
#[derive(Debug)]
pub struct UmlField {
    pub name: String,
    pub data_type: String,
    pub visibility: UmlVisibility,
}
#[derive(Debug)]
pub struct UmlInterface {
    pub name: String,
    pub modifier: Option<UmlClassModifier>,
    pub visibility: UmlVisibility,
    pub methods: Vec<UmlMethod>,
}
#[derive(Debug)]
pub struct UmlClass {
    pub name: String,
    pub modifier: Option<UmlClassModifier>,
    pub visibility: UmlVisibility,
    pub methods: Vec<UmlMethod>,
    pub fields: Vec<UmlField>,
    pub extends: Vec<String>,
    pub implements: Vec<String>,
    pub associations: Vec<UmlAssociation>,
}
#[derive(Debug)]
pub struct UmlStruct {
    name: String,
    visibility: UmlVisibility,
}
