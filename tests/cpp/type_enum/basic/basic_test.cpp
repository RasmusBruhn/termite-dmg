#include <iostream>
#include "basic.h"

int main() {
  auto value1 = test::DataType(test::DataType::TypeInt1{1});
  auto value2 = test::DataType(test::DataType::TypeInt2{1});
  auto value3 = test::DataType(test::DataType::TypeFloat{3.5});
  auto value4 = test::DataType(test::DataType::TypeEmpty{});

  if (value1 != value1) {
    return 1;
  }
  if (value2 != value2) {
    return 2;
  }
  if (value3 != value3) {
    return 3;
  }
  if (value4 != value4) {
    return 4;
  }
  if (value1 == value2) {
    return 5;
  }
  if (value1 == value3) {
    return 6;
  }
  if (value1 == value4) {
    return 7;
  }
  if (value2 == value3) {
    return 8;
  }
  if (value2 == value4) {
    return 9;
  }
  if (value3 == value4) {
    return 10;
  }

  if (value1.enum_type() != test::DataType::Enum::kInt1) {
    return 11;
  }
  if (value2.enum_type() != test::DataType::Enum::kInt2) {
    return 12;
  }
  if (value3.enum_type() != test::DataType::Enum::kFloat) {
    return 13;
  }
  if (value4.enum_type() != test::DataType::Enum::kEmpty) {
    return 14;
  }

  std::map<std::string, termite::Node> map1;
  map1.insert({"Int1", termite::Node(termite::Node(termite::Node::Value("1")))});
  auto node1 = termite::Node(termite::Node::Map(std::move(map1)));
  auto read_value1 = node1.to_value<test::DataType>();
  if (!read_value1.is_ok()) {
    return 15;
  }
  if (read_value1.get_ok() != value1) {
    return 16;
  }
  std::map<std::string, termite::Node> map2;
  map2.insert({"Int2", termite::Node(termite::Node(termite::Node::Value("1")))});
  auto node2 = termite::Node(termite::Node::Map(std::move(map2)));
  auto read_value2 = node2.to_value<test::DataType>();
  if (!read_value2.is_ok()) {
    return 17;
  }
  if (read_value2.get_ok() != value2) {
    return 18;
  }
  std::map<std::string, termite::Node> map3;
  map3.insert({"Float", termite::Node(termite::Node(termite::Node::Value("3.5")))});
  auto node3 = termite::Node(termite::Node::Map(std::move(map3)));
  auto read_value3 = node3.to_value<test::DataType>();
  if (!read_value3.is_ok()) {
    return 19;
  }
  if (read_value3.get_ok() != value3) {
    return 20;
  }
  auto node4 = termite::Node(termite::Node::Value("Empty"));
  auto read_value4 = node4.to_value<test::DataType>();
  if (!read_value4.is_ok()) {
    return 21;
  }
  if (read_value4.get_ok() != value4) {
    return 22;
  }

  auto wrong_node1 = termite::Node(termite::Node::Value("Int1"));
  auto wrong_read_value1 = wrong_node1.to_value<test::DataType>();
  if (wrong_read_value1.is_ok()) {
    return 23;
  }
  auto wrong_node2 = termite::Node(termite::Node::Value("Int2"));
  auto wrong_read_value2 = wrong_node2.to_value<test::DataType>();
  if (wrong_read_value2.is_ok()) {
    return 24;
  }
  auto wrong_node3 = termite::Node(termite::Node::Value("Float"));
  auto wrong_read_value3 = wrong_node3.to_value<test::DataType>();
  if (wrong_read_value3.is_ok()) {
    return 25;
  }
  std::map<std::string, termite::Node> map4;
  map4.insert({"Empty", termite::Node(termite::Node(termite::Node::Value("3.5")))});
  auto wrong_node4 = termite::Node(termite::Node::Map(std::move(map4)));
  auto wrong_read_value4 = wrong_node4.to_value<test::DataType>();
  if (wrong_read_value4.is_ok()) {
    return 26;
  }

  auto wrong_node_empty = termite::Node(termite::Node::Value("Test"));
  auto wrong_read_value_empty = wrong_node_empty.to_value<test::DataType>();
  if (wrong_read_value_empty.is_ok()) {
    return 27;
  }
  std::map<std::string, termite::Node> map_map;
  map_map.insert({"Test", termite::Node(termite::Node(termite::Node::Value("3.5")))});
  auto wrong_node_map = termite::Node(termite::Node::Map(std::move(map4)));
  auto wrong_read_value_map = wrong_node_map.to_value<test::DataType>();
  if (wrong_read_value_map.is_ok()) {
    return 28;
  }

  termite::Node converted_node1 = termite::Node::from_value(value1);
  auto converted_value1 = converted_node1.to_value<test::DataType>();
  if (!converted_value1.is_ok()) {
    return 29;
  }
  if (converted_value1.get_ok() != value1) {
    return 30;
  }

  termite::Node converted_node2 = termite::Node::from_value(value2);
  auto converted_value2 = converted_node2.to_value<test::DataType>();
  if (!converted_value2.is_ok()) {
    return 31;
  }
  if (converted_value2.get_ok() != value2) {
    return 32;
  }

  termite::Node converted_node3 = termite::Node::from_value(value3);
  auto converted_value3 = converted_node3.to_value<test::DataType>();
  if (!converted_value3.is_ok()) {
    return 33;
  }
  if (converted_value3.get_ok() != value3) {
    return 34;
  }

  termite::Node converted_node4 = termite::Node::from_value(value4);
  auto converted_value4 = converted_node4.to_value<test::DataType>();
  if (!converted_value4.is_ok()) {
    return 35;
  }
  if (converted_value4.get_ok() != value4) {
    return 36;
  }

  std::cout << "Done" << std::endl;

  return 0;
}