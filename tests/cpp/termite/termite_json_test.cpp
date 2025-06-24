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
  termite::Node correct(termite::Node::Value("Test"));
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
  std::vector<termite::Node> list;
  list.emplace_back(termite::Node::Value("Test1"));
  list.emplace_back(termite::Node::Value("Test2"));
  termite::Node correct(termite::Node::List(std::move(list)));
  nlohmann::json node;
  node.push_back(nlohmann::json("Test1"));
  node.push_back(nlohmann::json("Test2"));
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
  nlohmann::json node;
  node["field1"] = nlohmann::json("Test1");
  node["field2"] = nlohmann::json("Test2");
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
 * @brief Test if it can convert to a scalar
 *
 * @return An error string on error
 */
std::optional<std::string> test_to_scalar() {
  termite::Node node(termite::Node::Value("Test"));
  nlohmann::json result = termite::to_JSON(node);

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
  std::vector<termite::Node> list;
  list.emplace_back(termite::Node::Value("Test1"));
  list.emplace_back(termite::Node::Value("Test2"));
  termite::Node node(termite::Node::List(std::move(list)));
  nlohmann::json result = termite::to_JSON(node);

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
  std::map<std::string, termite::Node> map;
  map.insert(
      std::make_pair("field1", termite::Node(termite::Node::Value("Test1"))));
  map.insert(
      std::make_pair("field2", termite::Node(termite::Node::Value("Test2"))));
  termite::Node node(termite::Node::Map(std::move(map)));
  nlohmann::json result = termite::to_JSON(node);

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

int main() {
  auto names = {
      "test_scalar",    "test_list",    "test_map",
      "test_to_scalar", "test_to_list", "test_to_map",
  };
  auto functions = {
      test_scalar,    test_list,    test_map,
      test_to_scalar, test_to_list, test_to_map,
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
