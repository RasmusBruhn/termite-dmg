#include <iostream>
#include "generated/header.h"

/**
 * @brief Test if DataType can be constructed correctly
 *
 * @return An error string on error
 */
std::optional<std::string> test_default() {
  auto type = DataType();
  if (type != DataType()) {
    return "Error in default constructor";
  }
  return std::nullopt;
}

/**
 * @brief Test if DataType can be constructed from a map
 *
 * @return An error string on error
 */
std::optional<std::string> test_from_map() {
  std::map<std::string, termite::Node> map_correct;
  termite::Node node_correct(termite::Node::Map(std::move(map_correct)));
  auto value_read_correct = node_correct.to_value<DataType>();
  if (!value_read_correct.is_ok()) {
    return "Unable to construct from a map";
  }
}

int main() {
  auto names = {
      "test_default",
      "test_from_map",
  };
  auto functions = {
      test_default,
      test_from_map,
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
