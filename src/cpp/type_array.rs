use indoc::formatdoc;
use super::*;

/// The type specific information for an array
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Array {
  /// The data type for all elements of the array
  pub(super) data_type: String,
  /// A list of all the constraints all elements must uphold
  pub(super) constraints: Vec<String>,
}

impl Array {
  /// Constructs a new c++ array from a generic array
  /// 
  /// # Parameters
  /// 
  /// data: The generic array to convert
  pub(super) fn new(data: crate::Array) -> Result<Self, Error> {
    return Ok(Self {
      data_type: data.data_type,
      constraints: data.constraints,
    });
  }

  /// Converts the array to a string for use in the header file
  /// 
  /// # Parameters
  /// 
  /// name: The name of the array
  /// 
  /// indent: The number of spaces to use for indentation
  pub(super) fn get_source(&self, name: &str, indent: usize) -> String {  
    return formatdoc!("
      struct {name} {{
      public:
      {0:indent$}/**
      {0:indent$} * @brief Constructs a new {name} object
      {0:indent$} * 
      {0:indent$} * @param values The values of the array
      {0:indent$} */
      {0:indent$}explicit {name}(std::vector<{typename}> values) : values(std::move(values)) {{}}

      {0:indent$}/**
      {0:indent$} * @brief Checks if this object and the other object are identical
      {0:indent$} * 
      {0:indent$} * @param x The other object to compare with
      {0:indent$} * @return true if they are identical, false if not
      {0:indent$} */
      {0:indent$}[[nodiscard]] bool operator==(const {name} &x) {{
      {0:indent$}{0:indent$}if (values.size() != x.values.size()) {{
      {0:indent$}{0:indent$}{0:indent$}return false;
      {0:indent$}{0:indent$}}}

      {0:indent$}{0:indent$}for (auto lhs = values.cbegin(), rhs = x.values.cbegin(); lhs < values.cend(); ++lhs, ++rhs) {{
      {0:indent$}{0:indent$}{0:indent$}if (*lhs != *rhs) {{
      {0:indent$}{0:indent$}{0:indent$}{0:indent$}return false;
      {0:indent$}{0:indent$}{0:indent$}}}
      {0:indent$}{0:indent$}}}

      {0:indent$}{0:indent$}return true;
      {0:indent$}}}
      {0:indent$}/**
      {0:indent$} * @brief Checks if this object and the other object are different
      {0:indent$} * 
      {0:indent$} * @param x The other object to compare with
      {0:indent$} * @return true if they are different, false if not
      {0:indent$} */
      {0:indent$}[[nodiscard]] bool operator!=(const {name} &x) {{
      {0:indent$}  return !(*this == x);
      {0:indent$}}}
      {0:indent$}/**
      {0:indent$} * @brief Prints the object onto the output stream
      {0:indent$} * 
      {0:indent$} * @param os The output stream to print to
      {0:indent$} * @param x The object to print
      {0:indent$} * @return The output stream
      {0:indent$} */
      {0:indent$}friend std::ostream &operator<<(std::ostream &os, const {name} &x) {{
      {0:indent$}{0:indent$}os << \"{{ values: [ \";
      {0:indent$}{0:indent$}for (auto value = x.values.cbegin(); value < x.values.cend(); ++value) {{
      {0:indent$}{0:indent$}{0:indent$}if (value != x.values.cbegin()) {{
      {0:indent$}{0:indent$}{0:indent$}{0:indent$}os << \", \";
      {0:indent$}{0:indent$}{0:indent$}}}
      {0:indent$}{0:indent$}{0:indent$}os << *value;
      {0:indent$}{0:indent$}}}
      {0:indent$}{0:indent$}return os << \" ] }}\";
      {0:indent$}}}

      {0:indent$}/**
      {0:indent$} * @brief The values of the array
      {0:indent$} * 
      {0:indent$} */
      {0:indent$}std::vector<{typename}> values;
      }};",
      "",
      typename = self.data_type,
    );
  }

  /// Gets the source code for the parser for this array allowing it to be read from a file
  /// 
  /// # Parameters
  /// 
  /// name: The name of the array
  /// 
  /// indent: The number of spaces to use for indentation
  /// 
  /// namespace: The namespace of the array
  pub(super) fn get_parser(&self, name: &str, indent: usize, namespace: &[String]) -> String {
    // Get the namespace name
    let namespace = namespace.iter()
      .map(|single_name| format!("{single_name}::"))
      .collect::<Vec<String>>()
      .join("");
    let typename = format!("{namespace}{name}");

    return formatdoc!("
      template<>
      [[nodiscard]] Result<{typename}> Node::List::to_value() const {{
      {0:indent$}std::vector<{data_type}> values;
      {0:indent$}values.reserve(list_.size());
      {0:indent$}for (auto node = list_.cbegin(); node < list_.cend(); ++node) {{
      {0:indent$}{0:indent$}Result<{data_type}> value = node->to_value<{data_type}>();
      {0:indent$}{0:indent$}if (!value.is_ok()) {{
      {0:indent$}{0:indent$}{0:indent$}Error error = value.get_err();
      {0:indent$}{0:indent$}{0:indent$}error.add_list(node - list_.cbegin());
      {0:indent$}{0:indent$}{0:indent$}return Result<{typename}>::err(std::move(error));
      {0:indent$}{0:indent$}}}
      {0:indent$}{0:indent$}values.push_back(std::move(value.get_ok()));
      {0:indent$}}}

      {0:indent$}return Result<{typename}>::ok({typename}(std::move(values)));
      }}",
      "",
      data_type = self.data_type,
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
    compile_and_test("type_array/basic");

    // Make sure it generates the correct code
    let data_model = DataModel {
      headers: Headers { source: "".to_string() },
      footers: Footers { source: "".to_string() },
      data_types: vec![
        DataType {
          name: "DataType1".to_string(),
          description: None,
          data: DataTypeData::Array(Array {
            data_type: "int".to_string(),
            constraints: vec![],
          }),
        },
        DataType {
          name: "DataType2".to_string(),
          description: None,
          data: DataTypeData::Array(Array {
            data_type: "float".to_string(),
            constraints: vec![],
          }),
        },
      ],
      namespace: vec!["test".to_string()],
    };

    // Create the header file
    let header_file = data_model.get_source("HEADER", 2);
    let expected = include_str!("../../tests/cpp/type_array/basic/basic.hpp");

    // Check that they are the same
    assert_eq!(str_diff(&header_file, &expected), None);
  }
}
