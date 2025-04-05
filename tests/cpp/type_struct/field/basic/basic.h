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
public:
  /**
   * @brief Constructs a new DataType object
   *
   * @param field1
   * @param field2
   * @param extra_fields Any extra fields to attach to this struct
   */
  explicit DataType(int field1, float field2, ::termite::Node::Map extra_fields = ::termite::Node::Map()) : field1(std::move(field1)), field2(std::move(field2)), extra_fields(std::move(extra_fields)) {}


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
   * @brief
   *
   */
  int field1;
  /**
   * @brief
   *
   */
  float field2;
  /**
   * @brief All extra fields from when reading which could not be captured
   *
   */
  ::termite::Node::Map extra_fields;
};

} // namespace test

namespace termite {

template<>
[[nodiscard]] Result<test::DataType> Node::Map::to_value<test::DataType>() const;

template<>
[[nodiscard]] Node Node::from_value<test::DataType>(const test::DataType &value);

} // namespace termite



#endif
