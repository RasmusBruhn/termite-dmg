#include "generated/description.h"

#include <iostream>
#include <sstream>

/**
 * @brief Checks that empty structs are equal to themselves
 *
 * @return An error string on error
 */
std::optional<std::string> test_eq_self() {
  auto type1 = test::DataType1();
  if (type1 != test::DataType1()) {
    return "An empty struct was not equal to itself";
  }
  auto type2 = test::DataType2();
  if (type2 != test::DataType2()) {
    return "An empty struct was not equal to itself";
  }
  return std::nullopt;
}

/**
 * @brief Checks that empty structs can be loaded from empty maps
 *
 * @return An error string on error
 */
std::optional<std::string> test_load() {
  std::map<std::string, termite::Node> map_correct1;
  termite::Node node_correct1(termite::Node::Map(std::move(map_correct1)));
  auto value_read_correct1 = node_correct1.to_value<test::DataType1>();
  if (!value_read_correct1.is_ok()) {
    std::stringstream ss;
    ss << "Unable to convert empty map to struct (DataType1): "
       << value_read_correct1.get_err();
    return ss.str();
  }

  std::map<std::string, termite::Node> map_correct2;
  termite::Node node_correct2(termite::Node::Map(std::move(map_correct2)));
  auto value_read_correct2 = node_correct2.to_value<test::DataType2>();
  if (!value_read_correct2.is_ok()) {
    std::stringstream ss;
    ss << "Unable to convert empty map to struct (DataType2): "
       << value_read_correct2.get_err();
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Checks that empty structs can be loaded with extra fields
 *
 * @return An error string on error
 */
std::optional<std::string> test_load_extra_fields() {
  std::map<std::string, termite::Node> map_many1;
  map_many1.insert({"field1", termite::Node(termite::Node::Value("1"))});
  termite::Node node_many1(termite::Node::Map(std::move(map_many1)));
  auto value_read_many1 = node_many1.to_value<test::DataType1>();
  if (!value_read_many1.is_ok()) {
    std::stringstream ss;
    ss << "Unable to convert map with extra fields to struct (DataType1): "
       << value_read_many1.get_err();
    return ss.str();
  }

  std::map<std::string, termite::Node> map_many2;
  map_many2.insert({"field1", termite::Node(termite::Node::Value("1"))});
  termite::Node node_many2(termite::Node::Map(std::move(map_many2)));
  auto value_read_many2 = node_many2.to_value<test::DataType2>();
  if (!value_read_many2.is_ok()) {
    std::stringstream ss;
    ss << "Unable to convert map with extra fields to struct (DataType2): "
       << value_read_many2.get_err();
    return ss.str();
  }
  return std::nullopt;
}

/**
 * @brief Checks that structs cannot be loaded from invalid node types
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_load() {
  termite::Node node_wrong1(termite::Node::Value("1.0"));
  auto value_wrong_wrong1 = node_wrong1.to_value<test::DataType1>();
  if (value_wrong_wrong1.is_ok()) {
    return "A struct was constructed from invalid node type (DataType1)";
  }

  termite::Node node_wrong2(termite::Node::Value("1.0"));
  auto value_wrong_wrong2 = node_wrong2.to_value<test::DataType2>();
  if (value_wrong_wrong2.is_ok()) {
    return "A struct was constructed from invalid node type (DataType2)";
  }
  return std::nullopt;
}

/**
 * @brief Checks that structs can be saved to nodes and reloaded
 *
 * @return An error string on error
 */
std::optional<std::string> test_reload() {
  auto type1 = test::DataType1();
  termite::Node converted_node1 = termite::Node::from_value(type1);
  auto converted_value1 = converted_node1.to_value<test::DataType1>();
  if (!converted_value1.is_ok()) {
    std::stringstream ss;
    ss << "Unable to reload struct (DataType1): " << converted_value1.get_err();
    return ss.str();
  }
  auto inner_value1 = converted_value1.get_ok();
  if (inner_value1 != type1) {
    return "Failed to reload struct (DataType1)";
  }

  auto type2 = test::DataType2();
  termite::Node converted_node2 = termite::Node::from_value(type2);
  auto converted_value2 = converted_node2.to_value<test::DataType2>();
  if (!converted_value2.is_ok()) {
    std::stringstream ss;
    ss << "Unable to reload struct (DataType2): " << converted_value2.get_err();
    return ss.str();
  }
  auto inner_value2 = converted_value2.get_ok();
  if (inner_value2 != type2) {
    return "Failed to reload struct (DataType2)";
  }
  return std::nullopt;
}

int main() {
  auto names = {
      "test_eq_self",    "test_load",   "test_load_extra_fields",
      "test_error_load", "test_reload",
  };
  auto functions = {
      test_eq_self,    test_load,   test_load_extra_fields,
      test_error_load, test_reload,
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