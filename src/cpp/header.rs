use indoc::formatdoc;
use crate::DefaultType;
use super::*;

impl DataModel {
    /// Generates the header file
    /// 
    /// # Parameters
    /// 
    /// name: The name of the header file (used for header guard so should be capslocked)
    /// 
    /// indent: The number of spaces to use for indentation
    pub fn gen_header(&self, name: &str, indent: usize) -> String {
        // Get header and footer
        let header = if self.headers.header.is_empty() {
            "".to_string()
        } else {
            format!("{}\n\n", self.headers.header)
        };
        let footer = if self.footers.header.is_empty() {
            "".to_string()
        } else {
            format!("{}\n\n", self.footers.header)
        };

        // Get all structs
        let data_types = self.data_types.iter()
            .map(|data_type| format!("{}\n\n", data_type.gen_header(indent)))
            .collect::<Vec<String>>()
            .join("");

        return formatdoc!("
            // Generated with the Termite Data Model Generator
            #ifndef {name}_TERMITE_H_INCLUDED
            #define {name}_TERMITE_H_INCLUDED

            #include <optional>
            #include <variant>

            {header}{data_types}{footer}#endif
        ");
    }
}

impl DataType {
    /// Generates the description if it is supplied
    fn gen_description(&self) -> String {
        return match &self.description {
            Some(description) => formatdoc!("
                /**
                 * \\brief {description}
                 * 
                 */
            "),
            None => "".to_string(),
        };
    }

    /// Converts the data type to a string for use in the header file
    /// 
    /// # Parameters
    /// 
    /// indent: The number of spaces to use for indentation
    fn gen_header(&self, indent: usize) -> String {
        let description = self.gen_description();
        let definition = self.data.gen_header(&self.name, indent);

        return format!("{description}{definition}");
    }
}

impl DataTypeData {
    /// Converts the data type data to a string for use in the header file
    /// 
    /// # Parameters
    /// 
    /// name: The name of the data type
    /// 
    /// indent: The number of spaces to use for indentation
    fn gen_header(&self, name: &str, indent: usize) -> String {
        return match self {
            DataTypeData::Struct(data) => data.gen_header(name, indent),
        };
    }
}

impl Struct {
    /// Converts the struct to a string for use in the header file
    /// 
    /// # Parameters
    /// 
    /// name: The name of the struct
    /// 
    /// indent: The number of spaces to use for indentation
    fn gen_header(&self, name: &str, indent: usize) -> String {
        // Get the definitions of all the fields but without any initialization
        let field_definitions = self.fields.iter()
            .map(|field| format!(
                "{description}{0:indent$}{definition};\n",
                "",
                description = field.get_description(indent),
                definition = field.get_definition(),
            ))
            .collect::<Vec<String>>()
            .join("");
        let field_definitions = if field_definitions.is_empty() {
            "".to_string()
        } else {
            format!("\n{field_definitions}")
        };

        let internal_parameters = self.fields.iter()
            .map(|field| format!(
                "{definition}_",
                definition = field.get_definition(),
            ))
            .collect::<Vec<String>>()
            .join(", ");

        let internal_setters = self.fields.iter()
            .map(|field| format!(
                "{name}({name}_)",
                name = field.name,
            ))
            .collect::<Vec<String>>()
            .join(", ");
        let internal_setters = if internal_setters.is_empty() {
            "".to_string()
        } else {
            format!(" : {internal_setters}")
        };

        // Generate the code
        return formatdoc!("
            class {name} {{
            public:
            private:
            {0:indent$}explicit {name}({internal_parameters}){internal_setters} {{}}
            {field_definitions}}};", ""
        );
    }
}

impl StructField {
    /// Constructs the c++ typename of this field
    fn get_typename(&self) -> String {
        return match &self.default {
            DefaultType::Optional => format!(
                "std::optional<{data_type}>",
                data_type = self.data_type,
            ),
            _ => self.data_type.clone(),
        };
    }

    /// Gets the definition of the field
    fn get_definition(&self) -> String {
        return format!("{typename} {name}", typename = self.get_typename(), name = self.name);
    }

    /// Gets the description if it is supplied
    /// 
    /// # Parameters
    /// 
    /// indent: The number of spaces to use for indentation
    fn get_description(&self, indent: usize) -> String {
        return match &self.description {
            Some(description) => formatdoc!("
                {0:indent$}/**
                {0:indent$} * \\brief {description}
                {0:indent$} * 
                {0:indent$} */
                ", ""),
            None => "".to_string(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header() {
        // Create the data model
        let data_model = DataModel {
            headers: Headers { header: "header_data".to_string(), source: "".to_string() },
            footers: Footers { header: "".to_string(), source: "".to_string() },
            data_types: vec![],
        };
        
        // Create the header file
        let header_file = data_model.gen_header("HEADER", 2);

        let expected = formatdoc!("
            // Generated with the Termite Data Model Generator
            #ifndef HEADER_TERMITE_H_INCLUDED
            #define HEADER_TERMITE_H_INCLUDED
            
            #include <optional>
            #include <variant>

            header_data

            #endif
        ");

        assert_eq!(header_file, expected);
    }

    #[test]
    fn footer() {
        // Create the data model
        let data_model = DataModel {
            headers: Headers { header: "".to_string(), source: "".to_string() },
            footers: Footers { header: "footer_data".to_string(), source: "".to_string() },
            data_types: vec![],
        };
        
        // Create the header file
        let header_file = data_model.gen_header("HEADER", 2);

        let expected = formatdoc!("
            // Generated with the Termite Data Model Generator
            #ifndef HEADER_TERMITE_H_INCLUDED
            #define HEADER_TERMITE_H_INCLUDED
            
            #include <optional>
            #include <variant>

            footer_data

            #endif
        ");

        assert_eq!(header_file, expected);
    }

    mod type_struct {
        use super::*;

        #[test]
        fn basic() {
            // Create the data model
            let data_model = DataModel {
                headers: Headers { header: "".to_string(), source: "".to_string() },
                footers: Footers { header: "".to_string(), source: "".to_string() },
                data_types: vec![
                    DataType {
                        name: "DataType1".to_string(),
                        description: None,
                        data: DataTypeData::Struct(Struct { fields: vec![] }),
                    },
                    DataType {
                        name: "DataType2".to_string(),
                        description: None,
                        data: DataTypeData::Struct(Struct { fields: vec![] }),
                    },
                ],
            };
            
            // Create the header file
            let header_file = data_model.gen_header("HEADER", 2);

            let expected = formatdoc!("
                // Generated with the Termite Data Model Generator
                #ifndef HEADER_TERMITE_H_INCLUDED
                #define HEADER_TERMITE_H_INCLUDED
                
                #include <optional>
                #include <variant>

                class DataType1 {{
                public:
                private:
                  explicit DataType1() {{}}
                }};

                class DataType2 {{
                public:
                private:
                  explicit DataType2() {{}}
                }};

                #endif
            ");

            assert_eq!(header_file, expected);
        }

        #[test]
        fn description() {
            // Create the data model
            let data_model = DataModel {
                headers: Headers { header: "".to_string(), source: "".to_string() },
                footers: Footers { header: "".to_string(), source: "".to_string() },
                data_types: vec![
                    DataType {
                        name: "DataType1".to_string(),
                        description: Some("description1".to_string()),
                        data: DataTypeData::Struct(Struct { fields: vec![] }),
                    },
                    DataType {
                        name: "DataType2".to_string(),
                        description: Some("description2".to_string()),
                        data: DataTypeData::Struct(Struct { fields: vec![] }),
                    },
                ],
            };
            
            // Create the header file
            let header_file = data_model.gen_header("HEADER", 2);

            let expected = formatdoc!("
                // Generated with the Termite Data Model Generator
                #ifndef HEADER_TERMITE_H_INCLUDED
                #define HEADER_TERMITE_H_INCLUDED
                
                #include <optional>
                #include <variant>

                /**
                 * \\brief description1
                 * 
                 */
                class DataType1 {{
                public:
                private:
                  explicit DataType1() {{}}
                }};

                /**
                 * \\brief description2
                 * 
                 */
                class DataType2 {{
                public:
                private:
                  explicit DataType2() {{}}
                }};

                #endif
            ");

            assert_eq!(header_file, expected);
        }

        mod field {
            use super::*;

            //#[test]
            fn basic() {
                // Create the data model
                let data_model = DataModel {
                    headers: Headers { header: "".to_string(), source: "".to_string() },
                    footers: Footers { header: "".to_string(), source: "".to_string() },
                    data_types: vec![
                        DataType {
                            name: "DataType".to_string(),
                            description: None,
                            data: DataTypeData::Struct(Struct {
                                fields: vec![
                                    StructField {
                                        name: "field1".to_string(),
                                        description: None,
                                        data_type: "type1".to_string(),
                                        default: DefaultType::Required,
                                        constraints: vec![],
                                    },
                                    StructField {
                                        name: "field2".to_string(),
                                        description: None,
                                        data_type: "type2".to_string(),
                                        default: DefaultType::Required,
                                        constraints: vec![],
                                    },
                                ] 
                            }),
                        },
                    ],
                };
                
                // Create the header file
                let header_file = data_model.gen_header("HEADER", 2);

                let expected = formatdoc!("
                    // Generated with the Termite Data Model Generator
                    #ifndef HEADER_TERMITE_H_INCLUDED
                    #define HEADER_TERMITE_H_INCLUDED
                    
                    #include <optional>
                    #include <variant>
                    #include <termite>

                    class DataType {{
                    public:
                      /**
                       * \\brief Constructs a new DataType object 
                       * 
                       */
                      static termite::Result<DataType> new(type1 field1, type2 field2) {{
                        if (auto result = validate_field1(field1)) {{
                          return 
                        }}
                      }}

                    private:
                      explicit DataType(type1 field1_, type2 field2_) : field1(field1_), field2(field2_) {{}}

                      /**
                       * \\brief Validates if field1 is correct using the following constaints:
                       * 
                       */
                      static termite::Result<std::tuple<>> validate_field1(type1 &value) {{
                        return termite::Result<std::tuple<>>::ok({{}});
                      }}
                      /**
                       * \\brief Validates if field2 is correct using the following constaints:
                       * 
                       */
                      static termite::Result<std::tuple<>> validate_field2(type2 &value) {{
                        return termite::Result<std::tuple<>>::ok({{}});
                      }}

                      type1 field1;
                      type2 field2;
                    }};

                    #endif
                ");

                assert_eq!(header_file, expected);
            }

            #[test]
            fn description() {
                // Create the data model
                let data_model = DataModel {
                    headers: Headers { header: "".to_string(), source: "".to_string() },
                    footers: Footers { header: "".to_string(), source: "".to_string() },
                    data_types: vec![
                        DataType {
                            name: "DataType".to_string(),
                            description: None,
                            data: DataTypeData::Struct(Struct {
                                fields: vec![
                                    StructField {
                                        name: "field1".to_string(),
                                        description: Some("description1".to_string()),
                                        data_type: "type1".to_string(),
                                        default: DefaultType::Required,
                                        constraints: vec![],
                                    },
                                    StructField {
                                        name: "field2".to_string(),
                                        description: Some("description2".to_string()),
                                        data_type: "type2".to_string(),
                                        default: DefaultType::Default("default".to_string()),
                                        constraints: vec![],
                                    },
                                ] 
                            }),
                        },
                    ],
                };
                
                // Create the header file
                let header_file = data_model.gen_header("HEADER", 2);

                let expected = formatdoc!("
                    // Generated with the Termite Data Model Generator
                    #ifndef HEADER_TERMITE_H_INCLUDED
                    #define HEADER_TERMITE_H_INCLUDED
                    
                    #include <optional>
                    #include <variant>

                    class DataType {{
                    public:
                    private:
                      explicit DataType(type1 field1_, type2 field2_) : field1(field1_), field2(field2_) {{}}

                      /**
                       * \\brief description1
                       * 
                       */
                      type1 field1;
                      /**
                       * \\brief description2
                       * 
                       */
                      type2 field2;
                    }};

                    #endif
                ");

                assert_eq!(header_file, expected);
            }

            #[test]
            fn optional() {
                // Create the data model
                let data_model = DataModel {
                    headers: Headers { header: "".to_string(), source: "".to_string() },
                    footers: Footers { header: "".to_string(), source: "".to_string() },
                    data_types: vec![
                        DataType {
                            name: "DataType".to_string(),
                            description: None,
                            data: DataTypeData::Struct(Struct {
                                fields: vec![
                                    StructField {
                                        name: "field1".to_string(),
                                        description: None,
                                        data_type: "type1".to_string(),
                                        default: DefaultType::Optional,
                                        constraints: vec![],
                                    },
                                ] 
                            }),
                        },
                    ],
                };
                
                // Create the header file
                let header_file = data_model.gen_header("HEADER", 2);

                let expected = formatdoc!("
                    // Generated with the Termite Data Model Generator
                    #ifndef HEADER_TERMITE_H_INCLUDED
                    #define HEADER_TERMITE_H_INCLUDED
                    
                    #include <optional>
                    #include <variant>

                    class DataType {{
                    public:
                    private:
                      explicit DataType(std::optional<type1> field1_) : field1(field1_) {{}}

                      std::optional<type1> field1;
                    }};

                    #endif
                ");

                assert_eq!(header_file, expected);
            }
        }
    }

    #[test]
    fn outline() {
        // Create the data model
        let data_model = DataModel {
            headers: Headers { header: "header_data".to_string(), source: "".to_string() },
            footers: Footers { header: "footer_data".to_string(), source: "".to_string() },
            data_types: vec![
                DataType {
                    name: "DataType1".to_string(),
                    description: Some("description1".to_string()),
                    data: DataTypeData::Struct(Struct { fields: vec![] }),
                },
                DataType {
                    name: "DataType2".to_string(),
                    description: Some("description2".to_string()),
                    data: DataTypeData::Struct(Struct { fields: vec![] }),
                },
            ],
        };
        
        // Create the header file
        let header_file = data_model.gen_header("HEADER", 2);

        let expected = formatdoc!("
            // Generated with the Termite Data Model Generator
            #ifndef HEADER_TERMITE_H_INCLUDED
            #define HEADER_TERMITE_H_INCLUDED
            
            #include <optional>
            #include <variant>

            header_data

            /**
             * \\brief description1
             * 
             */
            class DataType1 {{
            public:
            private:
              explicit DataType1() {{}}
            }};

            /**
             * \\brief description2
             * 
             */
            class DataType2 {{
            public:
            private:
              explicit DataType2() {{}}
            }};

            footer_data

            #endif
        ");

        assert_eq!(header_file, expected);
    }
}
