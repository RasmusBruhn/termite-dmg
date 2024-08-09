use indoc::formatdoc;
use crate::DefaultType;
use super::*;

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
    // Make sure the required fields are first
    if let Some(name) = data.fields.iter()
      .scan(false, |found_optional, field| {
        if let crate::DefaultType::Required = field.default {
          if *found_optional {
            return Some(Some(&field.name));
          }
        } else {
          *found_optional = true;
        }

        return Some(None);
      })
      .filter_map(|value| value)
      .next() {
      return Err(Error {
        location: "".to_string(),
        error: ErrorCore::StructFieldOrder(name.clone()),
      })
    }

    // Convert the fields
    let fields = data.fields.into_iter().map(|data| StructField::new(data)).collect::<Result<Vec<StructField>, Error>>()?;

    // Move data
    return Ok(Self {
      fields
    });
  }

  /// Converts the struct to a string for use in the header file
  /// 
  /// # Parameters
  /// 
  /// name: The name of the struct
  /// 
  /// indent: The number of spaces to use for indentation
  pub(super) fn get_source(&self, name: &str, indent: usize) -> String {
    // Get the description for the constructor
    let constructor_description = self.fields.iter()
      .map(|field| field.get_constructor_description())
      .collect::<Vec<String>>()
      .join(&format!("\n{0:indent$} * ", ""));

    // Get the constructor parameters
    let constructor_parameters = self.fields.iter()
      .map(|field| field.get_constructor_parameter())
      .collect::<Vec<String>>()
      .join("");

    // Get the equality test
    let equality_test = self.fields.iter()
      .map(|field| field.get_equality_check())
      .collect::<Vec<String>>()
      .join("");

    // Get the printout for the operator<< function
    let printout = self.fields.iter()
      .map(|field| field.get_printout())
      .collect::<Vec<String>>()
      .join("");

    // Get the list of setters for the internal constructor
    let constructor_setters = self.fields.iter()
      .map(|field| field.get_constructor_setter())
      .collect::<Vec<String>>()
      .join("");

    // Get the definitions of all the fields but without any initialization
    let field_definitions = self.fields.iter()
      .map(|field| field.get_definition(indent))
      .collect::<Vec<String>>()
      .join("\n");

    // Generate the code
    return formatdoc!("
      struct {name} {{
      public:
      {0:indent$}/**
      {0:indent$} * @brief Constructs a new {name} object
      {0:indent$} * 
      {0:indent$} * {constructor_description}
      {0:indent$} */
      {0:indent$}explicit {name}({constructor_parameters}::termite::Node::Map extra_fields = ::termite::Node::Map()) : {constructor_setters}extra_fields(std::move(extra_fields)) {{}}

      {0:indent$}/**
      {0:indent$} * @brief Checks if this object and the other object are identical
      {0:indent$} * 
      {0:indent$} * @param x The other object to compare with
      {0:indent$} * @return true if they are identical, false if not
      {0:indent$} */
      {0:indent$}[[nodiscard]] bool operator==(const {name} &x) {{
      {0:indent$}{0:indent$}return {equality_test}extra_fields == x.extra_fields;
      {0:indent$}}}
      {0:indent$}/**
      {0:indent$} * @brief Checks if this object and the other object are different
      {0:indent$} * 
      {0:indent$} * @param x The other object to compare with
      {0:indent$} * @return true if they are different, false if not
      {0:indent$} */
      {0:indent$}[[nodiscard]] bool operator!=(const {name} &x) {{
      {0:indent$}{0:indent$}return !(*this == x);
      {0:indent$}}}
      {0:indent$}/**
      {0:indent$} * @brief Prints the object onto the output stream
      {0:indent$} * 
      {0:indent$} * @param os The output stream to print to
      {0:indent$} * @param x The object to print
      {0:indent$} * @return The output stream
      {0:indent$} */
      {0:indent$}friend std::ostream &operator<<(std::ostream &os, const {name} &x) {{
      {0:indent$}{0:indent$}return os << \"{{ \" << {printout}\"extra_fields: \" << x.extra_fields << \" }}\";
      {0:indent$}}}

      {field_definitions}
      {0:indent$}/**
      {0:indent$} * @brief All extra fields from when reading which could not be captured
      {0:indent$} * 
      {0:indent$} */
      {0:indent$}::termite::Node::Map extra_fields;
      }};", "",
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
  pub(super) fn get_parser(&self, name: &str, indent: usize, namespace: &[String]) -> String {
    // Get the namespace name
    let namespace = namespace.iter()
      .map(|single_name| format!("{single_name}::"))
      .collect::<Vec<String>>()
      .join("");
    let typename = format!("{namespace}{name}");

    // Get the parameter parsing
    let parsing = self.fields.iter()
      .map(|field| field.get_parsing(&typename, indent))
      .collect::<Vec<String>>()
      .join("\n\n");

    // Get this list of the names of all fields as strings
    let field_names = self.fields.iter()
      .map(|field| field.get_name_string())
      .collect::<Vec<String>>()
      .join(", ");

    // Get the parameter list for when retrieving them to return at the end
    let parameter_retrievals = self.fields.iter()
      .map(|field| field.get_parameter_retrieval())
      .collect::<Vec<String>>()
      .join("");

    return formatdoc!("
      template<>
      [[nodiscard]] Result<{typename}> Node::Map::to_value() const {{
      {parsing}

      {0:indent$}std::vector<std::string> keys;
      {0:indent$}std::transform(map_.cbegin(), map_.cend(), std::back_inserter(keys), [](const std::pair<const std::string, Node> &key_value) {{
      {0:indent$}{0:indent$}return key_value.first;
      {0:indent$}}});
      {0:indent$}std::vector<std::string> leftovers;
      {0:indent$}std::copy_if(keys.cbegin(), keys.cend(), std::back_inserter(leftovers), [](const std::string &value) {{
      {0:indent$}{0:indent$}std::vector<std::string> correct = {{{field_names}}};
      {0:indent$}{0:indent$}return std::find(correct.cbegin(), correct.cend(), value) == correct.cend();
      {0:indent$}}});
      {0:indent$}std::vector<std::pair<std::string, Node>> extra_fields;
      {0:indent$}std::transform(std::make_move_iterator(leftovers.begin()), std::make_move_iterator(leftovers.end()), std::back_inserter(extra_fields), [this](std::string name) {{
      {0:indent$}{0:indent$}auto value = map_.find(name);
      {0:indent$}{0:indent$}return std::make_pair(std::move(name), value->second);
      {0:indent$}}});

      {0:indent$}return Result<{typename}>::ok({typename}({parameter_retrievals}Map(std::map<std::string, Node>(std::make_move_iterator(extra_fields.begin()), std::make_move_iterator(extra_fields.end())))));
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
  /// A list of all the constraints the field must uphold
  pub(super) constraints: Vec<String>,
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
      constraints: data.constraints,
    });
  }

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
  fn get_constructor_description(&self) -> String {
    return format!(
      "@param {name} {description}",
      name = self.name,
      description = self.get_description(),
    );
  }

  /// Get the parameter definition for the constructor including default value
  fn get_constructor_parameter(&self) -> String {
    return format!(
      "{typename} {name}{default}, ",
      typename = self.get_typename(),
      name = self.name,
      default = self.get_default(),
    );
  }

  /// Gets the equality check for this field
  fn get_equality_check(&self) -> String {
    return format!(
      "{name} == x.{name} && ",
      name = self.name,
    );
  }

  /// Gets the printout of this field for the operator>> ostream function
  fn get_printout(&self) -> String {
    return format!(
      "\"{name}: \" << x.{name} << \", \" << ",
      name = self.name,
    );
  }

  /// Get the setter for this field for the internal constructor
  fn get_constructor_setter(&self) -> String {
    return format!(
      "{name}(std::move({name})), ",
      name = self.name
    );
  }

  /// Gets the description if it is supplied
  /// 
  /// # Parameters
  /// 
  /// indent: The number of spaces to use for indentation
  fn get_definition(&self, indent: usize) -> String {
    return formatdoc!("
      {0:indent$}/**
      {0:indent$} * @brief {description}
      {0:indent$} * 
      {0:indent$} */
      {0:indent$}{typename} {name};",
      "",
      typename = self.get_typename(),
      name = self.name,
      description = self.get_description(),
    )
  }
  
  /// Gets the parsing for this field if it is required
  /// 
  /// # Parameters
  /// 
  /// main_name: The name of the type which holds this field including namespace
  /// 
  /// indent: The indentation to use
  fn get_parsing_required(&self, main_name: &str, indent: usize) -> String {
    return formatdoc!("
      {0:indent$}auto location_{name} = map_.find(\"{name}\");
      {0:indent$}if (location_{name} == map_.end()) {{
      {0:indent$}{0:indent$}return Result<{main_name}>::err(Error(\"Missing {name}\"));
      {0:indent$}}}
      {0:indent$}Result<{typename}> raw_value_{name} = location_{name}->second.to_value<{typename}>();
      {0:indent$}if (!raw_value_{name}.is_ok()) {{
      {0:indent$}{0:indent$}Error error = raw_value_{name}.get_err();
      {0:indent$}{0:indent$}error.add_field(\"{name}\");
      {0:indent$}{0:indent$}return Result<{main_name}>::err(std::move(error));
      {0:indent$}}}
      {0:indent$}{typename} value_{name} = raw_value_{name}.get_ok();",
      "",
      name = self.name,
      typename = self.data_type,
    );
  }

  /// Gets the parsing for this field if it is optional
  /// 
  /// # Parameters
  /// 
  /// main_name: The name of the type which holds this field including namespace
  /// 
  /// indent: The indentation to use
  fn get_parsing_optional(&self, main_name: &str, indent: usize) -> String {
    return formatdoc!("
      {0:indent$}auto location_{name} = map_.find(\"{name}\");
      {0:indent$}{typename} value_{name}{default};
      {0:indent$}if (location_{name} != map_.end()) {{
      {0:indent$}{0:indent$}Result<{base_typename}> raw_value_{name} = location_{name}->second.to_value<{base_typename}>();
      {0:indent$}{0:indent$}if (!raw_value_{name}.is_ok()) {{
      {0:indent$}{0:indent$}{0:indent$}Error error = raw_value_{name}.get_err();
      {0:indent$}{0:indent$}{0:indent$}error.add_field(\"{name}\");
      {0:indent$}{0:indent$}{0:indent$}return Result<{main_name}>::err(std::move(error));
      {0:indent$}{0:indent$}}}
      {0:indent$}{0:indent$}value_{name} = raw_value_{name}.get_ok();
      {0:indent$}}}",
      "",
      name = self.name,
      typename = self.get_typename(),
      default = self.get_default(),
      base_typename = self.data_type,
    );
  }

  /// Gets the parsing for this field
  /// 
  /// # Parameters
  /// 
  /// main_name: The name of the type which holds this field including namespace
  /// 
  /// indent: The indentation to use
  fn get_parsing(&self, main_name: &str, indent: usize) -> String {
    return match self.default {
        DefaultType::Required => self.get_parsing_required(main_name, indent),
        _ => self.get_parsing_optional(main_name, indent),
    };
  }

  /// Gets the name of the field as a string
  fn get_name_string(&self) -> String {
    return format!("\"{name}\"", name = self.name);
  }

  /// Gets the value of this field when parsing after it is read
  fn get_parameter_retrieval(&self) -> String {
    return format!("std::move(value_{name}), ", name = self.name);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::{
    fs,
    path,
    process,
  };

  fn str_diff(lhs: &str, rhs: &str) -> Option<(usize, String, String)> {
    if let Some(error) = lhs.lines()
      .zip(rhs.lines())
      .enumerate()
      .filter_map(|(index, (lhs, rhs))| {
        return if lhs == rhs {
          None
        } else {
          Some((index + 1, lhs.to_string(), rhs.to_string()))
        };
      })
      .next() {
      return Some(error);
    }

    if lhs.lines().count() != rhs.lines().count() {
      return Some((0, "".to_string(), "".to_string()));
    }

    return None;
  }

  fn get_source_path(name: &str) -> path::PathBuf {
    // Get the filename
    let filename = path::Path::new(name).file_name().unwrap().to_str().unwrap();

    return path::Path::new("tests/cpp")
      .join(format!("{name}"))
      .join(format!("{filename}_test.cpp"));
  }

  fn get_exe_path(name: &str) -> path::PathBuf {
    // Get the filename
    let filename = path::Path::new(name).file_name().unwrap().to_str().unwrap();

    return path::Path::new("target/tests/cpp")
      .join(format!("{name}"))
      .join(filename);
  }

  fn compile_and_test(name: &str) {
    // Get the paths
    let source_path = get_source_path(name);
    let exe_path = get_exe_path(name);

    // Create the output directory
    fs::create_dir_all(exe_path.parent().unwrap()).unwrap();

    // Compile code
    let compile_output = if cfg!(target_os = "windows") {
      process::Command::new("cmd")
        .arg("/C")
        .arg(format!("g++ {} -Isrc/cpp -Wall -std=c++17 -o {}.exe", source_path.to_str().unwrap(), exe_path.to_str().unwrap()))
        .output()
        .expect("failed to compile")
    } else {
      process::Command::new("sh")
        .arg("-c")
        .arg(format!("g++ {} -Isrc/cpp -Wall -std=c++17 -o {}", source_path.to_str().unwrap(), exe_path.to_str().unwrap()))
        .output()
        .expect("failed to compile")
    };

    // Make sure it comiled without any warnings
    assert_eq!(compile_output.status.code().expect("Unable to compile"), 0);
    assert_eq!(compile_output.stdout.len(), 0);
    assert_eq!(compile_output.stderr.len(), 0);

    // Run the test executable
    let test_output = if cfg!(target_os = "windows") {
      process::Command::new("cmd")
        .arg("/C")
        .arg(format!(".\\{}.exe", exe_path.to_str().unwrap().replace('/', "\\")))
        .output()
        .expect("failed to test")
    } else {
      process::Command::new("sh")
        .arg("-c")
        .arg(format!("./{}", exe_path.to_str().unwrap()))
        .output()
        .expect("failed to test")
    };

    assert_eq!(test_output.status.code().expect("Unable to run test"), 0);
  }

  #[test]
  fn basic() {
    // Check c++ code
    compile_and_test("type_struct/basic");

    // Make sure it generates the correct code
    let data_model = DataModel {
      headers: Headers { source: "".to_string() },
      footers: Footers { source: "".to_string() },
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
    let header_file = data_model.get_source("HEADER", 2);
    let expected = include_str!("../../tests/cpp/type_struct/basic/basic.hpp");

    // Check that they are the same
    assert_eq!(str_diff(&header_file, &expected), None);
  }

  #[test]
  fn description() {
    // Check c++ code
    compile_and_test("type_struct/description");

    // Make sure it generates the correct code
    let data_model = DataModel {
      headers: Headers { source: "".to_string() },
      footers: Footers { source: "".to_string() },
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
    let header_file = data_model.get_source("HEADER", 2);
    let expected = include_str!("../../tests/cpp/type_struct/description/description.hpp");

    // Check that they are the same
    assert_eq!(str_diff(&header_file, &expected), None);
  }

  mod field {
    use super::*;

    #[test]
    fn basic() {
      // Check c++ code
      compile_and_test("type_struct/field/basic");

      // Make sure it generates the correct code
      let data_model = DataModel {
        headers: Headers { source: "".to_string() },
        footers: Footers { source: "".to_string() },
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
        namespace: vec!["test".to_string()],
      };

      // Create the header file
      let header_file = data_model.get_source("HEADER", 2);
      let expected = include_str!("../../tests/cpp/type_struct/field/basic/basic.hpp");

      // Check that they are the same
      assert_eq!(str_diff(&header_file, &expected), None);
    }

    #[test]
    fn description() {
      // Check c++ code
      compile_and_test("type_struct/field/description");

      // Make sure it generates the correct code
      let data_model = DataModel {
        headers: Headers { source: "".to_string() },
        footers: Footers { source: "".to_string() },
        data_types: vec![
          DataType {
            name: "DataType".to_string(),
            description: None,
            data: DataTypeData::Struct(Struct {
              fields: vec![
                StructField {
                  name: "field1".to_string(),
                  description: Some("description1".to_string()),
                  data_type: "int".to_string(),
                  default: DefaultType::Required,
                  constraints: vec![],
                },
                StructField {
                  name: "field2".to_string(),
                  description: Some("description2".to_string()),
                  data_type: "float".to_string(),
                  default: DefaultType::Required,
                  constraints: vec![],
                },
              ] 
            }),
          },
        ],
        namespace: vec!["test".to_string()],
      };

      // Create the header file
      let header_file = data_model.get_source("HEADER", 2);
      let expected = include_str!("../../tests/cpp/type_struct/field/description/description.hpp");

      // Check that they are the same
      assert_eq!(str_diff(&header_file, &expected), None);
    }

    #[test]
    fn optional() {
      // Check c++ code
      compile_and_test("type_struct/field/optional");

      // Make sure it generates the correct code
      let data_model = DataModel {
        headers: Headers { source: "".to_string() },
        footers: Footers { source: "".to_string() },
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
                  default: DefaultType::Default("1".to_string()),
                  constraints: vec![],
                },
                StructField {
                  name: "field2".to_string(),
                  description: None,
                  data_type: "float".to_string(),
                  default: DefaultType::Optional,
                  constraints: vec![],
                },
              ] 
            }),
          },
        ],
        namespace: vec!["test".to_string()],
      };

      // Create the header file
      let header_file = data_model.get_source("HEADER", 2);
      let expected = include_str!("../../tests/cpp/type_struct/field/optional/optional.hpp");

      // Check that they are the same
      assert_eq!(str_diff(&header_file, &expected), None); 
    }
  }

}
