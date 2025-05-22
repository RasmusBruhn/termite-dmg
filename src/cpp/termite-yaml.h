/**
 * @file termite_yaml.hpp
 * @brief The c++ Termite Data Model Generator yaml-cpp interface allowing for
 * converting between a YAML::Node and a termite::Node
 * @version 0.2.1
 * @date 2025-05-22
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
Result<Node> from_YAML(const YAML::Node &node);

/**
 * @brief Converts a termite::Node to a YAML::Node
 *
 * @param node The node to convert
 * @return The yaml node
 */
YAML::Node to_YAML(const Node &node);

} // namespace termite

#endif
