#include "generated/identical.h"

#include <iostream>
#include <sstream>

/**
 * @brief Checks that structs with default values are equal to themselves
 *
 * @return An error string on error
 */
std::optional<std::string> test_load() {
  auto value = test::DataType({{"field1", 1}, {"field2", 2}});
  termite::Node node_correct = termite::map{{"field1", "1"}, {"field2", "2"}};
  auto value_read_correct = node_correct.to_value<test::DataType>();
  if (!value_read_correct.is_ok()) {
    std::stringstream ss;
    ss << "Unable to convert map to struct with all fields: "
       << value_read_correct.get_err();
    return ss.str();
  }
  auto read_val = value_read_correct.get_ok();
  if (read_val != value) {
    std::stringstream ss;
    ss << "Failed to convert map to struct with all fields: expected " << value
       << ", got " << read_val;
    return ss.str();
  }
  return std::nullopt;
}

int main() {
  auto names = {
      "test_load",
  };
  auto functions = {
      test_load,
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