#include <iostream>
#include "constraints.h"

int main() {
  auto value1 = test::DataType1::from_value(2).get_ok();
  auto value2 = test::DataType2::from_value(1e-10).get_ok();

  if (value1 != value1) {
    return 1;
  }
  if (value1 == test::DataType1::from_value(4).get_ok()) {
    return 2;
  }
  if (value2 != value2) {
    return 3;
  }
  if (value2 == test::DataType2::from_value(1e-11).get_ok()) {
    return 4;
  }

  if (test::DataType1::from_value(0).is_ok()) {
    return 5;
  }
  if (test::DataType1::from_value(1).is_ok()) {
    return 6;
  }
  if (test::DataType2::from_value(1e-8).is_ok()) {
    return 7;
  }

  if (value1.get() != 2) {
    return 8;
  }
  if (std::abs(value2.get() - 1e-10) > 1e-16) {
    return 9;
  }

  if (!value1.set(4).is_ok()) {
    return 10;
  }
  if (!value2.set(1e-11).is_ok()) {
    return 11;
  }

  if (value1.get() != 4) {
    return 12;
  }
  if (std::abs(value2.get() - 1e-11) > 1e-16) {
    return 13;
  }

  if (value1.set(0).is_ok()) {
    return 14;
  }
  if (value1.set(1).is_ok()) {
    return 15;
  }
  if (value2.set(1e-8).is_ok()) {
    return 16;
  }

  if (value1.get() != 4) {
    return 17;
  }
  if (std::abs(value2.get() - 1e-11) > 1e-16) {
    return 18;
  }

  auto node1 = termite::Node(termite::Node::Value("4"));
  auto read_value1 = node1.to_value<test::DataType1>();
  if (!read_value1.is_ok()) {
    return 19;
  }
  if (read_value1.get_ok() != value1) {
    return 20;
  }
  auto node2 = termite::Node(termite::Node::Value("1e-11"));
  auto read_value2 = node2.to_value<test::DataType2>();
  if (!read_value2.is_ok()) {
    return 21;
  }
  if (std::abs(read_value2.get_ok().get() - value2.get()) > 1e-16) {
    return 22;
  }

  auto wrong_node11 = termite::Node(termite::Node::Value("0"));
  auto wrong_value11 = wrong_node11.to_value<test::DataType1>();
  if (wrong_value11.is_ok()) {
    return 23;
  }
  auto wrong_node12 = termite::Node(termite::Node::Value("1"));
  auto wrong_value12 = wrong_node12.to_value<test::DataType1>();
  if (wrong_value12.is_ok()) {
    return 24;
  }
  auto wrong_node13 = termite::Node(termite::Node::Value("1.5"));
  auto wrong_value13 = wrong_node13.to_value<test::DataType1>();
  if (wrong_value13.is_ok()) {
    return 25;
  }
  auto wrong_node2 = termite::Node(termite::Node::Value("1e-8"));
  auto wrong_value2 = wrong_node2.to_value<test::DataType1>();
  if (wrong_value2.is_ok()) {
    return 26;
  }

  termite::Node converted_node = termite::Node::from_value(value1);
  auto converted_value = converted_node.to_value<test::DataType1>();
  if (!converted_value.is_ok()) {
    return 27;
  }
  if (converted_value.get_ok() != value1) {
    return 28;
  }

  std::cout << "Done" << std::endl;

  return 0;
}