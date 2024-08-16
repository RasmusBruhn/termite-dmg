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
  /**
   * @brief The values of this enum
   * 
   */
  enum Enum {
    kInt1,
    kInt2,
    kFloat,
    kEmpty,
  };

  /**
   * @brief The data for when the enum is a Int1
   * 
   */
  struct TypeInt1 {
    /**
     * @brief The value
     * 
     */
    int value;

    /**
     * @brief Checks if this object and the other object are identical
     * 
     * @param x The other object to compare with
     * @return true if they are identical, false if not
     */
    [[nodiscard]] bool operator==(const TypeInt1 &x) {
      return value == x.value;
    }
    /**
     * @brief Checks if this object and the other object are different
     * 
     * @param x The other object to compare with
     * @return true if they are different, false if not
     */
    [[nodiscard]] bool operator!=(const TypeInt1 &x) {
      return !(*this == x);
    }
    /**
     * @brief Prints the object onto the output stream
     * 
     * @param os The output stream to print to
     * @param x The object to print
     * @return The output stream
     */
    friend std::ostream &operator<<(std::ostream &os, const TypeInt1 &x) {
      os << "{ value: " << x.value << " }";
    }
  };
  /**
   * @brief The data for when the enum is a Int2
   * 
   */
  struct TypeInt2 {
    /**
     * @brief The value
     * 
     */
    int value;

    /**
     * @brief Checks if this object and the other object are identical
     * 
     * @param x The other object to compare with
     * @return true if they are identical, false if not
     */
    [[nodiscard]] bool operator==(const TypeInt2 &x) {
      return value == x.value;
    }
    /**
     * @brief Checks if this object and the other object are different
     * 
     * @param x The other object to compare with
     * @return true if they are different, false if not
     */
    [[nodiscard]] bool operator!=(const TypeInt2 &x) {
      return !(*this == x);
    }
    /**
     * @brief Prints the object onto the output stream
     * 
     * @param os The output stream to print to
     * @param x The object to print
     * @return The output stream
     */
    friend std::ostream &operator<<(std::ostream &os, const TypeInt2 &x) {
      os << "{ value: " << x.value << " }";
    }
  };
  /**
   * @brief The data for when the enum is a Float
   * 
   */
  struct TypeFloat {
    /**
     * @brief The value
     * 
     */
    float value;

    /**
     * @brief Checks if this object and the other object are identical
     * 
     * @param x The other object to compare with
     * @return true if they are identical, false if not
     */
    [[nodiscard]] bool operator==(const TypeFloat &x) {
      return value == x.value;
    }
    /**
     * @brief Checks if this object and the other object are different
     * 
     * @param x The other object to compare with
     * @return true if they are different, false if not
     */
    [[nodiscard]] bool operator!=(const TypeFloat &x) {
      return !(*this == x);
    }
    /**
     * @brief Prints the object onto the output stream
     * 
     * @param os The output stream to print to
     * @param x The object to print
     * @return The output stream
     */
    friend std::ostream &operator<<(std::ostream &os, const TypeFloat &x) {
      os << "{ value: " << x.value << " }";
    }
  };
  /**
   * @brief The data for when the enum is a Empty
   * 
   */
  struct TypeEmpty {


    /**
     * @brief Checks if this object and the other object are identical
     * 
     * @param x The other object to compare with
     * @return true if they are identical, false if not
     */
    [[nodiscard]] bool operator==(const TypeEmpty &x) {
      return true;
    }
    /**
     * @brief Checks if this object and the other object are different
     * 
     * @param x The other object to compare with
     * @return true if they are different, false if not
     */
    [[nodiscard]] bool operator!=(const TypeEmpty &x) {
      return !(*this == x);
    }
    /**
     * @brief Prints the object onto the output stream
     * 
     * @param os The output stream to print to
     * @param x The object to print
     * @return The output stream
     */
    friend std::ostream &operator<<(std::ostream &os, const TypeEmpty &x) {
      os << "{  }";
    }
  };

  /**
   * @brief Constructs a new DataType object
   * 
   * @param value The value of the enum
   */
  explicit DataType(std::variant<TypeInt1, TypeInt2, TypeFloat, TypeEmpty> value) : value(std::move(value)) {}

  /**
   * @brief Returns the enum type that is stored
   * 
   * @return The enum type
   */
  [[nodiscard]] Enum enum_type() const {
    return static_cast<Enum>(value.index());
  }

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType &x) {
    return value == x.value;
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
    os << "{ value: ";
    switch (x.value.index()) {
    case Enum::kInt1:
      os << "Int1(" << std::get<TypeInt1>(x.value).value << ")";
      break;
    case Enum::kInt2:
      os << "Int2(" << std::get<TypeInt2>(x.value).value << ")";
      break;
    case Enum::kFloat:
      os << "Float(" << std::get<TypeFloat>(x.value).value << ")";
      break;
    case Enum::kEmpty:
      os << "Empty";
      break;
    default:
      os << "Unknown";
      break;
    }
    return os << " }";
  }

  /**
   * @brief The value of the enum
   * 
   */
  std::variant<TypeInt1, TypeInt2, TypeFloat, TypeEmpty> value;
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
[[nodiscard]] Result<test::DataType> Node::Value::to_value<test::DataType>() const {
  return Result<test::DataType>::err(Error(""));
}

} // namespace termite



#endif
