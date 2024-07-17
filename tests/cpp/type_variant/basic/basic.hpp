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
class DataType {
public:
  /**
   * @brief The types of variants
   * 
   */
  enum Variant {
    kInt = 0,
    kFloat = 1,
  };

  /**
   * @brief Constructs a new DataType object
   * 
   * @param value
   * @return The new variant
   */
  [[nodiscard]] static DataType from_values(std::variant<int, float> value) {
    return DataType(std::move(value));
  }

  /**
   * @brief Sets the value as a int
   * 
   * @param value The value to set
   */
  void set_int(int value) {
    value_ = std::move(value);
  }
  /**
   * @brief Sets the value as a float
   * 
   * @param value The value to set
   */
  void set_float(float value) {
    value_ = std::move(value);
  }
  /**
   * @brief Sets the value
   * 
   * @param value The value to set
   */
  void set_float(std::variant<int, float> value) {
    value_ = std::move(value);
  }

  /**
   * @brief Retrieves the type of variant stored
   * 
   * @return The type of variant
   */
  [[nodiscard]] Variant variant() const {
    return static_cast<Variant>(value_.index());
  }
  /**
   * @brief Moves the value out as a int
   * 
   * @return The value or an error if it is the wrong type
   */
  [[nodiscard]] termite::Result<int> get_int() {
    if (!std::holds_alternative<int>(value_)) {
      return termite::Result<int>::err(termite::Error("Value is not a int"));
    }

    return termite::Result<int>::ok(std::get<int>(std::move(value_)));
  }
  /**
   * @brief Moves the value out as a float
   * 
   * @return The value or an error if it is the wrong type
   */
  [[nodiscard]] termite::Result<float> get_float() {
    if (!std::holds_alternative<float>(value_)) {
      return termite::Result<float>::err(termite::Error("Value is not a float"));
    }

    return termite::Result<float>::ok(std::get<float>(std::move(value_)));
  }
  /**
   * @brief Retrieves a reference to the value
   * 
   * @return The reference or an error if it is the wrong type
   */
  [[nodiscard]] const std::variant<int, float> &get_float() const {
    return value_;
  }

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType &x) {
    return value_ == x.value_;
  }
  /**
   * @brief Checks if this object and the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const DataType &x) {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const DataType &x) {
    return os << "{ value: ";
    switch (x.value_.index())
    {
    case 0:
      os << "int " << std::get<int>(x.value_);
      break;
    case 1:
      os << "float " << std::get<float>(x.value_);
      break;
    default:
      os << "Unknown";
      break;
    }
    return os << " }";
  }

private:
  explicit DataType(std::variant<int, float> value) : value_(std::move(value)) {}

  /**
   * @brief The value of the variant
   * 
   */
  std::variant<int, float> value_;
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
[[nodiscard]] Result<test::DataType> Node::to_value(bool allow_skipping) const {
  std::stringstream error;
  error << "Unable to parse any variant: [ ";

  Result<int> result_int = to_value<int>(allow_skipping);
  if (result_int.is_ok()) {
    return Result<test::DataType>::ok(test::DataType::from_values(result_int.get_ok()));
  }
  error << "int { " << result_int.get_err() << " }";
  error << ", ";

  Result<float> result_float = to_value<float>(allow_skipping);
  if (result_float.is_ok()) {
    return Result<test::DataType>::ok(test::DataType::from_values(result_float.get_ok()));
  }
  error << "float { " << result_float.get_err() << " }";
  error << " ]";

  return Result<test::DataType>::err(Error(error.str()));
}

} // namespace termite



#endif
