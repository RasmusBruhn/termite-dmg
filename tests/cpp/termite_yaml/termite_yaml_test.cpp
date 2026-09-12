#include <optional>
#include <sstream>
#include <string>
#include <termite-yaml.h>

/**
 * @brief Test if it can convert a scalar
 *
 * @return An error string on error
 */
std::optional<std::string> test_scalar() {
  termite::Node correct = "Test";
  YAML::Node node("Test");
  auto result = termite::from_YAML(node);

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
 * @brief Test if it can convert a list
 *
 * @return An error string on error
 */
std::optional<std::string> test_list() {
  termite::Node correct = termite::list{"Test1", "Test2"};
  YAML::Node node;
  node.push_back(YAML::Node("Test1"));
  node.push_back(YAML::Node("Test2"));
  auto result = termite::from_YAML(node);

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
  YAML::Node node;
  node["field1"] = YAML::Node("Test1");
  node["field2"] = YAML::Node("Test2");
  auto result = termite::from_YAML(node);

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
 * @brief Test if it can handle a type error
 *
 * @return An error string on error
 */
std::optional<std::string> test_type_error() {
  YAML::Node node;
  auto result = termite::from_YAML(node);

  std::cout << "TypeError: " << result << std::endl;

  if (result.is_ok()) {
    return "Should be an error";
  }

  return std::nullopt;
}

/**
 * @brief Test if it can handle a list error
 *
 * @return An error string on error
 */
std::optional<std::string> test_list_error() {
  YAML::Node node;
  node.push_back(YAML::Node());
  auto result = termite::from_YAML(node);

  if (result.is_ok()) {
    return "Should be an error";
  }

  return std::nullopt;
}

/**
 * @brief Test if it can handle a map error
 *
 * @return An error string on error
 */
std::optional<std::string> test_map_error() {
  YAML::Node node;
  node["field1"] = YAML::Node();
  auto result = termite::from_YAML(node);

  if (result.is_ok()) {
    return "Should be an error";
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
  auto result = termite::to_YAML(node);

  if (!result.IsScalar()) {
    return "Should be a scalar";
  }
  if (result.as<std::string>() != "Test") {
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
  auto result = termite::to_YAML(node);

  if (!result.IsSequence()) {
    return "Should be a sequence";
  }
  if (result.size() != 2) {
    return "Wrong size";
  }
  if (result[0].as<std::string>() != "Test1") {
    return "Wrong value [0]";
  }
  if (result[1].as<std::string>() != "Test2") {
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
  auto result = termite::to_YAML(node);

  if (!result.IsMap()) {
    return "Should be a map";
  }
  if (result.size() != 2) {
    return "Wrong size";
  }
  if (result["field1"].as<std::string>() != "Test1") {
    return "Wrong value [0]";
  }
  if (result["field2"].as<std::string>() != "Test2") {
    return "Wrong value [1]";
  }

  return std::nullopt;
}

/**
 * @brief Test if it can convert from a YAML string
 *
 * @return An error string on error
 */
std::optional<std::string> test_yaml_string() {
  std::string yaml_string = "{ field1: Test1, field2: [Test2, Test3]}";
  termite::Node correct = termite::map{
      {"field1", "Test1"}, {"field2", termite::list{"Test2", "Test3"}}};
  auto result = termite::from_YAML_string(yaml_string);

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
 * @brief Test if it can convert from a YAML file
 *
 * @return An error string on error
 */
std::optional<std::string> test_yaml_file() {
  termite::Node correct = termite::map{
      {"field1", "Test1"}, {"field2", termite::list{"Test2", "Test3"}}};
  auto result = termite::from_YAML_file("../yaml_test.yaml");

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
 * @brief Test if it can convert to a YAML string
 *
 * @return An error string on error
 */
std::optional<std::string> test_to_yaml_string() {
  termite::Node correct = termite::map{
      {"field1", "Test1"}, {"field2", termite::list{"Test2", "Test3"}}};
  auto yaml_string = termite::to_YAML_string(correct);
  auto result = termite::from_YAML_string(yaml_string);

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
 * @brief Test if it can convert to a YAML file
 *
 * @return An error string on error
 */
std::optional<std::string> test_to_yaml_file() {
  termite::Node correct = termite::map{
      {"field1", "Test1"}, {"field2", termite::list{"Test2", "Test3"}}};

  auto write_result = termite::to_YAML_file(correct, "yaml_test.yaml");
  if (!write_result.is_ok()) {
    std::stringstream ss;
    ss << write_result.get_err();
    return ss.str();
  }

  auto result = termite::from_YAML_file("yaml_test.yaml");

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
  auto yaml_node = termite::to_YAML(node);
  auto result = termite::from_YAML(yaml_node);

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
  auto yaml_node = termite::to_YAML(node);
  auto result = termite::from_YAML(yaml_node);

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
      "test_type_error",    "test_list_error",     "test_map_error",
      "test_to_scalar",     "test_to_list",        "test_to_map",
      "test_to_list_empty", "test_to_map_empty",   "test_yaml_string",
      "test_yaml_file",     "test_to_yaml_string", "test_to_yaml_file",
  };
  auto functions = {
      test_scalar,     test_list,           test_map,          test_type_error,
      test_list_error, test_map_error,      test_to_scalar,    test_to_list,
      test_to_map,     test_to_list_empty,  test_to_map_empty, test_yaml_string,
      test_yaml_file,  test_to_yaml_string, test_to_yaml_file,
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
