#include "full_example.h"

#include <iostream>
#include <fstream>

#include "termite-yaml.hpp"
#include "termite.hpp"

using namespace test::name::space;

int main() {
  // Create an instance of the data model
  VersionString version("1.0.1");
  DefaultValues defaults(State(State::TypeFilled()),
                         Size(SizeValue(10), SizeValue(20)));
  GeometryList geometries({});
  geometries.values.push_back(Geometry(Rectangle(
      Point(15, -30), std::nullopt, State(State::TypeEdge(SizeValue(5))))));
  geometries.values.push_back(
      Geometry(Circle(Point(0, 0), SizeValue(7), std::nullopt)));
  DataModel data_model(std::move(version), std::move(defaults),
                       std::move(geometries));

  // Save the data model to yaml
  termite::Node node_out = termite::Node::from_value(data_model);
  YAML::Node yaml_out = termite::to_YAML(node_out);
  std::ofstream out_file("data_model.yaml");
  out_file << yaml_out;
  out_file.close();

  // Load it again

  std::cout << "Done" << std::endl;

  return 0;
}
