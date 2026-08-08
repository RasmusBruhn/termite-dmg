#include "generated/constraints.h"

#include <iostream>
#include <sstream>

/**
 * @brief Checks an element is equal to itself
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_eq_self() {
  auto value = test::DataType(2);
  if (value != value) {
    return "A constrained type was not equal to itself";
  }
  return std::nullopt;
}

/**
 * @brief Checks that two different constrained types are not equal
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_eq_diff() {
  auto value = test::DataType(2);
  auto compare = test::DataType(4);
  if (value == compare) {
    std::stringstream ss;
    ss << "Two different constrained types were equal: " << value.get()
       << " vs " << compare.get();
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Checks that a constrained type can be correctly loaded
 *
 * @return An error string on error
 */
std::optional<std::string> test_load() {
  auto value = test::DataType(2);
  termite::Node node(termite::Node::Value("2"));
  auto value_read = node.to_value<test::DataType>();
  if (!value_read.is_ok()) {
    std::stringstream ss;
    ss << "Unable to convert node to constrained type: "
       << value_read.get_err();
    return ss.str();
  }
  auto inner_value = value_read.get_ok();
  if (inner_value != value) {
    std::stringstream ss;
    ss << "Failed to convert node to constrained type: expected " << value.get()
       << ", got " << inner_value.get();
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Checks that a constrained type cannot be constructed if constraint 1
 * is invalidated
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_constraint_1() {
  termite::Node node(termite::Node::Value("0"));
  auto value_read = node.to_value<test::DataType>();
  if (value_read.is_ok()) {
    std::stringstream ss;
    ss << "A constrained type was constructed from node with invalid "
          "constraint 1 (value: 0)";
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Checks that a constrained type cannot be constructed if constraint 2
 * is invalidated
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_constraint_2() {
  termite::Node node(termite::Node::Value("1"));
  auto value_read = node.to_value<test::DataType>();
  if (value_read.is_ok()) {
    std::stringstream ss;
    ss << "A constrained type was constructed from node with invalid "
          "constraint 2 (value: 1)";
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Checks that a constrained type can be saved to a node and reloaded
 * again
 *
 * @return An error string on error
 */
std::optional<std::string> test_reload() {
  auto value = test::DataType(2);
  termite::Node converted_node = termite::Node::from_value(value);
  auto converted_value = converted_node.to_value<test::DataType>();
  if (!converted_value.is_ok()) {
    std::stringstream ss;
    ss << "Unable to reload constrained type: " << converted_value.get_err();
    return ss.str();
  }
  auto inner_value = converted_value.get_ok();
  if (inner_value != value) {
    std::stringstream ss;
    ss << "Failed to reload constrained type: expected " << value.get()
       << ", got " << inner_value.get();
    return ss.str();
  }
  return std::nullopt;
}

int main() {
  auto names = {
      "test_error_eq_self",      "test_error_eq_diff",      "test_load",
      "test_error_constraint_1", "test_error_constraint_2", "test_reload",
  };
  auto functions = {
      test_error_eq_self,      test_error_eq_diff,      test_load,
      test_error_constraint_1, test_error_constraint_2, test_reload,
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
