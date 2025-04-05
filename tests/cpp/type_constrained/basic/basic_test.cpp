#include <iostream>
#include "basic.h"

int main() {
  auto value1 = test::DataType1::from_value(1).get_ok();
  auto value2 = test::DataType2::from_value(1.5).get_ok();

  if (value1 != value1) {
    return 1;
  }
  if (value1 == test::DataType1::from_value(2).get_ok()) {
    return 2;
  }
  if (value2 != value2) {
    return 3;
  }
  if (value2 == test::DataType2::from_value(2.5).get_ok()) {
    return 4;
  }

  if (value1.get() != 1) {
    return 5;
  }
  if (value2.get() != 1.5) {
    return 6;
  }

  if (!value1.set(2).is_ok()) {
    return 7;
  }
  if (!value2.set(2.5).is_ok()) {
    return 8;
  }

  if (value1.get() != 2) {
    return 9;
  }
  if (value2.get() != 2.5) {
    return 10;
  }

  auto node1 = termite::Node(termite::Node::Value("2"));
  auto read_value1 = node1.to_value<test::DataType1>();
  if (!read_value1.is_ok()) {
    return 11;
  }
  if (read_value1.get_ok() != value1) {
    return 12;
  }
  auto node2 = termite::Node(termite::Node::Value("2.5"));
  auto read_value2 = node2.to_value<test::DataType2>();
  if (!read_value2.is_ok()) {
    return 13;
  }
  if (read_value2.get_ok() != value2) {
    return 14;
  }

  termite::Node converted_node = termite::Node::from_value(value1);
  auto converted_value = converted_node.to_value<test::DataType1>();
  if (!converted_value.is_ok()) {
    return 15;
  }
  if (converted_value.get_ok() != value1) {
    return 16;
  }

  std::cout << "Done" << std::endl;

  return 0;
}