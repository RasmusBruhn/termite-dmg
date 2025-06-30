/**
 * @file termite_json.hpp
 * @brief The c++ Termite Data Model Generator json interface allowing for
 * converting between a nlohmann::json and a termite::Node
 * @version 0.4.0
 * @date 2025-06-30
 *
 */

#ifndef TERMITE_JSON_H_INCLUDED
#define TERMITE_JSON_H_INCLUDED

#include "termite.hpp"
#include <nlohmann/json.hpp>

namespace termite {

/**
 * @brief Converts a nlohmann::json to a termite::Node
 *
 * @param node The node to convert
 * @return The termite node or an error if the node is not compatible
 */
Result<Node> from_JSON(const nlohmann::json &node);
/**
 * @brief Converts a JSON string to a termite::Node
 *
 * @param string The string to convert
 * @return The termite::Node or an error if the string is invalid
 */
[[nodiscard]] Result<Node> from_JSON_string(const std::string &string);
/**
 * @brief Reads a nlohmann::json from a file and converts it to a termite::Node
 *
 * @param path The path to the file to read
 * @return The termite::Node or an error if the file is invalid
 */
[[nodiscard]] Result<Node> from_JSON_file(const std::filesystem::path &path);
/**
 * @brief Constructs a termite object from a nlohmann::json
 *
 * @tparam T The type of the termite object to construct
 * @param node The nlohmann::json to convert
 * @return The termite object or an error if the node is not compatible
 */
template <typename T>
[[nodiscard]] Result<T> construct_from_JSON(const nlohmann::json &node) {
  Result<Node> result = from_JSON(node);
  if (!result.is_ok()) {
    return Result<T>::err(result.get_err());
  }
  return result.get_ok().to_value<T>();
}
/**
 * @brief Constructs a termite object from a JSON string
 *
 * @tparam T The type of the termite object to construct
 * @param string The JSON string to convert
 * @return The termite object or an error if the string is invalid
 */
template <typename T>
[[nodiscard]] Result<T> construct_from_JSON_string(const std::string &string) {
  Result<Node> result = from_JSON_string(string);
  if (!result.is_ok()) {
    return Result<T>::err(result.get_err());
  }
  return result.get_ok().to_value<T>();
}
/**
 * @brief Constructs a termite object from a JSON file
 *
 * @tparam T The type of the termite object to construct
 * @param path The path to the JSON file to read
 * @return The termite object or an error if the file is invalid
 */
template <typename T>
[[nodiscard]] Result<T>
construct_from_JSON_file(const std::filesystem::path &path) {
  Result<Node> result = from_JSON_file(path);
  if (!result.is_ok()) {
    return Result<T>::err(result.get_err());
  }
  return result.get_ok().to_value<T>();
}

/**
 * @brief Converts a termite::Node to a nlohmann::json
 *
 * @param node The node to convert
 * @return The json node
 */
nlohmann::json to_JSON(const Node &node);
/**
 * @brief Converts a termite::Node to a JSON string
 *
 * @param node The node to convert
 * @return The JSON string
 */
[[nodiscard]] std::string to_JSON_string(const Node &node);
/**
 * @brief Converts a termite::Node to a JSON file
 *
 * @param node The node to convert
 * @param path The path to the file to write
 * @return Ok on success, Error if the file could not be written
 */
[[nodiscard]] Result<Empty> to_JSON_file(const Node &node,
                                         const std::filesystem::path &path);
/**
 * @brief Converts a termite object to a nlohmann::json
 *
 * @tparam T The type of the termite object to convert
 * @param value The value to convert
 * @return The nlohmann::json representation of the value
 */
template <typename T> [[nodiscard]] nlohmann::json termite_to_JSON(const T &value) {
  Node node = Node::from_value(value);
  return to_JSON(node);
}
/**
 * @brief Converts a termite object to a JSON string
 *
 * @tparam T The type of the termite object to convert
 * @param value The value to convert
 * @return The JSON string representation of the value
 */
template <typename T>
[[nodiscard]] std::string termite_to_JSON_string(const T &value) {
  Node node = Node::from_value(value);
  return to_JSON_string(node);
}
/**
 * @brief Converts a termite object to a nlohmann::json
 *
 * @tparam T The type of the termite object to convert
 * @param value The value to convert
 * @return The nlohmann::json representation of the value
 */
template <typename T>
[[nodiscard]] Result<Empty>
termite_to_JSON_file(const T &value, const std::filesystem::path &path) {
  Node node = Node::from_value(value);
  return to_JSON_file(node, path);
}

} // namespace termite

#endif
