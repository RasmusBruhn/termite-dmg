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
    message_(std::move(message)), location_(std::move(location)) {}

  /**
   * @brief Gets the error message for this error
   * 
   * @return A reference to the error message
   */
  const std::string &get_message() const {
    return message_;
  }

  /**
   * @brief Gets the location in the data model this error occured at
   * 
   * @return A reference to the location
   */
  const std::string &get_location() const {
    return location_;
  }

  /**
   * @brief Checks if this error is identical to another error
   * 
   * @param other The other error to compare with
   * @return true if they are identical, false otherwise 
   */
  bool operator==(const Error &other) const {
    return location_ == other.location_ && message_ == other.message_;
  }

  /**
   * @brief Checks if this error is not identical to another error
   * 
   * @param other The other error to compare with
   * @return true if they are not identical, false otherwise 
   */
  bool operator!=(const Error &other) const {
    return !(*this == other);
  }

  /**
   * @brief Prints this error with the location to the output stream
   * 
   * @param stream The stream to print to
   * @param error The error to print
   * @return The same stream object
   */
  friend std::ostream &operator<<(std::ostream &stream, const Error &error) {
    return stream << error.location_ << ": " << error.message_;
  }

  /**
   * @brief Converts this error with its location to a string
   * 
   * @return The string with the error
   */
  std::string to_string() const {
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
  ~Result() {
    if (is_ok_) {
        value_.ok.~T();
    } else {
        value_.err.~Error();
    }
  }

  /**
   * @brief Constructs an ok result
   * 
   * @param value The value of the ok
   * @return The resulting Result object
   */
  static Result<T> from_ok(T value) {
    OkOrErr new_value;
    new_value.ok = std::move(value);
    return Result<T>(new_value, true);
  }
  /**
   * @brief Constructs an error result
   * 
   * @param error The error value
   * @return The resulting Result object
   */
  static Result<T> from_err(Error error) {
    OkOrErr new_value;
    new_value.err = std::move(error);
    return Result<T>(new_value, true);
  }

  bool is_ok() const {
    return is_ok_;
  }

  template <typename U = T>
  typename std::enable_if<has_insertion_operator<U>::value, std::ostream &>::type
  friend operator<<(std::ostream &stream, const Result<U> &result) {
    if (result.is_ok_) {
      return stream << "Ok ( " << result.value_.ok << " )";
    } else {
      return stream << "Err ( " << result.value_.err << " )";
    }
  }
/*
  typename std::enable_if<has_insertion_operator<T>::value, std::ostream &>::type
  std::string to_string() const {
    std::stringstream ss;
    ss << *this;
    return ss.str();
  }*/

private:
  /**
   * @brief The value type which is either ok or err
   * 
   */
  union OkOrErr {
    ~OkOrErr() = delete;

    /**
     * @brief The ok value of type T
     * 
     */
    T ok;
    /**
     * @brief The err type containing an Error
     * 
     */
    Error err;
  };

  /**
   * @brief Construct a new Result object
   * 
   * @param value The value of the result
   * @param is_ok true if the value is ok, false if it is err
   */
  Result(OkOrErr value, bool is_ok) : is_ok_(is_ok), value_(std::move(value)) {}

  /**
   * @brief If true the the result if ok otherwise it is err
   * 
   */
  bool is_ok_;
  /**
   * @brief The value of the result, is_ok_ describes how to interpret it
   * 
   */
  OkOrErr value_;
};

}

#endif
