/**
 * @file termite_json.hpp
 * @brief The c++ Termite Data Model Generator nlohmann::json interface allowing
 * for converting between a nlohmann::json and a termite::Node
 * @version 0.3.0
 * @date 2025-06-24
 *
 */

#include "termite-json.h"

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

}  // namespace termite
