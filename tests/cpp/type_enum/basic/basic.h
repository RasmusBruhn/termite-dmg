// Generated with the Termite Data Model Generator
#ifndef HEADER_TERMITE_H_INCLUDED
#define HEADER_TERMITE_H_INCLUDED

#include <iostream>
#include <sstream>
#include <optional>
#include <variant>
#include <algorithm>
#include <termite.hpp>



namespace test {

/**
 * @brief
 *
 */
struct DataType {
  /**
   * @brief The values of this enum
   *
   */
  enum class Enum {
    /**
     * @brief An integer
     *
     */
    kInt1,
    /**
     * @brief Another integer
     *
     */
    kInt2,
    /**
     * @brief
     *
     */
    kFloat,
    /**
     * @brief Nothing
     *
     */
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
     * @brief Constructs a new Int1 object
     *
     * @param value The value of the enum
     */
    explicit TypeInt1(int value) : value(std::move(value)) {}

    /**
     * @brief Checks if this object and the other object are identical
     *
     * @param x The other object to compare with
     * @return true if they are identical, false if not
     */
    [[nodiscard]] bool operator==(const TypeInt1 &x) const;
    /**
     * @brief Checks if this object and the other object are different
     *
     * @param x The other object to compare with
     * @return true if they are different, false if not
     */
    [[nodiscard]] bool operator!=(const TypeInt1 &x) const {
      return !(*this == x);
    }
    /**
     * @brief Prints the object onto the output stream
     *
     * @param os The output stream to print to
     * @param x The object to print
     * @return The output stream
     */
    friend std::ostream &operator<<(std::ostream &os, const TypeInt1 &x);
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
     * @brief Constructs a new Int2 object
     *
     * @param value The value of the enum
     */
    explicit TypeInt2(int value) : value(std::move(value)) {}

    /**
     * @brief Checks if this object and the other object are identical
     *
     * @param x The other object to compare with
     * @return true if they are identical, false if not
     */
    [[nodiscard]] bool operator==(const TypeInt2 &x) const;
    /**
     * @brief Checks if this object and the other object are different
     *
     * @param x The other object to compare with
     * @return true if they are different, false if not
     */
    [[nodiscard]] bool operator!=(const TypeInt2 &x) const {
      return !(*this == x);
    }
    /**
     * @brief Prints the object onto the output stream
     *
     * @param os The output stream to print to
     * @param x The object to print
     * @return The output stream
     */
    friend std::ostream &operator<<(std::ostream &os, const TypeInt2 &x);
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
     * @brief Constructs a new Float object
     *
     * @param value The value of the enum
     */
    explicit TypeFloat(float value) : value(std::move(value)) {}

    /**
     * @brief Checks if this object and the other object are identical
     *
     * @param x The other object to compare with
     * @return true if they are identical, false if not
     */
    [[nodiscard]] bool operator==(const TypeFloat &x) const;
    /**
     * @brief Checks if this object and the other object are different
     *
     * @param x The other object to compare with
     * @return true if they are different, false if not
     */
    [[nodiscard]] bool operator!=(const TypeFloat &x) const {
      return !(*this == x);
    }
    /**
     * @brief Prints the object onto the output stream
     *
     * @param os The output stream to print to
     * @param x The object to print
     * @return The output stream
     */
    friend std::ostream &operator<<(std::ostream &os, const TypeFloat &x);
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
    [[nodiscard]] bool operator==(const TypeEmpty &x) const;
    /**
     * @brief Checks if this object and the other object are different
     *
     * @param x The other object to compare with
     * @return true if they are different, false if not
     */
    [[nodiscard]] bool operator!=(const TypeEmpty &x) const {
      return !(*this == x);
    }
    /**
     * @brief Prints the object onto the output stream
     *
     * @param os The output stream to print to
     * @param x The object to print
     * @return The output stream
     */
    friend std::ostream &operator<<(std::ostream &os, const TypeEmpty &x);
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
  [[nodiscard]] bool operator==(const DataType &x) const;
  /**
   * @brief Checks if this object and the other object are different
   *
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const DataType &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   *
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const DataType &x);

  /**
   * @brief The value of the enum
   *
   */
  std::variant<TypeInt1, TypeInt2, TypeFloat, TypeEmpty> value;
};

} // namespace test

namespace termite {

template<>
[[nodiscard]] Result<test::DataType> Node::Value::to_value<test::DataType>() const;

template<>
[[nodiscard]] Result<test::DataType> Node::Map::to_value<test::DataType>() const;

template<>
[[nodiscard]] Node Node::from_value<test::DataType>(const test::DataType &value);

} // namespace termite



#endif
