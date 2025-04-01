// Generated with the Termite Data Model Generator
#ifndef FULL_EXAMPLE_TERMITE_H_INCLUDED
#define FULL_EXAMPLE_TERMITE_H_INCLUDED

#include <iostream>
#include <sstream>
#include <optional>
#include <variant>
#include <algorithm>
#include <termite.hpp>

// This is my .h file
#include "custom.h"
#include <string>

using String = std::string;


namespace test::name::space {

/**
 * @brief A string that represents a version number
 * 
 */
class VersionString {
public:
  /**
   * @brief Constructs a new VersionString object, it must be valid or an exception will be thrown
   * 
   * @param value The value to store 
   */
  explicit VersionString(String value) : VersionString(from_value(std::move(value)).get_ok()) {}
  /**
   * @brief Constructs a new VersionString object
   * 
   * @param value The value to store 
   * @return The new constrained type or an error if some constraints were not upheld
   */
  [[nodiscard]] static termite::Result<VersionString> from_value(String value);

  /**
   * @brief Sets the value if it fulfills the constraints:
   * - custom::is_valid_version_string(x)
   * 
   * @param value The value to set
   * @return An error if one of the constraints were not fulfilled
   */
  [[nodiscard]] termite::Result<termite::Empty> set(String value);

  /**
   * @brief Retrieves a reference to the value
   * 
   * @return The reference
   */
  [[nodiscard]] const String &get() const {
    return value_;
  }

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const VersionString &x) const;
  /**
   * @brief Checks if this object and the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const VersionString &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const VersionString &x);

private:
  /**
   * @brief Constructs a new VersionString object
   * 
   * @param value The value to store
   * @param _ A nullptr
   */
  explicit VersionString(String value, void *) : value_(std::move(value)) {}

  /**
   * @brief Validates if value is correct using the following constraints:
   * - custom::is_valid_version_string(x)
   * 
   * @param x The value of the parameter to validate
   */
  [[nodiscard]] static termite::Result<termite::Empty> validate(const String &x);

  /**
   * @brief The validated value
   * 
   */
  String value_;
};

/**
 * @brief A single size value
 * 
 */
class SizeValue {
public:
  /**
   * @brief Constructs a new SizeValue object, it must be valid or an exception will be thrown
   * 
   * @param value The value to store 
   */
  explicit SizeValue(int value) : SizeValue(from_value(std::move(value)).get_ok()) {}
  /**
   * @brief Constructs a new SizeValue object
   * 
   * @param value The value to store 
   * @return The new constrained type or an error if some constraints were not upheld
   */
  [[nodiscard]] static termite::Result<SizeValue> from_value(int value);

  /**
   * @brief Sets the value if it fulfills the constraints:
   * - x > 0
   * 
   * @param value The value to set
   * @return An error if one of the constraints were not fulfilled
   */
  [[nodiscard]] termite::Result<termite::Empty> set(int value);

  /**
   * @brief Retrieves a reference to the value
   * 
   * @return The reference
   */
  [[nodiscard]] const int &get() const {
    return value_;
  }

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const SizeValue &x) const;
  /**
   * @brief Checks if this object and the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const SizeValue &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const SizeValue &x);

private:
  /**
   * @brief Constructs a new SizeValue object
   * 
   * @param value The value to store
   * @param _ A nullptr
   */
  explicit SizeValue(int value, void *) : value_(std::move(value)) {}

  /**
   * @brief Validates if value is correct using the following constraints:
   * - x > 0
   * 
   * @param x The value of the parameter to validate
   */
  [[nodiscard]] static termite::Result<termite::Empty> validate(const int &x);

  /**
   * @brief The validated value
   * 
   */
  int value_;
};

/**
 * @brief The size of a geometry
 * 
 */
struct Size {
public:
  /**
   * @brief Constructs a new Size object
   * 
   * @param w The width of the geometry
   * @param h The height of the geometry
   * @param extra_fields Any extra fields to attach to this struct
   */
  explicit Size(SizeValue w, SizeValue h, ::termite::Node::Map extra_fields = ::termite::Node::Map()) : w(std::move(w)), h(std::move(h)), extra_fields(std::move(extra_fields)) {}


  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const Size &x) const;
  /**
   * @brief Checks if this object and the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const Size &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const Size &x);

  /**
   * @brief The width of the geometry
   * 
   */
  SizeValue w;
  /**
   * @brief The height of the geometry
   * 
   */
  SizeValue h;
  /**
   * @brief All extra fields from when reading which could not be captured
   * 
   */
  ::termite::Node::Map extra_fields;
};

/**
 * @brief A point in 2D space
 * 
 */
struct Point {
public:
  /**
   * @brief Constructs a new Point object
   * 
   * @param x The x coordinate of the point
   * @param y The y coordinate of the point
   * @param extra_fields Any extra fields to attach to this struct
   */
  explicit Point(int x, int y, ::termite::Node::Map extra_fields = ::termite::Node::Map()) : x(std::move(x)), y(std::move(y)), extra_fields(std::move(extra_fields)) {}


  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const Point &x) const;
  /**
   * @brief Checks if this object and the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const Point &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const Point &x);

  /**
   * @brief The x coordinate of the point
   * 
   */
  int x;
  /**
   * @brief The y coordinate of the point
   * 
   */
  int y;
  /**
   * @brief All extra fields from when reading which could not be captured
   * 
   */
  ::termite::Node::Map extra_fields;
};

/**
 * @brief The state of a geometry
 * 
 */
struct State {
  /**
   * @brief The values of this enum
   * 
   */
  enum class Enum {
    /**
     * @brief The geometry is filled
     * 
     */
    kFilled,
    /**
     * @brief The geometry is inactive
     * 
     */
    kEdge,
  };

  /**
   * @brief The data for when the enum is a Filled
   * 
   */
  struct TypeFilled {
    /**
     * @brief Checks if this object and the other object are identical
     * 
     * @param x The other object to compare with
     * @return true if they are identical, false if not
     */
    [[nodiscard]] bool operator==(const TypeFilled &x) const;
    /**
     * @brief Checks if this object and the other object are different
     * 
     * @param x The other object to compare with
     * @return true if they are different, false if not
     */
    [[nodiscard]] bool operator!=(const TypeFilled &x) const {
      return !(*this == x);
    }
    /**
     * @brief Prints the object onto the output stream
     * 
     * @param os The output stream to print to
     * @param x The object to print
     * @return The output stream
     */
    friend std::ostream &operator<<(std::ostream &os, const TypeFilled &x);
  };

  /**
   * @brief The data for when the enum is a Edge
   * 
   */
  struct TypeEdge {
    /**
     * @brief The value
     * 
     */
    SizeValue value;

    /**
     * @brief Constructs a new Edge object
     * 
     * @param value The value of the enum
     */
    explicit TypeEdge(SizeValue value) : value(std::move(value)) {}

    /**
     * @brief Checks if this object and the other object are identical
     * 
     * @param x The other object to compare with
     * @return true if they are identical, false if not
     */
    [[nodiscard]] bool operator==(const TypeEdge &x) const;
    /**
     * @brief Checks if this object and the other object are different
     * 
     * @param x The other object to compare with
     * @return true if they are different, false if not
     */
    [[nodiscard]] bool operator!=(const TypeEdge &x) const {
      return !(*this == x);
    }
    /**
     * @brief Prints the object onto the output stream
     * 
     * @param os The output stream to print to
     * @param x The object to print
     * @return The output stream
     */
    friend std::ostream &operator<<(std::ostream &os, const TypeEdge &x);
  };

  /**
   * @brief Constructs a new State object
   * 
   * @param value The value of the enum
   */
  explicit State(std::variant<TypeFilled, TypeEdge> value) : value(std::move(value)) {}

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
  [[nodiscard]] bool operator==(const State &x) const;
  /**
   * @brief Checks if this object and the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const State &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const State &x);

  /**
   * @brief The value of the enum
   * 
   */
  std::variant<TypeFilled, TypeEdge> value;
};

/**
 * @brief All global default values
 * 
 */
struct DefaultValues {
public:
  /**
   * @brief Constructs a new DefaultValues object
   * 
   * @param state The state of a geometry
   * @param size The default size of the geometry
   * @param extra_fields Any extra fields to attach to this struct
   */
  explicit DefaultValues(State state, Size size, ::termite::Node::Map extra_fields = ::termite::Node::Map()) : state(std::move(state)), size(std::move(size)), extra_fields(std::move(extra_fields)) {}

  /**
   * @brief Gets the default value for state
   * 
   * @return The default value for state
   */
  [[nodiscard]] static State default_state();

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DefaultValues &x) const;
  /**
   * @brief Checks if this object and the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const DefaultValues &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const DefaultValues &x);

  /**
   * @brief The state of a geometry
   * 
   */
  State state;
  /**
   * @brief The default size of the geometry
   * 
   */
  Size size;
  /**
   * @brief All extra fields from when reading which could not be captured
   * 
   */
  ::termite::Node::Map extra_fields;
};

/**
 * @brief A rectangle geometry
 * 
 */
struct Rectangle {
public:
  /**
   * @brief Constructs a new Rectangle object
   * 
   * @param center The center point of the rectangle
   * @param size The size of the rectangle
   * @param state The state of the circle
   * @param extra_fields Any extra fields to attach to this struct
   */
  explicit Rectangle(Point center, std::optional<Size> size, std::optional<State> state, ::termite::Node::Map extra_fields = ::termite::Node::Map()) : center(std::move(center)), size(std::move(size)), state(std::move(state)), extra_fields(std::move(extra_fields)) {}

  /**
   * @brief Gets the default value for size
   * 
   * @return The default value for size
   */
  [[nodiscard]] static std::optional<Size> default_size();
  /**
   * @brief Gets the default value for state
   * 
   * @return The default value for state
   */
  [[nodiscard]] static std::optional<State> default_state();

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const Rectangle &x) const;
  /**
   * @brief Checks if this object and the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const Rectangle &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const Rectangle &x);

  /**
   * @brief The center point of the rectangle
   * 
   */
  Point center;
  /**
   * @brief The size of the rectangle
   * 
   */
  std::optional<Size> size;
  /**
   * @brief The state of the circle
   * 
   */
  std::optional<State> state;
  /**
   * @brief All extra fields from when reading which could not be captured
   * 
   */
  ::termite::Node::Map extra_fields;
};

/**
 * @brief A circle geometry
 * 
 */
struct Circle {
public:
  /**
   * @brief Constructs a new Circle object
   * 
   * @param center The center point of the circle
   * @param radius The radius of the circle
   * @param state The state of the circle
   * @param extra_fields Any extra fields to attach to this struct
   */
  explicit Circle(Point center, SizeValue radius, std::optional<State> state, ::termite::Node::Map extra_fields = ::termite::Node::Map()) : center(std::move(center)), radius(std::move(radius)), state(std::move(state)), extra_fields(std::move(extra_fields)) {}

  /**
   * @brief Gets the default value for state
   * 
   * @return The default value for state
   */
  [[nodiscard]] static std::optional<State> default_state();

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const Circle &x) const;
  /**
   * @brief Checks if this object and the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const Circle &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const Circle &x);

  /**
   * @brief The center point of the circle
   * 
   */
  Point center;
  /**
   * @brief The radius of the circle
   * 
   */
  SizeValue radius;
  /**
   * @brief The state of the circle
   * 
   */
  std::optional<State> state;
  /**
   * @brief All extra fields from when reading which could not be captured
   * 
   */
  ::termite::Node::Map extra_fields;
};

/**
 * @brief A geometry that can be a rectangle or a circle
 * 
 */
struct Geometry {
  /**
   * @brief Constructs a new Geometry object
   * 
   * @param value The value of the variant
   */
  explicit Geometry(std::variant<Rectangle, Circle> value) : value(std::move(value)) {}

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const Geometry &x) const;
  /**
   * @brief Checks if this object and the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const Geometry &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const Geometry &x);

  /**
   * @brief The value of the variant
   * 
   */
  std::variant<Rectangle, Circle> value;
};

/**
 * @brief A list of geometries
 * 
 */
struct GeometryList {
public:
  /**
   * @brief Constructs a new GeometryList object
   * 
   * @param values The values of the array
   */
  explicit GeometryList(std::vector<Geometry> values) : values(std::move(values)) {}

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const GeometryList &x) const;
  /**
   * @brief Checks if this object and the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const GeometryList &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const GeometryList &x);

  /**
   * @brief The values of the array
   * 
   */
  std::vector<Geometry> values;
};

/**
 * @brief The main data model
 * 
 */
struct DataModel {
public:
  /**
   * @brief Constructs a new DataModel object
   * 
   * @param version The version string for the data model
   * @param defaults The default values for the data model
   * @param geometries A list of geometries
   * @param extra_fields Any extra fields to attach to this struct
   */
  explicit DataModel(VersionString version, DefaultValues defaults, GeometryList geometries, ::termite::Node::Map extra_fields = ::termite::Node::Map()) : version(std::move(version)), defaults(std::move(defaults)), geometries(std::move(geometries)), extra_fields(std::move(extra_fields)) {}

  /**
   * @brief Gets the default value for version
   * 
   * @return The default value for version
   */
  [[nodiscard]] static VersionString default_version();

  /**
   * @brief Checks if this object and the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataModel &x) const;
  /**
   * @brief Checks if this object and the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const DataModel &x) const {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param x The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const DataModel &x);

  /**
   * @brief The version string for the data model
   * 
   */
  VersionString version;
  /**
   * @brief The default values for the data model
   * 
   */
  DefaultValues defaults;
  /**
   * @brief A list of geometries
   * 
   */
  GeometryList geometries;
  /**
   * @brief All extra fields from when reading which could not be captured
   * 
   */
  ::termite::Node::Map extra_fields;
};

} // namespace test::name::space

namespace termite {

template<>
[[nodiscard]] Result<test::name::space::VersionString> Node::to_value<test::name::space::VersionString>() const;

template<>
[[nodiscard]] Result<test::name::space::SizeValue> Node::to_value<test::name::space::SizeValue>() const;

template<>
[[nodiscard]] Result<test::name::space::Size> Node::Map::to_value<test::name::space::Size>() const;

template<>
[[nodiscard]] Result<test::name::space::Point> Node::Map::to_value<test::name::space::Point>() const;

template<>
[[nodiscard]] Result<test::name::space::State> Node::Value::to_value<test::name::space::State>() const;

template<>
[[nodiscard]] Result<test::name::space::State> Node::Map::to_value<test::name::space::State>() const;

template<>
[[nodiscard]] Result<test::name::space::DefaultValues> Node::Map::to_value<test::name::space::DefaultValues>() const;

template<>
[[nodiscard]] Result<test::name::space::Rectangle> Node::Map::to_value<test::name::space::Rectangle>() const;

template<>
[[nodiscard]] Result<test::name::space::Circle> Node::Map::to_value<test::name::space::Circle>() const;

template<>
[[nodiscard]] Result<test::name::space::Geometry> Node::to_value<test::name::space::Geometry>() const;

template<>
[[nodiscard]] Result<test::name::space::GeometryList> Node::List::to_value<test::name::space::GeometryList>() const;

template<>
[[nodiscard]] Result<test::name::space::DataModel> Node::Map::to_value<test::name::space::DataModel>() const;

} // namespace termite

// This is the end of my .h file

#endif
