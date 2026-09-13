#include "generated/max_length_string.h"

#include <iostream>
#include <sstream>

/**
 * @brief Checks that a constrained type can be correctly loaded
 *
 * @return An error string on error
 */
std::optional<std::string> test_load() {
  test::DataType value = "12";
  termite::Node node = termite::string("12");
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
 * @brief Checks that a constrained type cannot be constructed if the constraint
 * is invalidated
 *
 * @return An error string on error
 */
std::optional<std::string> test_error() {
  termite::Node node = "123";
  auto value_read = node.to_value<test::DataType>();
  if (value_read.is_ok()) {
    std::stringstream ss;
    ss << "A constrained type was constructed from node with invalid "
          "constraint: "
       << value_read.get_ok();
    return ss.str();
  }
  return std::nullopt;
}

int main() {
  auto names = {
      "test_load",
      "test_error",
  };
  auto functions = {
      test_load,
      test_error,
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
