// Generated with the Termite Data Model Generator
#include "outline.h"

// Header source

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

[[nodiscard]] bool DataType1::operator==(const DataType1 &x) const {
  return extra_fields == x.extra_fields;
}

std::ostream &operator<<(std::ostream &os, const DataType1 &x) {
  return os << "{ " << "extra_fields: " << x.extra_fields << " }";
}

[[nodiscard]] bool DataType2::operator==(const DataType2 &x) const {
  return extra_fields == x.extra_fields;
}

std::ostream &operator<<(std::ostream &os, const DataType2 &x) {
  return os << "{ " << "extra_fields: " << x.extra_fields << " }";
}

} // namespace test

namespace termite {

template<>
[[nodiscard]] Result<test::DataType1> Node::Map::to_value<test::DataType1>() const {
  std::map<std::string, Node> map = map_;

  return Result<test::DataType1>::ok(test::DataType1(Map(std::move(map))));
}

template<>
[[nodiscard]] Node Node::from_value<test::DataType1>(const test::DataType1 &value) {
  std::map<std::string, Node> map = value.extra_fields.get();

  return Node(Node::Map(std::move(map)));
}

template<>
[[nodiscard]] Result<test::DataType2> Node::Map::to_value<test::DataType2>() const {
  std::map<std::string, Node> map = map_;

  return Result<test::DataType2>::ok(test::DataType2(Map(std::move(map))));
}

template<>
[[nodiscard]] Node Node::from_value<test::DataType2>(const test::DataType2 &value) {
  std::map<std::string, Node> map = value.extra_fields.get();

  return Node(Node::Map(std::move(map)));
}

} // namespace termite

// Footer source
