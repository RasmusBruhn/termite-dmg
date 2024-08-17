use indoc::formatdoc;
use super::*;

/// The type specific information for a variant
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Enum {
  /// The possible types for the variant
  pub(super) types: Vec<EnumType>,
}

impl Enum {
  /// Constructs a new c++ enum from a generic enum
  /// 
  /// # Parameters
  /// 
  /// data: The generic enum to convert
  pub(super) fn new(data: crate::Enum) -> Result<Self, Error> {
    // Convert the types
    let types = data.types.into_iter().map(|data| EnumType::new(data)).collect::<Result<Vec<EnumType>, Error>>()?;
    
    return Ok(Self {
      types,
    });
  }

  /// Converts the enum to a string for use in the header file
  /// 
  /// # Parameters
  /// 
  /// name: The name of the enum
  /// 
  /// indent: The number of spaces to use for indentation
  pub(super) fn get_source(&self, name: &str, indent: usize) -> String {
    // Get enum type definitions
    let type_definition = self.types.iter()
      .map(|enum_type| enum_type.get_definition(indent))
      .collect::<Vec<String>>()
      .join("\n");

    // Get the type wrappers
    let type_wrappers = self.types.iter()
      .map(|enum_type| enum_type.get_wrapper(indent))
      .collect::<Vec<String>>()
      .join("\n");

    // Get the wrapper name list
    let wrapper_list = self.types.iter()
      .map(|enum_type| enum_type.get_wrapper_name())
      .collect::<Vec<String>>()
      .join(", ");

    // Get the printers
    let printers = self.types.iter()
      .map(|enum_type| enum_type.get_printer(indent))
      .collect::<Vec<String>>()
      .join("\n");

    return formatdoc!("
      struct {name} {{
      {0:indent$}/**
      {0:indent$} * @brief The values of this enum
      {0:indent$} * 
      {0:indent$} */
      {0:indent$}enum Enum {{
      {type_definition}
      {0:indent$}}};

      {type_wrappers}

      {0:indent$}/**
      {0:indent$} * @brief Constructs a new {name} object
      {0:indent$} * 
      {0:indent$} * @param value The value of the enum
      {0:indent$} */
      {0:indent$}explicit {name}(std::variant<{wrapper_list}> value) : value(std::move(value)) {{}}

      {0:indent$}/**
      {0:indent$} * @brief Returns the enum type that is stored
      {0:indent$} * 
      {0:indent$} * @return The enum type
      {0:indent$} */
      {0:indent$}[[nodiscard]] Enum enum_type() const {{
      {0:indent$}{0:indent$}return static_cast<Enum>(value.index());
      {0:indent$}}}

      {0:indent$}/**
      {0:indent$} * @brief Checks if this object and the other object are identical
      {0:indent$} * 
      {0:indent$} * @param x The other object to compare with
      {0:indent$} * @return true if they are identical, false if not
      {0:indent$} */
      {0:indent$}[[nodiscard]] bool operator==(const {name} &x) const {{
      {0:indent$}{0:indent$}return value == x.value;
      {0:indent$}}}
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
      {0:indent$}friend std::ostream &operator<<(std::ostream &os, const {name} &x) {{
      {0:indent$}{0:indent$}os << \"{{ value: \";
      {0:indent$}{0:indent$}switch (x.value.index()) {{
      {printers}
      {0:indent$}{0:indent$}default:
      {0:indent$}{0:indent$}{0:indent$}os << \"Unknown\";
      {0:indent$}{0:indent$}{0:indent$}break;
      {0:indent$}{0:indent$}}}
      {0:indent$}{0:indent$}return os << \" }}\";
      {0:indent$}}}
 
      {0:indent$}/**
      {0:indent$} * @brief The value of the enum
      {0:indent$} * 
      {0:indent$} */
      {0:indent$}std::variant<{wrapper_list}> value;
      }};",
      "",
    );
  }

  /// Gets the source code for the parser for this enum allowing it to be read from a file
  /// 
  /// # Parameters
  /// 
  /// name: The name of the enum
  /// 
  /// indent: The number of spaces to use for indentation
  /// 
  /// namespace: The namespace of the enum
  pub(super) fn get_parser(&self, name: &str, indent: usize, namespace: &[String]) -> String {
    // Get the namespace name
    let namespace = namespace.iter()
      .map(|single_name| format!("{single_name}::"))
      .collect::<Vec<String>>()
      .join("");
    let typename = format!("{namespace}{name}");

    return formatdoc!("
      template<>
      [[nodiscard]] Result<{typename}> Node::Value::to_value<{typename}>() const {{
      {0:indent$}std::map<std::string, Node> empty_map;
      {0:indent$}Node empty_node(Node::Map(std::move(empty_map)));

      {0:indent$}std::stringstream ss;
      {0:indent$}ss << \"Unknown enum type \\\"\" << value_ << \"\\\"\";
      {0:indent$}return Result<{typename}>::err(Error(ss.str()));
      }}",
      "",
    );
  }
}

/// The data for an enum type
#[derive(Clone, Debug, PartialEq)]
pub(super) struct EnumType {
  /// The name of this enum type
  pub(super) name: String,
  /// The description describing this enum type
  pub(super) description: Option<String>,
  /// The type for this enum type, may be omitted for an empty type
  pub(super) data_type: Option<String>,
}

impl EnumType {
  /// Constructs a new c++ enum type from a generic enum type
  /// 
  /// # Parameters
  /// 
  /// data: The generic enum type to convert
  pub(super) fn new(data: crate::EnumType) -> Result<Self, Error> {
    return Ok(Self {
      name: data.name,
      description: data.description,
      data_type: data.data_type,
    });
  }

  /// Gets the description of this enum type
  fn get_description(&self) -> String {
    return match &self.description {
      Some(description) => description.clone(),
      None => "".to_string(),
    };
  }
  
  /// Gets the definition of this enum type
  /// 
  /// # Parameters
  /// 
  /// indent: The indentation to use
  fn get_definition(&self, indent: usize) -> String {
    return formatdoc!("
      {0:indent$}{0:indent$}/**
      {0:indent$}{0:indent$} * @brief {description}
      {0:indent$}{0:indent$} * 
      {0:indent$}{0:indent$} */
      {0:indent$}{0:indent$}k{name},",
      "",
      description = self.get_description(),
      name = self.name,
    );
  }

  /// Gets the wrapper struct of this enum type
  /// 
  /// # Parameters
  /// 
  /// indent: The indentation to use
  fn get_wrapper(&self, indent: usize) -> String {
    // Get the definition of the type
    let type_definition = match &self.data_type {
      Some(data_type) => formatdoc!("
        {0:indent$}{0:indent$}/**
        {0:indent$}{0:indent$} * @brief The value
        {0:indent$}{0:indent$} * 
        {0:indent$}{0:indent$} */
        {0:indent$}{0:indent$}{data_type} value;",
        "",
      ),
      None => "".to_string(),
    };

    // Get the comparison operation
    let comparison = match &self.data_type {
      Some(_) => "value == x.value",
      None => "true",
    };

    // Get the printer
    let printer = match &self.data_type {
      Some(_) => "\"{ value: \" << x.value << \" }\"",
      None => "\"{  }\"",
    };

    return formatdoc!("
      {0:indent$}/**
      {0:indent$} * @brief The data for when the enum is a {name}
      {0:indent$} * 
      {0:indent$} */
      {0:indent$}struct Type{name} {{
      {type_definition}

      {0:indent$}{0:indent$}/**
      {0:indent$}{0:indent$} * @brief Checks if this object and the other object are identical
      {0:indent$}{0:indent$} * 
      {0:indent$}{0:indent$} * @param x The other object to compare with
      {0:indent$}{0:indent$} * @return true if they are identical, false if not
      {0:indent$}{0:indent$} */
      {0:indent$}{0:indent$}[[nodiscard]] bool operator==(const Type{name} &x) const {{
      {0:indent$}{0:indent$}{0:indent$}return {comparison};
      {0:indent$}{0:indent$}}}
      {0:indent$}{0:indent$}/**
      {0:indent$}{0:indent$} * @brief Checks if this object and the other object are different
      {0:indent$}{0:indent$} * 
      {0:indent$}{0:indent$} * @param x The other object to compare with
      {0:indent$}{0:indent$} * @return true if they are different, false if not
      {0:indent$}{0:indent$} */
      {0:indent$}{0:indent$}[[nodiscard]] bool operator!=(const Type{name} &x) const {{
      {0:indent$}{0:indent$}{0:indent$}return !(*this == x);
      {0:indent$}{0:indent$}}}
      {0:indent$}{0:indent$}/**
      {0:indent$}{0:indent$} * @brief Prints the object onto the output stream
      {0:indent$}{0:indent$} * 
      {0:indent$}{0:indent$} * @param os The output stream to print to
      {0:indent$}{0:indent$} * @param x The object to print
      {0:indent$}{0:indent$} * @return The output stream
      {0:indent$}{0:indent$} */
      {0:indent$}{0:indent$}friend std::ostream &operator<<(std::ostream &os, const Type{name} &x) {{
      {0:indent$}{0:indent$}{0:indent$}return os << {printer};
      {0:indent$}{0:indent$}}}
      {0:indent$}}};",
      "",
      name = self.name,
    );
  }

  /// Gets the name of the wrapper struct of this enum type
  fn get_wrapper_name(&self) -> String {
    return format!("Type{name}", name = self.name);
  }

  /// Gets the printer of this enum type
  /// 
  /// # Parameters
  /// 
  /// indent: The indentation to use
  fn get_printer(&self, indent: usize) -> String {
    // Get what it should print
    let printer = match self.data_type {
      Some(_) => format!("\"{name}(\" << std::get<Type{name}>(x.value).value << \")\"", name = self.name),
      None => "\"Empty\"".to_string(),
    };

    return formatdoc!("
      {0:indent$}{0:indent$}case Enum::k{name}:
      {0:indent$}{0:indent$}{0:indent$}os << {printer};
      {0:indent$}{0:indent$}{0:indent$}break;",
      "",
      name = self.name,
    );
  }

  /// Gets the parser for the node value for this enum type
  /// 
  /// # Parameters
  /// 
  /// typename: The typename of the main type
  /// 
  /// indent: The indentation to use
  fn get_parser_value(&self, typename: &str, indent: usize) -> String {
    let internal = match &self.data_type {
        Some(data_type) => formatdoc!("
            {0:indent$}{0:indent$}Result<{data_type}> value = empty_node.to_value<{data_type}>();
            {0:indent$}{0:indent$}if (value.is_ok()) {{
            {0:indent$}{0:indent$}{0:indent$}return Result<{typename}>::ok({typename}({typename}::Type{name}{{value.get_ok()}}));
            {0:indent$}{0:indent$}}}
            {0:indent$}{0:indent$}return Result<{typename}>::err(Error(\"Enum type {name} must contain a value\"));",
            "",
            name = self.name,
          ),
        None => format!("{0:indent$}{0:indent$}return Result<{typename}>::ok({typename}({typename}::Type{name}{{}}));", "", name = self.name),
    };
    
    return formatdoc!("
      {0:indent$}if (value_ == \"{name}\") {{
      {internal}
      {0:indent$}}}",
      "",
      name = self.name,
    );
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
    compile_and_test("type_enum/basic");

    // Make sure it generates the correct code
    let data_model = DataModel {
      headers: Headers { source: "".to_string() },
      footers: Footers { source: "".to_string() },
      data_types: vec![
        DataType {
          name: "DataType".to_string(),
          description: None,
          data: DataTypeData::Enum(Enum {
            types: vec![
              EnumType {
                name: "Int1".to_string(),
                description: None,
                data_type: Some("int".to_string()),
              },
              EnumType {
                name: "Int2".to_string(),
                description: None,
                data_type: Some("int".to_string()),
              },
              EnumType {
                name: "Float".to_string(),
                description: None,
                data_type: Some("float".to_string()),
              },
              EnumType {
                name: "Empty".to_string(),
                description: None,
                data_type: None,
              },
            ],
          }),
        },
      ],
      namespace: vec!["test".to_string()],
    };

    // Create the header file
    let header_file = data_model.get_source("HEADER", 2);
    let expected = include_str!("../../tests/cpp/type_enum/basic/basic.hpp");

    // Check that they are the same
    assert_eq!(str_diff(&header_file, &expected), None);
  }
}
