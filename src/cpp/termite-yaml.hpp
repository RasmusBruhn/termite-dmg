/**
 * @file termite_yaml.hpp
 * @brief The c++ Termite Data Model Generator yaml-cpp interface allowing for
 * converting between a YAML::Node and a termite::Node
 * @version 0.2
 * @date 2025-03-25
 *
 */

#ifndef TERMITE_YAML_H_INCLUDED
#define TERMITE_YAML_H_INCLUDED

#include <yaml-cpp/yaml.h>

#include "termite.hpp"

namespace termite {

/**
 * @brief Converts a YAML::Node to a termite::Node
 *
 * @param node The node to convert
 * @return The termite node or an error if the node is not compatible
 */
Result<Node> from_YAML(const YAML::Node &node) {
  // Convert a map
  if (node.IsMap()) {
    std::map<std::string, Node> map;
    for (const std::pair<YAML::Node, YAML::Node> &key_value : node) {
      // Get the key
      std::string key;
      try {
        key = key_value.first.as<std::string>();
      } catch (const std::exception &err) {
        std::stringstream ss;
        ss << "Unable to read key: " << err.what();
        return Result<Node>::err(Error(ss.str()));
      }

      // Get the value
      Result<Node> value = from_YAML(key_value.second);
      if (!value.is_ok()) {
        return Result<Node>::err(value.get_err().add_field(key));
      }

      // Add to the map
      map.insert(std::make_pair(std::move(key), value.get_ok()));
    }

    // Return the node
    return Result<Node>::ok(Node(Node::Map(std::move(map))));
  }

  // Convert a list
  if (node.IsSequence()) {
    std::vector<Node> list;
    size_t index = 0;
    for (YAML::const_iterator value_it = node.begin(); value_it != node.end();
         ++value_it, ++index) {
      // Get the value
      Result<Node> value = from_YAML(*value_it);
      if (!value.is_ok()) {
        return Result<Node>::err(value.get_err().add_list(index));
      }

      // Add to the list
      list.push_back(value.get_ok());
    }

    // Return the node
    return Result<Node>::ok(Node(Node::List(std::move(list))));
  }

  // Convert a scalar
  if (node.IsScalar()) {
    // Read the value
    std::string value;
    try {
      value = node.as<std::string>();
    } catch (const std::exception &err) {
      std::stringstream ss;
      ss << "Unable to read value: " << err.what();
      return Result<Node>::err(Error(ss.str()));
    }

    // Return the node
    return Result<Node>::ok(Node(Node::Value(std::move(value))));
  }

  // Return an error
  return Result<Node>::err(
      Error("Unknown node type, must be either Scalar, Map or Sequence"));
}

/**
 * @brief Converts a termite::Node to a YAML::Node
 *
 * @param node The node to convert
 * @return The yaml node
 */
YAML::Node to_YAML(const Node &node) {
  if (std::holds_alternative<Node::Value>(node.get())) {
    return YAML::Node(std::get<Node::Value>(node.get()).get());
  }
  if (std::holds_alternative<Node::Map>(node.get())) {
    YAML::Node map;
    for (const std::pair<std::string, Node> &key_value :
         std::get<Node::Map>(node.get()).get()) {
      map[key_value.first] = to_YAML(key_value.second);
    }
    return map;
  }
  if (std::holds_alternative<Node::List>(node.get())) {
    YAML::Node list;
    for (const Node &key_value : std::get<Node::List>(node.get()).get()) {
      list.push_back(to_YAML(key_value));
    }
    return list;
  }
  return YAML::Node();
}

}  // namespace termite

#endif
