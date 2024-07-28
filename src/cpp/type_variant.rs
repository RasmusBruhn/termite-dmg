use std::char::{
  ToLowercase, 
  ToUppercase
};

use indoc::formatdoc;
use super::*;

/// Iterator to convert an iterator of chars to snake case converting all
/// uppercase characters to an underscore and the lowercase character
struct ToSnakeCase<'a> {
  /// The characters to convert to snake case
  chars: &'a mut dyn Iterator<Item = char>,
  /// The characters currently being converted to lowercase
  set_lower: Option<ToLowercase>,
}

impl<'a> ToSnakeCase<'a> {
  /// Creates a new ToSnakeCase object
  /// 
  /// # Parameters
  /// 
  /// chars: The iterator of the characters to convert
  fn new(chars: &'a mut dyn Iterator<Item = char>) -> Self {
    // Make sure the first character is lowercase without an underscore
    let set_lower = if let Some(first_char) = chars.next() {
      Some(first_char.to_lowercase())
    } else {
      None
    };

    return Self {
      chars,
      set_lower,
    }
  }
}

impl<'a> Iterator for ToSnakeCase<'a> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
      // Set to lower case
      if let Some(set_lower) = &mut self.set_lower {
        // Get the next character
        if let Some(next_char) = set_lower.next() {
          return Some(next_char);
        }

        // Finish up setting to lowercase
        self.set_lower = None;
      }

      // Get next character
      return if let Some(next_char) = self.chars.next() {
        // Set to lowercase if it is uppercase
        if next_char.is_uppercase() {
          self.set_lower = Some(next_char.to_lowercase());
          Some('_')
        } else {
          Some(next_char)
        }
      } else {
        None
      }
  }
}

/// Iterator to convert an iterator of chars to camel case converting all
/// underscores and a character to the uppercase character
struct ToCamelCase<'a> {
  /// The characters to convert to camel case
  chars: &'a mut dyn Iterator<Item = char>,
  /// The characters currently being converted to uppercase
  set_upper: Option<ToUppercase>,
}

impl<'a> ToCamelCase<'a> {
  /// Creates a new ToCamelCase object
  /// 
  /// # Parameters
  /// 
  /// chars: The iterator of the characters to convert
  fn new(chars: &'a mut dyn Iterator<Item = char>) -> Self {
    // Make sure the first character is uppercase without an underscore
    let set_upper = if let Some(first_char) = chars.next() {
      Some(first_char.to_uppercase())
    } else {
      None
    };

    return Self {
      chars,
      set_upper,
    }
  }
}

impl<'a> Iterator for ToCamelCase<'a> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
      // Set to uppercase
      if let Some(set_upper) = &mut self.set_upper {
        // Get the next character
        if let Some(next_char) = set_upper.next() {
          return Some(next_char);
        }

        // Finish up setting to uppercase
        self.set_upper = None;
      }

      // Get next character
      return if let Some(next_char) = self.chars.next() {
        // Set to uppercase if it is underscore
        if next_char == '_' {
          if let Some(to_upper) = self.chars.next() {
            let mut uppercase = to_upper.to_uppercase();
            let result = uppercase.next();
            self.set_upper = Some(uppercase);
            result
          } else {
            None
          }
        } else {
          Some(next_char)
        }
      } else {
        None
      }
  }
}

/// The type specific information for a variant
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Variant {
  /// The possible types for the variant
  pub(super) data_types: Vec<String>,
}

impl Variant {
  /// Constructs a new c++ variant from a generic variant
  /// 
  /// # Parameters
  /// 
  /// data: The generic variant to convert
  pub(super) fn new(data: crate::Variant) -> Result<Self, Error> {
    return Ok(Self {
      data_types: data.data_types,
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
    // Get the variants enum
    let variant_enum = self.data_types.iter()
      .enumerate()
      .map(|(index, data_type)| format!(
        "{0:indent$}{0:indent$}k{type_name} = {index},", 
        "", 
        type_name = ToCamelCase::new(&mut data_type.chars()).collect::<String>()
      ))
      .collect::<Vec<String>>()
      .join("\n");

    // Create list of the variants
    let variant_list = self.data_types.join(", ");

    // Get snake case naming
    let snake_case_data_types = self.data_types.iter()
      .map(|data_type| ToSnakeCase::new(&mut data_type.chars()).collect::<String>())
      .collect::<Vec<String>>();

    // Create the setters
    let setters = self.data_types.iter()
      .zip(snake_case_data_types.iter())
      .map(|(data_type, snake_case)| formatdoc!{"
        {0:indent$}/**
        {0:indent$} * @brief Sets the value as a {data_type}
        {0:indent$} * 
        {0:indent$} * @param value The value to set
        {0:indent$} */
        {0:indent$}void set_{snake_case}({data_type} value) {{
        {0:indent$}{0:indent$}value_ = std::move(value);
        {0:indent$}}}",
        "",
      })
      .collect::<Vec<String>>()
      .join("\n");

    let getters = self.data_types.iter()
      .zip(snake_case_data_types.iter())
      .map(|(data_type, snake_case)| formatdoc!("
        {0:indent$}/**
        {0:indent$} * @brief Retrieves a reference to the value as a {data_type}
        {0:indent$} * 
        {0:indent$} * @return The value or an error if it is the wrong type
        {0:indent$} */
        {0:indent$}[[nodiscard]] termite::Result<termite::Reference<{data_type}>> get_{snake_case}() {{
        {0:indent$}{0:indent$}if (!std::holds_alternative<{data_type}>(value_)) {{
        {0:indent$}{0:indent$}{0:indent$}return termite::Result<termite::Reference<{data_type}>>::err(termite::Error(\"Value is not a {data_type}\"));
        {0:indent$}{0:indent$}}}

        {0:indent$}{0:indent$}return termite::Result<termite::Reference<{data_type}>>::ok(termite::Reference(std::get<{data_type}>(std::move(value_))));
        {0:indent$}}}",
        "",
      ))
      .collect::<Vec<String>>()
      .join("\n");

    let writer_specifiers = self.data_types.iter()
      .enumerate()
      .map(|(index, data_type)| formatdoc!("
        {0:indent$}{0:indent$}case {index}:
        {0:indent$}{0:indent$}{0:indent$}os << \"{data_type} \" << std::get<{data_type}>(x.value_);
        {0:indent$}{0:indent$}{0:indent$}break;",
        "",
      ))
      .collect::<Vec<String>>()
      .join("\n");

    return formatdoc!("
      class {name} {{
      public:
      {0:indent$}/**
      {0:indent$} * @brief The types of variants
      {0:indent$} * 
      {0:indent$} */
      {0:indent$}enum Variant {{
      {variant_enum}
      {0:indent$}}};

      {0:indent$}/**
      {0:indent$} * @brief Constructs a new {name} object
      {0:indent$} * 
      {0:indent$} * @param value The value of the variant
      {0:indent$} * @return The new variant
      {0:indent$} */
      {0:indent$}[[nodiscard]] static {name} from_values(std::variant<{variant_list}> value) {{
      {0:indent$}{0:indent$}return {name}(std::move(value));
      {0:indent$}}}

      {setters}
      {0:indent$}/**
      {0:indent$} * @brief Sets the value
      {0:indent$} * 
      {0:indent$} * @param value The value to set
      {0:indent$} */
      {0:indent$}void set_value(std::variant<{variant_list}> value) {{
      {0:indent$}{0:indent$}value_ = std::move(value);
      {0:indent$}}}

      {0:indent$}/**
      {0:indent$} * @brief Retrieves the type of variant stored
      {0:indent$} * 
      {0:indent$} * @return The type of variant
      {0:indent$} */
      {0:indent$}[[nodiscard]] Variant variant() const {{
      {0:indent$}{0:indent$}return static_cast<Variant>(value_.index());
      {0:indent$}}}
      {getters}
      {0:indent$}/**
      {0:indent$} * @brief Retrieves a reference to the value
      {0:indent$} * 
      {0:indent$} * @return The reference or an error if it is the wrong type
      {0:indent$} */
      {0:indent$}[[nodiscard]] const std::variant<int, float> &get_value() const {{
      {0:indent$}{0:indent$}return value_;
      {0:indent$}}}

      {0:indent$}/**
      {0:indent$} * @brief Checks if this object and the other object are identical
      {0:indent$} * 
      {0:indent$} * @param x The other object to compare with
      {0:indent$} * @return true if they are identical, false if not
      {0:indent$} */
      {0:indent$}[[nodiscard]] bool operator==(const DataType &x) {{
      {0:indent$}{0:indent$}return value_ == x.value_;
      {0:indent$}}}
      {0:indent$}/**
      {0:indent$} * @brief Checks if this object and the other object are different
      {0:indent$} * 
      {0:indent$} * @param x The other object to compare with
      {0:indent$} * @return true if they are different, false if not
      {0:indent$} */
      {0:indent$}[[nodiscard]] bool operator!=(const DataType &x) {{
      {0:indent$}{0:indent$}return !(*this == x);
      {0:indent$}}}
      {0:indent$}/**
      {0:indent$} * @brief Prints the object onto the output stream
      {0:indent$} * 
      {0:indent$} * @param os The output stream to print to
      {0:indent$} * @param x The object to print
      {0:indent$} * @return The output stream
      {0:indent$} */
      {0:indent$}friend std::ostream &operator<<(std::ostream &os, const DataType &x) {{
      {0:indent$}{0:indent$}os << \"{{ value: \";
      {0:indent$}{0:indent$}switch (x.value_.index()) {{
      {writer_specifiers}
      {0:indent$}{0:indent$}default:
      {0:indent$}{0:indent$}{0:indent$}os << \"Unknown\";
      {0:indent$}{0:indent$}{0:indent$}break;
      {0:indent$}{0:indent$}}}
      {0:indent$}{0:indent$}return os << \" }}\";
      {0:indent$}}}

      private:
      {0:indent$}explicit {name}(std::variant<{variant_list}> value) : value_(std::move(value)) {{}}

      {0:indent$}/**
      {0:indent$} * @brief The value of the variant
      {0:indent$} * 
      {0:indent$} */
      {0:indent$}std::variant<{variant_list}> value_;
      }};",
      "",
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

    // Get snake case naming
    let snake_case_data_types = self.data_types.iter()
      .map(|data_type| ToSnakeCase::new(&mut data_type.chars()).collect::<String>())
      .collect::<Vec<String>>();

    // Get all the readers
    let readers = self.data_types.iter()
      .zip(snake_case_data_types.iter())
      .map(|(data_type, snake_case)| formatdoc!("
        {0:indent$}Result<{data_type}> result_{snake_case} = to_value<{data_type}>(allow_skipping);
        {0:indent$}if (result_{snake_case}.is_ok()) {{
        {0:indent$}{0:indent$}return Result<{typename}>::ok({typename}::from_values(result_{snake_case}.get_ok()));
        {0:indent$}}}
        {0:indent$}error << \"{data_type} {{ \" << result_{snake_case}.get_err() << \" }}\";",
      "",
      ))
      .collect::<Vec<String>>()
      .join(&formatdoc!("
          
          {0:indent$}error << \", \";

        ",
        "",
      ));

    return formatdoc!("
      template<>
      [[nodiscard]] Result<{typename}> Node::to_value(bool allow_skipping) const {{
      {0:indent$}std::stringstream error;
      {0:indent$}error << \"Unable to parse any variant: [ \";

      {readers}
      {0:indent$}error << \" ]\";

      {0:indent$}return Result<{typename}>::err(Error(error.str()));
      }}",
      "",
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
    compile_and_test("type_variant/basic");

    // Make sure it generates the correct code
    let data_model = DataModel {
      headers: Headers { source: "".to_string() },
      footers: Footers { source: "".to_string() },
      data_types: vec![
        DataType {
          name: "DataType".to_string(),
          description: None,
          data: DataTypeData::Variant(Variant {
            data_types: vec![
              "int".to_string(),
              "float".to_string(),
            ],
          }),
        },
      ],
      namespace: vec!["test".to_string()],
    };

    // Create the header file
    let header_file = data_model.get_source("HEADER", 2);
    let expected = include_str!("../../tests/cpp/type_variant/basic/basic.hpp");

    // Check that they are the same
    assert_eq!(str_diff(&header_file, &expected), None);
  }
}
