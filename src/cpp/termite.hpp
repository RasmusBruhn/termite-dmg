/**
 * @file termite.hpp
 * @brief The c++ Termite Data Model Generator code which implements errors and input output to yaml and json
 * @version 0.1
 * @date 2024-06-15
 * 
 */

#ifndef TERMITE_H_INCLUDED
#define TERMITE_H_INCLUDED

#include <string>
#include <sstream>
#include <iostream>
#include <variant>

namespace termite {

namespace {

// Helper trait to detect if T has operator<<
template <typename T, typename = void>
struct has_insertion_operator : std::false_type {};

template <typename T>
struct has_insertion_operator<T, std::void_t<decltype(std::declval<std::ostream&>() << std::declval<T>())>> : std::true_type {};

// Helper trait to detect if T has operator==
template <typename T, typename = void>
struct has_equality_operator : std::false_type
{
};
// T, std::enable_if_t<std::is_same_v<decltype(std::declval<T>() == std::declval<T>()), bool>>
template <typename T>
struct has_equality_operator<T, std::void_t<decltype(std::declval<T>() == std::declval<T>())>> : std::true_type
{
};
}

/**
 * @brief Describes any error within a data model
 * 
 */
class Error {
public:
  /**
   * @brief Construct a new Error object
   * 
   * @param message The message describing what is wrong
   * @param location The location in the data model where the error occured
   */
  explicit Error(std::string message, std::string location = "") : 
    location_(std::move(location)),message_(std::move(message)) {}

  /**
   * @brief Gets the error message for this error
   * 
   * @return A reference to the error message
   */
  [[nodiscard]] const std::string &get_message() const {
    return message_;
  }
  /**
   * @brief Gets the location in the data model this error occured at
   * 
   * @return A reference to the location
   */
  [[nodiscard]] const std::string &get_location() const
  {
    return location_;
  }

  /**
   * @brief Adds a field to the location such that the old location is a field
   * of the new location
   *
   * @param name The name of the new base
   */
  void add_field(const std::string &name) {
    if (location_.empty()) {
      location_ = name;
    } else {
      location_ = name + "." + location_;
    }
  }
  /**
   * @brief Adds a list element to the locations such that the old location is a
   * field of the new list element
   *
   * @param name The name of the base list
   * @param index The index of the list
   */
  void add_list(const std::string &name, const std::string &index) {
    if (location_.empty()) {
      location_ = name + "[" + index + "]";
    } else {
      location_ = name + "[" + index + "]." + location_;
    }
  }

  /**
   * @brief Checks if this error is identical to another error
   * 
   * @param other The other error to compare with
   * @return true if they are identical, false otherwise 
   */
  [[nodiscard]] bool operator==(const Error &other) const
  {
    return location_ == other.location_ && message_ == other.message_;
  }
  /**
   * @brief Checks if this error is not identical to another error
   * 
   * @param other The other error to compare with
   * @return true if they are not identical, false otherwise 
   */
  [[nodiscard]] bool operator!=(const Error &other) const
  {
    return !(*this == other);
  }

  /**
   * @brief Prints this error with the location to the output stream
   * 
   * @param stream The stream to print to
   * @param error The error to print
   * @return The same stream object
   */
  friend std::ostream &operator<<(std::ostream &os, const Error &error) {
    if (error.location_.empty()) {
      return os << error.message_;
    }
    return os << error.location_ << ": " << error.message_;
  }
  /**
   * @brief Converts this error with its location to a string
   * 
   * @return The string with the error
   */
  [[nodiscard]] std::string to_string() const
  {
    std::stringstream ss;
    ss << *this;
    return ss.str();
  }

private:
  /**
   * @brief The location in the data model where the error occured
   * 
   */
  std::string location_;
  /**
   * @brief The message describing what the error is
   * 
   */
  std::string message_;
};

/**
 * @brief Class containing a result, either it is ok and an object of type T can
 * be retrieved or it is err and an Error can be retrieved
 *
 * @tparam T The type of the ok value
 */
template <class T>
class Result {
public:
  /**
   * @brief Constructs an ok result
   * 
   * @param value The value of the ok
   * @return The resulting Result object
   */
  [[nodiscard]] static Result ok(T value)
  {
    return Result(value);
  }
  /**
   * @brief Constructs an error result
   * 
   * @param error The error value
   * @return The resulting Result object
   */
  [[nodiscard]] static Result err(Error error)
  {
    return Result(error);
  }

  /**
   * @brief Checks if this result is Ok or Err
   * 
   * @return true if it is ok, false if it is err
   */
  bool is_ok() const {
    return std::holds_alternative<T>(value_);
  }
  /**
   * @brief Retrieves the ok value, causes an exception if it is err. This
   * result is invalid after running this function
   *
   * @return The ok value
   */
  [[nodiscard]] T get_ok() {
    return std::move(std::get<T>(value_));
  }
  /**
   * @brief Retrieves the err value, causes an exception if it is ok. This
   * result is invalid after running this function
   *
   * @return The err value
   */
  [[nodiscard]] Error get_err() {
    return std::move(std::get<Error>(value_));
  }

  /**
   * @brief Checks equality with another result, only enabled if T has the
   * equality operator
   *
   * @param result The other result to compare with
   * @return true if they are identical, false otherwise
   */
  [[nodiscard]] typename std::enable_if<has_equality_operator<T>::value, bool>::type 
  operator==(const Result &result) const {
    return value_ == result.value_;
  }
  /**
   * @brief Checks inequality with another result, only enabled if T has the
   * equality operator
   *
   * @param result The other result to compare with
   * @return true if they are not identical, false otherwise
   */
  [[nodiscard]] bool operator!=(const Result &result) const {
    return !(value_ == result.value_);
  }

  /**
   * @brief Prints the result to a stream, only enabled if the stream print
   * operator exists for T
   *
   * @param os The output stream to print to 
   * @param result The result to print
   * @return The same output stream
   */
  typename std::enable_if<has_insertion_operator<T>::value, std::ostream &>::type
  friend operator<<(std::ostream &os, const Result &result) {
    if (result.is_ok()) {
      return os << "Ok ( " << std::get<T>(result.value_) << " )";
    } else {
      return os << "Err ( " << std::get<Error>(result.value_) << " )";
    }
  }
  /**
   * @brief Converts this result to a string, only enabled if the stream print
   * operator exists for T
   *
   * @return The string with this result in
   */
  [[nodiscard]] std::string to_string() const {
    std::stringstream ss;
    ss << *this;
    return ss.str();
  }

private:
  /**
   * @brief Construct a new Result object
   * 
   * @param value The value of the result
   * @param is_ok true if the value is ok, false if it is err
   */
  Result(std::variant<T, Error> value) : value_(value) {}

  /**
   * @brief The value of the result, is_ok_ describes how to interpret it
   * 
   */
  std::variant<T, Error> value_;
};

}

#endif
