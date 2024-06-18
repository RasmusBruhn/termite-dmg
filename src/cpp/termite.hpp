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
  [[nodiscard]] static Result from_ok(T value)
  {
    return Result(value);
  }
  /**
   * @brief Constructs an error result
   * 
   * @param error The error value
   * @return The resulting Result object
   */
  [[nodiscard]] static Result from_err(Error error)
  {
    return Result(error);
  }

  bool is_ok() const {
    return std::holds_alternative<T>(value_);
  }
  [[nodiscard]] T get_ok() {
    return std::move(std::get<T>(value_));
  }
  [[nodiscard]] Error get_err() {
    return std::move(std::get<Error>(value_));
  }

  typename std::enable_if<has_insertion_operator<T>::value, std::ostream &>::type
  friend operator<<(std::ostream &stream, const Result &result) {
    if (result.is_ok()) {
      return stream << "Ok ( " << std::get<T>(result.value_) << " )";
    } else {
      return stream << "Err ( " << std::get<Error>(result.value_) << " )";
    }
  }

  [[nodiscard]] typename std::enable_if<has_insertion_operator<T>::value, std::string>::type
  to_string() const {
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
