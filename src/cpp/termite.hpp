/**
 * @file termite.hpp
 * @brief The c++ Termite Data Model Generator code which implements errors and
 * input output to yaml and json
 * @version 0.6.0
 * @date 2025-09-28
 *
 */

#ifndef TERMITE_H_INCLUDED
#define TERMITE_H_INCLUDED

#include <algorithm>
#include <iostream>
#include <map>
#include <memory>
#include <sstream>
#include <string>
#include <variant>
#include <vector>
#include <cstdint>

namespace termite {

using string = std::string;
using number = double;
using integer = int64_t;
using boolean = bool;

// Helper trait to detect if T has operator<<
template <typename T, typename = void>
struct has_insertion_operator : std::false_type {};
template <typename T>
struct has_insertion_operator<
    T,
    std::void_t<decltype(std::declval<std::ostream &>() << std::declval<T>())>>
    : std::true_type {};
template <typename T>
constexpr bool has_insertion_operator_v = has_insertion_operator<T>::value;

// Helper trait to detect if T has operator>>
template <typename T, typename = void>
struct has_parsing_operator : std::false_type {};
template <typename T>
struct has_parsing_operator<
    T, std::void_t<decltype(std::declval<std::istream &>() >>
                            std::declval<T &>())>> : std::true_type {};
template <typename T>
constexpr bool has_parsing_operator_v = has_parsing_operator<T>::value;

// Helper trait to detect if T has operator==
template <typename T, typename = void>
struct has_equality_operator : std::false_type {};
// T, std::enable_if_t<std::is_same_v<decltype(std::declval<T>() ==
// std::declval<T>()), bool>>
template <typename T>
struct has_equality_operator<
    T, std::void_t<decltype(std::declval<T>() == std::declval<T>())>>
    : std::true_type {};
template <typename T>
constexpr bool has_equality_operator_v = has_equality_operator<T>::value;

/**
 * @brief An empty class used as the ok value of a result if only the error
 * matters
 *
 */
class Empty {
public:
  /**
   * @brief Checks if this value and the other value are identical
   *
   * @param other The other value to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const Empty &other) const { return true; }
  /**
   * @brief Checks if this value and the other value are different
   *
   * @param other The other value to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const Empty &other) const {
    return !(*this == other);
  }
  /**
   * @brief Prints the value to an ostream
   *
   * @param os The stream to print to
   * @param value The value to print
   * @return The same ostream
   */
  friend std::ostream &operator<<(std::ostream &os, const Empty &value) {
    return os << "{  }";
  }
};

template <typename T> class Reference {
public:
  /**
   * @brief Constructs a new reference
   *
   * @param ref The reference to store
   */
  explicit Reference(const T &ref) : ref_(ref) {}

  /**
   * @brief Gets the stored reference
   *
   * @return The reference
   */
  const T &get() const { return ref_; }

  /**
   * @brief Checks if this value and the other value are identical
   *
   * @param other The other value to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]]
  typename std::enable_if_t<has_equality_operator_v<T>, bool>
  operator==(const Reference &other) const {
    return ref_ == other.ref_;
  }
  /**
   * @brief Checks if this value and the other value are different
   *
   * @param other The other value to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] typename std::enable_if_t<has_equality_operator_v<T>, bool>
  operator!=(const Reference &other) const {
    return !(*this == other);
  }
  /**
   * @brief Prints the value to an ostream
   *
   * @param os The stream to print to
   * @param value The value to print
   * @return The same ostream
   */
  typename std::enable_if_t<has_insertion_operator_v<T>, std::ostream &> friend
  operator<<(std::ostream &os, const Reference &value) {
    return os << "{ " << value.ref_ << " }";
  }

private:
  /**
   * @brief The reference stored
   *
   */
  const T &ref_;
};

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
  explicit Error(std::string message, std::string location = "")
      : location_(std::move(location)), message_(std::move(message)) {}

  /**
   * @brief Gets the error message for this error
   *
   * @return A reference to the error message
   */
  [[nodiscard]] const std::string &get_message() const { return message_; }
  /**
   * @brief Gets the location in the data model this error occured at
   *
   * @return A reference to the location
   */
  [[nodiscard]] const std::string &get_location() const { return location_; }

  /**
   * @brief Adds a field to the location such that the old location is a field
   * of the new location
   *
   * @param name The name of the new base
   */
  Error &add_field(const std::string &name) {
    std::ostringstream ss;
    ss << name;
    if (!location_.empty() && location_[0] != '[') {
      ss << ".";
    }
    ss << location_;
    location_ = ss.str();
    return *this;
  }
  /**
   * @brief Adds a list element to the locations such that the old location is a
   * field of the new list element
   *
   * @param index The index of the list
   */
  Error &add_list(size_t index) {
    std::ostringstream ss;
    ss << "[" << index << "]";
    if (!location_.empty() && location_[0] != '[') {
      ss << ".";
    }
    ss << location_;
    location_ = ss.str();
    return *this;
  }

  /**
   * @brief Checks if this error is identical to another error
   *
   * @param other The other error to compare with
   * @return true if they are identical, false otherwise
   */
  [[nodiscard]] bool operator==(const Error &other) const {
    return location_ == other.location_ && message_ == other.message_;
  }
  /**
   * @brief Checks if this error is not identical to another error
   *
   * @param other The other error to compare with
   * @return true if they are not identical, false otherwise
   */
  [[nodiscard]] bool operator!=(const Error &other) const {
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
template <typename T> class Result {
public:
  /**
   * @brief Constructs an ok result
   *
   * @param value The value of the ok
   * @return The resulting Result object
   */
  [[nodiscard]] static Result ok(T value) { return Result(value); }
  /**
   * @brief Constructs an error result
   *
   * @param error The error value
   * @return The resulting Result object
   */
  [[nodiscard]] static Result err(Error error) { return Result(error); }

  /**
   * @brief Checks if this result is Ok or Err
   *
   * @return true if it is ok, false if it is err
   */
  [[nodiscard]] bool is_ok() const { return std::holds_alternative<T>(value_); }
  /**
   * @brief Throws an exception if result is Err
   *
   */
  void unwrap() const {
    if (!is_ok()) {
      std::stringstream ss;
      ss << "Result is error: \"" << get_err() << "\"";
      throw std::exception(ss.str());
    }
  }
  /**
   * @brief Retrieves the ok value, causes an exception if it is err. This
   * result is invalid after running this function
   *
   * @return The ok value
   */
  [[nodiscard]] T get_ok() { return std::move(std::get<T>(value_)); }
  /**
   * @brief Retrieves the err value, causes an exception if it is ok. This
   * result is invalid after running this function
   *
   * @return The err value
   */
  [[nodiscard]] Error get_err() { return std::move(std::get<Error>(value_)); }

  /**
   * @brief Checks equality with another result, only enabled if T has the
   * equality operator
   *
   * @param result The other result to compare with
   * @return true if they are identical, false otherwise
   */
  [[nodiscard]]
  typename std::enable_if_t<has_equality_operator_v<T>, bool>
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
  [[nodiscard]] std::enable_if_t<has_equality_operator_v<T>, bool>
  operator!=(const Result &result) const {
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
  typename std::enable_if_t<has_insertion_operator_v<T>, std::ostream &> friend
  operator<<(std::ostream &os, const Result &result) {
    if (result.is_ok()) {
      return os << "Ok ( " << std::get<T>(result.value_) << " )";
    } else {
      return os << "Err ( " << std::get<Error>(result.value_) << " )";
    }
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

/**
 * @brief A node which can be any kind
 *
 */
class Node {
public:
  /**
   * @brief A single node value represented by a trimmed string
   *
   */
  class Value {
  public:
    /**
     * @brief Constructs a new node value with trimmed input
     *
     * @param value The value of this node
     */
    explicit Value(std::string value) {
      // Trim the value
      value.erase(value.begin(), std::find_if(value.begin(), value.end(),
                                              [](unsigned char ch) {
                                                return !std::isspace(ch);
                                              }));
      value.erase(
          std::find_if(value.rbegin(), value.rend(),
                       [](unsigned char ch) { return !std::isspace(ch); })
              .base(),
          value.end());

      value_ = std::move(value);
    }

    /**
     * @brief Retrieves the value
     *
     * @return The value
     */
    [[nodiscard]] const std::string &get() const { return value_; }

    /**
     * @brief Casts the node value to the given type, if operator>> is not
     * defined then an error occurs
     *
     * @tparam T The type to cast to
     * @return A result of the given type which is always err
     */
    template <typename T>
    [[nodiscard]]
    typename std::enable_if_t<!has_parsing_operator_v<T>, Result<T>>
    to_value() const {
      return Result<T>::err(Error("Parsing not implemented for given type"));
    }

    /**
     * @brief Casts the node value to the given type
     *
     * @tparam T The type to cast to
     * @return A result of the given type
     */
    template <typename T>
    [[nodiscard]]
    typename std::enable_if_t<has_parsing_operator_v<T>, Result<T>>
    to_value() const {
      // Take care of booleans seperately
      if constexpr (std::is_same_v<T, bool>) {
        if (value_ == "true" || value_ == "True" || value_ == "1") {
          return Result<T>::ok(true);
        }
        if (value_ == "false" || value_ == "False" || value_ == "0") {
          return Result<T>::ok(false);
        }
        return Result<T>::err(Error("Unable to parse"));
      } else {
        // Create the value
        std::istringstream ss(value_);
        T output;
        ss >> output;

        // Make sure it did not fail
        if (ss.fail()) {
          return Result<T>::err(Error("Unable to parse"));
        }
        // Make sure everything was used ie. 123et is not the integer 123
        if (!ss.eof()) {
          std::ostringstream error;
          error << "Value has unused characters: \"" << ss.str() << "\"";
          return Result<T>::err(Error(error.str()));
        }

        // Return the value
        return Result<T>::ok(std::move(output));
      }
    }

    /**
     * @brief Checks if this node value and another node value are identical
     *
     * @param other The other node value to compare with
     * @return true if they are identical, false if not
     */
    [[nodiscard]] bool operator==(const Value &other) const {
      return value_ == other.value_;
    }
    /**
     * @brief Checks if this node value and another node value are different
     *
     * @param other The other node value to compare with
     * @return true if they are different, false if not
     */
    [[nodiscard]] bool operator!=(const Value &other) const {
      return !(*this == other);
    }

    /**
     * @brief Prints the node value to an output stream
     *
     * @param os The output stream to print to
     * @param value The node value to print
     * @return The same output stream
     */
    friend std::ostream &operator<<(std::ostream &os, const Value &value) {
      return os << "{ value: " << value.value_ << " }";
    }

  private:
    /**
     * @brief The value of this node
     *
     */
    std::string value_;
  };

  /**
   * @brief A map of nodes which can be cast into structs
   *
   */
  class Map {
  public:
    /**
     * @brief Constructs an empty map
     *
     */
    explicit Map() = default;
    /**
     * @brief Constructs a new node map
     *
     * @param map The map of this node
     */
    explicit Map(std::map<std::string, Node> map) : map_(std::move(map)) {}

    /**
     * @brief Retrieves the map
     *
     * @return The map
     */
    [[nodiscard]] const std::map<std::string, Node> &get() const {
      return map_;
    }

    /**
     * @brief Casts the node map to the given type, if not specialized then it
     * will always return an error
     *
     * @tparam T The type to cast to
     * @return A result of the given type
     */
    template <typename T> [[nodiscard]] Result<T> to_value() const {
      return Result<T>::err(Error("Parsing not implemented for given type"));
    }

    /**
     * @brief Checks if this node map and another node map are identical
     *
     * @param other The other node map to compare with
     * @return true if they are identical, false if not
     */
    [[nodiscard]] bool operator==(const Map &other) const {
      return map_ == other.map_;
    }
    /**
     * @brief Checks if this node map and another node map are different
     *
     * @param other The other node map to compare with
     * @return true if they are different, false if not
     */
    [[nodiscard]] bool operator!=(const Map &other) const {
      return !(*this == other);
    }

    /**
     * @brief Prints the node map to an output stream
     *
     * @param os The output stream to print to
     * @param value The node map to print
     * @return The same output stream
     */
    friend std::ostream &operator<<(std::ostream &os, const Map &value) {
      os << "{ map: { ";
      for (auto key_value = value.map_.cbegin(); key_value != value.map_.cend();
           ++key_value) {
        if (key_value != value.map_.cbegin()) {
          os << ", ";
        }
        os << "\"" << key_value->first << "\": " << key_value->second;
      }
      return os << " } }";
    }

  private:
    /**
     * @brief The map of this node
     *
     */
    std::map<std::string, Node> map_;
  };

  /**
   * @brief A vector of nodes which can be cast into lists
   *
   */
  class List {
  public:
    /**
     * @brief Constructs an empty list
     *
     */
    explicit List() = default;
    /**
     * @brief Constructs a new node list
     *
     * @param list The list of this node
     */
    explicit List(std::vector<Node> list) : list_(std::move(list)) {}

    /**
     * @brief Retrieves the list
     *
     * @return The list
     */
    [[nodiscard]] const std::vector<Node> &get() const { return list_; }

    /**
     * @brief Casts the node list to the given type, if not specialized then it
     * will always return an error
     *
     * @tparam T The type to cast to
     * @return A result of the given type
     */
    template <typename T> [[nodiscard]] Result<T> to_value() const {
      return Result<T>::err(Error("Parsing not implemented for given type"));
    }

    /**
     * @brief Checks if this node list and another node list are identical
     *
     * @param other The other node list to compare with
     * @return true if they are identical, false if not
     */
    [[nodiscard]] bool operator==(const List &other) const {
      return list_ == other.list_;
    }
    /**
     * @brief Checks if this node list and another node list are different
     *
     * @param other The other node list to compare with
     * @return true if they are different, false if not
     */
    [[nodiscard]] bool operator!=(const List &other) const {
      return !(*this == other);
    }

    /**
     * @brief Prints the node list to an output stream
     *
     * @param os The output stream to print to
     * @param value The node list to print
     * @return The same output stream
     */
    friend std::ostream &operator<<(std::ostream &os, const List &value) {
      os << "{ list: [ ";
      os << "[ ";
      for (auto value_it = value.list_.cbegin(); value_it != value.list_.cend();
           ++value_it) {
        if (value_it != value.list_.cbegin()) {
          os << ", ";
        }
        os << *value_it;
      }
      return os << " ] }";
    }

  private:
    /**
     * @brief The list of this node
     *
     */
    std::vector<Node> list_;
  };

  /**
   * @brief Constructs a new node
   *
   * @param value The value of the node
   */
  explicit Node(std::variant<Value, Map, List> value)
      : value_(std::move(value)) {}
  Node(const Node &node) = default;

  /**
   * @brief Retrieves the value
   *
   * @return The value
   */
  [[nodiscard]] const std::variant<Value, Map, List> &get() const {
    return value_;
  }

  /**
   * @brief Casts the node to the given type
   *
   * @tparam T The type to cast to
   * @return A result of the given type
   */
  template <typename T> [[nodiscard]] Result<T> to_value() const {
    return std::visit(
        [](const auto &value) -> Result<T> {
          return value.template to_value<T>();
        },
        value_);
  }

  /**
   * @brief Constructs a node from a given value
   *
   * @tparam T The type of the value
   * @param value The value to convert to a node
   * @return The node
   */
  template <typename T> [[nodiscard]] static Node from_value(const T &value) {
    static_assert(has_insertion_operator_v<T>, "Type must have operator<<");
    std::stringstream ss;
    ss << value;
    return Node(Value(ss.str()));
  }

  /**
   * @brief Checks if this node and another node are identical
   *
   * @param other The other node to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const Node &other) const {
    return value_ == other.value_;
  }
  /**
   * @brief Checks if this node and another node are different
   *
   * @param other The other node to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const Node &other) const {
    return !(*this == other);
  }

  /**
   * @brief Prints the node to an output stream
   *
   * @param os The output stream to print to
   * @param value The node to print
   * @return The same output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const Node &value) {
    if (std::holds_alternative<Map>(value.value_)) {
      return os << "{ Map " << std::get<Map>(value.value_) << " }";
    }
    if (std::holds_alternative<List>(value.value_)) {
      return os << "{ List " << std::get<List>(value.value_) << " }";
    }
    return os << "{ Value " << std::get<Value>(value.value_) << " }";
  }

private:
  /**
   * @brief The value of this node
   *
   */
  std::variant<Value, Map, List> value_;
};

} // namespace termite

#endif
