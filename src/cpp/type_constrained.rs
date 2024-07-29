use indoc::formatdoc;
use super::*;

/// The type specific information for a constrained type
#[derive(Clone, Debug, PartialEq)]
pub(super) struct ConstrainedType {
  /// The type that is constrained
  pub(super) data_type: String,
  /// All extra constraints for the type
  pub(super) constraints: Vec<String>,
}

impl ConstrainedType {
  /// Constructs a new c++ variant from a generic variant
  /// 
  /// # Parameters
  /// 
  /// data: The generic variant to convert
  pub(super) fn new(data: crate::ConstrainedType) -> Result<Self, Error> {
    return Ok(Self {
      data_type: data.data_type,
      constraints: data.constraints,
    });
  }

  /// Converts the variant to a string for use in the header file
  /// 
  /// # Parameters
  /// 
  /// name: The name of the variant
  /// 
  /// indent: The number of spaces to use for indentation
  pub(super) fn get_source(&self, name: &str, indent: usize) -> String {
    // Create the constraints description
    let constraints = self.constraints.iter()
      .map(|constraint| {
        return format!("\n{0:indent$} * - {constraint}", "");
      })
      .collect::<Vec<String>>()
      .join("");

    // Create the tests
    let tests = self.constraints.iter()
      .map(|constraint| formatdoc!("
        {0:indent$}{0:indent$}if (!({constraint})) {{
        {0:indent$}{0:indent$}{0:indent$}return termite::Result<termite::Empty>::err(termite::Error(\"Did not pass constaint: {constraint}\"));
        {0:indent$}{0:indent$}}}",
        "",
      ))
      .collect::<Vec<String>>()
      .join("\n\n");

    // The name of the validation parameter, should not exist if there are no constraints
    let param_name = if self.constraints.is_empty() {
      "".to_string()
    } else {
      "x".to_string()
    };

    return formatdoc!("
      class {name} {{
      public:
      {0:indent$}/**
      {0:indent$} * @brief Constructs a new {name} object
      {0:indent$} * 
      {0:indent$} * @param value The value to store 
      {0:indent$} * @return The new constrained type or an error if some constraints were not upheld
      {0:indent$} */
      {0:indent$}[[nodiscard]] static termite::Result<{name}> from_values({data_type} value) {{
      {0:indent$}{0:indent$}termite::Result<termite::Empty> validate_result = validate(value);
      {0:indent$}{0:indent$}if (!validate_result.is_ok()) {{
      {0:indent$}{0:indent$}{0:indent$}termite::Error error = validate_result.get_err();
      {0:indent$}{0:indent$}{0:indent$}return termite::Result<{name}>::err(std::move(error));
      {0:indent$}{0:indent$}}}

      {0:indent$}{0:indent$}return termite::Result<{name}>::ok({name}(std::move(value)));
      {0:indent$}}}

      {0:indent$}/**
      {0:indent$} * @brief Sets the value if it fulfills the constraints:{constraints}
      {0:indent$} * 
      {0:indent$} * @param value The value to set
      {0:indent$} * @return An error if one of the constraints were not fulfilled
      {0:indent$} */
      {0:indent$}[[nodiscard]] termite::Result<termite::Empty> set({data_type} value) {{
      {0:indent$}{0:indent$}termite::Result<termite::Empty> validate_result = validate(value);
      {0:indent$}{0:indent$}if (!validate_result.is_ok()) {{
      {0:indent$}{0:indent$}{0:indent$}return validate_result;
      {0:indent$}{0:indent$}}}

      {0:indent$}{0:indent$}value_ = std::move(value);
      {0:indent$}{0:indent$}return termite::Result<termite::Empty>::ok(termite::Empty());
      {0:indent$}}}

      {0:indent$}/**
      {0:indent$} * @brief Retrieves a reference to the value
      {0:indent$} * 
      {0:indent$} * @return The reference
      {0:indent$} */
      {0:indent$}[[nodiscard]] const {data_type} &get() const {{
      {0:indent$}{0:indent$}return value_;
      {0:indent$}}}

      {0:indent$}/**
      {0:indent$} * @brief Checks if this object and the other object are identical
      {0:indent$} * 
      {0:indent$} * @param x The other object to compare with
      {0:indent$} * @return true if they are identical, false if not
      {0:indent$} */
      {0:indent$}[[nodiscard]] bool operator==(const {name} &x) {{
      {0:indent$}{0:indent$}return value_ == x.value_;
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
      {0:indent$}{0:indent$}return os << x.value_;
      {0:indent$}}}

      private:
      {0:indent$}explicit {name}({data_type} value) : value_(std::move(value)) {{}}

      {0:indent$}/**
      {0:indent$} * @brief Validates if value is correct using the following constaints:{constraints}
      {0:indent$} * 
      {0:indent$} * @param {param_name} The value of the parameter to validate
      {0:indent$} */
      {0:indent$}[[nodiscard]] static termite::Result<termite::Empty> validate(const {data_type} &{param_name}) {{
      {tests}

      {0:indent$}{0:indent$}return termite::Result<termite::Empty>::ok(termite::Empty());
      {0:indent$}}}

      {0:indent$}/**
      {0:indent$} * @brief The value of the int
      {0:indent$} * 
      {0:indent$} */
      {0:indent$}{data_type} value_;
      }};",
      "",
      data_type = self.data_type,
    );
  }

  /// Gets the source code for the parser for this variant allowing it to be read from a file
  /// 
  /// # Parameters
  /// 
  /// name: The name of the variant
  /// 
  /// indent: The number of spaces to use for indentation
  /// 
  /// namespace: The namespace of the variant
  pub(super) fn get_parser(&self, name: &str, indent: usize, namespace: &[String]) -> String {
    // Get the namespace name
    let namespace = namespace.iter()
      .map(|single_name| format!("{single_name}::"))
      .collect::<Vec<String>>()
      .join("");
    let typename = format!("{namespace}{name}");



    return formatdoc!("
      template<>
      [[nodiscard]] Result<{typename}> Node::to_value(bool allow_skipping) const {{
      {0:indent$}Result<{data_type}> value = to_value<{data_type}>(allow_skipping);
      {0:indent$}if (!value.is_ok()) {{
      {0:indent$}{0:indent$}return Result<{typename}>::err(Error(value.get_err()));
      {0:indent$}}}

      {0:indent$}return {typename}::from_values(value.get_ok());
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
    compile_and_test("type_constrained/basic");

    // Make sure it generates the correct code
    let data_model = DataModel {
      headers: Headers { source: "".to_string() },
      footers: Footers { source: "".to_string() },
      data_types: vec![
        DataType {
          name: "DataType1".to_string(),
          description: None,
          data: DataTypeData::ConstrainedType(ConstrainedType {
            data_type: "int".to_string(),
            constraints: vec![],
          }),
        },
        DataType {
          name: "DataType2".to_string(),
          description: None,
          data: DataTypeData::ConstrainedType(ConstrainedType {
            data_type: "float".to_string(),
            constraints: vec![],
          }),
        },
      ],
      namespace: vec!["test".to_string()],
    };

    // Create the header file
    let header_file = data_model.get_source("HEADER", 2);
    let expected = include_str!("../../tests/cpp/type_constrained/basic/basic.hpp");

    // Check that they are the same
    assert_eq!(str_diff(&header_file, &expected), None);
  }

  #[test]
  fn constraints() {
    // Check c++ code
    compile_and_test("type_constrained/constraints");

    // Make sure it generates the correct code
    let data_model = DataModel {
      headers: Headers { source: "".to_string() },
      footers: Footers { source: "".to_string() },
      data_types: vec![
        DataType {
          name: "DataType1".to_string(),
          description: None,
          data: DataTypeData::ConstrainedType(ConstrainedType {
            data_type: "int".to_string(),
            constraints: vec![
              "x > 0".to_string(),
              "x % 2 == 0".to_string(),
            ],
          }),
        },
        DataType {
          name: "DataType2".to_string(),
          description: None,
          data: DataTypeData::ConstrainedType(ConstrainedType {
            data_type: "float".to_string(),
            constraints: vec![
              "std::abs(x) < 1e-9".to_string(),
            ],
          }),
        },
      ],
      namespace: vec!["test".to_string()],
    };

    // Create the header file
    let header_file = data_model.get_source("HEADER", 2);
    let expected = include_str!("../../tests/cpp/type_constrained/constraints/constraints.hpp");

    // Check that they are the same
    assert_eq!(str_diff(&header_file, &expected), None);
  }
}
