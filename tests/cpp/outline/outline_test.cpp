#include <iostream>
#include "outline.hpp"

int main() {
  auto type1 = test::DataType1::from_values().get_ok();
  if (type1 != test::DataType1::from_values().get_ok()) {
    return 1;
  }
  auto type2 = test::DataType2::from_values().get_ok();
  if (type2 != test::DataType2::from_values().get_ok()) {
    return 2;
  }

  std::cout << "Done" << std::endl;

  return 0;
}