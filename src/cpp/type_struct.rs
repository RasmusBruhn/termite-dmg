use super::*;
use crate::DefaultType;
use indoc::formatdoc;

/// The type specific information for a struct
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Struct {
    /// A list of all the fields of the struct
    pub(super) fields: Vec<StructField>,
}

impl Struct {
    /// Constructs a new c++ struct from a generic struct
    ///
    /// # Parameters
    ///
    /// data: The generic struct to convert
    pub(super) fn new(data: crate::Struct) -> Result<Self, Error> {
        // Convert the fields
        let fields = data
            .fields
            .into_iter()
            .map(|data| StructField::new(data))
            .collect::<Result<Vec<StructField>, Error>>()?;

        // Move data
        return Ok(Self { fields });
    }

    /// Converts the struct to a string for use in the header file
    ///
    /// # Parameters
    ///
    /// name: The name of the struct
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_definition_header(&self, name: &str, indent: usize) -> String {
        // Get the description for the constructor
        let constructor_description = self
            .fields
            .iter()
            .map(|field| field.get_constructor_description(indent))
            .collect::<Vec<String>>()
            .join("");

        // Get the constructor parameters
        let constructor_parameters = self
            .fields
            .iter()
            .map(|field| field.get_constructor_parameter())
            .collect::<Vec<String>>()
            .join("");

        // Get the list of setters for the internal constructor
        let constructor_setters = self
            .fields
            .iter()
            .map(|field| field.get_constructor_setter())
            .collect::<Vec<String>>()
            .join("");

        // Get all constructors for the fields with default values
        let default_constructors = self
            .fields
            .iter()
            .map(|field| field.get_default_constructor_header(indent))
            .collect::<Vec<String>>()
            .join("");
        let default_constructors = format!("\n{default_constructors}");

        // Get the definitions of all the fields but without any initialization
        let field_definitions = self
            .fields
            .iter()
            .map(|field| field.get_definition(indent))
            .collect::<Vec<String>>()
            .join("");

        // Generate the code
        return formatdoc!("
            struct {name} {{
            public:
            {0:indent$}/**
            {0:indent$} * @brief Constructs a new {name} object
            {0:indent$} * {constructor_description}
            {0:indent$} * @param extra_fields Any extra fields to attach to this struct
            {0:indent$} */
            {0:indent$}explicit {name}({constructor_parameters}::termite::Node::Map extra_fields = ::termite::Node::Map()) : {constructor_setters}extra_fields(std::move(extra_fields)) {{}}
            {default_constructors}
            {0:indent$}/**
            {0:indent$} * @brief Checks if this object and the other object are identical
            {0:indent$} * 
            {0:indent$} * @param x The other object to compare with
            {0:indent$} * @return true if they are identical, false if not
            {0:indent$} */
            {0:indent$}[[nodiscard]] bool operator==(const {name} &x) const;
            {0:indent$}/**
            {0:indent$} * @brief Checks if this object and the other object are different
            {0:indent$} * 
            {0:indent$} * @param x The other object to compare with
            {0:indent$} * @return true if they are different, false if not
            {0:indent$} */
            {0:indent$}[[nodiscard]] bool operator!=(const {name} &x) const {{
            {0:indent$}{0:indent$}return !(*this == x);
            {0:indent$}}}
            {0:indent$}/**
            {0:indent$} * @brief Prints the object onto the output stream
            {0:indent$} * 
            {0:indent$} * @param os The output stream to print to
            {0:indent$} * @param x The object to print
            {0:indent$} * @return The output stream
            {0:indent$} */
            {0:indent$}friend std::ostream &operator<<(std::ostream &os, const {name} &x);
            {field_definitions}
            {0:indent$}/**
            {0:indent$} * @brief All extra fields from when reading which could not be captured
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}::termite::Node::Map extra_fields;
            }};", "",
        );
    }

    /// Converts the struct to a string for use in the source file
    ///
    /// # Parameters
    ///
    /// name: The name of the struct
    ///
    /// indent: The number of spaces to use for indentation
    pub(super) fn get_definition_source(&self, name: &str, indent: usize) -> String {
        // Get the equality test
        let equality_test = self
            .fields
            .iter()
            .map(|field| field.get_equality_check())
            .collect::<Vec<String>>()
            .join("");

        // Get the printout for the operator<< function
        let printout = self
            .fields
            .iter()
            .map(|field| field.get_printout())
            .collect::<Vec<String>>()
            .join("");

        // Get all constructors for the fields with default values
        let default_constructors = self
            .fields
            .iter()
            .map(|field| field.get_default_constructor_source(name, indent))
            .collect::<Vec<String>>()
            .join("");

        // Generate the code
        return formatdoc!("
            [[nodiscard]] bool {name}::operator==(const {name} &x) const {{
            {0:indent$}return {equality_test}extra_fields == x.extra_fields;
            }}
            {default_constructors}
            std::ostream &operator<<(std::ostream &os, const {name} &x) {{
            {0:indent$}return os << \"{{ \" << {printout}\"extra_fields: \" << x.extra_fields << \" }}\";
            }}", "",
        );
    }

    /// Gets the header code for the parser for this struct allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// name: The name of the struct
    ///
    /// namespace: The namespace of the struct
    pub(super) fn get_parser_header(&self, name: &str, namespace: &[String]) -> String {
        // Get the namespace name
        let namespace = namespace
            .iter()
            .map(|single_name| format!("{single_name}::"))
            .collect::<Vec<String>>()
            .join("");
        let typename = format!("{namespace}{name}");

        return formatdoc!(
            "
            template<>
            [[nodiscard]] Result<{typename}> Node::Map::to_value<{typename}>() const;",
        );
    }

    /// Gets the source code for the parser for this struct allowing it to be read from a file
    ///
    /// # Parameters
    ///
    /// name: The name of the struct
    ///
    /// indent: The number of spaces to use for indentation
    ///
    /// namespace: The namespace of the struct
    ///
    /// data_types: List of all the data types defined in the data model
    pub(super) fn get_parser_source(
        &self,
        name: &str,
        indent: usize,
        namespace: &[String],
        data_types: &[DataType],
    ) -> String {
        // Get the namespace name
        let namespace = namespace
            .iter()
            .map(|single_name| format!("{single_name}::"))
            .collect::<Vec<String>>()
            .join("");
        let typename = format!("{namespace}{name}");

        // Get the parameter parsing
        let parsing = self
            .fields
            .iter()
            .map(|field| field.get_parsing(&typename, &namespace, data_types, indent))
            .collect::<Vec<String>>()
            .join("");

        // Get the parameter list for when retrieving them to return at the end
        let parameter_retrievals = self
            .fields
            .iter()
            .map(|field| field.get_parameter_retrieval())
            .collect::<Vec<String>>()
            .join("");

        return formatdoc!("
            template<>
            [[nodiscard]] Result<{typename}> Node::Map::to_value<{typename}>() const {{
            {0:indent$}std::map<std::string, Node> map = map_;
            {parsing}
            {0:indent$}return Result<{typename}>::ok({typename}({parameter_retrievals}Map(std::move(map))));
            }}",
            "",
        );
    }
}

/// A single field for a struct
#[derive(Clone, Debug, PartialEq)]
pub(super) struct StructField {
    /// The name of the field
    pub(super) name: String,
    /// A description of the field
    pub(super) description: Option<String>,
    /// The data type of the field
    pub(super) data_type: String,
    /// Describes if the field is required or not, if optional it gives the
    /// default value
    pub(super) default: crate::DefaultType,
}

impl StructField {
    /// Constructs a new c++ struct field from a generic struct field
    ///
    /// # Parameters
    ///
    /// data: The generic struct field to convert
    fn new(data: crate::StructField) -> Result<Self, Error> {
        return Ok(Self {
            name: data.name,
            description: data.description,
            data_type: data.data_type,
            default: data.default,
        });
    }

    /// Constructs the c++ typename of this field
    fn get_typename(&self) -> String {
        return match &self.default {
            DefaultType::Optional => {
                format!("std::optional<{data_type}>", data_type = self.data_type,)
            }
            _ => self.data_type.clone(),
        };
    }

    /// Gets the description of this field
    fn get_description(&self) -> String {
        return match &self.description {
            Some(description) => description.clone(),
            None => "".to_string(),
        };
    }

    /// Gets the description for the public constructor
    ///
    /// # Parameters
    ///
    /// indent: The number of spaces to use for indentation
    fn get_constructor_description(&self, indent: usize) -> String {
        return format!(
            "\n{0:indent$} * @param {name} {description}",
            "",
            name = self.name,
            description = self.get_description(),
        );
    }

    /// Get the parameter definition for the constructor including default value
    fn get_constructor_parameter(&self) -> String {
        return format!(
            "{typename} {name}, ",
            typename = self.get_typename(),
            name = self.name,
        );
    }

    /// Get the parameter definition for the constructor including default value
    ///
    /// # Parameters
    ///
    /// indent: The number of spaces to use for indentation
    fn get_default_constructor_header(&self, indent: usize) -> String {
        return match &self.default {
            DefaultType::Required => format!(""),
            _ => formatdoc!(
                "
                {0:indent$}/**
                {0:indent$} * @brief Gets the default value for {name}
                {0:indent$} * 
                {0:indent$} * @return The default value for {name}
                {0:indent$} */
                {0:indent$}[[nodiscard]] static {typename} default_{name}();\n",
                "",
                typename = self.get_typename(),
                name = self.name,
            ),
        };
    }

    /// Get the source code for the parameter definition for the constructor including default value
    ///
    /// # Parameters
    ///
    /// main_name: The name of the type which holds this field
    ///
    /// indent: The number of spaces to use for indentation
    fn get_default_constructor_source(&self, main_name: &str, indent: usize) -> String {
        return match &self.default {
            DefaultType::Required => format!(""),
            DefaultType::Optional => formatdoc!(
                "
                \n[[nodiscard]] {typename} {main_name}::default_{snake_case}() {{
                {0:indent$}return std::nullopt;
                }}\n",
                "",
                typename = self.get_typename(),
                snake_case = ToSnakeCase::new(&mut self.name.chars()).collect::<String>(),
            ),
            DefaultType::Default(default_value) => formatdoc!(
                "
                \n[[nodiscard]] {typename} {main_name}::default_{snake_case}() {{
                {0:indent$}return {typename}({default_value});
                }}\n",
                "",
                typename = self.get_typename(),
                snake_case = ToSnakeCase::new(&mut self.name.chars()).collect::<String>(),
            ),
        };
    }

    /// Gets the equality check for this field
    fn get_equality_check(&self) -> String {
        return format!("this->{name} == x.{name} && ", name = self.name);
    }

    /// Gets the printout of this field for the operator>> ostream function
    fn get_printout(&self) -> String {
        return format!("\"{name}: \" << x.{name} << \", \" << ", name = self.name);
    }

    /// Get the setter for this field for the internal constructor
    fn get_constructor_setter(&self) -> String {
        return format!("{name}(std::move({name})), ", name = self.name);
    }

    /// Gets the description if it is supplied
    ///
    /// # Parameters
    ///
    /// indent: The number of spaces to use for indentation
    fn get_definition(&self, indent: usize) -> String {
        return formatdoc!(
            "
            \n{0:indent$}/**
            {0:indent$} * @brief {description}
            {0:indent$} * 
            {0:indent$} */
            {0:indent$}{typename} {name};",
            "",
            typename = self.get_typename(),
            name = self.name,
            description = self.get_description(),
        );
    }

    /// Gets the parsing for this field if it is required
    ///
    /// # Parameters
    ///
    /// main_name: The name of the type which holds this field including namespace
    ///
    /// namespace: The namespace of the struct
    ///
    /// data_types: List of all the data types defined in the data model
    ///
    /// indent: The indentation to use
    fn get_parsing_required(
        &self,
        main_name: &str,
        namespace: &str,
        data_types: &[DataType],
        indent: usize,
    ) -> String {
        // Add possible namespace to the typename
        let typename = if let Some(_) = data_types
            .iter()
            .find(|data_type| data_type.name == self.data_type)
        {
            format!("{namespace}{data_type}", data_type = self.data_type)
        } else {
            format!("{data_type}", data_type = self.data_type)
        };

        return formatdoc!("
            \n{0:indent$}auto location_{name} = map.find(\"{name}\");
            {0:indent$}if (location_{name} == map.end()) {{
            {0:indent$}{0:indent$}return Result<{main_name}>::err(Error(\"Missing {name}\"));
            {0:indent$}}}
            {0:indent$}Result<{typename}> raw_value_{name} = location_{name}->second.to_value<{typename}>();
            {0:indent$}if (!raw_value_{name}.is_ok()) {{
            {0:indent$}{0:indent$}Error error = raw_value_{name}.get_err();
            {0:indent$}{0:indent$}error.add_field(\"{name}\");
            {0:indent$}{0:indent$}return Result<{main_name}>::err(std::move(error));
            {0:indent$}}}
            {0:indent$}{typename} value_{name} = raw_value_{name}.get_ok();
            {0:indent$}map.erase(location_{name});\n",
            "",
            name = self.name,
        );
    }

    /// Gets the parsing for this field if it is optional
    ///
    /// # Parameters
    ///
    /// main_name: The name of the type which holds this field including namespace
    ///
    /// namespace: The namespace of the struct
    ///
    /// data_types: List of all the data types defined in the data model
    ///
    /// indent: The indentation to use
    fn get_parsing_optional(
        &self,
        main_name: &str,
        namespace: &str,
        data_types: &[DataType],
        indent: usize,
    ) -> String {
        // Add possible namespace to the typename
        let base_typename = if let Some(_) = data_types
            .iter()
            .find(|data_type| data_type.name == self.data_type)
        {
            format!("{namespace}{data_type}", data_type = self.data_type)
        } else {
            format!("{data_type}", data_type = self.data_type)
        };

        let typename = match &self.default {
            DefaultType::Optional => format!("std::optional<{base_typename}>"),
            _ => base_typename.clone(),
        };

        // Get default value
        let default = match &self.default {
            DefaultType::Required => format!(""),
            _ => format!(
                " = {main_name}::default_{snake_case}();",
                snake_case = ToSnakeCase::new(&mut self.name.chars()).collect::<String>(),
            ),
        };

        return formatdoc!("
            \n{0:indent$}auto location_{name} = map.find(\"{name}\");
            {0:indent$}{typename} value_{name}{default};
            {0:indent$}if (location_{name} != map.end()) {{
            {0:indent$}{0:indent$}Result<{base_typename}> raw_value_{name} = location_{name}->second.to_value<{base_typename}>();
            {0:indent$}{0:indent$}if (!raw_value_{name}.is_ok()) {{
            {0:indent$}{0:indent$}{0:indent$}Error error = raw_value_{name}.get_err();
            {0:indent$}{0:indent$}{0:indent$}error.add_field(\"{name}\");
            {0:indent$}{0:indent$}{0:indent$}return Result<{main_name}>::err(std::move(error));
            {0:indent$}{0:indent$}}}
            {0:indent$}{0:indent$}value_{name} = raw_value_{name}.get_ok();
            {0:indent$}{0:indent$}map.erase(location_{name});
            {0:indent$}}}\n",
            "",
            name = self.name,
        );
    }

    /// Gets the parsing for this field
    ///
    /// # Parameters
    ///
    /// main_name: The name of the type which holds this field including namespace
    ///
    /// namespace: The namespace of the struct
    ///
    /// data_types: List of all the data types defined in the data model
    ///
    /// indent: The indentation to use
    fn get_parsing(
        &self,
        main_name: &str,
        namespace: &str,
        data_types: &[DataType],
        indent: usize,
    ) -> String {
        return match self.default {
            DefaultType::Required => {
                self.get_parsing_required(main_name, namespace, data_types, indent)
            }
            _ => self.get_parsing_optional(main_name, namespace, data_types, indent),
        };
    }

    /// Gets the value of this field when parsing after it is read
    fn get_parameter_retrieval(&self) -> String {
        return format!("std::move(value_{name}), ", name = self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpp::test_utils::*;

    #[test]
    fn basic() {
        // Check c++ code
        compile_and_test("type_struct/basic");

        // Make sure it generates the correct code
        let data_model = DataModel {
            headers: Headers {
                header: "".to_string(),
                source: "".to_string(),
            },
            footers: Footers {
                header: "".to_string(),
                source: "".to_string(),
            },
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
            namespace: vec!["test".to_string()],
        };

        // Create the header file
        let header_file = data_model.get_header("HEADER", 2);
        let source_file = data_model.get_source("basic", 2);
        let expected_header = include_str!("../../tests/cpp/type_struct/basic/basic.h");
        let expected_source = include_str!("../../tests/cpp/type_struct/basic/basic.cpp");

        // Check that they are the same
        assert_eq!(str_diff(&header_file, &expected_header), None);
        assert_eq!(str_diff(&source_file, &expected_source), None);
    }

    #[test]
    fn description() {
        // Check c++ code
        compile_and_test("type_struct/description");

        // Make sure it generates the correct code
        let data_model = DataModel {
            headers: Headers {
                header: "".to_string(),
                source: "".to_string(),
            },
            footers: Footers {
                header: "".to_string(),
                source: "".to_string(),
            },
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
            namespace: vec!["test".to_string()],
        };

        // Create the header file
        let header_file = data_model.get_header("HEADER", 2);
        let source_file = data_model.get_source("description", 2);
        let expected_header = include_str!("../../tests/cpp/type_struct/description/description.h");
        let expected_source = include_str!("../../tests/cpp/type_struct/description/description.cpp");

        // Check that they are the same
        assert_eq!(str_diff(&header_file, &expected_header), None);
        assert_eq!(str_diff(&source_file, &expected_source), None);
    }

    mod field {
        use super::*;

        #[test]
        fn basic() {
            // Check c++ code
            compile_and_test("type_struct/field/basic");

            // Make sure it generates the correct code
            let data_model = DataModel {
                headers: Headers {
                    header: "".to_string(),
                    source: "".to_string(),
                },
                footers: Footers {
                    header: "".to_string(),
                    source: "".to_string(),
                },
                data_types: vec![DataType {
                    name: "DataType".to_string(),
                    description: None,
                    data: DataTypeData::Struct(Struct {
                        fields: vec![
                            StructField {
                                name: "field1".to_string(),
                                description: None,
                                data_type: "int".to_string(),
                                default: DefaultType::Required,
                            },
                            StructField {
                                name: "field2".to_string(),
                                description: None,
                                data_type: "float".to_string(),
                                default: DefaultType::Required,
                            },
                        ],
                    }),
                }],
                namespace: vec!["test".to_string()],
            };

            // Create the header file
            let header_file = data_model.get_header("HEADER", 2);
            let source_file = data_model.get_source("basic", 2);
            let expected_header = include_str!("../../tests/cpp/type_struct/field/basic/basic.h");
            let expected_source = include_str!("../../tests/cpp/type_struct/field/basic/basic.cpp");

            // Check that they are the same
            assert_eq!(str_diff(&header_file, &expected_header), None);
            assert_eq!(str_diff(&source_file, &expected_source), None);
        }

        #[test]
        fn description() {
            // Check c++ code
            compile_and_test("type_struct/field/description");

            // Make sure it generates the correct code
            let data_model = DataModel {
                headers: Headers {
                    header: "".to_string(),
                    source: "".to_string(),
                },
                footers: Footers {
                    header: "".to_string(),
                    source: "".to_string(),
                },
                data_types: vec![DataType {
                    name: "DataType".to_string(),
                    description: None,
                    data: DataTypeData::Struct(Struct {
                        fields: vec![
                            StructField {
                                name: "field1".to_string(),
                                description: Some("description1".to_string()),
                                data_type: "int".to_string(),
                                default: DefaultType::Required,
                            },
                            StructField {
                                name: "field2".to_string(),
                                description: Some("description2".to_string()),
                                data_type: "float".to_string(),
                                default: DefaultType::Required,
                            },
                        ],
                    }),
                }],
                namespace: vec!["test".to_string()],
            };

            // Create the header file
            let header_file = data_model.get_header("HEADER", 2);
            let source_file = data_model.get_source("description", 2);
            let expected_header = include_str!("../../tests/cpp/type_struct/field/description/description.h");
            let expected_source = include_str!("../../tests/cpp/type_struct/field/description/description.cpp");

            // Check that they are the same
            assert_eq!(str_diff(&header_file, &expected_header), None);
            assert_eq!(str_diff(&source_file, &expected_source), None);
        }

        #[test]
        fn optional() {
            // Check c++ code
            compile_and_test("type_struct/field/optional");

            // Make sure it generates the correct code
            let data_model = DataModel {
                headers: Headers {
                    header: "".to_string(),
                    source: "".to_string(),
                },
                footers: Footers {
                    header: "".to_string(),
                    source: "".to_string(),
                },
                data_types: vec![DataType {
                    name: "DataType".to_string(),
                    description: None,
                    data: DataTypeData::Struct(Struct {
                        fields: vec![
                            StructField {
                                name: "field1".to_string(),
                                description: None,
                                data_type: "int".to_string(),
                                default: DefaultType::Default("1".to_string()),
                            },
                            StructField {
                                name: "field2".to_string(),
                                description: None,
                                data_type: "float".to_string(),
                                default: DefaultType::Optional,
                            },
                        ],
                    }),
                }],
                namespace: vec!["test".to_string()],
            };

            // Create the header file
            let header_file = data_model.get_header("HEADER", 2);
            let source_file = data_model.get_source("optional", 2);
            let expected_header = include_str!("../../tests/cpp/type_struct/field/optional/optional.h");
            let expected_source = include_str!("../../tests/cpp/type_struct/field/optional/optional.cpp");

            // Check that they are the same
            assert_eq!(str_diff(&header_file, &expected_header), None);
            assert_eq!(str_diff(&source_file, &expected_source), None);
        }
    }
}
