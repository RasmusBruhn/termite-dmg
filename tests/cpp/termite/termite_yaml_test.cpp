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
  termite::Node correct(termite::Node::Value("Test"));
  YAML::Node node("Test");
  termite::Result<termite::Node> result = termite::from_YAML(node);

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
  std::vector<termite::Node> list;
  list.emplace_back(termite::Node::Value("Test1"));
  list.emplace_back(termite::Node::Value("Test2"));
  termite::Node correct(termite::Node::List(std::move(list)));
  YAML::Node node;
  node.push_back(YAML::Node("Test1"));
  node.push_back(YAML::Node("Test2"));
  termite::Result<termite::Node> result = termite::from_YAML(node);

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
 * @brief Test if it can convert a map
 *
 * @return An error string on error
 */
std::optional<std::string> test_map() {
  std::map<std::string, termite::Node> map;
  map.insert(
      std::make_pair("field1", termite::Node(termite::Node::Value("Test1"))));
  map.insert(
      std::make_pair("field2", termite::Node(termite::Node::Value("Test2"))));
  termite::Node correct(termite::Node::Map(std::move(map)));
  YAML::Node node;
  node["field1"] = YAML::Node("Test1");
  node["field2"] = YAML::Node("Test2");
  termite::Result<termite::Node> result = termite::from_YAML(node);

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
 * @brief Test if it can handle a type error
 *
 * @return An error string on error
 */
std::optional<std::string> test_type_error() {
  YAML::Node node;
  termite::Result<termite::Node> result = termite::from_YAML(node);

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
  termite::Result<termite::Node> result = termite::from_YAML(node);

  std::cout << "TypeError: " << result << std::endl;

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
  termite::Result<termite::Node> result = termite::from_YAML(node);

  std::cout << "TypeError: " << result << std::endl;

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
  termite::Node node(termite::Node::Value("Test"));
  YAML::Node result = termite::to_YAML(node);

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
  std::vector<termite::Node> list;
  list.emplace_back(termite::Node::Value("Test1"));
  list.emplace_back(termite::Node::Value("Test2"));
  termite::Node node(termite::Node::List(std::move(list)));
  YAML::Node result = termite::to_YAML(node);

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
  std::map<std::string, termite::Node> map;
  map.insert(
      std::make_pair("field1", termite::Node(termite::Node::Value("Test1"))));
  map.insert(
      std::make_pair("field2", termite::Node(termite::Node::Value("Test2"))));
  termite::Node node(termite::Node::Map(std::move(map)));
  YAML::Node result = termite::to_YAML(node);

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

int main() {
  auto names = {
      "test_scalar",     "test_list",       "test_map",
      "test_type_error", "test_list_error", "test_map_error",
      "test_to_scalar",  "test_to_list",    "test_to_map",
  };
  auto functions = {
      test_scalar,     test_list,       test_map,
      test_type_error, test_list_error, test_map_error,
      test_to_scalar,  test_to_list,    test_to_map,
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
