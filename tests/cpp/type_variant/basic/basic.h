// Generated with the Termite Data Model Generator
#ifndef HEADER_TERMITE_H_INCLUDED
#define HEADER_TERMITE_H_INCLUDED

#include <iostream>
#include <sstream>
#include <optional>
#include <variant>
#include <algorithm>
#include <termite.hpp>



namespace test {

/**
 * @brief
 *
 */
struct DataType {
  /**
   * @brief Constructs a new DataType object
   *
   * @param value The value of the variant
   */
  explicit DataType(std::variant<int, float> value) : value(std::move(value)) {}

  /**
   * @brief Checks if this object and the other object are identical
   *
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType &x) const;
  /**
   * @brief Checks if this object and the other object are different
   *
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const DataType &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   *
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const DataType &x);

  /**
   * @brief The value of the variant
   *
   */
  std::variant<int, float> value;
};

} // namespace test

namespace termite {

template<>
[[nodiscard]] Result<test::DataType> Node::to_value<test::DataType>() const;

} // namespace termite



#endif
