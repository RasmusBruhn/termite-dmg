/**
 * @file termite_json.hpp
 * @brief The c++ Termite Data Model Generator json interface allowing for
 * converting between a nlohmann::json and a termite::Node
 * @version 0.3.0
 * @date 2025-06-24
 *
 */

#ifndef TERMITE_JSON_H_INCLUDED
#define TERMITE_JSON_H_INCLUDED

#include <nlohmann/json.hpp>

#include "termite.hpp"

namespace termite {

/**
 * @brief Converts a nlohmann::json to a termite::Node
 *
 * @param node The node to convert
 * @return The termite node or an error if the node is not compatible
 */
Result<Node> from_JSON(const nlohmann::json &node);

/**
 * @brief Converts a termite::Node to a nlohmann::json
 *
 * @param node The node to convert
 * @return The json node
 */
nlohmann::json to_JSON(const Node &node);

} // namespace termite

#endif
