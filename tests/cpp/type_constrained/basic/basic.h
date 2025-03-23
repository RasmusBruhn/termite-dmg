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
class DataType1 {
public:
  /**
   * @brief Constructs a new DataType1 object, it must be valid or an exception will be thrown
   *
   * @param value The value to store
   */
  explicit DataType1(int value) : DataType1(from_value(std::move(value)).get_ok()) {}
  /**
   * @brief Constructs a new DataType1 object
   *
   * @param value The value to store
   * @return The new constrained type or an error if some constraints were not upheld
   */
  [[nodiscard]] static termite::Result<DataType1> from_value(int value);

  /**
   * @brief Sets the value if it fulfills the constraints:
   *
   * @param value The value to set
   * @return An error if one of the constraints were not fulfilled
   */
  [[nodiscard]] termite::Result<termite::Empty> set(int value);

  /**
   * @brief Retrieves a reference to the value
   *
   * @return The reference
   */
  [[nodiscard]] const int &get() const {
    return value_;
  }

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

private:
  /**
   * @brief Constructs a new DataType1 object
   *
   * @param value The value to store
   * @param _ A nullptr
   */
  explicit DataType1(int value, void *) : value_(std::move(value)) {}

  /**
   * @brief Validates if value is correct using the following constaints:
   *
   * @param x The value of the parameter to validate
   */
  [[nodiscard]] static termite::Result<termite::Empty> validate(const int &x);

  /**
   * @brief The validated value
   *
   */
  int value_;
};

/**
 * @brief
 *
 */
class DataType2 {
public:
  /**
   * @brief Constructs a new DataType2 object, it must be valid or an exception will be thrown
   *
   * @param value The value to store
   */
  explicit DataType2(float value) : DataType2(from_value(std::move(value)).get_ok()) {}
  /**
   * @brief Constructs a new DataType2 object
   *
   * @param value The value to store
   * @return The new constrained type or an error if some constraints were not upheld
   */
  [[nodiscard]] static termite::Result<DataType2> from_value(float value);

  /**
   * @brief Sets the value if it fulfills the constraints:
   *
   * @param value The value to set
   * @return An error if one of the constraints were not fulfilled
   */
  [[nodiscard]] termite::Result<termite::Empty> set(float value);

  /**
   * @brief Retrieves a reference to the value
   *
   * @return The reference
   */
  [[nodiscard]] const float &get() const {
    return value_;
  }

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

private:
  /**
   * @brief Constructs a new DataType2 object
   *
   * @param value The value to store
   * @param _ A nullptr
   */
  explicit DataType2(float value, void *) : value_(std::move(value)) {}

  /**
   * @brief Validates if value is correct using the following constaints:
   *
   * @param x The value of the parameter to validate
   */
  [[nodiscard]] static termite::Result<termite::Empty> validate(const float &x);

  /**
   * @brief The validated value
   *
   */
  float value_;
};

} // namespace test

namespace termite {

template<>
[[nodiscard]] Result<test::DataType1> Node::to_value<test::DataType1>() const;

template<>
[[nodiscard]] Result<test::DataType2> Node::to_value<test::DataType2>() const;

} // namespace termite



#endif
