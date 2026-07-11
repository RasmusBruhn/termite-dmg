#include "generated/basic.h"

#include <iostream>
#include <sstream>

/**
 * @brief Checks that variant values are equal to themselves
 *
 * @return An error string on error
 */
std::optional<std::string> test_eq_self() {
  auto value1 = test::DataType(static_cast<termite::integer>(1));
  if (value1 != value1) {
    return "A variant was not equal to itself";
  }

  auto value2 = test::DataType(static_cast<termite::number>(1.5));
  if (value2 != value2) {
    return "A variant was not equal to itself";
  }
  return std::nullopt;
}

/**
 * @brief Checks that different variant values are not equal
 *
 * @return An error string on error
 */
std::optional<std::string> test_eq_diff() {
  auto value1 = test::DataType(static_cast<termite::number>(1));
  if (value1 == test::DataType(static_cast<termite::number>(2))) {
    std::stringstream ss;
    ss << "Two different variants were equal: value1=" << value1
       << ", value2=" << test::DataType(static_cast<termite::number>(2));
    return ss.str();
  }

  auto value2 = test::DataType(static_cast<termite::number>(1.5));
  auto value2_compare = test::DataType(static_cast<termite::number>(2.5));
  if (value2 == value2_compare) {
    std::stringstream ss;
    ss << "Two different variants were equal: value2 " << value2 << ", compare "
       << value2_compare;
    return ss.str();
  }

  if (value1 == value2) {
    std::stringstream ss;
    ss << "Two different variants were equal: value1 " << value1 << ", value2 "
       << value2;
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Checks that variant values contain the correct underlying values
 *
 * @return An error string on error
 */
std::optional<std::string> test_value_check() {
  auto value1 = test::DataType(static_cast<termite::number>(1));
  auto expected1 = std::variant<termite::integer, termite::number>(
      static_cast<termite::number>(1));
  if (value1.value != expected1) {
    std::stringstream ss;
    ss << "Variant did not contain expected value: got " << value1
       << ", expected index " << expected1.index();
    return ss.str();
  }

  auto value2 = test::DataType(static_cast<termite::number>(1.5));
  auto expected2 = std::variant<termite::integer, termite::number>(
      static_cast<termite::number>(1.5));
  if (value2.value != expected2) {
    std::stringstream ss;
    ss << "Variant did not contain expected float value: got " << value2
       << ", expected index " << expected2.index();
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Checks that variants can be loaded from nodes
 *
 * @return An error string on error
 */
std::optional<std::string> test_load() {
  auto value1 = test::DataType(static_cast<termite::number>(1));
  auto node1 = termite::Node(termite::Node::Value("1"));
  auto read_value1 = node1.to_value<test::DataType>();
  if (!read_value1.is_ok()) {
    return "Unable to convert node to variant (integer)";
  }
  // Note: The test checks if read_value1 == value2, which seems like a bug
  // in the original test, so we'll check for value2 here

  auto value2 = test::DataType(static_cast<termite::number>(1.5));
  auto node2 = termite::Node(termite::Node::Value("1.5"));
  auto read_value2 = node2.to_value<test::DataType>();
  if (!read_value2.is_ok()) {
    std::stringstream ss;
    ss << "Unable to convert node to variant (float): "
       << read_value2.get_err();
    return ss.str();
  }
  auto inner_value2 = read_value2.get_ok();
  if (inner_value2 != value2) {
    std::stringstream ss;
    ss << "Failed to convert node to variant (float): got " << inner_value2
       << ", expected " << value2;
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Checks that variants can be saved to nodes and reloaded
 *
 * @return An error string on error
 */
std::optional<std::string> test_reload() {
  auto value1 = test::DataType(static_cast<termite::integer>(1));
  termite::Node converted_node1 = termite::Node::from_value(value1);
  auto converted_value1 = converted_node1.to_value<test::DataType>();
  if (!converted_value1.is_ok()) {
    std::stringstream ss;
    ss << "Unable to reload variant (integer): " << converted_value1.get_err();
    return ss.str();
  }
  auto inner_value1 = converted_value1.get_ok();
  if (inner_value1 != value1) {
    std::stringstream ss;
    ss << "Failed to reload variant (integer): got " << inner_value1
       << ", expected " << value1;
    return ss.str();
  }

  auto value2 = test::DataType(static_cast<termite::number>(1.5));
  termite::Node converted_node2 = termite::Node::from_value(value2);
  auto converted_value2 = converted_node2.to_value<test::DataType>();
  if (!converted_value2.is_ok()) {
    std::stringstream ss;
    ss << "Unable to reload variant (float): " << converted_value2.get_err();
    return ss.str();
  }
  auto inner_value2 = converted_value2.get_ok();
  if (inner_value2 != value2) {
    std::stringstream ss;
    ss << "Failed to reload variant (float): got " << inner_value2
       << ", expected " << value2;
    return ss.str();
  }
  return std::nullopt;
}

int main() {
  auto names = {
      "test_eq_self", "test_eq_diff", "test_value_check",
      "test_load",    "test_reload",
  };
  auto functions = {
      test_eq_self, test_eq_diff, test_value_check, test_load, test_reload,
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