// use crate::uml::*;

// impl<T: LangParser> PlantUml for T {
//     fn to_plantuml(&self) -> String {
//         let mut x = 100;
//         let mut y = 100;
//         let mut w = 100;
//         let mut h = 100;

//         let mut classes: Vec<String> = vec![];
//         for class in self.classes() {
//             classes.push(format!(
//                 "
// {}
// <element>
//     <id>Relation</id>
//     <coordinates>
//       <x>850</x>
//       <y>952</y>
//       <w>408</w>
//       <h>51</h>
//     </coordinates>
//     <panel_attributes>lt=&lt;&lt;-</panel_attributes>
//     <additional_attributes>220.0;10.0;10.0;10.0</additional_attributes>
//   </element>

// "
//             ))
//         }
//         format!(
//             "

// "
//         )
//     }
// }
// impl Umlet for UmlClass {
//     fn to_umlet(&self) -> String {
//         format!(
//             "
//   <element>
//     <id>UMLClass</id>
//     <coordinates>
//       <x>680</x>
//       <y>860</y>
//       <w>100</w>
//       <h>30</h>
//     </coordinates>
//     <panel_attributes>SimpleClass</panel_attributes>
//     <additional_attributes/>
//   </element>
// "
//         )
//     }
// }
// pub trait Umlet {
//     fn to_umlet(&self) -> String;
// }
