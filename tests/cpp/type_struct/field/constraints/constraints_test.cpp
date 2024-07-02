#include <iostream>
#include "constraints.hpp"

int main() {
  if (!DataType::from_values(2, -1e-10).is_ok()) {
    return 1;
  }
  if (DataType::from_values(2, 1.1e-9).is_ok()) {
    return 2;
  }
  if (DataType::from_values(3, 1e-10).is_ok()) {
    return 3;
  }
  if (DataType::from_values(0, 1e-10).is_ok()) {
    return 4;
  }
  if (DataType::from_values(0, 1.1e-9).is_ok()) {
    return 5;
  }

  auto value = DataType::from_values(2, -1e-10).get_ok();

  if (value.set_field1(0).is_ok()) {
    return 6;
  }
  if (value.get_field1() != 2) {
    return 7;
  }
  if (value.set_field1(3).is_ok()) {
    return 8;
  }
  if (value.get_field1() != 2) {
    return 9;
  }
  if (!value.set_field1(4).is_ok()) {
    return 10;
  }
  if (value.get_field1() != 4) {
    return 11;
  }
  if (value.set_field2(1e-8).is_ok()) {
    return 12;
  }
  if (std::abs(value.get_field2() + 1e-10) > 1e-16) {
    return 13;
  }
  if (!value.set_field2(1e-11).is_ok()) {
    return 14;
  }
  if (std::abs(value.get_field2() - 1e-11) > 1e-16) {
    return 15;
  }

  std::cout << "Done" << std::endl;

  return 0;
}