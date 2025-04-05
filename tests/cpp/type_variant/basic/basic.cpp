// Generated with the Termite Data Model Generator
#include "basic.h"



namespace test {

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

[[nodiscard]] bool DataType::operator==(const DataType &x) const {
  return value == x.value;
}

std::ostream &operator<<(std::ostream &os, const DataType &x) {
  os << "{ value: ";
  switch (x.value.index()) {
  case 0:
    os << "int " << std::get<int>(x.value);
    break;
  case 1:
    os << "float " << std::get<float>(x.value);
    break;
  default:
    os << "Unknown(" << x.value.index() << ")";
    break;
  }
  return os << " }";
}

} // namespace test

namespace termite {

template<>
[[nodiscard]] Result<test::DataType> Node::to_value<test::DataType>() const {
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

template<>
[[nodiscard]] Node Node::from_value<test::DataType>(const test::DataType &value) {
  return std::visit([](const auto &x) {
    return Node::from_value(x);
  }, value.value);
}

} // namespace termite


