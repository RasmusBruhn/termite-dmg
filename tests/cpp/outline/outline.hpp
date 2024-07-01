// Generated with the Termite Data Model Generator
#ifndef HEADER_TERMITE_H_INCLUDED
#define HEADER_TERMITE_H_INCLUDED

#include <optional>
#include <termite.hpp>
#include <variant>

namespace test {

/**
 * \\brief description1
 *
 */
class DataType1 {
public:
  /**
   * \\brief Constructs a new DataType1 object
   *
   *
   *
   */
  [[nodiscard]] static termite::Result<DataType1> new_instance() {
    return termite::Result<DataType1>::ok(DataType1());
  }

  /**
   * \\brief Checks if this object the the other object are identical
   *
   * \\param other The other object to compare with
   * \\return true if they are identical, false if not
   *
   */
  [[nodiscard]] bool operator==(const DataType1 &other) const { return true; }
  /**
   * \\brief Checks if this object the the other object are different
   *
   * \\param other The other object to compare with
   * \\return true if they are different, false if not
   *
   */
  [[nodiscard]] bool operator!=(const DataType1 &other) const {
    return !(*this == other);
  }
  /**
   * \\brief Prints the object onto the output stream
   * 
   * \\param os The output stream to print to
   * \\param value The object to print
   * \\return The output stream
   * 
   */
  friend std::ostream &operator<<(std::ostream &os, const DataType1 &value) {
    return os << "{ " << "" << " }";
  }

private:
  explicit DataType1() {}
};

/**
 * \\brief description2
 *
 */
class DataType2 {
public:
  /**
   * \\brief Constructs a new DataType2 object
   *
   *
   *
   */
  [[nodiscard]] static termite::Result<DataType2> new_instance() {
    return termite::Result<DataType2>::ok(DataType2());
  }

  /**
   * \\brief Checks if this object the the other object are identical
   *
   * \\param other The other object to compare with
   * \\return true if they are identical, false if not
   *
   */
  [[nodiscard]] bool operator==(const DataType2 &other) const { return true; }
  /**
   * \\brief Checks if this object the the other object are different
   *
   * \\param other The other object to compare with
   * \\return true if they are different, false if not
   *
   */
  [[nodiscard]] bool operator!=(const DataType2 &other) const {
    return !(*this == other);
  }
  /**
   * \\brief Prints the object onto the output stream
   * 
   * \\param os The output stream to print to
   * \\param value The object to print
   * \\return The output stream
   * 
   */
  friend std::ostream &operator<<(std::ostream &os, const DataType2 &value) {
    return os << "{ " << "" << " }";
  }

private:
  explicit DataType2() {}
};

}  // namespace test

#endif
