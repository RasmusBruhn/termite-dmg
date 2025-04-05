// Generated with the Termite Data Model Generator
#ifndef HEADER_TERMITE_H_INCLUDED
#define HEADER_TERMITE_H_INCLUDED

#include <iostream>
#include <sstream>
#include <optional>
#include <variant>
#include <algorithm>
#include <termite.hpp>

// Header header

namespace test {

/**
 * @brief description1
 *
 */
struct DataType1 {
public:
  /**
   * @brief Constructs a new DataType1 object
   *
   * @param extra_fields Any extra fields to attach to this struct
   */
  explicit DataType1(::termite::Node::Map extra_fields = ::termite::Node::Map()) : extra_fields(std::move(extra_fields)) {}


  /**
   * @brief Checks if this object and the other object are identical
   *
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType1 &x) const;
  /**
   * @brief Checks if this object and the other object are different
   *
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const DataType1 &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   *
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const DataType1 &x);

  /**
   * @brief All extra fields from when reading which could not be captured
   *
   */
  ::termite::Node::Map extra_fields;
};

/**
 * @brief description2
 *
 */
struct DataType2 {
public:
  /**
   * @brief Constructs a new DataType2 object
   *
   * @param extra_fields Any extra fields to attach to this struct
   */
  explicit DataType2(::termite::Node::Map extra_fields = ::termite::Node::Map()) : extra_fields(std::move(extra_fields)) {}


  /**
   * @brief Checks if this object and the other object are identical
   *
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType2 &x) const;
  /**
   * @brief Checks if this object and the other object are different
   *
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const DataType2 &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   *
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const DataType2 &x);

  /**
   * @brief All extra fields from when reading which could not be captured
   *
   */
  ::termite::Node::Map extra_fields;
};

} // namespace test

namespace termite {

template<>
[[nodiscard]] Result<test::DataType1> Node::Map::to_value<test::DataType1>() const;

template<>
[[nodiscard]] Node Node::from_value<test::DataType1>(const test::DataType1 &value);

template<>
[[nodiscard]] Result<test::DataType2> Node::Map::to_value<test::DataType2>() const;

template<>
[[nodiscard]] Node Node::from_value<test::DataType2>(const test::DataType2 &value);

} // namespace termite

// Footer header

#endif
