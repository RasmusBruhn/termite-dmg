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
 * @brief description1
 * 
 */
class DataType1 {
public:
  /**
   * @brief Constructs a new DataType1 object
   * 
   * 
   * @return The new struct or an error if some constraints were not upheld
   */
  [[nodiscard]] static termite::Result<DataType1> from_values() {
    

    return termite::Result<DataType1>::ok(DataType1());
  }





  /**
   * @brief Checks if this object the the other object are identical
   * 
   * @param  The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType1 &) {
    return true;
  }
  /**
   * @brief Checks if this object the the other object are different
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
   * @param  The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const DataType1 &) {
    return os << "{ " << "" << " }";
  }

private:
  explicit DataType1() {}




};

/**
 * @brief description2
 * 
 */
class DataType2 {
public:
  /**
   * @brief Constructs a new DataType2 object
   * 
   * 
   * @return The new struct or an error if some constraints were not upheld
   */
  [[nodiscard]] static termite::Result<DataType2> from_values() {
    

    return termite::Result<DataType2>::ok(DataType2());
  }





  /**
   * @brief Checks if this object the the other object are identical
   * 
   * @param  The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType2 &) {
    return true;
  }
  /**
   * @brief Checks if this object the the other object are different
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
   * @param  The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const DataType2 &) {
    return os << "{ " << "" << " }";
  }

private:
  explicit DataType2() {}




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
[[nodiscard]] Result<test::DataType1> NodeMap::to_value(bool allow_skipping) const {


  if (!allow_skipping) {
    std::vector<std::string> keys;
    std::transform(map_.cbegin(), map_.cend(), std::back_inserter(keys), [](const std::pair<const std::string, std::unique_ptr<Node>> &key_value) {
      return key_value.first;
    });
    std::vector<std::string> leftovers;
    std::copy_if(keys.cbegin(), keys.cend(), std::back_inserter(leftovers), [](const std::string &value) {
      std::vector<std::string> correct = {};
      return std::find(correct.cbegin(), correct.cend(), value) == correct.cend();
    });
    if (!leftovers.empty()) {
      std::ostringstream ss;
      ss << "Found unused fields: " << leftovers;
      return Result<test::DataType1>::err(Error(ss.str()));
    }
  }

  return test::DataType1::from_values();
}

template<>
[[nodiscard]] Result<test::DataType2> NodeMap::to_value(bool allow_skipping) const {


  if (!allow_skipping) {
    std::vector<std::string> keys;
    std::transform(map_.cbegin(), map_.cend(), std::back_inserter(keys), [](const std::pair<const std::string, std::unique_ptr<Node>> &key_value) {
      return key_value.first;
    });
    std::vector<std::string> leftovers;
    std::copy_if(keys.cbegin(), keys.cend(), std::back_inserter(leftovers), [](const std::string &value) {
      std::vector<std::string> correct = {};
      return std::find(correct.cbegin(), correct.cend(), value) == correct.cend();
    });
    if (!leftovers.empty()) {
      std::ostringstream ss;
      ss << "Found unused fields: " << leftovers;
      return Result<test::DataType2>::err(Error(ss.str()));
    }
  }

  return test::DataType2::from_values();
}

} // namespace termite



#endif