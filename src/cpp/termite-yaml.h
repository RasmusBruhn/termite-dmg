/**
 * @file termite_yaml.hpp
 * @brief The c++ Termite Data Model Generator yaml-cpp interface allowing for
 * converting between a YAML::Node and a termite::Node
 * @version 0.4.0
 * @date 2025-06-30
 *
 */

#ifndef TERMITE_YAML_H_INCLUDED
#define TERMITE_YAML_H_INCLUDED

#include "termite.hpp"
#include <filesystem>
#include <string>
#include <variant>
#include <yaml-cpp/yaml.h>

namespace termite {

/**
 * @brief Converts a YAML::Node to a termite::Node
 *
 * @param node The node to convert
 * @return The termite::Node or an error if the node is not compatible
 */
[[nodiscard]] Result<Node> from_YAML(const YAML::Node &node);
/**
 * @brief Converts a YAML string to a termite::Node
 *
 * @param string The string to convert
 * @return The termite::Node or an error if the string is invalid
 */
[[nodiscard]] Result<Node> from_YAML_string(const std::string &string);
/**
 * @brief Reads a YAML::Node from a file and converts it to a termite::Node
 *
 * @param path The path to the file to read
 * @return The termite::Node or an error if the file is invalid
 */
[[nodiscard]] Result<Node> from_YAML_file(const std::filesystem::path &path);
/**
 * @brief Constructs a termite object from a YAML::Node
 *
 * @tparam T The type of the termite object to construct
 * @param node The YAML::Node to convert
 * @return The termite object or an error if the node is not compatible
 */
template <typename T>
[[nodiscard]] Result<T> construct_from_YAML(const YAML::Node &node) {
  Result<Node> result = from_YAML(node);
  if (!result.is_ok()) {
    return Result<T>::err(result.get_err());
  }
  return result.get_ok().to_value<T>();
}
/**
 * @brief Constructs a termite object from a YAML string
 *
 * @tparam T The type of the termite object to construct
 * @param string The YAML string to convert
 * @return The termite object or an error if the string is invalid
 */
template <typename T>
[[nodiscard]] Result<T> construct_from_YAML_string(const std::string &string) {
  Result<Node> result = from_YAML_string(string);
  if (!result.is_ok()) {
    return Result<T>::err(result.get_err());
  }
  return result.get_ok().to_value<T>();
}
/**
 * @brief Constructs a termite object from a YAML file
 *
 * @tparam T The type of the termite object to construct
 * @param path The path to the YAML file to read
 * @return The termite object or an error if the file is invalid
 */
template <typename T>
[[nodiscard]] Result<T>
construct_from_YAML_file(const std::filesystem::path &path) {
  Result<Node> result = from_YAML_file(path);
  if (!result.is_ok()) {
    return Result<T>::err(result.get_err());
  }
  return result.get_ok().to_value<T>();
}

/**
 * @brief Converts a termite::Node to a YAML::Node
 *
 * @param node The node to convert
 * @return The yaml node
 */
[[nodiscard]] YAML::Node to_YAML(const Node &node);
/**
 * @brief Converts a termite::Node to a YAML string
 *
 * @param node The node to convert
 * @return The yaml string
 */
[[nodiscard]] std::string to_YAML_string(const Node &node);
/**
 * @brief Converts a termite::Node to a YAML file
 *
 * @param node The node to convert
 * @param path The path to the file to write
 * @return Ok on success, Error if the file could not be written
 */
[[nodiscard]] Result<Empty> to_YAML_file(const Node &node,
                                         const std::filesystem::path &path);
/**
 * @brief Converts a termite object to a YAML::Node
 *
 * @tparam T The type of the termite object to convert
 * @param value The value to convert
 * @return The YAML::Node representation of the value
 */
template <typename T> [[nodiscard]] YAML::Node termite_to_YAML(const T &value) {
  Node node = Node::from_value(value);
  return to_YAML(node);
}
/**
 * @brief Converts a termite object to a YAML string
 *
 * @tparam T The type of the termite object to convert
 * @param value The value to convert
 * @return The YAML string representation of the value
 */
template <typename T>
[[nodiscard]] std::string termite_to_YAML_string(const T &value) {
  Node node = Node::from_value(value);
  return to_YAML_string(node);
}
/**
 * @brief Converts a termite object to a YAML::Node
 *
 * @tparam T The type of the termite object to convert
 * @param value The value to convert
 * @return The YAML::Node representation of the value
 */
template <typename T>
[[nodiscard]] Result<Empty>
termite_to_YAML_file(const T &value, const std::filesystem::path &path) {
  Node node = Node::from_value(value);
  return to_YAML_file(node, path);
}

} // namespace termite

#endif
