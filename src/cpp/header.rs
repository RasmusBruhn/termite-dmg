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

            #include <iostream>
            #include <optional>
            #include <variant>
            #include <termite>

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
                "{description}{0:indent$}{definition}_;",
                "",
                description = field.get_description(indent),
                definition = field.get_definition(),
            ))
            .collect::<Vec<String>>()
            .join("\n");

        // Get the validation functions
        let validation_functions = self.fields.iter()
            .map(|field| field.get_validation(indent))
            .collect::<Vec<String>>()
            .join("\n");

        // Get the list of parameters for the internal constructor
        let internal_parameters = self.fields.iter()
            .map(|field| format!(
                "{definition}",
                definition = field.get_definition(),
            ))
            .collect::<Vec<String>>()
            .join(", ");

        // Get the list of setters for the internal constructor
        let internal_setters = self.fields.iter()
            .map(|field| format!(
                "{name}_({name})",
                name = field.name,
            ))
            .collect::<Vec<String>>()
            .join(", ");
        let internal_setters = if internal_setters.is_empty() {
            "".to_string()
        } else {
            format!(" : {internal_setters}")
        };

        // Get the constructor parameters
        let constructor_parameters = self.fields.iter()
            .map(|field| format!(
                "{definition}{default}",
                definition = field.get_definition(),
                default = field.get_default(),
            ))
            .collect::<Vec<String>>()
            .join(", ");

        // Get the constructor validation
        let constructor_validators = self.fields.iter()
            .map(|field| formatdoc!("
                {0:indent$}{0:indent$}validate_result = validate_{field_name}({field_name});
                {0:indent$}{0:indent$}if (!validate_result.is_ok()) {{
                {0:indent$}{0:indent$}{0:indent$}termite::Error error = validate_result.get_err();
                {0:indent$}{0:indent$}{0:indent$}error.add_field(\"{field_name}\");
                {0:indent$}{0:indent$}{0:indent$}return termite::Result<DataType>::err(std::move(error));
                {0:indent$}{0:indent$}}}",
                "",
                field_name = field.name,
            ))
            .collect::<Vec<String>>()
            .join("\n\n");
        let constructor_validators = if constructor_validators.is_empty() {
            "".to_string()
        } else {
            format!("{0:indent$}{0:indent$}termite::Result<std::tuple<>> validate_result = termite::Result<std::tuple<>>::ok({{}});\n\n{constructor_validators}", "")
        };

        // Get the move parameter list for the constructor
        let constructor_move_parameters = self.fields.iter()
            .map(|field| format!("std::move({field_name})", field_name = field.name))
            .collect::<Vec<String>>()
            .join(", ");

        // Get the description for the constructor
        let constructor_description = self.fields.iter()
            .map(|field| format!(
                "\\param {field_name} {description}", 
                field_name = field.name, 
                description = &field.description.as_ref().unwrap_or(&"".to_string())
            ))
            .collect::<Vec<String>>()
            .join(&format!("\n{0:indent$} * ", ""));

        // Get the setter methods
        let setter_methods = self.fields.iter()
            .map(|field| field.get_setter(indent))
            .collect::<Vec<String>>()
            .join("\n");

        // Get the getter methods
        let getter_methods = self.fields.iter()
            .map(|field| field.get_getter(indent))
            .collect::<Vec<String>>()
            .join("\n");

        // Get the equality test
        let equality_test = self.fields.iter()
            .map(|field| format!("{field_name}_ == other.{field_name}_", field_name = field.name))
            .collect::<Vec<String>>()
            .join(" && ");
        let equality_test = if equality_test.is_empty() {
            "true".to_string()
        } else {
            equality_test
        };

        // Get the printout for the operator<< function
        let printout = self.fields.iter()
            .map(|field| format!("\"{field_name}: \" << value.{field_name}_", field_name = field.name))
            .collect::<Vec<String>>()
            .join(" << \", \" << ");
        let printout = if printout.is_empty() {
            "\"\"".to_string()
        } else {
            printout
        };

        // Generate the code
        return formatdoc!("
            class {name} {{
            public:
            {0:indent$}/**
            {0:indent$} * \\brief Constructs a new {name} object 
            {0:indent$} * 
            {0:indent$} * {constructor_description}
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}[[nodiscard]] static termite::Result<{name}> from_values({constructor_parameters}) {{
            {constructor_validators}

            {0:indent$}{0:indent$}return termite::Result<{name}>::ok({name}({constructor_move_parameters}));
            {0:indent$}}}

            {setter_methods}

            {getter_methods}

            {0:indent$}/**
            {0:indent$} * \\brief Checks if this object the the other object are identical
            {0:indent$} * 
            {0:indent$} * \\param other The other object to compare with
            {0:indent$} * \\return true if they are identical, false if not
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}[[nodiscard]] bool operator==(const {name} &other) {{
            {0:indent$}{0:indent$}return {equality_test};
            {0:indent$}}}
            {0:indent$}/**
            {0:indent$} * \\brief Checks if this object the the other object are different
            {0:indent$} * 
            {0:indent$} * \\param other The other object to compare with
            {0:indent$} * \\return true if they are different, false if not
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}[[nodiscard]] bool operator!=(const {name} &other) {{
            {0:indent$}{0:indent$}return !(*this == other);
            {0:indent$}}}
            {0:indent$}/**
            {0:indent$} * \\brief Prints the object onto the output stream
            {0:indent$} * 
            {0:indent$} * \\param os The output stream to print to
            {0:indent$} * \\param value The object to print
            {0:indent$} * \\return The output stream
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}friend std::ostream &operator<<(std::ostream &os, const {name} &value) {{
            {0:indent$}{0:indent$}return os << \"{{ \" << {printout} << \" }}\";
            {0:indent$}}}

            private:
            {0:indent$}explicit {name}({internal_parameters}){internal_setters} {{}}

            {validation_functions}

            {field_definitions}
            }};", ""
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

    /// Gets the default value for this field
    fn get_default(&self) -> String {
        return match &self.default {
            DefaultType::Required => "".to_string(),
            DefaultType::Optional => " = std::nullopt".to_string(),
            DefaultType::Default(value) => format!(" = {value}"),
        }
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

    /// Gets the validation function
    /// 
    /// # Parameters
    /// 
    /// indent: The number of spaces to use for indentation
    fn get_validation(&self, indent: usize) -> String {
        // Create the description
        let description = self.constraints.iter()
            .map(|constraint| {
                return format!("\n{0:indent$} * - {constraint}", "");
            })
            .collect::<Vec<String>>()
            .join("");

        // Create the tests
        let tests = self.constraints.iter()
            .map(|constraint| formatdoc!("
                \n{0:indent$}{0:indent$}if (!({constraint})) {{
                {0:indent$}{0:indent$}{0:indent$}return termite::Result<std::tuple<>>::err(termite::Error(\"{field_name} did not pass constaint: {constraint}\"));
                {0:indent$}{0:indent$}}}
            ", "", field_name = self.name))
            .collect::<Vec<String>>()
            .join("");
        
        return formatdoc!("
            {0:indent$}/**
            {0:indent$} * \\brief Validates if {field_name} is correct using the following constaints:{description}
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}[[nodiscard]] static termite::Result<std::tuple<>> validate_{field_name}(const {type_name} &value) {{{tests}
            {0:indent$}{0:indent$}return termite::Result<std::tuple<>>::ok({{}});
            {0:indent$}}}",
            "", 
            field_name = self.name, 
            type_name = self.data_type
        );
    }

    /// Gets the setter function
    /// 
    /// # Parameters
    /// 
    /// indent: The number of spaces to use for indentation
    fn get_setter(&self, indent: usize) -> String {
        // Create the description
        let description = self.constraints.iter()
            .map(|constraint| {
                return format!("\n{0:indent$} * - {constraint}", "");
            })
            .collect::<Vec<String>>()
            .join("");
        
        return formatdoc!("
            {0:indent$}/**
            {0:indent$} * \\brief Sets the value of {field_name} if it fulfills the constraints:{description}
            {0:indent$} * 
            {0:indent$} * \\param value The value of {field_name}
            {0:indent$} * \\return An error if one of the constraints were not fulfilled
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}[[nodiscard]] termite::Result<std::tuple<>> set_{field_name}({type_name} value) {{
            {0:indent$}{0:indent$}termite::Result<std::tuple<>> validate_result = validate_{field_name}(value);
            {0:indent$}{0:indent$}if (!validate_result.is_ok()) {{
            {0:indent$}{0:indent$}{0:indent$}return termite::Result<DataType>::err(std::move(error));
            {0:indent$}{0:indent$}}}

            {0:indent$}{0:indent$}{field_name}_ = std::move(value);
            {0:indent$}{0:indent$}return termite::Result<std::tuple<>>::ok({{}});
            {0:indent$}}}",
            "", 
            field_name = self.name, 
            type_name = self.data_type
        );
    }

    /// Gets the getter function
    /// 
    /// # Parameters
    /// 
    /// indent: The number of spaces to use for indentation
    fn get_getter(&self, indent: usize) -> String {
        return formatdoc!("
            {0:indent$}/**
            {0:indent$} * \\brief Retrieves a reference to the value of {field_name}
            {0:indent$} * 
            {0:indent$} * \\return The reference 
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}[[nodiscard]] const {type_name} &get_{field_name}() const {{
            {0:indent$}{0:indent$}return {field_name}_;
            {0:indent$}}}",
            "", 
            field_name = self.name, 
            type_name = self.data_type
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_diff(lhs: &str, rhs: &str) -> Option<(usize, String, String)> {
        return lhs.lines()
            .zip(rhs.lines())
            .enumerate()
            .filter_map(|(index, (lhs, rhs))| {
                return if lhs == rhs {
                    None
                } else {
                    Some((index, lhs.to_string(), rhs.to_string()))
                };
            })
            .next();
    }

    #[test]
    fn header() {
        // Create the data model
        let data_model = DataModel {
            headers: Headers { header: "// header data".to_string(), source: "".to_string() },
            footers: Footers { header: "".to_string(), source: "".to_string() },
            data_types: vec![],
        };
        
        // Create the header file
        let header_file = data_model.gen_header("HEADER", 2);

        let expected = formatdoc!("
            // Generated with the Termite Data Model Generator
            #ifndef HEADER_TERMITE_H_INCLUDED
            #define HEADER_TERMITE_H_INCLUDED
            
            #include <iostream>
            #include <optional>
            #include <variant>
            #include <termite>

            // header data

            #endif
        ");

        assert_eq!(str_diff(&header_file, &expected), None);
    }

    #[test]
    fn footer() {
        // Create the data model
        let data_model = DataModel {
            headers: Headers { header: "".to_string(), source: "".to_string() },
            footers: Footers { header: "// footer data".to_string(), source: "".to_string() },
            data_types: vec![],
        };
        
        // Create the header file
        let header_file = data_model.gen_header("HEADER", 2);

        let expected = formatdoc!("
            // Generated with the Termite Data Model Generator
            #ifndef HEADER_TERMITE_H_INCLUDED
            #define HEADER_TERMITE_H_INCLUDED
            
            #include <iostream>
            #include <optional>
            #include <variant>
            #include <termite>

            // footer data

            #endif
        ");

        assert_eq!(str_diff(&header_file, &expected), None);
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
                
                #include <iostream>
                #include <optional>
                #include <variant>
                #include <termite>

                class DataType1 {{
                public:
                  /**
                   * \\brief Constructs a new DataType1 object 
                   * 
                   * 
                   * 
                   */
                  [[nodiscard]] static termite::Result<DataType1> from_values() {{
                
                
                    return termite::Result<DataType1>::ok(DataType1());
                  }}

                



                  /**
                   * \\brief Checks if this object the the other object are identical
                   * 
                   * \\param other The other object to compare with
                   * \\return true if they are identical, false if not
                   * 
                   */
                  [[nodiscard]] bool operator==(const DataType1 &other) {{
                    return true;
                  }}
                  /**
                   * \\brief Checks if this object the the other object are different
                   * 
                   * \\param other The other object to compare with
                   * \\return true if they are different, false if not
                   * 
                   */
                  [[nodiscard]] bool operator!=(const DataType1 &other) {{
                    return !(*this == other);
                  }}
                  /**
                   * \\brief Prints the object onto the output stream
                   * 
                   * \\param os The output stream to print to
                   * \\param value The object to print
                   * \\return The output stream
                   * 
                   */
                  friend std::ostream &operator<<(std::ostream &os, const DataType1 &value) {{
                    return os << \"{{ \" << \"\" << \" }}\";
                  }}

                private:
                  explicit DataType1() {{}}




                }};

                class DataType2 {{
                public:
                  /**
                   * \\brief Constructs a new DataType2 object 
                   * 
                   * 
                   * 
                   */
                  [[nodiscard]] static termite::Result<DataType2> from_values() {{
                
                
                    return termite::Result<DataType2>::ok(DataType2());
                  }}

                

                

                  /**
                   * \\brief Checks if this object the the other object are identical
                   * 
                   * \\param other The other object to compare with
                   * \\return true if they are identical, false if not
                   * 
                   */
                  [[nodiscard]] bool operator==(const DataType2 &other) {{
                    return true;
                  }}
                  /**
                   * \\brief Checks if this object the the other object are different
                   * 
                   * \\param other The other object to compare with
                   * \\return true if they are different, false if not
                   * 
                   */
                  [[nodiscard]] bool operator!=(const DataType2 &other) {{
                    return !(*this == other);
                  }}
                  /**
                   * \\brief Prints the object onto the output stream
                   * 
                   * \\param os The output stream to print to
                   * \\param value The object to print
                   * \\return The output stream
                   * 
                   */
                  friend std::ostream &operator<<(std::ostream &os, const DataType2 &value) {{
                    return os << \"{{ \" << \"\" << \" }}\";
                  }}

                private:
                  explicit DataType2() {{}}




                }};

                #endif
            ");

            assert_eq!(str_diff(&header_file, &expected), None);
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
                
                #include <iostream>
                #include <optional>
                #include <variant>
                #include <termite>

                /**
                 * \\brief description1
                 * 
                 */
                class DataType1 {{
                public:
                  /**
                   * \\brief Constructs a new DataType1 object 
                   * 
                   * 
                   * 
                   */
                  [[nodiscard]] static termite::Result<DataType1> from_values() {{
                
                
                    return termite::Result<DataType1>::ok(DataType1());
                  }}

                



                  /**
                   * \\brief Checks if this object the the other object are identical
                   * 
                   * \\param other The other object to compare with
                   * \\return true if they are identical, false if not
                   * 
                   */
                  [[nodiscard]] bool operator==(const DataType1 &other) {{
                    return true;
                  }}
                  /**
                   * \\brief Checks if this object the the other object are different
                   * 
                   * \\param other The other object to compare with
                   * \\return true if they are different, false if not
                   * 
                   */
                  [[nodiscard]] bool operator!=(const DataType1 &other) {{
                    return !(*this == other);
                  }}
                  /**
                   * \\brief Prints the object onto the output stream
                   * 
                   * \\param os The output stream to print to
                   * \\param value The object to print
                   * \\return The output stream
                   * 
                   */
                  friend std::ostream &operator<<(std::ostream &os, const DataType1 &value) {{
                    return os << \"{{ \" << \"\" << \" }}\";
                  }}

                private:
                  explicit DataType1() {{}}




                }};

                /**
                 * \\brief description2
                 * 
                 */
                class DataType2 {{
                public:
                  /**
                   * \\brief Constructs a new DataType2 object 
                   * 
                   * 
                   * 
                   */
                  [[nodiscard]] static termite::Result<DataType2> from_values() {{
                
                
                    return termite::Result<DataType2>::ok(DataType2());
                  }}

                


                
                  /**
                   * \\brief Checks if this object the the other object are identical
                   * 
                   * \\param other The other object to compare with
                   * \\return true if they are identical, false if not
                   * 
                   */
                  [[nodiscard]] bool operator==(const DataType2 &other) {{
                    return true;
                  }}
                  /**
                   * \\brief Checks if this object the the other object are different
                   * 
                   * \\param other The other object to compare with
                   * \\return true if they are different, false if not
                   * 
                   */
                  [[nodiscard]] bool operator!=(const DataType2 &other) {{
                    return !(*this == other);
                  }}
                  /**
                   * \\brief Prints the object onto the output stream
                   * 
                   * \\param os The output stream to print to
                   * \\param value The object to print
                   * \\return The output stream
                   * 
                   */
                  friend std::ostream &operator<<(std::ostream &os, const DataType2 &value) {{
                    return os << \"{{ \" << \"\" << \" }}\";
                  }}

                private:
                  explicit DataType2() {{}}




                }};

                #endif
            ");

            assert_eq!(str_diff(&header_file, &expected), None);
        }

        mod field {
            use super::*;

            #[test]
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
                                        data_type: "int".to_string(),
                                        default: DefaultType::Required,
                                        constraints: vec![],
                                    },
                                    StructField {
                                        name: "field2".to_string(),
                                        description: None,
                                        data_type: "float".to_string(),
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
                    
                    #include <iostream>
                    #include <optional>
                    #include <variant>
                    #include <termite>

                    class DataType {{
                    public:
                      /**
                       * \\brief Constructs a new DataType object 
                       * 
                       * \\param field1 
                       * \\param field2 
                       * 
                       */
                      [[nodiscard]] static termite::Result<DataType> from_values(int field1, float field2) {{
                        termite::Result<std::tuple<>> validate_result = termite::Result<std::tuple<>>::ok({{}});

                        validate_result = validate_field1(field1);
                        if (!validate_result.is_ok()) {{
                          termite::Error error = validate_result.get_err();
                          error.add_field(\"field1\");
                          return termite::Result<DataType>::err(std::move(error));
                        }}

                        validate_result = validate_field2(field2);
                        if (!validate_result.is_ok()) {{
                          termite::Error error = validate_result.get_err();
                          error.add_field(\"field2\");
                          return termite::Result<DataType>::err(std::move(error));
                        }}

                        return termite::Result<DataType>::ok(DataType(std::move(field1), std::move(field2)));
                      }}

                      /**
                       * \\brief Sets the value of field1 if it fulfills the constraints:
                       * 
                       * \\param value The value of field1
                       * \\return An error if one of the constraints were not fulfilled
                       * 
                       */
                      [[nodiscard]] termite::Result<std::tuple<>> set_field1(int value) {{
                        termite::Result<std::tuple<>> validate_result = validate_field1(value);
                        if (!validate_result.is_ok()) {{
                          return termite::Result<DataType>::err(std::move(error));
                        }}

                        field1_ = std::move(value);
                        return termite::Result<std::tuple<>>::ok({{}});
                      }}
                      /**
                       * \\brief Sets the value of field2 if it fulfills the constraints:
                       * 
                       * \\param value The value of field2
                       * \\return An error if one of the constraints were not fulfilled
                       * 
                       */
                      [[nodiscard]] termite::Result<std::tuple<>> set_field2(float value) {{
                        termite::Result<std::tuple<>> validate_result = validate_field2(value);
                        if (!validate_result.is_ok()) {{
                          return termite::Result<DataType>::err(std::move(error));
                        }}

                        field2_ = std::move(value);
                        return termite::Result<std::tuple<>>::ok({{}});
                      }}

                      /**
                       * \\brief Retrieves a reference to the value of field1
                       * 
                       * \\return The reference 
                       * 
                       */
                      [[nodiscard]] const int &get_field1() const {{
                        return field1_;
                      }}
                      /**
                       * \\brief Retrieves a reference to the value of field2
                       * 
                       * \\return The reference 
                       * 
                       */
                      [[nodiscard]] const float &get_field2() const {{
                        return field2_;
                      }}

                      /**
                       * \\brief Checks if this object the the other object are identical
                       * 
                       * \\param other The other object to compare with
                       * \\return true if they are identical, false if not
                       * 
                       */
                      [[nodiscard]] bool operator==(const DataType &other) {{
                        return field1_ == other.field1_ && field2_ == other.field2_;
                      }}
                      /**
                       * \\brief Checks if this object the the other object are different
                       * 
                       * \\param other The other object to compare with
                       * \\return true if they are different, false if not
                       * 
                       */
                      [[nodiscard]] bool operator!=(const DataType &other) {{
                        return !(*this == other);
                      }}
                      /**
                       * \\brief Prints the object onto the output stream
                       * 
                       * \\param os The output stream to print to
                       * \\param value The object to print
                       * \\return The output stream
                       * 
                       */
                      friend std::ostream &operator<<(std::ostream &os, const DataType &value) {{
                        return os << \"{{ \" << \"field1: \" << value.field1_ << \", \" << \"field2: \" << value.field2_ << \" }}\";
                      }}

                    private:
                      explicit DataType(int field1, float field2) : field1_(field1), field2_(field2) {{}}

                      /**
                       * \\brief Validates if field1 is correct using the following constaints:
                       * 
                       */
                      [[nodiscard]] static termite::Result<std::tuple<>> validate_field1(const int &value) {{
                        return termite::Result<std::tuple<>>::ok({{}});
                      }}
                      /**
                       * \\brief Validates if field2 is correct using the following constaints:
                       * 
                       */
                      [[nodiscard]] static termite::Result<std::tuple<>> validate_field2(const float &value) {{
                        return termite::Result<std::tuple<>>::ok({{}});
                      }}

                      int field1_;
                      float field2_;
                    }};

                    #endif
                ");

                assert_eq!(str_diff(&header_file, &expected), None);
            }

            //#[test]
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

            //#[test]
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
            headers: Headers { header: "namespace test {".to_string(), source: "".to_string() },
            footers: Footers { header: "} // namespace test".to_string(), source: "".to_string() },
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
            
            #include <iostream>
            #include <optional>
            #include <variant>
            #include <termite>

            namespace test {{

            /**
             * \\brief description1
             * 
             */
            class DataType1 {{
            public:
              /**
               * \\brief Constructs a new DataType1 object 
               * 
               * 
               * 
               */
              [[nodiscard]] static termite::Result<DataType1> from_values() {{
            
            
                return termite::Result<DataType1>::ok(DataType1());
              }}


            


              /**
               * \\brief Checks if this object the the other object are identical
               * 
               * \\param other The other object to compare with
               * \\return true if they are identical, false if not
               * 
               */
              [[nodiscard]] bool operator==(const DataType1 &other) {{
                return true;
              }}
              /**
               * \\brief Checks if this object the the other object are different
               * 
               * \\param other The other object to compare with
               * \\return true if they are different, false if not
               * 
               */
              [[nodiscard]] bool operator!=(const DataType1 &other) {{
                return !(*this == other);
              }}
              /**
               * \\brief Prints the object onto the output stream
               * 
               * \\param os The output stream to print to
               * \\param value The object to print
               * \\return The output stream
               * 
               */
              friend std::ostream &operator<<(std::ostream &os, const DataType1 &value) {{
                return os << \"{{ \" << \"\" << \" }}\";
              }}

            private:
              explicit DataType1() {{}}




            }};

            /**
             * \\brief description2
             * 
             */
            class DataType2 {{
            public:
              /**
               * \\brief Constructs a new DataType2 object 
               * 
               * 
               * 
               */
              [[nodiscard]] static termite::Result<DataType2> from_values() {{
            
            
                return termite::Result<DataType2>::ok(DataType2());
              }}



            

              /**
               * \\brief Checks if this object the the other object are identical
               * 
               * \\param other The other object to compare with
               * \\return true if they are identical, false if not
               * 
               */
              [[nodiscard]] bool operator==(const DataType2 &other) {{
                return true;
              }}
              /**
               * \\brief Checks if this object the the other object are different
               * 
               * \\param other The other object to compare with
               * \\return true if they are different, false if not
               * 
               */
              [[nodiscard]] bool operator!=(const DataType2 &other) {{
                return !(*this == other);
              }}
              /**
               * \\brief Prints the object onto the output stream
               * 
               * \\param os The output stream to print to
               * \\param value The object to print
               * \\return The output stream
               * 
               */
              friend std::ostream &operator<<(std::ostream &os, const DataType2 &value) {{
                return os << \"{{ \" << \"\" << \" }}\";
              }}

            private:
              explicit DataType2() {{}}




            }};

            }} // namespace test

            #endif
        ");

        assert_eq!(str_diff(&header_file, &expected), None);
    }
}
