#include <iostream>
#include "generated/namespace.h"

int main() {
  auto data_type = test1::test2::DataType();
  if (data_type != test1::test2::DataType()) {
    return 1;
  }

  std::map<std::string, termite::Node> map_correct;
  termite::Node node_correct(termite::Node::Map(std::move(map_correct)));
  auto value_read_correct = node_correct.to_value<test1::test2::DataType>();
  if (!value_read_correct.is_ok()) {
    return 2;
  }

  std::cout << "Done" << std::endl;

  return 0;
}