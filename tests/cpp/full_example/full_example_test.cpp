#include "generated/full_example.h"

#include <fstream>
#include <iostream>

#include "termite-yaml.h"
#include "termite.hpp"

using namespace test::name::space;

/**
 * @brief Test if an example model can be constructed, saved to YAML and
 * reloaded again
 *
 * @return An error string on error
 */
std::optional<std::string> test_reload() {
  // Create an instance of the data model
  VersionString version = "1.0.1";
  DefaultValues defaults(
      {{"size", Size({{"w", 10}, {"h", 20}})}, {"state", State::TypeFilled()}});
  GeometryList geometries(
      {Rectangle({{"center", Point({{"x", 15}, {"y", -30}})},
                  {"state", State::TypeEdge(5)}}),
       Circle({{"center", Point({{"x", 0}, {"y", 0}})}, {"radius", 7}})});
  DataModel data_model({{"version", version},
                        {"defaults", defaults},
                        {"geometries", geometries}});

  // Save the data model to yaml
  auto node_out = termite::Node::from_value(data_model);
  auto yaml_out = termite::to_YAML(node_out);
  std::ofstream out_file("data_model.yaml");
  out_file << yaml_out;
  out_file.close();

  // Load it again
  std::ifstream in_file("data_model.yaml");
  auto yaml_in = YAML::Load(in_file);
  in_file.close();
  auto node_in = termite::from_YAML(yaml_in);
  if (!node_in.is_ok()) {
    std::stringstream ss;
    ss << "Error loading yaml: " << node_in.get_err();
    return ss.str();
  }
  auto data_model_in = node_in.get_ok().to_value<DataModel>();
  if (!data_model_in.is_ok()) {
    std::stringstream ss;
    ss << "Error converting to DataModel: " << data_model_in.get_err();
    return ss.str();
  }
  auto data_model_loaded = data_model_in.get_ok();
  if (data_model != data_model_loaded) {
    std::stringstream ss;
    ss << "Data model loaded does not match the original, "
       << "Original: " << data_model << ", Loaded: " << data_model_loaded;
    return ss.str();
  }

  return std::nullopt;
}

int main() {
  auto names = {
      "test_reload",
  };
  auto functions = {
      test_reload,
  };

  std::cout << "Running " << names.size() << " tests" << std::endl;

  int progress = 1;
  auto name_it = names.begin();
  for (auto function_it = functions.begin(); function_it < functions.end();
       ++function_it, ++name_it, ++progress) {
    if (auto error = (*function_it)()) {
      std::cout << "Error occured at \"" << *name_it << "\": " << *error
                << std::endl;
      return progress;
    }
  }

  std::cout << "No errors" << std::endl;

  return 0;
}
