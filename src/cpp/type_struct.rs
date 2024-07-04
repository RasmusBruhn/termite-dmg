use indoc::formatdoc;
use crate::DefaultType;
use super::*;

impl Struct {
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
      .join(", ");

    // Get the constructor validation
    let constructor_validators = self.fields.iter()
      .map(|field| field.get_constructor_validator(indent))
      .collect::<Vec<String>>()
      .join("\n\n");
    let constructor_validators = if constructor_validators.is_empty() {
      "".to_string()
    } else {
      format!("termite::Result<termite::Empty> validate_result = termite::Result<termite::Empty>::ok(termite::Empty());\n\n{constructor_validators}")
    };

    // Get the move parameter list for the constructor
    let constructor_move_parameters = self.fields.iter()
      .map(|field| field.get_constructor_move_parameter())
      .collect::<Vec<String>>()
      .join(", ");

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

    // Get the name for "other" parameter which is gone if there are no fields
    let other_name = if self.fields.is_empty() {
      "".to_string()
    } else {
      "x".to_string()
    };

    // Get the equality test
    let equality_test = self.fields.iter()
      .map(|field| field.get_equality_check())
      .collect::<Vec<String>>()
      .join(" && ");
    let equality_test = if equality_test.is_empty() {
      "true".to_string()
    } else {
      equality_test
    };

    // Get the printout for the operator<< function
    let printout = self.fields.iter()
      .map(|field| field.get_printout())
      .collect::<Vec<String>>()
      .join(" << \", \" << ");
    let printout = if printout.is_empty() {
      "\"\"".to_string()
    } else {
      printout
    };

    // Get the list of parameters for the internal constructor
    let internal_parameters = self.fields.iter()
      .map(|field| field.get_internal_parameter())
      .collect::<Vec<String>>()
      .join(", ");

    // Get the list of setters for the internal constructor
    let internal_setters = self.fields.iter()
      .map(|field| field.get_internal_setter())
      .collect::<Vec<String>>()
      .join(", ");
    let internal_setters = if internal_setters.is_empty() {
      "".to_string()
    } else {
      format!(" : {internal_setters}")
    };

    // Get the validation functions
    let validation_functions = self.fields.iter()
      .map(|field| field.get_validation(indent))
      .collect::<Vec<String>>()
      .join("\n");

    // Get the definitions of all the fields but without any initialization
    let field_definitions = self.fields.iter()
      .map(|field| field.get_definition(indent))
      .collect::<Vec<String>>()
      .join("\n");

    // Generate the code
    return formatdoc!("
      class {name} {{
      public:
      {0:indent$}/**
      {0:indent$} * @brief Constructs a new {name} object 
      {0:indent$} * 
      {0:indent$} * {constructor_description}
      {0:indent$} */
      {0:indent$}[[nodiscard]] static termite::Result<{name}> from_values({constructor_parameters}) {{
      {0:indent$}{0:indent$}{constructor_validators}

      {0:indent$}{0:indent$}return termite::Result<{name}>::ok({name}({constructor_move_parameters}));
      {0:indent$}}}

      {setter_methods}

      {getter_methods}

      {0:indent$}/**
      {0:indent$} * @brief Checks if this object the the other object are identical
      {0:indent$} * 
      {0:indent$} * @param {other_name} The other object to compare with
      {0:indent$} * @return true if they are identical, false if not
      {0:indent$} */
      {0:indent$}[[nodiscard]] bool operator==(const {name} &{other_name}) {{
      {0:indent$}{0:indent$}return {equality_test};
      {0:indent$}}}
      {0:indent$}/**
      {0:indent$} * @brief Checks if this object the the other object are different
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
      {0:indent$} * @param {other_name} The object to print
      {0:indent$} * @return The output stream
      {0:indent$} */
      {0:indent$}friend std::ostream &operator<<(std::ostream &os, const {name} &{other_name}) {{
      {0:indent$}{0:indent$}return os << \"{{ \" << {printout} << \" }}\";
      {0:indent$}}}

      private:
      {0:indent$}explicit {name}({internal_parameters}){internal_setters} {{}}

      {validation_functions}

      {field_definitions}
      }};", "",
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
      "{typename} {name}{default}",
      typename = self.get_typename(),
      name = self.name,
      default = self.get_default(),
    );
  }

  /// Get the validation code for the constructor
  /// 
  /// # Parameters
  /// 
  /// indent: The number of spaces to indent
  fn get_constructor_validator(&self, indent: usize) -> String {
    return formatdoc!("
      {0:indent$}{0:indent$}validate_result = validate_{name}({name});
      {0:indent$}{0:indent$}if (!validate_result.is_ok()) {{
      {0:indent$}{0:indent$}{0:indent$}termite::Error error = validate_result.get_err();
      {0:indent$}{0:indent$}{0:indent$}error.add_field(\"{name}\");
      {0:indent$}{0:indent$}{0:indent$}return termite::Result<DataType>::err(std::move(error));
      {0:indent$}{0:indent$}}}",
      "",
      name = self.name,
    );
  }

  /// Gets the parameter move definition for finishing up the constructor
  fn get_constructor_move_parameter(&self) -> String {
    return format!(
      "std::move({name})",
      name = self.name
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
      {0:indent$} * @brief Sets the value of {name} if it fulfills the constraints:{description}
      {0:indent$} * 
      {0:indent$} * @param value The value of {name}
      {0:indent$} * @return An error if one of the constraints were not fulfilled
      {0:indent$} */
      {0:indent$}[[nodiscard]] termite::Result<termite::Empty> set_{name}({typename} value) {{
      {0:indent$}{0:indent$}termite::Result<termite::Empty> validate_result = validate_{name}(value);
      {0:indent$}{0:indent$}if (!validate_result.is_ok()) {{
      {0:indent$}{0:indent$}{0:indent$}return validate_result;
      {0:indent$}{0:indent$}}}

      {0:indent$}{0:indent$}{name}_ = std::move(value);
      {0:indent$}{0:indent$}return termite::Result<termite::Empty>::ok(termite::Empty());
      {0:indent$}}}",
      "", 
      name = self.name, 
      typename = self.get_typename(),
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
      {0:indent$} * @brief Retrieves a reference to the value of {name}
      {0:indent$} * 
      {0:indent$} * @return The reference 
      {0:indent$} */
      {0:indent$}[[nodiscard]] const {typename} &get_{name}() const {{
      {0:indent$}{0:indent$}return {name}_;
      {0:indent$}}}",
      "", 
      name = self.name, 
      typename = self.get_typename(),
    );
  }

  /// Gets the equality check for this field
  fn get_equality_check(&self) -> String {
    return format!(
      "{name}_ == x.{name}_",
      name = self.name,
    );
  }

  /// Gets the printout of this field for the operator>> ostream function
  fn get_printout(&self) -> String {
    return format!(
      "\"{name}: \" << x.{name}_",
      name = self.name,
    );
  }

  /// Gets the parameter for the internal constructor
  fn get_internal_parameter(&self) -> String {
    return format!(
      "{typename} {name}",
      typename = self.get_typename(),
      name = self.name,
    );
  }

  /// Get the setter for this field for the internal constructor
  fn get_internal_setter(&self) -> String {
    return format!(
      "{name}_(std::move({name}))",
      name = self.name
    );
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
        {0:indent$}{0:indent$}if (!({constraint})) {{
        {0:indent$}{0:indent$}{0:indent$}return termite::Result<termite::Empty>::err(termite::Error(\"{name} did not pass constaint: {constraint}\"));
        {0:indent$}{0:indent$}}}",
        "",
        name = self.name
        ))
      .collect::<Vec<String>>()
      .join("\n\n");

    let param_name = if self.constraints.is_empty() {
      "".to_string()
    } else {
      "x".to_string()
    };
    
    return formatdoc!("
      {0:indent$}/**
      {0:indent$} * @brief Validates if {name} is correct using the following constaints:{description}
      {0:indent$} * 
      {0:indent$} * @param {param_name} The value of the parameter to validate
      {0:indent$} */
      {0:indent$}[[nodiscard]] static termite::Result<termite::Empty> validate_{name}(const {typename} &{param_name}) {{
      {tests}

      {0:indent$}{0:indent$}return termite::Result<termite::Empty>::ok(termite::Empty());
      {0:indent$}}}",
      "", 
      name = self.name, 
      typename = self.get_typename(),
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
      {0:indent$}{typename} {name}_;",
      "",
      typename = self.get_typename(),
      name = self.name,
      description = self.get_description(),
    )
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
      };

      // Create the header file
      let header_file = data_model.get_source("HEADER", 2);
      let expected = include_str!("../../tests/cpp/type_struct/field/optional/optional.hpp");

      // Check that they are the same
      assert_eq!(str_diff(&header_file, &expected), None); 
    }

    #[test]
    fn constraints() {
      // Check c++ code
      compile_and_test("type_struct/field/constraints");

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
                  constraints: vec![
                    "x > 0".to_string(),
                    "x % 2 == 0".to_string(),
                  ],
                },
                StructField {
                  name: "field2".to_string(),
                  description: None,
                  data_type: "float".to_string(),
                  default: DefaultType::Required,
                  constraints: vec![
                    "std::abs(x) < 1e-9".to_string(),
                  ],
                },
              ] 
            }),
          },
        ],
      };

      // Create the header file
      let header_file = data_model.get_source("HEADER", 2);
      let expected = include_str!("../../tests/cpp/type_struct/field/constraints/constraints.hpp");

      // Check that they are the same
      assert_eq!(str_diff(&header_file, &expected), None); 
    }
  }

}