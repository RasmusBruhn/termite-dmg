#include "generated/basic.h"
#include <iostream>

/**
 * @brief Checks an element is equal to itself
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_eq_self() {
  auto value = test::DataType({1, 2});
  if (value != value) {
    return "An array was not equal to itself";
  }
  return std::nullopt;
}

/**
 * @brief Checks that it is not equal when there are too few elements
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_eq_count_few() {
  auto value = test::DataType({1, 2});
  if (value == test::DataType({1})) {
    return "Two arrays with different number of elements were equal";
  }
  return std::nullopt;
}

/**
 * @brief Checks that it is not equal when there are too many elements
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_eq_count_many() {
  auto value = test::DataType({1, 2});
  if (value == test::DataType({1, 2, 3})) {
    return "Two arrays with different number of elements were equal";
  }
  return std::nullopt;
}

/**
 * @brief Checks that two arrays with different element values are not equal
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_eq_elem_diff() {
  auto value = test::DataType({1, 2});
  if (value == test::DataType({1, 3})) {
    return "Two arrays with different elements were equal";
  }
  return std::nullopt;
}

/**
 * @brief Checks that an array can be correctly loaded
 *
 * @return An error string on error
 */
std::optional<std::string> test_load() {
  auto value = test::DataType({1, 2});
  std::vector<termite::Node> vector_correct;
  vector_correct.emplace_back(termite::Node::Value("1"));
  vector_correct.emplace_back(termite::Node::Value("2"));
  termite::Node node_correct(termite::Node::List(std::move(vector_correct)));
  auto value_read_correct = node_correct.to_value<test::DataType>();
  if (!value_read_correct.is_ok()) {
    return "Unable to convert node to array";
  }
  if (value_read_correct.get_ok() != value) {
    return "Failed to convert node to array";
  }
  return std::nullopt;
}

/**
 * @brief Checks that an array cannot be constructed from a node with invalid
 * sub elements
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_load_elem() {
  std::vector<termite::Node> vector_correct;
  vector_correct.emplace_back(termite::Node::Value("1"));
  vector_correct.emplace_back(termite::Node::Value("2.5"));
  termite::Node node_correct(termite::Node::List(std::move(vector_correct)));
  auto value_read_correct = node_correct.to_value<test::DataType>();
  if (value_read_correct.is_ok()) {
    return "Array was constructed from node with invalid sub elements";
  }
  return std::nullopt;
}

/**
 * @brief Checks that an array cannot be constructed from an invalid node
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_load() {
  termite::Node node_wrong(termite::Node::Value("1.0"));
  auto value_read_correct = node_wrong.to_value<test::DataType>();
  if (value_read_correct.is_ok()) {
    return "Array was constructed from invalid node";
  }
  return std::nullopt;
}

/**
 * @brief Checks that an array can be saved to a node and reloaded again
 *
 * @return An error string on error
 */
std::optional<std::string> test_reload() {
  auto value = test::DataType({1, 2});
  termite::Node converted_node = termite::Node::from_value(value);
  auto converted_value = converted_node.to_value<test::DataType>();
  if (!converted_value.is_ok()) {
    return "Unable to reload array";
  }
  if (converted_value.get_ok() != value) {
    return "Failed to reload array";
  }
  return std::nullopt;
}

int main() {
  auto names = {
      "test_error_eq_self",
      "test_error_eq_count_few",
      "test_error_eq_count_many",
      "test_error_eq_elem_diff",
      "test_load",
      "test_error_load_elem",
      "test_error_load",
      "test_reload",
  };
  auto functions = {
      test_error_eq_self,
      test_error_eq_count_few,
      test_error_eq_count_many,
      test_error_eq_elem_diff,
      test_load,
      test_error_load_elem,
      test_error_load,
      test_reload,
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
