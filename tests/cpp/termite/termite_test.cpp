#include "termite.hpp"

#include <optional>
#include <sstream>
#include <string>

/**
 * @brief Test if the error message is set correctly
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_message() {
  std::string correct = "ErrorMessage";
  std::string result = termite::Error(correct).get_message();

  if (result == correct) {
    return std::nullopt;
  } else {
    return result;
  }
}

/**
 * @brief Test if the default error location is set correctly
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_location_default() {
  std::string correct = "";
  std::string result = termite::Error("Message").get_location();

  if (result == correct) {
    return std::nullopt;
  } else {
    return result;
  }
}

/**
 * @brief Test if the error location is set correctly
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_location() {
  std::string correct = "Location";
  std::string result = termite::Error("Message", correct).get_location();

  if (result == correct) {
    return std::nullopt;
  } else {
    return result;
  }
}

/**
 * @brief Test adding a field to an error
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_add_field() {
  std::string correct = "field1";
  termite::Error error("Message");
  error.add_field("field1");
  std::string result = error.get_location();

  if (correct != result) {
    return result;
  }

  correct = "field2.field1";
  error.add_field("field2");
  result = error.get_location();

  if (correct != result) {
    return result;
  }

  return std::nullopt;
}

/**
 * @brief Test adding a list to an error
 *
 * @return An error string on error
 */
std::optional<std::string> test_error_add_list() {
  std::string correct = "list1[1]";
  termite::Error error("Message");
  error.add_list(1);
  error.add_field("list1");
  std::string result = error.get_location();

  if (correct != result) {
    return result;
  }

  correct = "list2[2].list1[1]";
  error.add_list(2);
  error.add_field("list2");
  result = error.get_location();

  if (correct != result) {
    return result;
  }

  return std::nullopt;
}

/**
 * @brief Test if results can print
 *
 * @return An error string on error
 */
std::optional<std::string> test_result_print() {
  auto result_ok = termite::Result<int>::ok(1);
  std::cout << "test_result_print ok: " << result_ok << std::endl;

  auto result_err = termite::Result<int>::err(termite::Error("Error"));
  std::cout << "test_result_print err: " << result_err << std::endl;

  return std::nullopt;
}

/**
 * @brief Test if equality operator works for results
 *
 * @return An error string on error
 */
std::optional<std::string> test_result_equality() {
  auto result_ok = termite::Result<int>::ok(1);
  auto result_err = termite::Result<int>::err(termite::Error("Error"));

  if (result_ok == termite::Result<int>::ok(0)) {
    return "1 != 0";
  }
  if (result_ok != termite::Result<int>::ok(1)) {
    return "1 == 1";
  }
  if (result_ok == termite::Result<int>::err(termite::Error("Error"))) {
    return "1 != Error";
  }
  if (result_err == termite::Result<int>::err(termite::Error("Error2"))) {
    return "Error != Error2";
  }
  if (result_err != termite::Result<int>::err(termite::Error("Error"))) {
    return "Error == Error";
  }
  if (result_err == termite::Result<int>::ok(1)) {
    return "Error != 1";
  }

  return std::nullopt;
}

/**
 * @brief Test if results can check if they are ok
 *
 * @return An error string on error
 */
std::optional<std::string> test_result_is_ok() {
  auto result_ok = termite::Result<int>::ok(1);
  if (!result_ok.is_ok()) {
    return "Not Ok";
  }

  auto result_err = termite::Result<int>::err(termite::Error("Error"));
  if (result_err.is_ok()) {
    return "Not Err";
  }

  return std::nullopt;
}

/**
 * @brief Test if results can get values
 *
 * @return An error string on error
 */
std::optional<std::string> test_result_get() {
  int correct_ok = 1;
  int result_ok = termite::Result<int>::ok(correct_ok).get_ok();
  if (correct_ok != result_ok) {
    std::stringstream ss;
    ss << result_ok;
    return ss.str();
  }

  termite::Error correct_err("Error");
  termite::Error result_err = termite::Result<int>::err(correct_err).get_err();
  if (correct_err != result_err) {
    std::stringstream ss;
    ss << result_err;
    return ss.str();
  }

  return std::nullopt;
}

/**
 * @brief Test if NodeValue can get read an integer
 *
 * @return An error string on error
 */
std::optional<std::string> test_node_value_parse_simple() {
  termite::Node node(termite::Node::Value("123"));

  auto value = node.to_value<int>();
  if (!value.is_ok()) {
    std::stringstream ss;
    ss << value.get_err();
    return ss.str();
  }

  if (value.get_ok() != 123) {
    return "Wrong value";
  }

  return std::nullopt;
}

/**
 * @brief Test if NodeValue can get read an integer with spaces
 *
 * @return An error string on error
 */
std::optional<std::string> test_node_value_parse_spaces() {
  termite::Node node(termite::Node::Value(" 123 "));

  auto value = node.to_value<int>();
  if (!value.is_ok()) {
    std::stringstream ss;
    ss << value.get_err();
    return ss.str();
  }

  if (value.get_ok() != 123) {
    return "Wrong value";
  }

  return std::nullopt;
}

/**
 * @brief Test if NodeValue gets an error with wrong input
 *
 * @return An error string on error
 */
std::optional<std::string> test_node_value_parse_error_begin() {
  termite::Node node(termite::Node::Value(".123"));

  auto value = node.to_value<int>();
  if (value.is_ok()) {
    std::stringstream ss;
    ss << "Should not parse: " << value.get_ok();
    return ss.str();
  }

  return std::nullopt;
}

/**
 * @brief Test if NodeValue gets an error with wrong input
 *
 * @return An error string on error
 */
std::optional<std::string> test_node_value_parse_error_end() {
  termite::Node node(termite::Node::Value("123."));

  auto value = node.to_value<int>();
  if (value.is_ok()) {
    std::stringstream ss;
    ss << "Should not parse: " << value.get_ok();
    return ss.str();
  }

  return std::nullopt;
}

/**
 * @brief Test if NodeValue gets an error with wrong type
 *
 * @return An error string on error
 */
std::optional<std::string> test_node_value_parse_error_class() {
  termite::Node node(termite::Node::Value("123"));

  auto value = node.to_value<termite::Empty>();
  if (value.is_ok()) {
    return "Should not parse";
  }

  return std::nullopt;
}

/**
 * @brief Test if Node can be copied
 *
 * @return An error string on error
 */
std::optional<std::string> test_node_copy() {
  termite::Node node(termite::Node::Value("123"));
  termite::Node node2 = node;

  if (node != node2) {
    return "Could not copy";
  }

  return std::nullopt;
}

int main() {
  auto names = {
      "test_error_message",
      "test_error_location_default",
      "test_error_location",
      "test_error_add_field",
      "test_error_add_list",
      "test_result_print",
      "test_result_equality",
      "test_result_is_ok",
      "test_result_get",
      "test_node_value_parse_simple",
      "test_node_value_parse_spaces",
      "test_node_value_parse_error_begin",
      "test_node_value_parse_error_end",
      "test_node_value_parse_error_class",
  };
  auto functions = {
      test_error_message,
      test_error_location_default,
      test_error_location,
      test_error_add_field,
      test_error_add_list,
      test_result_print,
      test_result_equality,
      test_result_is_ok,
      test_result_get,
      test_node_value_parse_simple,
      test_node_value_parse_spaces,
      test_node_value_parse_error_begin,
      test_node_value_parse_error_end,
      test_node_value_parse_error_class,
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
