#include "termite.hpp"
#include <optional>
#include <string>
#include <sstream>

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
 * @brief Test if results can print
 * 
 * @return An error string on error
 */
std::optional<std::string> test_result_print() {
  auto result_ok = termite::Result<int>::from_ok(1);
  std::cout << "test_result_print ok: " << result_ok << std::endl;
  std::cout << "test_result_print ok: " << result_ok.to_string() << std::endl;

  auto result_err = termite::Result<int>::from_err(termite::Error("Error"));
  std::cout << "test_result_print err: " << result_err << std::endl;
  std::cout << "test_result_print err: " << result_err.to_string() << std::endl;

  return std::nullopt;
}

/**
 * @brief Test if results can check if they are ok
 *
 * @return An error string on error
 */
std::optional<std::string> test_result_is_ok()
{
  auto result_ok = termite::Result<int>::from_ok(1);
  if (!result_ok.is_ok()) {
    return "Not Ok";
  }

  auto result_err = termite::Result<int>::from_err(termite::Error("Error"));
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
std::optional<std::string> test_result_get()
{
  int correct_ok = 1;
  int result_ok = termite::Result<int>::from_ok(correct_ok).get_ok();
  if (correct_ok != result_ok) {
    std::stringstream ss;
    ss << result_ok;
    return ss.str();
  }

  termite::Error correct_err("Error");
  termite::Error result_err = termite::Result<int>::from_err(correct_err).get_err();
  if (correct_err != result_err)
  {
    std::stringstream ss;
    ss << result_err;
    return ss.str();
  }

  return std::nullopt;
}

int main() {
  auto names = {
    "test_error_message",
    "test_error_location_default",
    "test_error_location",
    "test_result_print",
    "test_result_is_ok",
    "test_result_get",
  };
  auto functions = {
    test_error_message,
    test_error_location_default,
    test_error_location,
    test_result_print,
    test_result_is_ok,
    test_result_get,
  };

  int progress = 1;
  auto name_it = names.begin();
  for (auto function_it = functions.begin(); function_it < functions.end(); ++function_it, ++name_it, ++progress) {
    if (auto error = (*function_it)()) {
        std::cout << "Error occured at \"" << *name_it << "\": " << *error << std::endl;
        return progress;
    }
  }

  std::cout << "No errors" << std::endl;

  return 0;
}
