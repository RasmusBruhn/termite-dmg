// Generated with the Termite Data Model Generator
#ifndef HEADER_TERMITE_H_INCLUDED
#define HEADER_TERMITE_H_INCLUDED

#include <iostream>
#include <sstream>
#include <optional>
#include <variant>
#include <algorithm>
#include <termite.hpp>

// Header

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
 * @brief description1
 * 
 */
struct DataType1 {
public:
  /**
   * @brief Constructs a new DataType1 object
   * 
   * 
   * @param extra_fields Any extra fields to attach to this struct
   */
  explicit DataType1(::termite::Node::Map extra_fields = ::termite::Node::Map()) : extra_fields(std::move(extra_fields)) {}

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType1 &x) {
    return extra_fields == x.extra_fields;
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
    return os << "{ " << "extra_fields: " << x.extra_fields << " }";
  }


  /**
   * @brief All extra fields from when reading which could not be captured
   * 
   */
  ::termite::Node::Map extra_fields;
};

/**
 * @brief description2
 * 
 */
struct DataType2 {
public:
  /**
   * @brief Constructs a new DataType2 object
   * 
   * 
   * @param extra_fields Any extra fields to attach to this struct
   */
  explicit DataType2(::termite::Node::Map extra_fields = ::termite::Node::Map()) : extra_fields(std::move(extra_fields)) {}

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType2 &x) {
    return extra_fields == x.extra_fields;
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
    return os << "{ " << "extra_fields: " << x.extra_fields << " }";
  }


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
[[nodiscard]] Result<test::DataType1> Node::Map::to_value<test::DataType1>() const {


  std::vector<std::string> keys;
  std::transform(map_.cbegin(), map_.cend(), std::back_inserter(keys), [](const std::pair<const std::string, Node> &key_value) {
    return key_value.first;
  });
  std::vector<std::string> leftovers;
  std::copy_if(keys.cbegin(), keys.cend(), std::back_inserter(leftovers), [](const std::string &value) {
    std::vector<std::string> correct = {};
    return std::find(correct.cbegin(), correct.cend(), value) == correct.cend();
  });
  std::vector<std::pair<std::string, Node>> extra_fields;
  std::transform(std::make_move_iterator(leftovers.begin()), std::make_move_iterator(leftovers.end()), std::back_inserter(extra_fields), [this](std::string name) {
    auto value = map_.find(name);
    return std::make_pair(std::move(name), value->second);
  });

  return Result<test::DataType1>::ok(test::DataType1(Map(std::map<std::string, Node>(std::make_move_iterator(extra_fields.begin()), std::make_move_iterator(extra_fields.end())))));
}

template<>
[[nodiscard]] Result<test::DataType2> Node::Map::to_value<test::DataType2>() const {


  std::vector<std::string> keys;
  std::transform(map_.cbegin(), map_.cend(), std::back_inserter(keys), [](const std::pair<const std::string, Node> &key_value) {
    return key_value.first;
  });
  std::vector<std::string> leftovers;
  std::copy_if(keys.cbegin(), keys.cend(), std::back_inserter(leftovers), [](const std::string &value) {
    std::vector<std::string> correct = {};
    return std::find(correct.cbegin(), correct.cend(), value) == correct.cend();
  });
  std::vector<std::pair<std::string, Node>> extra_fields;
  std::transform(std::make_move_iterator(leftovers.begin()), std::make_move_iterator(leftovers.end()), std::back_inserter(extra_fields), [this](std::string name) {
    auto value = map_.find(name);
    return std::make_pair(std::move(name), value->second);
  });

  return Result<test::DataType2>::ok(test::DataType2(Map(std::map<std::string, Node>(std::make_move_iterator(extra_fields.begin()), std::make_move_iterator(extra_fields.end())))));
}

} // namespace termite

// Footer

#endif