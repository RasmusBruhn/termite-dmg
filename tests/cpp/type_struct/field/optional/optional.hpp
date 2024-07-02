// Generated with the Termite Data Model Generator
#ifndef HEADER_TERMITE_H_INCLUDED
#define HEADER_TERMITE_H_INCLUDED

#include <iostream>
#include <optional>
#include <variant>
#include <termite.hpp>



/**
 * @brief 
 * 
 */
class DataType {
public:
  /**
   * @brief Constructs a new DataType object 
   * 
   * @param field1 
   * @param field2 
   */
  [[nodiscard]] static termite::Result<DataType> from_values(int field1 = 1, std::optional<float> field2 = std::nullopt) {
    termite::Result<termite::Empty> validate_result = termite::Result<termite::Empty>::ok(termite::Empty());

    validate_result = validate_field1(field1);
    if (!validate_result.is_ok()) {
      termite::Error error = validate_result.get_err();
      error.add_field("field1");
      return termite::Result<DataType>::err(std::move(error));
    }

    validate_result = validate_field2(field2);
    if (!validate_result.is_ok()) {
      termite::Error error = validate_result.get_err();
      error.add_field("field2");
      return termite::Result<DataType>::err(std::move(error));
    }

    return termite::Result<DataType>::ok(DataType(std::move(field1), std::move(field2)));
  }

  /**
   * @brief Sets the value of field1 if it fulfills the constraints:
   * 
   * @param value The value of field1
   * @return An error if one of the constraints were not fulfilled
   */
  [[nodiscard]] termite::Result<termite::Empty> set_field1(int value) {
    termite::Result<termite::Empty> validate_result = validate_field1(value);
    if (!validate_result.is_ok()) {
      return validate_result;
    }

    field1_ = std::move(value);
    return termite::Result<termite::Empty>::ok(termite::Empty());
  }
  /**
   * @brief Sets the value of field2 if it fulfills the constraints:
   * 
   * @param value The value of field2
   * @return An error if one of the constraints were not fulfilled
   */
  [[nodiscard]] termite::Result<termite::Empty> set_field2(std::optional<float> value) {
    termite::Result<termite::Empty> validate_result = validate_field2(value);
    if (!validate_result.is_ok()) {
      return validate_result;
    }

    field2_ = std::move(value);
    return termite::Result<termite::Empty>::ok(termite::Empty());
  }

  /**
   * @brief Retrieves a reference to the value of field1
   * 
   * @return The reference 
   */
  [[nodiscard]] const int &get_field1() const {
    return field1_;
  }
  /**
   * @brief Retrieves a reference to the value of field2
   * 
   * @return The reference 
   */
  [[nodiscard]] const std::optional<float> &get_field2() const {
    return field2_;
  }

  /**
   * @brief Checks if this object the the other object are identical
   * 
   * @param x The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType &x) {
    return field1_ == x.field1_ && field2_ == x.field2_;
  }
  /**
   * @brief Checks if this object the the other object are different
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
    return os << "{ " << "field1: " << x.field1_ << ", " << "field2: " << x.field2_ << " }";
  }

private:
  explicit DataType(int field1, std::optional<float> field2) : field1_(std::move(field1)), field2_(std::move(field2)) {}

  /**
   * @brief Validates if field1 is correct using the following constaints:
   * 
   * @param  The value of the parameter to validate
   */
  [[nodiscard]] static termite::Result<termite::Empty> validate_field1(const int &) {


    return termite::Result<termite::Empty>::ok(termite::Empty());
  }
  /**
   * @brief Validates if field2 is correct using the following constaints:
   * 
   * @param  The value of the parameter to validate
   */
  [[nodiscard]] static termite::Result<termite::Empty> validate_field2(const std::optional<float> &) {


    return termite::Result<termite::Empty>::ok(termite::Empty());
  }

  /**
   * @brief 
   * 
   */
  int field1_;
  /**
   * @brief 
   * 
   */
  std::optional<float> field2_;
};



#endif
