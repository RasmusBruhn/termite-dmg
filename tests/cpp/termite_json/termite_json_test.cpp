#include <termite-json.h>

#include <optional>
#include <sstream>
#include <string>

/**
 * @brief Test if it can convert a scalar
 *
 * @return An error string on error
 */
std::optional<std::string> test_scalar() {
  termite::Node correct = "Test";
  nlohmann::json node("Test");
  termite::Result<termite::Node> result = termite::from_JSON(node);

  if (!result.is_ok()) {
    std::stringstream ss;
    ss << result.get_err();
    return ss.str();
  }

  termite::Node result_ok = result.get_ok();
  if (result_ok != correct) {
    std::stringstream ss;
    ss << result_ok;
    return ss.str();
  }

  return std::nullopt;
}

/**
 * @brief Test if it can convert a list
 *
 * @return An error string on error
 */
std::optional<std::string> test_list() {
  termite::Node correct = termite::list{"Test1", "Test2"};
  nlohmann::json node;
  node.push_back(nlohmann::json("Test1"));
  node.push_back(nlohmann::json("Test2"));
  auto result = termite::from_JSON(node);

  if (!result.is_ok()) {
    std::stringstream ss;
    ss << result.get_err();
    return ss.str();
  }

  auto result_ok = result.get_ok();
  if (result_ok != correct) {
    std::stringstream ss;
    ss << result_ok;
    return ss.str();
  }

  return std::nullopt;
}

/**
 * @brief Test if it can convert a map
 *
 * @return An error string on error
 */
std::optional<std::string> test_map() {
  termite::Node correct =
      termite::map{{"field1", "Test1"}, {"field2", "Test2"}};
  nlohmann::json node;
  node["field1"] = nlohmann::json("Test1");
  node["field2"] = nlohmann::json("Test2");
  auto result = termite::from_JSON(node);

  if (!result.is_ok()) {
    std::stringstream ss;
    ss << result.get_err();
    return ss.str();
  }

  auto result_ok = result.get_ok();
  if (result_ok != correct) {
    std::stringstream ss;
    ss << result_ok;
    return ss.str();
  }

  return std::nullopt;
}

/**
 * @brief Test if it can convert to a scalar
 *
 * @return An error string on error
 */
std::optional<std::string> test_to_scalar() {
  termite::Node node = "Test";
  auto result = termite::to_JSON(node);

  if (!result.is_string()) {
    return "Should be a scalar";
  }
  if (static_cast<std::string>(result) != "Test") {
    return "Wrong value";
  }

  return std::nullopt;
}

/**
 * @brief Test if it can convert to a list
 *
 * @return An error string on error
 */
std::optional<std::string> test_to_list() {
  termite::Node node = termite::list{"Test1", "Test2"};
  auto result = termite::to_JSON(node);

  if (!result.is_array()) {
    return "Should be a sequence";
  }
  if (result.size() != 2) {
    return "Wrong size";
  }
  if (static_cast<std::string>(result[0]) != "Test1") {
    return "Wrong value [0]";
  }
  if (static_cast<std::string>(result[1]) != "Test2") {
    return "Wrong value [1]";
  }

  return std::nullopt;
}

/**
 * @brief Test if it can convert to a map
 *
 * @return An error string on error
 */
std::optional<std::string> test_to_map() {
  termite::Node node = termite::map{{"field1", "Test1"}, {"field2", "Test2"}};
  auto result = termite::to_JSON(node);

  if (!result.is_structured()) {
    return "Should be a map";
  }
  if (result.size() != 2) {
    return "Wrong size";
  }
  if (static_cast<std::string>(result["field1"]) != "Test1") {
    return "Wrong value [0]";
  }
  if (static_cast<std::string>(result["field2"]) != "Test2") {
    return "Wrong value [1]";
  }

  return std::nullopt;
}

/**
 * @brief Test if it can convert from a JSON string
 *
 * @return An error string on error
 */
std::optional<std::string> test_json_string() {
  std::string json_string =
      "{ \"field1\": \"Test1\", \"field2\": [\"Test2\", \"Test3\"] }";
  termite::Node correct = termite::map{
      {"field1", "Test1"}, {"field2", termite::list{"Test2", "Test3"}}};
  auto result = termite::from_JSON_string(json_string);

  if (!result.is_ok()) {
    std::stringstream ss;
    ss << result.get_err();
    return ss.str();
  }
  auto result_node = result.get_ok();
  if (result_node != correct) {
    std::stringstream ss;
    ss << "Result does not match expected: " << result_node;
    return ss.str();
  }

  return std::nullopt;
}

/**
 * @brief Test if it can convert from a JSON file
 *
 * @return An error string on error
 */
std::optional<std::string> test_json_file() {
  termite::Node correct = termite::map{
      {"field1", "Test1"}, {"field2", termite::list{"Test2", "Test3"}}};
  auto result = termite::from_JSON_file("../json_test.json");

  if (!result.is_ok()) {
    std::stringstream ss;
    ss << result.get_err();
    return ss.str();
  }
  auto result_node = result.get_ok();
  if (result_node != correct) {
    std::stringstream ss;
    ss << "Result does not match expected: " << result_node;
    return ss.str();
  }

  return std::nullopt;
}

/**
 * @brief Test if it can convert to a JSON string
 *
 * @return An error string on error
 */
std::optional<std::string> test_to_json_string() {
  termite::Node correct = termite::map{
      {"field1", "Test1"}, {"field2", termite::list{"Test2", "Test3"}}};
  std::string json_string = termite::to_JSON_string(correct);
  auto result = termite::from_JSON_string(json_string);

  if (!result.is_ok()) {
    std::stringstream ss;
    ss << result.get_err();
    return ss.str();
  }
  auto result_node = result.get_ok();
  if (result_node != correct) {
    std::stringstream ss;
    ss << "Result does not match expected: " << result_node;
    return ss.str();
  }

  return std::nullopt;
}

/**
 * @brief Test if it can convert to a JSON file
 *
 * @return An error string on error
 */
std::optional<std::string> test_to_json_file() {
  termite::Node correct = termite::map{
      {"field1", "Test1"}, {"field2", termite::list{"Test2", "Test3"}}};

  auto write_result = termite::to_JSON_file(correct, "json_test.json");
  if (!write_result.is_ok()) {
    std::stringstream ss;
    ss << write_result.get_err();
    return ss.str();
  }

  auto result = termite::from_JSON_file("json_test.json");

  if (!result.is_ok()) {
    std::stringstream ss;
    ss << result.get_err();
    return ss.str();
  }
  auto result_node = result.get_ok();
  if (result_node != correct) {
    std::stringstream ss;
    ss << "Result does not match expected: " << result_node;
    return ss.str();
  }

  return std::nullopt;
}

/**
 * @brief Test if it can convert to an empty list
 *
 * @return An error string on error
 */
std::optional<std::string> test_to_list_empty() {
  termite::Node node = termite::list{};
  auto json_node = termite::to_JSON(node);
  auto result = termite::from_JSON(json_node);

  if (!result.is_ok()) {
    std::stringstream ss;
    ss << result.get_err();
    return ss.str();
  }
  auto result_node = result.get_ok();
  if (result_node != node) {
    std::stringstream ss;
    ss << "Result does not match expected: " << result_node;
    return ss.str();
  }

  return std::nullopt;
}

/**
 * @brief Test if it can convert to an empty map
 *
 * @return An error string on error
 */
std::optional<std::string> test_to_map_empty() {
  termite::Node node = termite::map{};
  auto json_node = termite::to_JSON(node);
  auto result = termite::from_JSON(json_node);

  if (!result.is_ok()) {
    std::stringstream ss;
    ss << result.get_err();
    return ss.str();
  }
  auto result_node = result.get_ok();
  if (result_node != node) {
    std::stringstream ss;
    ss << "Result does not match expected: " << result_node;
    return ss.str();
  }

  return std::nullopt;
}

int main() {
  auto names = {
      "test_scalar",        "test_list",           "test_map",
      "test_to_scalar",     "test_to_list",        "test_to_map",
      "test_to_list_empty", "test_to_map_empty",   "test_json_string",
      "test_json_file",     "test_to_json_string", "test_to_json_file",
  };
  auto functions = {
      test_scalar,        test_list,           test_map,
      test_to_scalar,     test_to_list,        test_to_map,
      test_to_list_empty, test_to_map_empty,   test_json_string,
      test_json_file,     test_to_json_string, test_to_json_file,
  };

  std::cout << "Running " << names.size() << " tests" << std::endl;

  int progress = 1;
  int return_value = 0;
  auto name_it = names.begin();
  for (auto function_it = functions.begin(); function_it < functions.end();
       ++function_it, ++name_it, ++progress) {
    if (auto error = (*function_it)()) {
      std::cout << "Error occured at \"" << *name_it << "\": " << *error
                << std::endl;
      if (return_value == 0) {
        return_value = progress;
      }
    }
  }

  if (return_value == 0) {
    std::cout << "No errors" << std::endl;
  }

  return return_value;
}
