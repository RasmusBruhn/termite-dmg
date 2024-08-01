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
   * @param values The values of the array
   */
  explicit DataType1(std::vector<int> values) : values_(std::move(values)) {}

  /**
   * @brief Sets the values
   * 
   * @param values The values to set
   */
  void set_values(std::vector<int> values) { values_ = std::move(values); }

  /**
   * @brief Pushes a single value
   * 
   * @param value The value to set
   */
  void push_value(int value) { values_.push_back(std::move(value)); }

  /**
   * @brief Retrieves a reference to the values
   * 
   * @return The reference
   */
  [[nodiscard]] const std::vector<int> &get_values() const { return values_; }

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType1 &x) {
    if (values_.size() != x.values_.size()) {
      return false;
    }

    for (auto lhs = values_.cbegin(), rhs = x.values_.cbegin(); lhs < values_.cend(); ++lhs, ++rhs) {
      if (*lhs != *rhs) {
        return false;
      }
    }

    return true;
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
    os << "{ values: [ ";
    for (auto value = x.values_.cbegin(); value < x.values_.cend(); ++value) {
      if (value != x.values_.cbegin()) {
        os << ", ";
      }
      os << *value;
    }
    return os << " ] }";
  }

private:
  /**
   * @brief The values of the array
   * 
   */
  std::vector<int> values_;
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
   * @param values The values of the array
   */
  explicit DataType2(std::vector<float> values) : values_(std::move(values)) {}

  /**
   * @brief Sets the values
   * 
   * @param values The values to set
   */
  void set_values(std::vector<float> values) { values_ = std::move(values); }

  /**
   * @brief Pushes a single value
   * 
   * @param value The value to set
   */
  void push_value(float value) { values_.push_back(std::move(value)); }

  /**
   * @brief Retrieves a reference to the values
   * 
   * @return The reference
   */
  [[nodiscard]] const std::vector<float> &get_values() const { return values_; }

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType2 &x) {
    if (values_.size() != x.values_.size()) {
      return false;
    }

    for (auto lhs = values_.cbegin(), rhs = x.values_.cbegin(); lhs < values_.cend(); ++lhs, ++rhs) {
      if (*lhs != *rhs) {
        return false;
      }
    }

    return true;
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
    os << "{ values: [ ";
    for (auto value = x.values_.cbegin(); value < x.values_.cend(); ++value) {
      if (value != x.values_.cbegin()) {
        os << ", ";
      }
      os << *value;
    }
    return os << " ] }";
  }

private:
  /**
   * @brief The values of the array
   * 
   */
  std::vector<float> values_;
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
[[nodiscard]] Result<test::DataType1> NodeList::to_value(bool allow_skipping) const {
  std::vector<int> values;
  values.reserve(list_.size());
  for (auto node = list_.cbegin(); node < list_.cend(); ++node) {
    Result<int> value = node->to_value<int>(allow_skipping);
    if (!value.is_ok()) {
      Error error = value.get_err();
      error.add_list(node - list_.cbegin());
      return Result<test::DataType1>::err(std::move(error));
    }
    values.push_back(std::move(value.get_ok()));
  }

  return Result<test::DataType1>::ok(test::DataType1(std::move(values)));
}

template<>
[[nodiscard]] Result<test::DataType2> NodeList::to_value(bool allow_skipping) const {
  std::vector<float> values;
  values.reserve(list_.size());
  for (auto node = list_.cbegin(); node < list_.cend(); ++node) {
    Result<float> value = node->to_value<float>(allow_skipping);
    if (!value.is_ok()) {
      Error error = value.get_err();
      error.add_list(node - list_.cbegin());
      return Result<test::DataType2>::err(std::move(error));
    }
    values.push_back(std::move(value.get_ok()));
  }

  return Result<test::DataType2>::ok(test::DataType2(std::move(values)));
}

} // namespace termite



#endif