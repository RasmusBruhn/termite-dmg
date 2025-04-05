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

[[nodiscard]] bool DataType::TypeInt1::operator==(const TypeInt1 &x) const {
  return value == x.value;
}

std::ostream &operator<<(std::ostream &os, const DataType::TypeInt1 &x) {
  return os << "{ value: " << x.value << " }";
}

[[nodiscard]] bool DataType::TypeInt2::operator==(const TypeInt2 &x) const {
  return value == x.value;
}

std::ostream &operator<<(std::ostream &os, const DataType::TypeInt2 &x) {
  return os << "{ value: " << x.value << " }";
}

[[nodiscard]] bool DataType::TypeFloat::operator==(const TypeFloat &x) const {
  return value == x.value;
}

std::ostream &operator<<(std::ostream &os, const DataType::TypeFloat &x) {
  return os << "{ value: " << x.value << " }";
}

[[nodiscard]] bool DataType::TypeEmpty::operator==(const TypeEmpty &) const {
  return true;
}

std::ostream &operator<<(std::ostream &os, const DataType::TypeEmpty &) {
  return os << "{  }";
}

[[nodiscard]] bool DataType::operator==(const DataType &x) const {
  return value == x.value;
}

std::ostream &operator<<(std::ostream &os, const DataType &x) {
  os << "{ value: ";
  switch (static_cast<DataType::Enum>(x.value.index())) {
  case DataType::Enum::kInt1:
    os << "Int1(" << std::get<DataType::TypeInt1>(x.value).value << ")";
    break;
  case DataType::Enum::kInt2:
    os << "Int2(" << std::get<DataType::TypeInt2>(x.value).value << ")";
    break;
  case DataType::Enum::kFloat:
    os << "Float(" << std::get<DataType::TypeFloat>(x.value).value << ")";
    break;
  case DataType::Enum::kEmpty:
    os << "Empty";
    break;
  default:
    os << "Unknown (" << x.value.index() << ")";
    break;
  }
  return os << " }";
}

} // namespace test

namespace termite {

template<>
[[nodiscard]] Result<test::DataType> Node::Value::to_value<test::DataType>() const {
  if (value_ == "Int1") {
    return Result<test::DataType>::err(Error("Enum type Int1 must contain a value"));
  }
  if (value_ == "Int2") {
    return Result<test::DataType>::err(Error("Enum type Int2 must contain a value"));
  }
  if (value_ == "Float") {
    return Result<test::DataType>::err(Error("Enum type Float must contain a value"));
  }
  if (value_ == "Empty") {
    return Result<test::DataType>::ok(test::DataType(test::DataType::TypeEmpty{}));
  }

  std::stringstream ss;
  ss << "Unknown enum type \"" << value_ << "\"";
  return Result<test::DataType>::err(Error(ss.str()));
}

template<>
[[nodiscard]] Result<test::DataType> Node::Map::to_value<test::DataType>() const {
  if (map_.size() != 1) {
    std::stringstream ss;
    ss << "There must be exactly one enum type specified but received " << map_.size();
    return Result<test::DataType>::err(Error(ss.str()));
  }

  if (map_.cbegin()->first == "Int1") {
    Result<int> value = map_.cbegin()->second.to_value<int>();
    if (value.is_ok()) {
      return Result<test::DataType>::ok(test::DataType(test::DataType::TypeInt1{value.get_ok()}));
    }
    return Result<test::DataType>::err(value.get_err().add_field("Int1"));
  }
  if (map_.cbegin()->first == "Int2") {
    Result<int> value = map_.cbegin()->second.to_value<int>();
    if (value.is_ok()) {
      return Result<test::DataType>::ok(test::DataType(test::DataType::TypeInt2{value.get_ok()}));
    }
    return Result<test::DataType>::err(value.get_err().add_field("Int2"));
  }
  if (map_.cbegin()->first == "Float") {
    Result<float> value = map_.cbegin()->second.to_value<float>();
    if (value.is_ok()) {
      return Result<test::DataType>::ok(test::DataType(test::DataType::TypeFloat{value.get_ok()}));
    }
    return Result<test::DataType>::err(value.get_err().add_field("Float"));
  }
  if (map_.cbegin()->first == "Empty") {
    return Result<test::DataType>::err(Error("Enum type Empty must not include values"));
  }

  std::stringstream ss;
  ss << "Unknown enum type \"" << map_.cbegin()->first << "\"";
  return Result<test::DataType>::err(Error(ss.str()));
}

template<>
[[nodiscard]] Node Node::from_value<test::DataType>(const test::DataType &value) {
  std::map<std::string, Node> map;
  switch (value.enum_type()) {
  case test::DataType::Enum::kInt1:
    map.insert({
      "Int1",
      Node::from_value(std::get<test::DataType::TypeInt1>(value.value).value)
    });
    return Node(Node::Map(std::move(map)));
  case test::DataType::Enum::kInt2:
    map.insert({
      "Int2",
      Node::from_value(std::get<test::DataType::TypeInt2>(value.value).value)
    });
    return Node(Node::Map(std::move(map)));
  case test::DataType::Enum::kFloat:
    map.insert({
      "Float",
      Node::from_value(std::get<test::DataType::TypeFloat>(value.value).value)
    });
    return Node(Node::Map(std::move(map)));
  case test::DataType::Enum::kEmpty:
    return Node(Node::Value("Empty"));
  default:
    return Node(Node::Value(""));
  }
}

} // namespace termite


