/**
 * @file termite_yaml.hpp
 * @brief The c++ Termite Data Model Generator yaml-cpp interface allowing for
 * converting between a YAML::Node and a termite::Node
 * @version 0.1
 * @date 2024-06-15
 *
 */

#ifndef TERMITE_YAML_H_INCLUDED
#define TERMITE_YAML_H_INCLUDED

#include "termite.hpp"
#include <yaml-cpp/yaml.h>

namespace termite {

/**
 * @brief Converts a YAML::Node to a termite::Node
 * 
 * @param yaml_node The node to convert
 * @return The termite node or an error if the node is not compatible
 */
Result<Node> from_YAML(const YAML::Node &yaml_node) {
  // Convert a map
  if (yaml_node.IsMap()) {
    std::map<std::string, Node> map;
    for (const std::pair<YAML::Node, YAML::Node> &key_value : yaml_node) {
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
  if (yaml_node.IsSequence()) {
    std::vector<Node> list;
    size_t index = 0;
    for (YAML::const_iterator value_it = yaml_node.begin(); value_it != yaml_node.end(); ++value_it, ++index) {
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
  if (yaml_node.IsScalar()) {
    // Read the value
    std::string value;
    try {
      value = yaml_node.as<std::string>();
    } catch (const std::exception &err) {
      std::stringstream ss;
      ss << "Unable to read value: " << err.what();
      return Result<Node>::err(Error(ss.str()));
    }

    // Return the node
    return Result<Node>::ok(Node(Node::Value(std::move(value))));
  }

  // Return an error
  return Result<Node>::err(Error("Unknown node type, must be either Scalar, Map or Sequence"));
}

}

#endif
