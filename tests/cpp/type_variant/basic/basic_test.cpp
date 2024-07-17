#include <iostream>
#include "basic.hpp"

int main() {
  auto value1 = test::DataType::from_values(1);
  auto value2 = test::DataType::from_values((float)1.5);

  std::cout << "Done" << std::endl;

  return 0;
}