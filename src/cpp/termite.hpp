/**
 * @file termite.hpp
 * @brief The c++ Termite Data Model Generator code which implements errors and
 * input output to yaml and json
 * @version 0.1
 * @date 2024-06-15
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

namespace termite {

namespace {

// Helper trait to detect if T has operator<<
template <typename T, typename = void>
struct has_insertion_operator : std::false_type {};

template <typename T>
struct has_insertion_operator<
    T,
    std::void_t<decltype(std::declval<std::ostream &>() << std::declval<T>())>>
    : std::true_type {};

// Helper trait to detect if T has operator>>
template <typename T, typename = void>
struct has_parsing_operator : std::false_type {};

template <typename T>
struct has_parsing_operator<
    T, std::void_t<decltype(std::declval<std::istream &>() >>
                            std::declval<T &>())>> : std::true_type {};

// Helper trait to detect if T has operator==
template <typename T, typename = void>
struct has_equality_operator : std::false_type {};
// T, std::enable_if_t<std::is_same_v<decltype(std::declval<T>() ==
// std::declval<T>()), bool>>
template <typename T>
struct has_equality_operator<
    T, std::void_t<decltype(std::declval<T>() == std::declval<T>())>>
    : std::true_type {};
}  // namespace

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

template <class T>
class Reference {
public:
  explicit Reference(const T &ref) : ref_(ref) {}

  /**
   * @brief Gets the stored reference
   * 
   * @return The reference
   */
  const T &get() const {
    return ref_;
  }

  /**
   * @brief Checks if this value and the other value are identical
   *
   * @param other The other value to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]]
  typename std::enable_if<has_equality_operator<T>::value, bool>::type
  operator==(const Reference &other) const {
    return ref_ == other.ref_;
  }
  /**
   * @brief Checks if this value and the other value are different
   *
   * @param other The other value to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const Reference &other) const {
    return !(*this == other);
  }
  /**
   * @brief Prints the value to an ostream
   *
   * @param os The stream to print to
   * @param value The value to print
   * @return The same ostream
   */
  typename std::enable_if<has_insertion_operator<T>::value,
                          std::ostream &>::type friend
  operator<<(std::ostream &os, const Reference &value) {
    return os << "{ " << value.ref_ << " }";
  }

private:
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
  /**
   * @brief Converts this error with its location to a string
   *
   * @return The string with the error
   */
  [[nodiscard]] std::string to_string() const {
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
  bool is_ok() const { return std::holds_alternative<T>(value_); }
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
  typename std::enable_if<has_equality_operator<T>::value, bool>::type
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
  typename std::enable_if<has_insertion_operator<T>::value,
                          std::ostream &>::type friend
  operator<<(std::ostream &os, const Result &result) {
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
      value.erase(value.begin(),
                  std::find_if(value.begin(), value.end(), [](unsigned char ch) {
                    return !std::isspace(ch);
                  }));
      value.erase(std::find_if(value.rbegin(), value.rend(),
                              [](unsigned char ch) { return !std::isspace(ch); })
                      .base(),
                  value.end());

      value_ = std::move(value);
    }

    /**
     * @brief Casts the node value to the given type, if operator>> is not defined
     * then an error occurs
     *
     * @tparam T The type to cast to
     * @return A result of the given type which is always err
     */
    template <typename T>
    [[nodiscard]]
    typename std::enable_if<!has_parsing_operator<T>::value, Result<T>>::type
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
    typename std::enable_if<has_parsing_operator<T>::value, Result<T>>::type
    to_value() const {
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

    /**
     * @brief Creates a cloned copy of this node value
     * 
     * @return The new copy
     *//*
    [[nodiscard]] std::unique_ptr<std::variant<NodeValue, NodeMap, NodeList>> Clone() const {
      return std::make_unique<std::variant<NodeValue, NodeMap, NodeList>>(NodeValue(value_));
    }*/

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
    /**
     * @brief Converts this node value to a string
     *
     * @return The string with this node value in
     */
    [[nodiscard]] std::string to_string() const {
      std::stringstream ss;
      ss << *this;
      return ss.str();
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
     * @brief Constructs a new node map
     *
     * @param map The map of this node
     */
    explicit Map(std::map<std::string, Node> map)
        : map_(std::move(map)) {}

    /**
     * @brief Casts the node map to the given type, if not specialized then it
     * will always return an error
     *
     * @tparam T The type to cast to
     * @param allow_skipping if true then key-value pairs can be skipped when
     * parsing maps, otherwise an error is thrown if all key-value pairs are not
     * used
     * @return A result of the given type
     */
    template <typename T>
    [[nodiscard]] Result<T> to_value(bool allow_skipping = false) const {
      return Result<T>::err(Error("Parsing not implemented for given type"));
    }

    /**
     * @brief Creates a cloned copy of this node map
     * 
     * @return The new copy
     *//*
    [[nodiscard]] std::unique_ptr<std::variant<NodeValue, NodeMap, NodeList>> Clone() const {
      std::vector<std::pair<std::string, Node>> key_values;
      std::transform(map_.cbegin(), map_.cend(), std::back_inserter(key_values), [](const std::pair<std::string, std::unique_ptr<Node>> &key_value) {
        return std::make_pair(key_value.first, key_value.second->Clone());
      });
      return std::make_unique<std::variant<NodeValue, NodeMap, NodeList>>(NodeMap(std::map<std::string, std::unique_ptr<Node>>(std::make_move_iterator(key_values.begin()), std::make_move_iterator(key_values.end()))));
    }*/

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
    /**
     * @brief Converts this node map to a string
     *
     * @return The string with this node map in
     */
    [[nodiscard]] std::string to_string() const {
      std::stringstream ss;
      ss << *this;
      return ss.str();
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
     * @brief Constructs a new node list
     *
     * @param list The list of this node
     */
    explicit List(std::vector<Node> list) : list_(std::move(list)) {}

    /**
     * @brief Casts the node list to the given type, if not specialized then it
     * will always return an error
     *
     * @tparam T The type to cast to
     * @param allow_skipping if true then key-value pairs can be skipped when
     * parsing maps, otherwise an error is thrown if all key-value pairs are not
     * used
     * @return A result of the given type
     */
    template <typename T>
    [[nodiscard]] Result<T> to_value(bool allow_skipping = false) const {
      return Result<T>::err(Error("Parsing not implemented for given type"));
    }

    /**
     * @brief Creates a cloned copy of this node list
     * 
     * @return The new copy
     *//*
    [[nodiscard]] std::unique_ptr<std::variant<NodeValue, NodeMap, NodeList>> Clone() const {
      std::vector<Node> values;
      std::transform(list_.cbegin(), list_.cend(), std::back_inserter(values), [](const Node &node) {
        return node.Clone();
      });
      return std::make_unique<std::variant<NodeValue, NodeMap, NodeList>>(NodeList(std::move(values)));
    }*/

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
      return os << "{ list: [ ";
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
    /**
     * @brief Converts this node list to a string
     *
     * @return The string with this node list in
     */
    [[nodiscard]] std::string to_string() const {
      std::stringstream ss;
      ss << *this;
      return ss.str();
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
  Node(const Node &node) : Node(node.value_) {}

  /**
   * @brief Casts the node to the given type
   *
   * @tparam T The type to cast to
   * @param allow_skipping if true then key-value pairs can be skipped when
   * parsing maps, otherwise an error is thrown if all key-value pairs are not
   * used
   * @return A result of the given type
   */
  template <typename T>
  [[nodiscard]] Result<T> to_value(bool allow_skipping = false) const {
    if (std::holds_alternative<Map>(value_)) {
      return std::get<Map>(value_).to_value<T>(allow_skipping);
    }
    if (std::holds_alternative<List>(value_)) {
      return std::get<List>(value_).to_value<T>(allow_skipping);
    }
    return std::get<Value>(value_).to_value<T>();
  }

  /*[[nodiscard]] Node Clone() const {
    if (std::holds_alternative<NodeMap>(*value_)) {
      return Node(std::get<NodeMap>(*value_).Clone());
    }
    if (std::holds_alternative<NodeList>(*value_)) {
      return Node(std::get<NodeList>(*value_).Clone());
    }
      return Node(std::get<NodeValue>(*value_).Clone());
  }*/

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
  /**
   * @brief Converts this node to a string
   *
   * @return The string with this node in
   */
  [[nodiscard]] std::string to_string() const {
    std::stringstream ss;
    ss << *this;
    return ss.str();
  }

private:
  /**
   * @brief The value of this node
   *
   */
  std::variant<Value, Map, List> value_;
};

}  // namespace termite

#endif
