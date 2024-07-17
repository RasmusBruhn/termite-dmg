#include <iostream>
#include "basic.hpp"

int main() {
  auto value1 = test::DataType::from_values(1);
  auto value2 = test::DataType::from_values((float)1.5);

  if (value1 != value1) {
    return 1;
  }
  if (value1 == test::DataType::from_values(2)) {
    return 2;
  }
  if (value2 != value2) {
    return 3;
  }
  if (value2 == test::DataType::from_values((float)2.5)) {
    return 4;
  }
  if (value1 == value2) {
    return 5;
  }

  std::cout << "Done" << std::endl;

  return 0;
}