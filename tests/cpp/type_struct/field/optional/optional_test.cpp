#include <iostream>
#include "optional.hpp"

int main() {
  auto value1 = DataType::from_values().get_ok();
  auto value2 = DataType::from_values(-2, 3.5).get_ok();
  if (value1 != value1) {
    return 1;
  }
  if (value2 != value2) {
    return 2;
  }
  if (value1 == value2) {
    return 3;
  }
  if (value1.get_field1() != 1) {
    return 4;
  }
  if (value1.get_field2()) {
    return 5;
  }
  if (value2.get_field1() != -2) {
    return 6;
  }
  if (*value2.get_field2() != 3.5) {
    return 7;
  }
  if (!value2.set_field1(3).is_ok()) {
    return 8;
  }
  if (!value2.set_field2(7.5).is_ok()) {
    return 9;
  }
  if (value2.get_field1() != 3) {
    return 10;
  }
  if (*value2.get_field2() != 7.5) {
    return 11;
  }

  std::cout << "Done" << std::endl;

  return 0;
}