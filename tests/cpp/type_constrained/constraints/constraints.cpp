// Generated with the Termite Data Model Generator
#include "constraints.h"



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
[[nodiscard]] termite::Result<DataType1> DataType1::from_value(int value) {
  termite::Result<termite::Empty> validate_result = validate(value);
  if (!validate_result.is_ok()) {
    termite::Error error = validate_result.get_err();
    return termite::Result<DataType1>::err(std::move(error));
  }

  return termite::Result<DataType1>::ok(DataType1(std::move(value), nullptr));
}

[[nodiscard]] termite::Result<termite::Empty> DataType1::set(int value) {
  termite::Result<termite::Empty> validate_result = validate(value);
  if (!validate_result.is_ok()) {
    return validate_result;
  }

  value_ = std::move(value);
  return termite::Result<termite::Empty>::ok(termite::Empty());
}

[[nodiscard]] bool DataType1::operator==(const DataType1 &x) const {
  return value_ == x.value_;
}
std::ostream &operator<<(std::ostream &os, const DataType1 &x) {
  return os << x.value_;
}

[[nodiscard]] termite::Result<termite::Empty> DataType1::validate(const int &x) {
  if (!(x > 0)) {
    return termite::Result<termite::Empty>::err(termite::Error("Did not pass constaint: x > 0"));
  }

  if (!(x % 2 == 0)) {
    return termite::Result<termite::Empty>::err(termite::Error("Did not pass constaint: x % 2 == 0"));
  }

  return termite::Result<termite::Empty>::ok(termite::Empty());
}

/**
 * @brief
 *
 */
[[nodiscard]] termite::Result<DataType2> DataType2::from_value(float value) {
  termite::Result<termite::Empty> validate_result = validate(value);
  if (!validate_result.is_ok()) {
    termite::Error error = validate_result.get_err();
    return termite::Result<DataType2>::err(std::move(error));
  }

  return termite::Result<DataType2>::ok(DataType2(std::move(value), nullptr));
}

[[nodiscard]] termite::Result<termite::Empty> DataType2::set(float value) {
  termite::Result<termite::Empty> validate_result = validate(value);
  if (!validate_result.is_ok()) {
    return validate_result;
  }

  value_ = std::move(value);
  return termite::Result<termite::Empty>::ok(termite::Empty());
}

[[nodiscard]] bool DataType2::operator==(const DataType2 &x) const {
  return value_ == x.value_;
}
std::ostream &operator<<(std::ostream &os, const DataType2 &x) {
  return os << x.value_;
}

[[nodiscard]] termite::Result<termite::Empty> DataType2::validate(const float &x) {
  if (!(std::abs(x) < 1e-9)) {
    return termite::Result<termite::Empty>::err(termite::Error("Did not pass constaint: std::abs(x) < 1e-9"));
  }

  return termite::Result<termite::Empty>::ok(termite::Empty());
}

} // namespace test

namespace termite {

template<>
[[nodiscard]] Result<test::DataType1> Node::to_value<test::DataType1>() const {
  Result<int> value = to_value<int>();
  if (!value.is_ok()) {
    return Result<test::DataType1>::err(Error(value.get_err()));
  }

  return test::DataType1::from_value(value.get_ok());
}

template<>
[[nodiscard]] Result<test::DataType2> Node::to_value<test::DataType2>() const {
  Result<float> value = to_value<float>();
  if (!value.is_ok()) {
    return Result<test::DataType2>::err(Error(value.get_err()));
  }

  return test::DataType2::from_value(value.get_ok());
}

} // namespace termite


