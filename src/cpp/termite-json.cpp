/**
 * @file termite_json.hpp
 * @brief The c++ Termite Data Model Generator nlohmann::json interface allowing
 * for converting between a nlohmann::json and a termite::Node
 * @version 0.4.0
 * @date 2025-07-16
 *
 */

#include "termite-json.h"

#include <fstream>

namespace termite {

Result<Node> from_JSON(const nlohmann::json &node) {
  // Convert a list
  if (node.is_array()) {
    std::vector<Node> list;
    size_t index = 0;
    for (auto value_it = node.begin(); value_it != node.end();
         ++value_it, ++index) {
      // Get the value
      Result<Node> value = from_JSON(*value_it);
      if (!value.is_ok()) {
        return Result<Node>::err(value.get_err().add_list(index));
      }

      // Add to the list
      list.push_back(value.get_ok());
    }

    // Return the node
    return Result<Node>::ok(Node(Node::List(std::move(list))));
  }

  // Convert a map
  if (node.is_structured()) {
    std::map<std::string, Node> map;
    for (const auto &key_value : node.items()) {
      // Get the key
      std::string key = key_value.key();

      // Get the value
      Result<Node> value = from_JSON(key_value.value());
      if (!value.is_ok()) {
        return Result<Node>::err(value.get_err().add_field(key));
      }

      // Add to the map
      map.insert(std::make_pair(std::move(key), value.get_ok()));
    }

    // Return the node
    return Result<Node>::ok(Node(Node::Map(std::move(map))));
  }

  // Convert a scalar
  if (node.is_primitive()) {
    // Read the value
    std::string value =
        node.is_string() ? static_cast<std::string>(node) : node.dump();

    // Return the node
    return Result<Node>::ok(Node(Node::Value(std::move(value))));
  }

  // Return an error
  return Result<Node>::err(Error(
      "Unknown node type, must be either Primitive, Structured or Array"));
}

Result<Node> from_JSON_string(const std::string &string) {
  try {
    return from_JSON(nlohmann::json::parse(string));
  } catch (const std::exception &e) {
    std::stringstream ss;
    ss << "Unable to parse JSON string: " << e.what();
    return Result<Node>::err(Error(ss.str()));
  }
}

Result<Node> from_JSON_file(const std::filesystem::path &path) {
  std::ifstream file(path);
  if (!file.is_open()) {
    std::stringstream ss;
    ss << "Unable to open file: " << path.generic_string();
    return Result<Node>::err(Error(ss.str()));
  }

  std::string json_string((std::istreambuf_iterator<char>(file)),
                          std::istreambuf_iterator<char>());

  file.close();
  if (file.fail()) {
    std::stringstream ss;
    ss << "Unable to read file: " << path.generic_string();
    return Result<Node>::err(Error(ss.str()));
  }
  return from_JSON_string(json_string);
}

nlohmann::json to_JSON(const Node &node) {
  if (std::holds_alternative<Node::Value>(node.get())) {
    return nlohmann::json(std::get<Node::Value>(node.get()).get());
  }
  if (std::holds_alternative<Node::Map>(node.get())) {
    nlohmann::json map;
    for (const std::pair<const std::string, Node> &key_value :
         std::get<Node::Map>(node.get()).get()) {
      map[key_value.first] = to_JSON(key_value.second);
    }
    return map;
  }
  if (std::holds_alternative<Node::List>(node.get())) {
    nlohmann::json list;
    for (const Node &key_value : std::get<Node::List>(node.get()).get()) {
      list.push_back(to_JSON(key_value));
    }
    return list;
  }
  return nlohmann::json();
}

std::string to_JSON_string(const Node &node) { return to_JSON(node).dump(); }

Result<Empty> to_JSON_file(const Node &node,
                           const std::filesystem::path &path) {
  std::string JSON_string = to_JSON_string(node);
  std::ofstream file(path);
  if (!file.is_open()) {
    std::stringstream ss;
    ss << "Unable to open file: " << path.generic_string();
    return Result<Empty>::err(Error(ss.str()));
  }
  file << JSON_string;
  file.close();
  if (file.fail()) {
    std::stringstream ss;
    ss << "Unable to write file: " << path.generic_string();
    return Result<Empty>::err(Error(ss.str()));
  }
  return Result<Empty>::ok(Empty{});
}

}  // namespace termite
