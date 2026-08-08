#include <iostream>
#include <sstream>
#include "generated/outline.h"

/**
 * @brief Test if DataType1 can be constructed correctly
 *
 * @return An error string on error
 */
std::optional<std::string> test_default_1() {
  auto type = test::DataType1();
  if (type != test::DataType1()) {
    return "Error in default constructor for DataType1";
  }
  return std::nullopt;
}

/**
 * @brief Test if DataType2 can be constructed correctly
 *
 * @return An error string on error
 */
std::optional<std::string> test_default_2() {
  auto type = test::DataType2();
  if (type != test::DataType2()) {
    return "Error in default constructor for DataType2";
  }
  return std::nullopt;
}

/**
 * @brief Test if DataType1 can be constructed from a map
 *
 * @return An error string on error
 */
std::optional<std::string> test_from_map_1() {
  std::map<std::string, termite::Node> map_correct;
  termite::Node node_correct(termite::Node::Map(std::move(map_correct)));
  auto value_read_correct = node_correct.to_value<test::DataType1>();
  if (!value_read_correct.is_ok()) {
    std::stringstream ss;
    ss << "Unable to construct DataType1 from a map: " << value_read_correct.get_err();
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Test if DataType2 can be constructed from a map
 *
 * @return An error string on error
 */
std::optional<std::string> test_from_map_2() {
  std::map<std::string, termite::Node> map_correct;
  termite::Node node_correct(termite::Node::Map(std::move(map_correct)));
  auto value_read_correct = node_correct.to_value<test::DataType2>();
  if (!value_read_correct.is_ok()) {
    std::stringstream ss;
    ss << "Unable to construct DataType2 from a map: " << value_read_correct.get_err();
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Test if DataType1 can be constructed from a map with too many values
 *
 * @return An error string on error
 */
std::optional<std::string> test_from_map_large_1() {
  std::map<std::string, termite::Node> map_many;
  map_many.insert({"field1", termite::Node(termite::Node(termite::Node::Value("1")))});
  termite::Node node_many(termite::Node::Map(std::move(map_many)));
  auto value_read_many = node_many.to_value<test::DataType1>();
  if (!value_read_many.is_ok()) {
    std::stringstream ss;
    ss << "Unable to construct DataType1 from a map with too many fields: " << value_read_many.get_err();
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Test if DataType1 can be constructed from a map with too many values
 *
 * @return An error string on error
 */
std::optional<std::string> test_from_map_large_2() {
  std::map<std::string, termite::Node> map_many;
  map_many.insert({"field1", termite::Node(termite::Node(termite::Node::Value("1")))});
  termite::Node node_many(termite::Node::Map(std::move(map_many)));
  auto value_read_many = node_many.to_value<test::DataType2>();
  if (!value_read_many.is_ok()) {
    std::stringstream ss;
    ss << "Unable to construct DataType2 from a map with too many fields: " << value_read_many.get_err();
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Test an error when DataType1 is constructed from wrong node type
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_node_type_1() {
  termite::Node node_wrong(termite::Node::Value("1.0"));
  auto value_wrong_wrong = node_wrong.to_value<test::DataType1>();
  if (value_wrong_wrong.is_ok()) {
    return "Constructed DataType1 from wrong node type";
  }
  return std::nullopt;
}

/**
 * @brief Test an error when DataType2 is constructed from wrong node type
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_node_type_2() {
  termite::Node node_wrong(termite::Node::Value("1.0"));
  auto value_wrong_wrong = node_wrong.to_value<test::DataType2>();
  if (value_wrong_wrong.is_ok()) {
    return "Constructed DataType2 from wrong node type";
  }
  return std::nullopt;
}

int main() {
  auto names = {
      "test_default_1", "test_default_2",
      "test_from_map_1", "test_from_map_2",
      "test_from_map_large_1", "test_from_map_large_2",
      "test_error_node_type_1", "test_error_node_type_2",
  };
  auto functions = {
      test_default_1, test_default_2,
      test_from_map_1, test_from_map_2,
      test_from_map_large_1, test_from_map_large_2,
      test_error_node_type_1, test_error_node_type_2,
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
