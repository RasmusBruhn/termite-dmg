#include <iostream>
#include "optional.h"

int main() {
  auto value1 = test::DataType(test::DataType::default_field1(), test::DataType::default_field2());
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

  std::map<std::string, termite::Node> map_correct1;
  termite::Node node_correct1(termite::Node::Map(std::move(map_correct1)));
  auto value1_read_correct = node_correct1.to_value<test::DataType>();
  if (!value1_read_correct.is_ok()) {
    return 12;
  }
  if (value1_read_correct.get_ok() != value1) {
    return 13;
  }

  std::map<std::string, termite::Node> map_correct2;
  map_correct2.insert({"field1", termite::Node(termite::Node(termite::Node::Value("-2")))});
  map_correct2.insert({"field2", termite::Node(termite::Node(termite::Node::Value("3.5")))});
  termite::Node node_correct2(termite::Node::Map(std::move(map_correct2)));
  auto value2_read_correct = node_correct2.to_value<test::DataType>();
  if (!value2_read_correct.is_ok()) {
    return 14;
  }
  if (value2_read_correct.get_ok() != value2) {
    return 15;
  }

  std::map<std::string, termite::Node> map_many;
  map_many.insert({"field3", termite::Node(termite::Node(termite::Node::Value("test")))});
  termite::Node node_many(termite::Node::Map(std::move(map_many)));
  auto value1_read_many = node_many.to_value<test::DataType>();
  if (!value1_read_many.is_ok()) {
    return 16;
  }
  if (value1_read_many.get_ok() == value1) {
    return 17;
  }

  std::map<std::string, termite::Node> map_type;
  map_type.insert({"field1", termite::Node(termite::Node(termite::Node::Value("1.0")))});
  map_type.insert({"field2", termite::Node(termite::Node(termite::Node::Value("5.0")))});
  termite::Node node_type(termite::Node::Map(std::move(map_type)));
  auto value_wrong_type = node_type.to_value<test::DataType>();
  if (value_wrong_type.is_ok()) {
    return 19;
  }

  termite::Node node_wrong(termite::Node::Value("1.0"));
  auto value_wrong_wrong = node_wrong.to_value<test::DataType>();
  if (value_wrong_wrong.is_ok()) {
    return 20;
  }

  std::cout << "Done" << std::endl;

  return 0;
}