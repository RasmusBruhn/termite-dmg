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

class Test {
public:
  friend std::ostream &operator<<(std::ostream &stream, const Test &) {
    return stream << "Test";
  }
};

/**
 * @brief Test if the error location is set correctly
 * 
 * @return An error string on error
 */
std::optional<std::string> test_result_print() {
  auto result = termite::Result<Test>::from_ok(Test());

  return std::nullopt;
}

int main() {
  auto names = {
    "test_error_message",
    "test_error_location_default",
    "test_error_location"
  };
  auto functions = {
    test_error_message,
    test_error_location_default,
    test_error_location,
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
