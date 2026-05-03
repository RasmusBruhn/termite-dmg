#include <iostream>
#include "generated/footer.h"

int main() {
  auto data_type = DataType();
  if (data_type != DataType()) {
    return 1;
  }

  std::map<std::string, termite::Node> map_correct;
  termite::Node node_correct(termite::Node::Map(std::move(map_correct)));
  auto value_read_correct = node_correct.to_value<DataType>();
  if (!value_read_correct.is_ok()) {
    return 2;
  }

  std::cout << "Done" << std::endl;

  return 0;
}