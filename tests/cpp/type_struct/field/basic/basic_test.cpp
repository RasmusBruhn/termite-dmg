#include <iostream>
#include "basic.h"

int main() {
  auto value1 = test::DataType(1, 5.0);
  auto value2 = test::DataType(-2, 3.5);
  if (value1 != value1) {
    return 1;
  }
  if (value2 != value2) {
    return 2;
  }
  if (value1 == value2) {
    return 3;
  }

  std::map<std::string, termite::Node> map_correct;
  map_correct.insert({"field1", termite::Node(termite::Node(termite::Node::Value("1")))});
  map_correct.insert({"field2", termite::Node(termite::Node(termite::Node::Value("5.0")))});
  termite::Node node_correct(termite::Node::Map(std::move(map_correct)));
  auto value1_read_correct = node_correct.to_value<test::DataType>();
  if (!value1_read_correct.is_ok()) {
    return 10;
  }
  if (value1_read_correct.get_ok() != value1) {
    return 11;
  }

  std::map<std::string, termite::Node> map_many;
  map_many.insert({"field1", termite::Node(termite::Node(termite::Node::Value("1")))});
  map_many.insert({"field2", termite::Node(termite::Node(termite::Node::Value("5.0")))});
  map_many.insert({"field3", termite::Node(termite::Node(termite::Node::Value("test")))});
  termite::Node node_many(termite::Node::Map(std::move(map_many)));
  auto value1_read_many = node_many.to_value<test::DataType>();
  if (!value1_read_many.is_ok()) {
    return 12;
  }
  if (value1_read_many.get_ok() == value1) {
    return 13;
  }

  std::map<std::string, termite::Node> map_few;
  map_few.insert({"field1", termite::Node(termite::Node(termite::Node::Value("1")))});
  termite::Node node_few(termite::Node::Map(std::move(map_many)));
  auto value_wrong_few = node_few.to_value<test::DataType>();
  if (value_wrong_few.is_ok()) {
    return 15;
  }

  std::map<std::string, termite::Node> map_type;
  map_type.insert({"field1", termite::Node(termite::Node(termite::Node::Value("1.0")))});
  map_type.insert({"field2", termite::Node(termite::Node(termite::Node::Value("5.0")))});
  termite::Node node_type(termite::Node::Map(std::move(map_type)));
  auto value_wrong_type = node_type.to_value<test::DataType>();
  if (value_wrong_type.is_ok()) {
    return 16;
  }

  termite::Node node_wrong(termite::Node::Value("1.0"));
  auto value_wrong_wrong = node_wrong.to_value<test::DataType>();
  if (value_wrong_wrong.is_ok()) {
    return 17;
  }

  termite::Node converted_node1 = termite::Node::from_value(value1);
  auto converted_value1 = converted_node1.to_value<test::DataType>();
  if (!converted_value1.is_ok()) {
    return 18;
  }
  if (converted_value1.get_ok() != value1) {
    return 19;
  }

  termite::Node converted_node2 = termite::Node::from_value(value2);
  auto converted_value2 = converted_node2.to_value<test::DataType>();
  if (!converted_value2.is_ok()) {
    return 20;
  }
  if (converted_value2.get_ok() != value2) {
    return 21;
  }

  std::cout << "Done" << std::endl;

  return 0;
}