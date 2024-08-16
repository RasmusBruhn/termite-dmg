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
  [[nodiscard]] bool operator==(const DataType &x) {
    return value == x.value;
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
    os << "{ value: ";
    switch (x.value.index()) {
    case 0:
      os << "int " << std::get<int>(x.value);
      break;
    case 1:
      os << "float " << std::get<float>(x.value);
      break;
    default:
      os << "Unknown";
      break;
    }
    return os << " }";
  }

  /**
   * @brief The value of the variant
   * 
   */
  std::variant<int, float> value;
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
[[nodiscard]] Result<test::DataType> Node::to_value() const {
  std::stringstream error;
  error << "Unable to parse any variant: [ ";

  Result<int> result_int = to_value<int>();
  if (result_int.is_ok()) {
    return Result<test::DataType>::ok(test::DataType(result_int.get_ok()));
  }
  error << "int { " << result_int.get_err() << " }";
  error << ", ";

  Result<float> result_float = to_value<float>();
  if (result_float.is_ok()) {
    return Result<test::DataType>::ok(test::DataType(result_float.get_ok()));
  }
  error << "float { " << result_float.get_err() << " }";
  error << " ]";

  return Result<test::DataType>::err(Error(error.str()));
}

} // namespace termite



#endif
