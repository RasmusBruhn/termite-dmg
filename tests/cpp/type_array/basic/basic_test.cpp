#include <iostream>
#include "basic.hpp"

int main() {
  auto value1 = test::DataType1::from_values({1, 2}).get_ok();
  auto value2 = test::DataType2::from_values({1.5, -3.5}).get_ok();
  if (value1 != value1) {
    return 1;
  }
  if (value2.get_values() != std::vector<float>{1.5, -3.5}) {
    return 2;
  }
  if (value1 == test::DataType1::from_values({1, 2, 3}).get_ok()) {
    return 3;
  }
  if (value1 == test::DataType1::from_values({1, 3}).get_ok()) {
    return 4;
  }
  if (value2 == test::DataType2::from_values({1.5}).get_ok()) {
    return 5;
  }
  if (value2 == test::DataType2::from_values({1.5, 3.5}).get_ok()) {
    return 6;
  }
  if (!value1.set_values({1, 2, 3}).is_ok()) {
    return 7;
  }
  if (value1.get_values() != std::vector<int>{1, 2, 3}) {
    return 8;
  }
  if (!value2.push_value(3.5).is_ok()) {
    return 9;
  }
  if (value2.get_values() != std::vector<float>{1.5, -3.5, 3.5}) {
    return 10;
  }

  value1 = test::DataType1::from_values({1, 2}).get_ok();
  value2 = test::DataType2::from_values({1.5, -3.5}).get_ok();

  std::vector<termite::Node> vector_correct1;
  vector_correct1.emplace_back(termite::NodeValue("1"));
  vector_correct1.emplace_back(termite::NodeValue("2"));
  termite::Node node_correct1(termite::NodeList(std::move(vector_correct1)));
  auto value_read_correct1 = node_correct1.to_value<test::DataType1>();
  if (!value_read_correct1.is_ok()) {
    return 11;
  }
  if (value_read_correct1.get_ok() != value1) {
    return 12;
  }

  std::vector<termite::Node> vector_correct2;
  vector_correct2.emplace_back(termite::NodeValue("1.5"));
  vector_correct2.emplace_back(termite::NodeValue("-3.5"));
  termite::Node node_correct2(termite::NodeList(std::move(vector_correct2)));
  auto value_read_correct2 = node_correct2.to_value<test::DataType2>();
  if (!value_read_correct2.is_ok()) {
    return 13;
  }
  if (value_read_correct2.get_ok() != value2) {
    return 14;
  }

  std::vector<termite::Node> vector_type1;
  vector_type1.emplace_back(termite::NodeValue("1"));
  vector_type1.emplace_back(termite::NodeValue("2.5"));
  termite::Node node_type1(termite::NodeList(std::move(vector_type1)));
  auto value_read_type1 = node_type1.to_value<test::DataType1>();
  if (value_read_type1.is_ok()) {
    return 15;
  }

  std::vector<termite::Node> vector_type2;
  vector_type2.emplace_back(termite::NodeValue("1k"));
  vector_type2.emplace_back(termite::NodeValue("-3.5"));
  termite::Node node_type2(termite::NodeList(std::move(vector_type2)));
  auto value_read_type2 = node_type2.to_value<test::DataType2>();
  if (value_read_type2.is_ok()) {
    return 16;
  }

  termite::Node node_wrong(termite::NodeValue("1.0"));
  auto value_wrong_wrong = node_wrong.to_value<test::DataType1>();
  if (value_wrong_wrong.is_ok()) {
    return 17;
  }

  std::cout << "Done" << std::endl;

  return 0;
}
