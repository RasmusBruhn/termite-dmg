// Generated with the Termite Data Model Generator
#ifndef HEADER_TERMITE_H_INCLUDED
#define HEADER_TERMITE_H_INCLUDED

#include <iostream>
#include <sstream>
#include <optional>
#include <variant>
#include <algorithm>
#include <termite.hpp>



namespace {

template <typename T, typename = void>
struct has_insertion_operator : std::false_type {};

template <typename T>
struct has_insertion_operator<T, std::void_t<decltype(std::declval<std::ostream &>() << std::declval<T>())>> : std::true_type {};

} // namespace

namespace test {

namespace {

template <typename T>
typename std::enable_if<has_insertion_operator<T>::value, std::ostream &>::type
operator<<(std::ostream &os, const std::optional<T> &value) {
  if (value) {
    return os << *value;
  } else {
    return os << "nullopt";
  }
}

} // namespace

/**
 * @brief 
 * 
 */
class DataType1 {
public:
  /**
   * @brief Constructs a new DataType1 object
   * 
   * @param value The value to store 
   * @return The new constrained type or an error if some constraints were not upheld
   */
  [[nodiscard]] static termite::Result<DataType1> from_values(int value) {
    termite::Result<termite::Empty> validate_result = validate(value);
    if (!validate_result.is_ok()) {
      termite::Error error = validate_result.get_err();
      return termite::Result<DataType1>::err(std::move(error));
    }

    return termite::Result<DataType1>::ok(DataType1(std::move(value)));
  }

  /**
   * @brief Sets the value if it fulfills the constraints:
   * - x > 0
   * - x % 2 == 0
   * 
   * @param value The value to set
   * @return An error if one of the constraints were not fulfilled
   */
  [[nodiscard]] termite::Result<termite::Empty> set(int value) {
    termite::Result<termite::Empty> validate_result = validate(value);
    if (!validate_result.is_ok()) {
      return validate_result;
    }

    value_ = std::move(value);
    return termite::Result<termite::Empty>::ok(termite::Empty());
  }

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
  [[nodiscard]] bool operator==(const DataType1 &x) {
    return value_ == x.value_;
  }
  /**
   * @brief Checks if this object and the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const DataType1 &x) {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const DataType1 &x) {
    return os << x.value_;
  }

private:
  explicit DataType1(int value) : value_(std::move(value)) {}

  /**
   * @brief Validates if value is correct using the following constaints:
   * - x > 0
   * - x % 2 == 0
   * 
   * @param x The value of the parameter to validate
   */
  [[nodiscard]] static termite::Result<termite::Empty> validate(const int &x) {
    if (!(x > 0)) {
      return termite::Result<termite::Empty>::err(termite::Error("Did not pass constaint: x > 0"));
    }

    if (!(x % 2 == 0)) {
      return termite::Result<termite::Empty>::err(termite::Error("Did not pass constaint: x % 2 == 0"));
    }

    return termite::Result<termite::Empty>::ok(termite::Empty());
  }

  /**
   * @brief The value of the int
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
   * @brief Constructs a new DataType2 object
   * 
   * @param value The value to store 
   * @return The new constrained type or an error if some constraints were not upheld
   */
  [[nodiscard]] static termite::Result<DataType2> from_values(float value) {
    termite::Result<termite::Empty> validate_result = validate(value);
    if (!validate_result.is_ok()) {
      termite::Error error = validate_result.get_err();
      return termite::Result<DataType2>::err(std::move(error));
    }

    return termite::Result<DataType2>::ok(DataType2(std::move(value)));
  }

  /**
   * @brief Sets the value if it fulfills the constraints:
   * - std::abs(x) < 1e-9
   * 
   * @param value The value to set
   * @return An error if one of the constraints were not fulfilled
   */
  [[nodiscard]] termite::Result<termite::Empty> set(float value) {
    termite::Result<termite::Empty> validate_result = validate(value);
    if (!validate_result.is_ok()) {
      return validate_result;
    }

    value_ = std::move(value);
    return termite::Result<termite::Empty>::ok(termite::Empty());
  }

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
  [[nodiscard]] bool operator==(const DataType2 &x) {
    return value_ == x.value_;
  }
  /**
   * @brief Checks if this object and the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const DataType2 &x) {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const DataType2 &x) {
    return os << x.value_;
  }

private:
  explicit DataType2(float value) : value_(std::move(value)) {}

  /**
   * @brief Validates if value is correct using the following constaints:
   * - std::abs(x) < 1e-9
   * 
   * @param x The value of the parameter to validate
   */
  [[nodiscard]] static termite::Result<termite::Empty> validate(const float &x) {
    if (!(std::abs(x) < 1e-9)) {
      return termite::Result<termite::Empty>::err(termite::Error("Did not pass constaint: std::abs(x) < 1e-9"));
    }

    return termite::Result<termite::Empty>::ok(termite::Empty());
  }

  /**
   * @brief The value of the int
   * 
   */
  float value_;
};

} // namespace test

namespace termite {

namespace {

template <typename T>
typename std::enable_if<has_insertion_operator<T>::value, std::ostream &>::type
operator<<(std::ostream &os, const std::vector<T> &value) {
  os << "[ ";
  for (auto value_it = value.cbegin(); value_it != value.cend(); ++value_it) {
    if (value_it != value.cbegin()) {
      os << ", ";
    }
    os << *value_it;
  }
  return os << " ]";
}

} // namespace

template<>
[[nodiscard]] Result<test::DataType1> Node::to_value() const {
  Result<int> value = to_value<int>();
  if (!value.is_ok()) {
    return Result<test::DataType1>::err(Error(value.get_err()));
  }

  return test::DataType1::from_values(value.get_ok());
}

template<>
[[nodiscard]] Result<test::DataType2> Node::to_value() const {
  Result<float> value = to_value<float>();
  if (!value.is_ok()) {
    return Result<test::DataType2>::err(Error(value.get_err()));
  }

  return test::DataType2::from_values(value.get_ok());
}

} // namespace termite



#endif
