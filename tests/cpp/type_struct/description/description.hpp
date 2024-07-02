// Generated with the Termite Data Model Generator
#ifndef HEADER_TERMITE_H_INCLUDED
#define HEADER_TERMITE_H_INCLUDED

#include <iostream>
#include <optional>
#include <variant>
#include <termite.hpp>



/**
 * @brief description1
 * 
 */
class DataType1 {
public:
  /**
   * @brief Constructs a new DataType1 object 
   * 
   * 
   */
  [[nodiscard]] static termite::Result<DataType1> from_values() {
    

    return termite::Result<DataType1>::ok(DataType1());
  }





  /**
   * @brief Checks if this object the the other object are identical
   * 
   * @param  The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType1 &) {
    return true;
  }
  /**
   * @brief Checks if this object the the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const DataType1 &x) {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param  The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const DataType1 &) {
    return os << "{ " << "" << " }";
  }

private:
  explicit DataType1() {}




};

/**
 * @brief description2
 * 
 */
class DataType2 {
public:
  /**
   * @brief Constructs a new DataType2 object 
   * 
   * 
   */
  [[nodiscard]] static termite::Result<DataType2> from_values() {
    

    return termite::Result<DataType2>::ok(DataType2());
  }





  /**
   * @brief Checks if this object the the other object are identical
   * 
   * @param  The other object to compare with
   * @return true if they are identical, false if not
   */
  [[nodiscard]] bool operator==(const DataType2 &) {
    return true;
  }
  /**
   * @brief Checks if this object the the other object are different
   * 
   * @param x The other object to compare with
   * @return true if they are different, false if not
   */
  [[nodiscard]] bool operator!=(const DataType2 &x) {
    return !(*this == x);
  }
  /**
   * @brief Prints the object onto the output stream
   * 
   * @param os The output stream to print to
   * @param  The object to print
   * @return The output stream
   */
  friend std::ostream &operator<<(std::ostream &os, const DataType2 &) {
    return os << "{ " << "" << " }";
  }

private:
  explicit DataType2() {}




};



#endif
