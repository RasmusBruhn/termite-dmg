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

  if (value1.get_value() != std::variant<int, float>(1)) {
    return 6;
  }
  if (value2.get_value() != std::variant<int, float>((float)1.5)) {
    return 7;
  }
  if (value1.variant() != test::DataType::Variant::kInt) {
    return 8;
  }
  if (value2.variant() != test::DataType::Variant::kFloat) {
    return 9;
  }

  if (!value1.get_int().is_ok()) {
    return 10;
  }
  if (value1.get_int().get_ok().get() != 1) {
    return 11;
  }
  if (value1.get_float().is_ok()) {
    return 12;
  }
  if (!value2.get_float().is_ok()) {
    return 13;
  }
  if (value2.get_float().get_ok().get() != 1.5) {
    return 14;
  }
  if (value2.get_int().is_ok()) {
    return 15;
  }

  value1.set_float(2.5);
  if (!value1.get_float().is_ok()) {
    return 16;
  }
  if (value1.get_float().get_ok().get() != 2.5) {
    return 17;
  }
  value2.set_int(3);
  if (!value2.get_int().is_ok()) {
    return 18;
  }
  if (value2.get_int().get_ok().get() != 3) {
    return 19;
  }

  auto node1 = termite::Node(termite::Node::Value("2.5"));
  auto read_value1 = node1.to_value<test::DataType>();
  if (!read_value1.is_ok()) {
    return 20;
  }
  if (read_value1.get_ok() != value1) {
    return 21;
  }
  auto node2 = termite::Node(termite::Node::Value("3"));
  auto read_value2 = node2.to_value<test::DataType>();
  if (!read_value2.is_ok()) {
    return 22;
  }
  if (read_value2.get_ok() != value2) {
    return 23;
  }

  std::cout << "Done" << std::endl;

  return 0;
}