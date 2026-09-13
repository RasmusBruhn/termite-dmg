#include "generated/basic.h"

#include <iostream>
#include <sstream>

/**
 * @brief Checks that an enum value is equal to itself
 *
 * @return An error string on error
 */
std::optional<std::string> test_eq_self() {
  test::DataType value1 = test::DataType::TypeInt1{1};
  if (value1 != value1) {
    return "An enum was not equal to itself";
  }

  test::DataType value2 = test::DataType::TypeInt2{1};
  if (value2 != value2) {
    return "An enum was not equal to itself";
  }

  test::DataType value3 = test::DataType::TypeFloat{3.5};
  if (value3 != value3) {
    return "An enum was not equal to itself";
  }

  test::DataType value4 = test::DataType::TypeEmpty{};
  if (value4 != value4) {
    return "An enum was not equal to itself";
  }

  return std::nullopt;
}

/**
 * @brief Checks that different enum values are not equal
 *
 * @return An error string on error
 */
std::optional<std::string> test_eq_diff() {
  test::DataType value1 = test::DataType::TypeInt1{1};
  test::DataType value2 = test::DataType::TypeInt2{1};
  test::DataType value3 = test::DataType::TypeFloat{3.5};
  test::DataType value4 = test::DataType::TypeEmpty{};

  if (value1 == value2) {
    std::stringstream ss;
    ss << "Two different enum values were equal: " << value1 << " vs "
       << value2;
    return ss.str();
  }
  if (value1 == value3) {
    std::stringstream ss;
    ss << "Two different enum values were equal: " << value1 << " vs "
       << value3;
    return ss.str();
  }
  if (value1 == value4) {
    std::stringstream ss;
    ss << "Two different enum values were equal: " << value1 << " vs "
       << value4;
    return ss.str();
  }
  if (value2 == value3) {
    std::stringstream ss;
    ss << "Two different enum values were equal: " << value2 << " vs "
       << value3;
    return ss.str();
  }
  if (value2 == value4) {
    std::stringstream ss;
    ss << "Two different enum values were equal: " << value2 << " vs "
       << value4;
    return ss.str();
  }
  if (value3 == value4) {
    std::stringstream ss;
    ss << "Two different enum values were equal: " << value3 << " vs "
       << value4;
    return ss.str();
  }

  return std::nullopt;
}

/**
 * @brief Checks that enum_type returns the correct type
 *
 * @return An error string on error
 */
std::optional<std::string> test_enum_type() {
  test::DataType value1 = test::DataType::TypeInt1{1};
  if (value1.enum_type() != test::DataType::Enum::kInt1) {
    std::stringstream ss;
    ss << "enum_type returned wrong value for Int1: got "
       << value1.enum_type() << ", expected "
       << test::DataType::Enum::kInt1;
    return ss.str();
  }

  test::DataType value2 = test::DataType::TypeInt2{1};
  if (value2.enum_type() != test::DataType::Enum::kInt2) {
    std::stringstream ss;
    ss << "enum_type returned wrong value for Int2: got "
       << value2.enum_type() << ", expected "
       << test::DataType::Enum::kInt2;
    return ss.str();
  }

  test::DataType value3 = test::DataType::TypeFloat{3.5};
  if (value3.enum_type() != test::DataType::Enum::kFloat) {
    std::stringstream ss;
    ss << "enum_type returned wrong value for Float: got "
       << value3.enum_type() << ", expected "
       << test::DataType::Enum::kFloat;
    return ss.str();
  }

  test::DataType value4 = test::DataType::TypeEmpty{};
  if (value4.enum_type() != test::DataType::Enum::kEmpty) {
    std::stringstream ss;
    ss << "enum_type returned wrong value for Empty: got "
       << value4.enum_type() << ", expected "
       << test::DataType::Enum::kEmpty;
    return ss.str();
  }

  return std::nullopt;
}

/**
 * @brief Checks that enum values can be correctly loaded from nodes
 *
 * @return An error string on error
 */
std::optional<std::string> test_load() {
  test::DataType value1 = test::DataType::TypeInt1{1};
  termite::Node node1 = termite::map{{"Int1", "1"}};
  auto read_value1 = node1.to_value<test::DataType>();
  if (!read_value1.is_ok()) {
    std::stringstream ss;
    ss << "Unable to convert node to enum (Int1): " << read_value1.get_err();
    return ss.str();
  }
  auto inner_value1 = read_value1.get_ok();
  if (inner_value1 != value1) {
    std::stringstream ss;
    ss << "Failed to convert node to enum (Int1): got " << inner_value1
       << ", expected " << value1;
    return ss.str();
  }

  test::DataType value2 = test::DataType::TypeInt2{1};
  termite::Node node2 = termite::map{{"Int2", "1"}};
  auto read_value2 = node2.to_value<test::DataType>();
  if (!read_value2.is_ok()) {
    std::stringstream ss;
    ss << "Unable to convert node to enum (Int2): " << read_value2.get_err();
    return ss.str();
  }
  auto inner_value2 = read_value2.get_ok();
  if (inner_value2 != value2) {
    std::stringstream ss;
    ss << "Failed to convert node to enum (Int2): got " << inner_value2
       << ", expected " << value2;
    return ss.str();
  }

  test::DataType value3 = test::DataType::TypeFloat{3.5};
  termite::Node node3 = termite::map{{"Float", "3.5"}};
  auto read_value3 = node3.to_value<test::DataType>();
  if (!read_value3.is_ok()) {
    std::stringstream ss;
    ss << "Unable to convert node to enum (Float): " << read_value3.get_err();
    return ss.str();
  }
  auto inner_value3 = read_value3.get_ok();
  if (inner_value3 != value3) {
    std::stringstream ss;
    ss << "Failed to convert node to enum (Float): got " << inner_value3
       << ", expected " << value3;
    return ss.str();
  }

  test::DataType value4 = test::DataType::TypeEmpty{};
  termite::Node node4 = "Empty";
  auto read_value4 = node4.to_value<test::DataType>();
  if (!read_value4.is_ok()) {
    std::stringstream ss;
    ss << "Unable to convert node to enum (Empty): " << read_value4.get_err();
    return ss.str();
  }
  auto inner_value4 = read_value4.get_ok();
  if (inner_value4 != value4) {
    std::stringstream ss;
    ss << "Failed to convert node to enum (Empty): got " << inner_value4
       << ", expected " << value4;
    return ss.str();
  }

  return std::nullopt;
}

/**
 * @brief Checks that enum values cannot be constructed from invalid nodes
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_load() {
  auto wrong_node1 = termite::Node(termite::Node::Value("Int1"));
  auto wrong_read_value1 = wrong_node1.to_value<test::DataType>();
  if (wrong_read_value1.is_ok()) {
    return "An enum was constructed from invalid node (Int1 as string)";
  }

  auto wrong_node2 = termite::Node(termite::Node::Value("Int2"));
  auto wrong_read_value2 = wrong_node2.to_value<test::DataType>();
  if (wrong_read_value2.is_ok()) {
    return "An enum was constructed from invalid node (Int2 as string)";
  }

  auto wrong_node3 = termite::Node(termite::Node::Value("Float"));
  auto wrong_read_value3 = wrong_node3.to_value<test::DataType>();
  if (wrong_read_value3.is_ok()) {
    return "An enum was constructed from invalid node (Float as string)";
  }

  termite::Node wrong_node4 = termite::map{{"Empty", "3.5"}};
  auto wrong_read_value4 = wrong_node4.to_value<test::DataType>();
  if (wrong_read_value4.is_ok()) {
    return "An enum was constructed from invalid node (Empty with value)";
  }

  auto wrong_node_empty = termite::Node(termite::Node::Value("Test"));
  auto wrong_read_value_empty = wrong_node_empty.to_value<test::DataType>();
  if (wrong_read_value_empty.is_ok()) {
    return "An enum was constructed from invalid node (unknown type)";
  }

  termite::Node wrong_node_map = termite::map{{"Test", "3.5"}};
  auto wrong_read_value_map = wrong_node_map.to_value<test::DataType>();
  if (wrong_read_value_map.is_ok()) {
    return "An enum was constructed from invalid node (unknown type with "
           "value)";
  }

  return std::nullopt;
}

/**
 * @brief Checks that enum values can be saved to nodes and reloaded
 *
 * @return An error string on error
 */
std::optional<std::string> test_reload() {
  test::DataType value1 = test::DataType::TypeInt1{1};
  auto converted_node1 = termite::Node::from_value(value1);
  auto converted_value1 = converted_node1.to_value<test::DataType>();
  if (!converted_value1.is_ok()) {
    return "Unable to reload enum (Int1)";
  }
  if (converted_value1.get_ok() != value1) {
    return "Failed to reload enum (Int1)";
  }

  test::DataType value2 = test::DataType::TypeInt2{1};
  termite::Node converted_node2 = termite::Node::from_value(value2);
  auto converted_value2 = converted_node2.to_value<test::DataType>();
  if (!converted_value2.is_ok()) {
    return "Unable to reload enum (Int2)";
  }
  if (converted_value2.get_ok() != value2) {
    return "Failed to reload enum (Int2)";
  }

  test::DataType value3 = test::DataType::TypeFloat{3.5};
  termite::Node converted_node3 = termite::Node::from_value(value3);
  auto converted_value3 = converted_node3.to_value<test::DataType>();
  if (!converted_value3.is_ok()) {
    return "Unable to reload enum (Float)";
  }
  if (converted_value3.get_ok() != value3) {
    return "Failed to reload enum (Float)";
  }

  test::DataType value4 = test::DataType::TypeEmpty{};
  termite::Node converted_node4 = termite::Node::from_value(value4);
  auto converted_value4 = converted_node4.to_value<test::DataType>();
  if (!converted_value4.is_ok()) {
    return "Unable to reload enum (Empty)";
  }
  if (converted_value4.get_ok() != value4) {
    return "Failed to reload enum (Empty)";
  }

  return std::nullopt;
}

int main() {
  auto names = {
      "test_eq_self", "test_eq_diff",    "test_enum_type",
      "test_load",    "test_error_load", "test_reload",
  };
  auto functions = {
      test_eq_self, test_eq_diff,    test_enum_type,
      test_load,    test_error_load, test_reload,
  };

  std::cout << "Running " << names.size() << " tests" << std::endl;

  int progress = 1;
  auto name_it = names.begin();
  for (auto function_it = functions.begin(); function_it < functions.end();
       ++function_it, ++name_it, ++progress) {
    if (auto error = (*function_it)()) {
      std::cout << "Error occured at \"" << *name_it << "\": " << *error
                << std::endl;
      return progress;
    }
  }

  std::cout << "No errors" << std::endl;

  return 0;
}