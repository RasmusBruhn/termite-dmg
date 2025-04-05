#include <iostream>
#include "basic.h"

int main() {
  auto value1 = test::DataType(1);
  auto value2 = test::DataType((float)1.5);

  if (value1 != value1) {
    return 1;
  }
  if (value1 == test::DataType(2)) {
    return 2;
  }
  if (value2 != value2) {
    return 3;
  }
  if (value2 == test::DataType((float)2.5)) {
    return 4;
  }
  if (value1 == value2) {
    return 5;
  }

  if (value1.value != std::variant<int, float>(1)) {
    return 6;
  }
  if (value2.value != std::variant<int, float>((float)1.5)) {
    return 7;
  }

  auto node1 = termite::Node(termite::Node::Value("1.5"));
  auto read_value1 = node1.to_value<test::DataType>();
  if (!read_value1.is_ok()) {
    return 20;
  }
  if (read_value1.get_ok() != value2) {
    return 21;
  }
  auto node2 = termite::Node(termite::Node::Value("1"));
  auto read_value2 = node2.to_value<test::DataType>();
  if (!read_value2.is_ok()) {
    return 22;
  }
  if (read_value1.get_ok() != value2) {
    return 23;
  }

  termite::Node converted_node1 = termite::Node::from_value(value1);
  auto converted_value1 = converted_node1.to_value<test::DataType>();
  if (!converted_value1.is_ok()) {
    return 24;
  }
  if (converted_value1.get_ok() != value1) {
    return 25;
  }

  termite::Node converted_node2 = termite::Node::from_value(value2);
  auto converted_value2 = converted_node2.to_value<test::DataType>();
  if (!converted_value2.is_ok()) {
    return 26;
  }
  if (converted_value2.get_ok() != value2) {
    return 27;
  }

  std::cout << "Done" << std::endl;

  return 0;
}