#include <iostream>
#include "constraints.hpp"

int main() {
  if (!test::DataType::from_values(2, -1e-10).is_ok()) {
    return 1;
  }
  if (test::DataType::from_values(2, 1.1e-9).is_ok()) {
    return 2;
  }
  if (test::DataType::from_values(3, 1e-10).is_ok()) {
    return 3;
  }
  if (test::DataType::from_values(0, 1e-10).is_ok()) {
    return 4;
  }
  if (test::DataType::from_values(0, 1.1e-9).is_ok()) {
    return 5;
  }

  auto value = test::DataType::from_values(2, -1e-10).get_ok();

  if (value.set_field1(0).is_ok()) {
    return 6;
  }
  if (value.get_field1() != 2) {
    return 7;
  }
  if (value.set_field1(3).is_ok()) {
    return 8;
  }
  if (value.get_field1() != 2) {
    return 9;
  }
  if (!value.set_field1(4).is_ok()) {
    return 10;
  }
  if (value.get_field1() != 4) {
    return 11;
  }
  if (value.set_field2(1e-8).is_ok()) {
    return 12;
  }
  if (std::abs(value.get_field2() + 1e-10) > 1e-16) {
    return 13;
  }
  if (!value.set_field2(1e-11).is_ok()) {
    return 14;
  }
  if (std::abs(value.get_field2() - 1e-11) > 1e-16) {
    return 15;
  }

  value = test::DataType::from_values(2, 0.0).get_ok();

  std::map<std::string, std::unique_ptr<termite::Node>> map_correct;
  map_correct.insert({"field1", std::unique_ptr<termite::Node>(new termite::Node(termite::NodeValue("2")))});
  map_correct.insert({"field2", std::unique_ptr<termite::Node>(new termite::Node(termite::NodeValue("0.0")))});
  termite::Node node_correct(termite::NodeMap(std::move(map_correct)));
  auto value_read_correct = node_correct.to_value<test::DataType>();
  if (!value_read_correct.is_ok()) {
    return 16;
  }
  if (value_read_correct.get_ok() != value) {
    return 17;
  }

  std::map<std::string, std::unique_ptr<termite::Node>> map_correct1;
  map_correct1.insert({"field1", std::unique_ptr<termite::Node>(new termite::Node(termite::NodeValue("3")))});
  map_correct1.insert({"field2", std::unique_ptr<termite::Node>(new termite::Node(termite::NodeValue("0.0")))});
  termite::Node node_correct1(termite::NodeMap(std::move(map_correct1)));
  auto value_read_correct1 = node_correct1.to_value<test::DataType>();
  if (value_read_correct1.is_ok()) {
    return 18;
  }

  std::map<std::string, std::unique_ptr<termite::Node>> map_correct2;
  map_correct2.insert({"field1", std::unique_ptr<termite::Node>(new termite::Node(termite::NodeValue("2")))});
  map_correct2.insert({"field2", std::unique_ptr<termite::Node>(new termite::Node(termite::NodeValue("1.0")))});
  termite::Node node_correct2(termite::NodeMap(std::move(map_correct2)));
  auto value_read_correct2 = node_correct2.to_value<test::DataType>();
  if (value_read_correct2.is_ok()) {
    return 19;
  }

  std::map<std::string, std::unique_ptr<termite::Node>> map_many;
  map_many.insert({"field1", std::unique_ptr<termite::Node>(new termite::Node(termite::NodeValue("2")))});
  map_many.insert({"field2", std::unique_ptr<termite::Node>(new termite::Node(termite::NodeValue("0.0")))});
  map_many.insert({"field3", std::unique_ptr<termite::Node>(new termite::Node(termite::NodeValue("test")))});
  termite::Node node_many(termite::NodeMap(std::move(map_many)));
  auto value1_read_many = node_many.to_value<test::DataType>(true);
  if (!value1_read_many.is_ok()) {
    return 20;
  }
  if (value1_read_many.get_ok() != value) {
    return 21;
  }
  auto value_wrong_many = node_many.to_value<test::DataType>();
  if (value_wrong_many.is_ok()) {
    return 22;
  }

  std::map<std::string, std::unique_ptr<termite::Node>> map_type;
  map_type.insert({"field1", std::unique_ptr<termite::Node>(new termite::Node(termite::NodeValue("1.0")))});
  map_type.insert({"field2", std::unique_ptr<termite::Node>(new termite::Node(termite::NodeValue("5.0")))});
  termite::Node node_type(termite::NodeMap(std::move(map_type)));
  auto value_wrong_type = node_type.to_value<test::DataType>();
  if (value_wrong_type.is_ok()) {
    return 23;
  }

  termite::Node node_wrong(termite::NodeValue("1.0"));
  auto value_wrong_wrong = node_wrong.to_value<test::DataType>();
  if (value_wrong_wrong.is_ok()) {
    return 24;
  }

  std::cout << "Done" << std::endl;

  return 0;
}