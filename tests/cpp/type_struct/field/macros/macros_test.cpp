#include "generated/macros.h"

#include <iostream>
#include <sstream>

/**
 * @brief Checks that structs with default values are equal to themselves
 *
 * @return An error string on error
 */
std::optional<std::string> test_eq_self() {
  std::cout << "TEST-1" << std::endl;
  auto test1 = test::DataType::default_field1();
  std::cout << "TEST-1.1" << std::endl;
  auto test2 = test::DataType::default_field2();
  std::cout << "TEST-1.2" << std::endl;
  auto value1 = test::DataType(test::DataType::default_field1(),
                               test::DataType::default_field2());
  std::cout << "TEST-2" << std::endl;
  if (value1 != value1) {
    return "A struct was not equal to itself";
  }
  std::cout << "TEST-3" << std::endl;

  auto value2 = test::DataType(-2, 3.5);
  std::cout << "TEST-4" << std::endl;
  if (value2 != value2) {
    return "A struct was not equal to itself";
  }
  std::cout << "TEST-5" << std::endl;
  return std::nullopt;
}

/**
 * @brief Checks that different structs are not equal
 *
 * @return An error string on error
 */
std::optional<std::string> test_eq_diff() {
  auto value1 = test::DataType(test::DataType::default_field1(),
                               test::DataType::default_field2());
  auto value2 = test::DataType(-2, 3.5);
  if (value1 == value2) {
    std::stringstream ss;
    ss << "Two different structs were equal: " << value1 << " vs " << value2;
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Checks that structs can be loaded from empty maps using defaults
 *
 * @return An error string on error
 */
std::optional<std::string> test_load_defaults() {
  auto value1 = test::DataType(test::DataType::default_field1(),
                               test::DataType::default_field2());
  std::map<std::string, termite::Node> map_correct1;
  termite::Node node_correct1(termite::Node::Map(std::move(map_correct1)));
  auto value1_read_correct = node_correct1.to_value<test::DataType>();
  if (!value1_read_correct.is_ok()) {
    std::stringstream ss;
    ss << "Unable to convert empty map to struct with defaults: "
       << value1_read_correct.get_err();
    return ss.str();
  }
  auto read_val1 = value1_read_correct.get_ok();
  if (read_val1 != value1) {
    std::stringstream ss;
    ss << "Failed to convert empty map to struct with defaults: expected "
       << value1 << ", got " << read_val1;
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Checks that structs can be loaded from maps with all fields
 *
 * @return An error string on error
 */
std::optional<std::string> test_load() {
  auto value2 = test::DataType(-2, 3.5);
  std::map<std::string, termite::Node> map_correct2;
  map_correct2.insert({"field1", termite::Node(termite::Node::Value("-2"))});
  map_correct2.insert({"field2", termite::Node(termite::Node::Value("3.5"))});
  termite::Node node_correct2(termite::Node::Map(std::move(map_correct2)));
  auto value2_read_correct = node_correct2.to_value<test::DataType>();
  if (!value2_read_correct.is_ok()) {
    std::stringstream ss;
    ss << "Unable to convert map to struct with all fields: "
       << value2_read_correct.get_err();
    return ss.str();
  }
  auto read_val1 = value2_read_correct.get_ok();
  if (read_val1 != value2) {
    std::stringstream ss;
    ss << "Failed to convert map to struct with all fields: expected " << value2
       << ", got " << read_val1;
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Checks that structs can be loaded with extra fields
 *
 * @return An error string on error
 */
std::optional<std::string> test_load_extra_fields() {
  auto value1 = test::DataType(test::DataType::default_field1(),
                               test::DataType::default_field2());
  std::map<std::string, termite::Node> map_many;
  map_many.insert({"field3", termite::Node(termite::Node::Value("test"))});
  termite::Node node_many(termite::Node::Map(std::move(map_many)));
  auto value1_read_many = node_many.to_value<test::DataType>();
  if (!value1_read_many.is_ok()) {
    std::stringstream ss;
    ss << "Unable to convert map with extra fields to struct: "
       << value1_read_many.get_err();
    return ss.str();
  }
  if (value1_read_many.get_ok() == value1) {
    return "Struct with extra fields was equal to struct without extra fields";
  }
  return std::nullopt;
}

/**
 * @brief Checks that structs cannot be loaded from maps with invalid field
 * types
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_load_invalid_type() {
  std::map<std::string, termite::Node> map_type;
  map_type.insert({"field1", termite::Node(termite::Node::Value("1.0"))});
  map_type.insert({"field2", termite::Node(termite::Node::Value("5.0"))});
  termite::Node node_type(termite::Node::Map(std::move(map_type)));
  auto value_wrong_type = node_type.to_value<test::DataType>();
  if (value_wrong_type.is_ok()) {
    return "A struct was constructed from map with invalid field type";
  }
  return std::nullopt;
}

/**
 * @brief Checks that structs cannot be loaded from invalid node types
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_load() {
  termite::Node node_wrong(termite::Node::Value("1.0"));
  auto value_wrong_wrong = node_wrong.to_value<test::DataType>();
  if (value_wrong_wrong.is_ok()) {
    return "A struct was constructed from invalid node type";
  }
  return std::nullopt;
}

/**
 * @brief Checks that structs can be saved to nodes and reloaded
 *
 * @return An error string on error
 */
std::optional<std::string> test_reload() {
  auto value1 = test::DataType(test::DataType::default_field1(),
                               test::DataType::default_field2());
  termite::Node converted_node1 = termite::Node::from_value(value1);
  auto converted_value1 = converted_node1.to_value<test::DataType>();
  if (!converted_value1.is_ok()) {
    std::stringstream ss;
    ss << "Unable to reload struct: " << converted_value1.get_err();
    return ss.str();
  }
  auto read_val1 = converted_value1.get_ok();
  if (read_val1 != value1) {
    std::stringstream ss;
    ss << "Failed to reload struct: expected " << value1 << ", got "
       << read_val1;
    return ss.str();
  }

  auto value2 = test::DataType(-2, 3.5);
  termite::Node converted_node2 = termite::Node::from_value(value2);
  auto converted_value2 = converted_node2.to_value<test::DataType>();
  if (!converted_value2.is_ok()) {
    std::stringstream ss;
    ss << "Unable to reload struct: " << converted_value2.get_err();
    return ss.str();
  }
  auto read_val2 = converted_value2.get_ok();
  if (read_val2 != value2) {
    std::stringstream ss;
    ss << "Failed to reload struct: expected " << value2 << ", got "
       << read_val2;
    return ss.str();
  }
  return std::nullopt;
}

int main() {
  auto names = {
      "test_eq_self",           "test_eq_diff",
      "test_load_defaults",     "test_load",
      "test_load_extra_fields", "test_error_load_invalid_type",
      "test_error_load",        "test_reload",
  };
  auto functions = {
      test_eq_self,           test_eq_diff,
      test_load_defaults,     test_load,
      test_load_extra_fields, test_error_load_invalid_type,
      test_error_load,        test_reload,
  };

  std::cout << "Running " << names.size() << " tests" << std::endl;

  int progress = 1;
  auto name_it = names.begin();
  for (auto function_it = functions.begin(); function_it < functions.end();
       ++function_it, ++name_it, ++progress) {
        std::cout << "TEST: " << *name_it << std::endl;
    if (auto error = (*function_it)()) {
      std::cout << "Error occured at \"" << *name_it << "\": " << *error
                << std::endl;
      return progress;
    }
  }

  std::cout << "No errors" << std::endl;

  return 0;
}