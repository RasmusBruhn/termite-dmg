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
struct DataType1 {
public:
  /**
   * @brief Constructs a new DataType1 object
   * 
   * @param values The values of the array
   */
  explicit DataType1(std::vector<int> values) : values(std::move(values)) {}

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType1 &x) const {
    if (values.size() != x.values.size()) {
      return false;
    }

    for (auto lhs = values.cbegin(), rhs = x.values.cbegin(); lhs < values.cend(); ++lhs, ++rhs) {
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
  friend std::ostream &operator<<(std::ostream &os, const DataType1 &x) {
    os << "{ values: [ ";
    for (auto value = x.values.cbegin(); value < x.values.cend(); ++value) {
      if (value != x.values.cbegin()) {
        os << ", ";
      }
      os << *value;
    }
    return os << " ] }";
  }

  /**
   * @brief The values of the array
   * 
   */
  std::vector<int> values;
};

/**
 * @brief 
 * 
 */
struct DataType2 {
public:
  /**
   * @brief Constructs a new DataType2 object
   * 
   * @param values The values of the array
   */
  explicit DataType2(std::vector<float> values) : values(std::move(values)) {}

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType2 &x) const {
    if (values.size() != x.values.size()) {
      return false;
    }

    for (auto lhs = values.cbegin(), rhs = x.values.cbegin(); lhs < values.cend(); ++lhs, ++rhs) {
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
  friend std::ostream &operator<<(std::ostream &os, const DataType2 &x) {
    os << "{ values: [ ";
    for (auto value = x.values.cbegin(); value < x.values.cend(); ++value) {
      if (value != x.values.cbegin()) {
        os << ", ";
      }
      os << *value;
    }
    return os << " ] }";
  }

  /**
   * @brief The values of the array
   * 
   */
  std::vector<float> values;
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
[[nodiscard]] Result<test::DataType1> Node::List::to_value<test::DataType1>() const {
  std::vector<int> values;
  values.reserve(list_.size());
  for (auto node = list_.cbegin(); node < list_.cend(); ++node) {
    Result<int> value = node->to_value<int>();
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
[[nodiscard]] Result<test::DataType2> Node::List::to_value<test::DataType2>() const {
  std::vector<float> values;
  values.reserve(list_.size());
  for (auto node = list_.cbegin(); node < list_.cend(); ++node) {
    Result<float> value = node->to_value<float>();
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