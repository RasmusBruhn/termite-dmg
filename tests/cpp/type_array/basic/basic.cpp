// Generated with the Termite Data Model Generator
#include "basic.h"



namespace {

// Code to make printing easier
template <typename T, typename = void>
struct has_insertion_operator : std::false_type {};
template <typename T>
struct has_insertion_operator<T, std::void_t<decltype(std::declval<std::ostream &>() << std::declval<T>())>> : std::true_type {};

template <typename T>
typename std::enable_if<has_insertion_operator<T>::value, std::ostream &>::type
operator<<(std::ostream &os, const std::optional<T> &value) {
  if (value) {
    return os << *value;
  } else {
    return os << "nullopt";
  }
}

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

namespace test {

/**
 * @brief
 *
 */
bool DataType1::operator==(const DataType1 &x) const {
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

std::ostream &operator<<(std::ostream &os, const DataType1 &x) {
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
 * @brief
 *
 */
bool DataType2::operator==(const DataType2 &x) const {
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

std::ostream &operator<<(std::ostream &os, const DataType2 &x) {
  os << "{ values: [ ";
  for (auto value = x.values.cbegin(); value < x.values.cend(); ++value) {
    if (value != x.values.cbegin()) {
      os << ", ";
    }
    os << *value;
  }
  return os << " ] }";
}

} // namespace test

namespace termite {

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

