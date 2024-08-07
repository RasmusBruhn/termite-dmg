#include <iostream>
#include "outline.hpp"

int main() {
  auto type1 = test::DataType1();
  if (type1 != test::DataType1()) {
    return 1;
  }
  auto type2 = test::DataType2();
  if (type2 != test::DataType2()) {
    return 2;
  }

  std::map<std::string, termite::Node> map_correct1;
  termite::Node node_correct1(termite::Node::Map(std::move(map_correct1)));
  auto value_read_correct1 = node_correct1.to_value<test::DataType1>();
  if (!value_read_correct1.is_ok()) {
    return 3;
  }

  std::map<std::string, termite::Node> map_correct2;
  termite::Node node_correct2(termite::Node::Map(std::move(map_correct2)));
  auto value_read_correct2 = node_correct2.to_value<test::DataType2>();
  if (!value_read_correct2.is_ok()) {
    return 4;
  }

  std::map<std::string, termite::Node> map_many1;
  map_many1.insert({"field1", termite::Node(termite::Node(termite::Node::Value("1")))});
  termite::Node node_many1(termite::Node::Map(std::move(map_many1)));
  auto value_read_many1 = node_many1.to_value<test::DataType1>(true);
  if (!value_read_many1.is_ok()) {
    return 5;
  }
  auto value_wrong_many1 = node_many1.to_value<test::DataType1>();
  if (value_wrong_many1.is_ok()) {
    return 6;
  }

  std::map<std::string, termite::Node> map_many2;
  map_many2.insert({"field1", termite::Node(termite::Node(termite::Node::Value("1")))});
  termite::Node node_many2(termite::Node::Map(std::move(map_many2)));
  auto value_read_many2 = node_many2.to_value<test::DataType2>(true);
  if (!value_read_many2.is_ok()) {
    return 7;
  }
  auto value_wrong_many2 = node_many2.to_value<test::DataType2>();
  if (value_wrong_many2.is_ok()) {
    return 8;
  }

  termite::Node node_wrong1(termite::Node::Value("1.0"));
  auto value_wrong_wrong1 = node_wrong1.to_value<test::DataType1>();
  if (value_wrong_wrong1.is_ok()) {
    return 9;
  }

  termite::Node node_wrong2(termite::Node::Value("1.0"));
  auto value_wrong_wrong2 = node_wrong2.to_value<test::DataType2>();
  if (value_wrong_wrong2.is_ok()) {
    return 10;
  }

  std::cout << "Done" << std::endl;

  return 0;
}