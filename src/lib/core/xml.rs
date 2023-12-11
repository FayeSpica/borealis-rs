use crate::lib::core::base_view::{AutoAttributeHandler, BoolAttributeHandler, ColorAttributeHandler, FilePathAttributeHandler, FloatAttributeHandler, StringAttributeHandler, BaseView};

// pub trait XmlReader {
//     /**
//      * Creates a view from the given XML file content.
//      *
//      * The method handleXMLElement() is executed for each child node in the XML.
//      *
//      * Uses the internal lookup table to instantiate the views.
//      * Use registerXMLView() to add your own views to the table so that
//      * you can use them in your own XML files.
//      */
//     fn create_from_xml_string(xml: &str) -> Option<Box<View>> {
//         // Create an XML parser from the XML string
//         let parser_config = xml::ParserConfig::new().trim_whitespace(true);
//         let parser = xml::EventReader::new_with_config(xml.as_bytes(), parser_config);
//
//         // Process each XML event
//         for event in parser {
//             match event {
//                 Ok(xml::reader::XmlEvent::StartElement { name, attributes, .. }) => {
//                     // Handle the start element event
//                     println!("Start Element: {:?}", name);
//
//                     // Print attributes, if any
//                     for attr in attributes {
//                         println!("  Attribute: {}={}", attr.name, attr.value);
//                     }
//                 }
//                 Ok(xml::reader::XmlEvent::EndElement { name, .. }) => {
//                     // Handle the end element event
//                     println!("End Element: {:?}", name);
//                 }
//                 Ok(xml::reader::XmlEvent::Characters(chars)) => {
//                     // Handle the characters event (text content)
//                     println!("Text Content: {}", chars);
//                 }
//                 Err(e) => {
//                     // Handle parsing errors
//                     eprintln!("Error: {:?}", e);
//                 }
//                 _ => {}
//             }
//         }
//
//         let view = View::create_from_xml_element(root);
//         view.bind_xml_document(document);
//         Some(Box::new(view))
//     }
//
//     /**
//      * Creates a view from the given XML element (node and attributes).
//      *
//      * The method handleXMLElement() is executed for each child node in the XML.
//      *
//      * Uses the internal lookup table to instantiate the views.
//      * Use registerXMLView() to add your own views to the table so that
//      * you can use them in your own XML files.
//      */
//     fn create_from_xml_element(element: &tinyxml2::XMLElement) -> Option<Box<View>> {
//         // Placeholder; replace with your implementation
//         None
//     }
//
//     /**
//      * Creates a view from the given XML file path.
//      *
//      * The method handleXMLElement() is executed for each child node in the XML.
//      *
//      * Uses the internal lookup table to instantiate the views.
//      * Use registerXMLView() to add your own views to the table so that
//      * you can use them in your own XML files.
//      */
//     fn create_from_xml_file(path: &str) -> Option<Box<View>> {
//         // Open the file
//         let file = std::fs::File::open(path).unwrap();
//
//         // Create an XML parser from the file
//         let parser_config = xml::ParserConfig::new().trim_whitespace(true);
//         let parser = xml::EventReader::new_with_config(file, parser_config);
//
//         // Process each XML event
//         for event in parser {
//             match event {
//                 Ok(xml::reader::XmlEvent::StartElement { name, attributes, .. }) => {
//                     // Handle the start element event
//                     println!("Start Element: {:?}", name);
//
//                     // Print attributes, if any
//                     for attr in attributes {
//                         println!("  Attribute: {}={}", attr.name, attr.value);
//                     }
//                 }
//                 Ok(xml::reader::XmlEvent::EndElement { name, .. }) => {
//                     // Handle the end element event
//                     println!("End Element: {:?}", name);
//                 }
//                 Ok(xml::reader::XmlEvent::Characters(chars)) => {
//                     // Handle the characters event (text content)
//                     println!("Text Content: {}", chars);
//                 }
//                 Err(e) => {
//                     // Handle parsing errors
//                     eprintln!("Error: {:?}", e);
//                 }
//                 _ => {}
//             }
//         }
//     }
//
//     /**
//      * Creates a view from the given XML resource file name.
//      *
//      * The method handleXMLElement() is executed for each child node in the XML.
//      *
//      * Uses the internal lookup table to instantiate the views.
//      * Use registerXMLView() to add your own views to the table so that
//      * you can use them in your own XML files.
//      */
//     fn create_from_xml_resource(name: &str) -> Option<Box<View>> {
//         // Placeholder; replace with your implementation
//         None
//     }
//
//     /**
//      * Handles a child XML element.
//      *
//      * You can redefine this method to handle child XML like
//      * as you want in your own views.
//      *
//      * If left unimplemented, will throw an exception because raw
//      * views cannot handle child XML elements (Boxes can).
//      */
//     fn handle_xml_element(&self, element: &tinyxml2::XMLElement) {
//         // Placeholder; replace with your implementation
//     }
//
//     /**
//      * Applies the attributes of the given XML element to the view.
//      *
//      * You can add your own attributes to by calling registerXMLAttribute()
//      * in the view constructor.
//      */
//     fn apply_xml_attributes(&self, element: &tinyxml2::XMLElement) {
//         // Placeholder; replace with your implementation
//     }
//
//     /**
//      * Applies the given attribute to the view.
//      *
//      * You can add your own attributes to by calling registerXMLAttribute()
//      * in the view constructor.
//      */
//     fn apply_xml_attribute(&self, name: &str, value: &str) -> bool {
//         // Placeholder; replace with your implementation
//         false
//     }
//
//     /**
//      * Register a new XML attribute with the given name and handler
//      * method. You can have multiple attributes registered with the same
//      * name but different types / handlers, except if the type is string.
//      *
//      * The method will be called if the attribute has the value "auto".
//      */
//     fn register_auto_xml_attribute(&self, name: &str, handler: AutoAttributeHandler) {
//         // Placeholder; replace with your implementation
//     }
//
//     /**
//      * Register a new XML attribute with the given name and handler
//      * method. You can have multiple attributes registered with the same
//      * name but different types / handlers, except if the type is string.
//      *
//      * The method will be called if the attribute has a percentage value (an integer with "%" suffix).
//      * The given float value is guaranteed to be between 0.0f and 1.0f.
//      */
//     fn register_percentage_xml_attribute(&self, name: &str, handler: FloatAttributeHandler) {
//         // Placeholder; replace with your implementation
//     }
//
//     /**
//      * Register a new XML attribute with the given name and handler
//      * method. You can have multiple attributes registered with the same
//      * name but different types / handlers, except if the type is string.
//      *
//      * The method will be called if the attribute has an integer, float, @style or "px" value.
//      */
//     fn register_float_xml_attribute(&self, name: &str, handler: FloatAttributeHandler) {
//         // Placeholder; replace with your implementation
//     }
//
//     /**
//      * Register a new XML attribute with the given name and handler
//      * method. You can have multiple attributes registered with the same
//      * name but different types / handlers, except if the type is string.
//      *
//      * The method will be called if the attribute has a string or @i18n value.
//      *
//      * If you use string as a type, you can only have one handler for the attribute.
//      */
//     fn register_string_xml_attribute(&self, name: &str, handler: StringAttributeHandler) {
//         // Placeholder; replace with your implementation
//     }
//
//     /**
//      * Register a new XML attribute with the given name and handler
//      * method. You can have multiple attributes registered with the same
//      * name but different types / handlers, except if the type is string.
//      *
//      * The method will be called if the attribute has a color value ("#XXXXXX" or "#XXXXXXXX")
//      * or a @theme value.
//      */
//     fn register_color_xml_attribute(&self, name: &str, handler: ColorAttributeHandler) {
//         // Placeholder; replace with your implementation
//     }
//
//     /**
//      * Register a new XML attribute with the given name and handler
//      * method. You can have multiple attributes registered with the same
//      * name but different types / handlers, except if the type is string.
//      *
//      * The method will be called if the attribute has a boolean value ("true" or "false").
//      */
//     fn register_bool_xml_attribute(&self, name: &str, handler: BoolAttributeHandler) {
//         // Placeholder; replace with your implementation
//     }
//
//     /**
//      * Register a new XML attribute with the given name and handler
//      * method. You can have multiple attributes registered with the same
//      * name but different types / handlers, except if the type is string.
//      *
//      * The method will be called if the attribute has a file path value ("@res/" or raw path).
//      */
//     fn register_file_path_xml_attribute(&self, name: &str, handler: FilePathAttributeHandler) {
//         // Placeholder; replace with your implementation
//     }
//
//     /**
//      * Binds the given XML document to the view for ownership. The
//      * document will then be deleted when the view is.
//      */
//     fn bind_xml_document(&mut self, document: Box<tinyxml2::XMLDocument>) {
//         self.bound_documents.push(document);
//     }
//
//     /**
//      * Returns if the given XML attribute name is valid for that view.
//      */
//     fn is_xml_attribute_valid(&self, attribute_name: &str) -> bool;
//
//     /**
//      * Sets the maximum number of allowed children XML elements
//      * when using a view of that class in an XML file.
//      */
//     fn set_maximum_allowed_xml_elements(&mut self, max: u32);
//
//     // Gets the maximum number of allowed children XML elements
//     fn get_maximum_allowed_xml_elements(&self) -> u32;
// }