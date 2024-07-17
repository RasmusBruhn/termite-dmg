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
   * @return The new array or an error if some constraints were not upheld
   */
  [[nodiscard]] static termite::Result<DataType1> from_values(std::vector<int> values) {
    for (auto value = values.cbegin(); value < values.cend(); ++value) {
      termite::Result<termite::Empty> validate_result = validate(*value);
      if (!validate_result.is_ok()) {
        termite::Error error = validate_result.get_err();
        error.add_list(value - values.cbegin());
        return termite::Result<DataType1>::err(std::move(error));
      }
    }

    return termite::Result<DataType1>::ok(DataType1(std::move(values)));
  }

  /**
   * @brief Sets the values if they fulfill the constraints:
   * 
   * @param values The values to set
   * @return An error if one of the constraints were not fulfilled
   */
  [[nodiscard]] termite::Result<termite::Empty> set_values(std::vector<int> values) {
    for (auto value = values.cbegin(); value < values.cend(); ++value) {
      termite::Result<termite::Empty> validate_result = validate(*value);
      if (!validate_result.is_ok()) {
        termite::Error error = validate_result.get_err();
        error.add_list(value - values.cbegin());
        return termite::Result<termite::Empty>::err(std::move(error));
      }
    }

    values_ = std::move(values);
    return termite::Result<termite::Empty>::ok(termite::Empty());
  }

  /**
   * @brief Pushes a single value if it fulfill the constraints:
   * 
   * @param value The value to set
   * @return An error if one of the constraints were not fulfilled
   */
  [[nodiscard]] termite::Result<termite::Empty> push_value(int value) {
    termite::Result<termite::Empty> validate_result = validate(value);
    if (!validate_result.is_ok()) {
      return validate_result;
    }

    values_.push_back(std::move(value));
    return termite::Result<termite::Empty>::ok(termite::Empty());
  }

  /**
   * @brief Retrieves a reference to the values
   * 
   * @return The reference
   */
  [[nodiscard]] const std::vector<int> &get_values() const {
    return values_;
  }

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
  explicit DataType1(std::vector<int> values) : values_(std::move(values)) {}

  /**
   * @brief Validates if an element is correct using the following constaints:
   * 
   * @param  The value of the parameter to validate
   */
  [[nodiscard]] static termite::Result<termite::Empty> validate(const int &) {


    return termite::Result<termite::Empty>::ok(termite::Empty());
  }

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
   * @return The new array or an error if some constraints were not upheld
   */
  [[nodiscard]] static termite::Result<DataType2> from_values(std::vector<float> values) {
    for (auto value = values.cbegin(); value < values.cend(); ++value) {
      termite::Result<termite::Empty> validate_result = validate(*value);
      if (!validate_result.is_ok()) {
        termite::Error error = validate_result.get_err();
        error.add_list(value - values.cbegin());
        return termite::Result<DataType2>::err(std::move(error));
      }
    }

    return termite::Result<DataType2>::ok(DataType2(std::move(values)));
  }

  /**
   * @brief Sets the values if they fulfill the constraints:
   * 
   * @param values The values to set
   * @return An error if one of the constraints were not fulfilled
   */
  [[nodiscard]] termite::Result<termite::Empty> set_values(std::vector<float> values) {
    for (auto value = values.cbegin(); value < values.cend(); ++value) {
      termite::Result<termite::Empty> validate_result = validate(*value);
      if (!validate_result.is_ok()) {
        termite::Error error = validate_result.get_err();
        error.add_list(value - values.cbegin());
        return termite::Result<termite::Empty>::err(std::move(error));
      }
    }

    values_ = std::move(values);
    return termite::Result<termite::Empty>::ok(termite::Empty());
  }

  /**
   * @brief Pushes a single value if it fulfill the constraints:
   * 
   * @param value The value to set
   * @return An error if one of the constraints were not fulfilled
   */
  [[nodiscard]] termite::Result<termite::Empty> push_value(float value) {
    termite::Result<termite::Empty> validate_result = validate(value);
    if (!validate_result.is_ok()) {
      return validate_result;
    }

    values_.push_back(std::move(value));
    return termite::Result<termite::Empty>::ok(termite::Empty());
  }

  /**
   * @brief Retrieves a reference to the values
   * 
   * @return The reference
   */
  [[nodiscard]] const std::vector<float> &get_values() const {
    return values_;
  }

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
  explicit DataType2(std::vector<float> values) : values_(std::move(values)) {}

  /**
   * @brief Validates if an element is correct using the following constaints:
   * 
   * @param  The value of the parameter to validate
   */
  [[nodiscard]] static termite::Result<termite::Empty> validate(const float &) {


    return termite::Result<termite::Empty>::ok(termite::Empty());
  }

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

  return test::DataType1::from_values(std::move(values));
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

  return test::DataType2::from_values(std::move(values));
}

} // namespace termite



#endif