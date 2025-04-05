// Generated with the Termite Data Model Generator
#include "description.h"



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
  return this->field1 == x.field1 && this->field2 == x.field2 && extra_fields == x.extra_fields;
}

std::ostream &operator<<(std::ostream &os, const DataType &x) {
  return os << "{ " << "field1: " << x.field1 << ", " << "field2: " << x.field2 << ", " << "extra_fields: " << x.extra_fields << " }";
}

} // namespace test

namespace termite {

template<>
[[nodiscard]] Result<test::DataType> Node::Map::to_value<test::DataType>() const {
  std::map<std::string, Node> map = map_;

  auto location_field1 = map.find("field1");
  if (location_field1 == map.end()) {
    return Result<test::DataType>::err(Error("Missing field1"));
  }
  Result<int> raw_value_field1 = location_field1->second.to_value<int>();
  if (!raw_value_field1.is_ok()) {
    Error error = raw_value_field1.get_err();
    error.add_field("field1");
    return Result<test::DataType>::err(std::move(error));
  }
  int value_field1 = raw_value_field1.get_ok();
  map.erase(location_field1);

  auto location_field2 = map.find("field2");
  if (location_field2 == map.end()) {
    return Result<test::DataType>::err(Error("Missing field2"));
  }
  Result<float> raw_value_field2 = location_field2->second.to_value<float>();
  if (!raw_value_field2.is_ok()) {
    Error error = raw_value_field2.get_err();
    error.add_field("field2");
    return Result<test::DataType>::err(std::move(error));
  }
  float value_field2 = raw_value_field2.get_ok();
  map.erase(location_field2);

  return Result<test::DataType>::ok(test::DataType(std::move(value_field1), std::move(value_field2), Map(std::move(map))));
}

template<>
[[nodiscard]] Node Node::from_value<test::DataType>(const test::DataType &value) {
  std::map<std::string, Node> map = value.extra_fields.get();

  map.insert({"field1", Node::from_value(value.field1)});

  map.insert({"field2", Node::from_value(value.field2)});

  return Node(Node::Map(std::move(map)));
}

} // namespace termite


