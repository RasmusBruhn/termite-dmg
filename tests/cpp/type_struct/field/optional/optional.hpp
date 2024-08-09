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
struct DataType {
public:
  /**
   * @brief Constructs a new DataType object
   * 
   * @param field1 
   * @param field2 
   */
  explicit DataType(int field1 = 1, std::optional<float> field2 = std::nullopt, ::termite::Node::Map extra_fields = ::termite::Node::Map()) : field1(std::move(field1)), field2(std::move(field2)), extra_fields(std::move(extra_fields)) {}

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType &x) {
    return field1 == x.field1 && field2 == x.field2 && extra_fields == x.extra_fields;
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
    return os << "{ " << "field1: " << x.field1 << ", " << "field2: " << x.field2 << ", " << "extra_fields: " << x.extra_fields << " }";
  }

  /**
   * @brief 
   * 
   */
  int field1;
  /**
   * @brief 
   * 
   */
  std::optional<float> field2;
  /**
   * @brief All extra fields from when reading which could not be captured
   * 
   */
  ::termite::Node::Map extra_fields;
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
[[nodiscard]] Result<test::DataType> Node::Map::to_value() const {
  auto location_field1 = map_.find("field1");
  int value_field1 = 1;
  if (location_field1 != map_.end()) {
    Result<int> raw_value_field1 = location_field1->second.to_value<int>();
    if (!raw_value_field1.is_ok()) {
      Error error = raw_value_field1.get_err();
      error.add_field("field1");
      return Result<test::DataType>::err(std::move(error));
    }
    value_field1 = raw_value_field1.get_ok();
  }

  auto location_field2 = map_.find("field2");
  std::optional<float> value_field2 = std::nullopt;
  if (location_field2 != map_.end()) {
    Result<float> raw_value_field2 = location_field2->second.to_value<float>();
    if (!raw_value_field2.is_ok()) {
      Error error = raw_value_field2.get_err();
      error.add_field("field2");
      return Result<test::DataType>::err(std::move(error));
    }
    value_field2 = raw_value_field2.get_ok();
  }

  std::vector<std::string> keys;
  std::transform(map_.cbegin(), map_.cend(), std::back_inserter(keys), [](const std::pair<const std::string, Node> &key_value) {
    return key_value.first;
  });
  std::vector<std::string> leftovers;
  std::copy_if(keys.cbegin(), keys.cend(), std::back_inserter(leftovers), [](const std::string &value) {
    std::vector<std::string> correct = {"field1", "field2"};
    return std::find(correct.cbegin(), correct.cend(), value) == correct.cend();
  });
  std::vector<std::pair<std::string, Node>> extra_fields;
  std::transform(std::make_move_iterator(leftovers.begin()), std::make_move_iterator(leftovers.end()), std::back_inserter(extra_fields), [this](std::string name) {
    auto value = map_.find(name);
    return std::make_pair(std::move(name), value->second);
  });

  return Result<test::DataType>::ok(test::DataType(std::move(value_field1), std::move(value_field2), Map(std::map<std::string, Node>(std::make_move_iterator(extra_fields.begin()), std::make_move_iterator(extra_fields.end())))));
}

} // namespace termite



#endif