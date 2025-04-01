// Generated with the Termite Data Model Generator
#include "full_example.h"

// This is my .cpp file

namespace test::name::space {

namespace {

// Code to make printing easier
template <typename T, typename = void>
struct has_insertion_operator : std::false_type {};
template <typename T>
struct has_insertion_operator<T, std::void_t<decltype(std::declval<std::ostream &>() << std::declval<T>())>> : std::true_type {};

template <typename T>
typename std::enable_if<has_insertion_operator<T>::value, std::ostream &>::type
operator<<(std::ostream &os, const std::optional<T> &value) {
  if (value) {
    return os << *value;
  } else {
    return os << "nullopt";
  }
}

template <typename T>
typename std::enable_if<has_insertion_operator<T>::value, std::ostream &>::type
operator<<(std::ostream &os, const std::vector<T> &value) {
  os << "[ ";
  for (auto value_it = value.cbegin(); value_it != value.cend(); ++value_it) {
    if (value_it != value.cbegin()) {
      os << ", ";
    }
    os << *value_it;
  }
  return os << " ]";
}

} // namespace

[[nodiscard]] termite::Result<VersionString> VersionString::from_value(String value) {
  termite::Result<termite::Empty> validate_result = validate(value);
  if (!validate_result.is_ok()) {
    termite::Error error = validate_result.get_err();
    return termite::Result<VersionString>::err(std::move(error));
  }

  return termite::Result<VersionString>::ok(VersionString(std::move(value), nullptr));
}

[[nodiscard]] termite::Result<termite::Empty> VersionString::set(String value) {
  termite::Result<termite::Empty> validate_result = validate(value);
  if (!validate_result.is_ok()) {
    return validate_result;
  }

  value_ = std::move(value);
  return termite::Result<termite::Empty>::ok(termite::Empty());
}

[[nodiscard]] bool VersionString::operator==(const VersionString &x) const {
  return value_ == x.value_;
}
std::ostream &operator<<(std::ostream &os, const VersionString &x) {
  return os << x.value_;
}

[[nodiscard]] termite::Result<termite::Empty> VersionString::validate(const String &x) {
  if (!(custom::is_valid_version_string(x))) {
    return termite::Result<termite::Empty>::err(termite::Error("Did not pass constraint: custom::is_valid_version_string(x)"));
  }

  return termite::Result<termite::Empty>::ok(termite::Empty());
}

[[nodiscard]] termite::Result<SizeValue> SizeValue::from_value(int value) {
  termite::Result<termite::Empty> validate_result = validate(value);
  if (!validate_result.is_ok()) {
    termite::Error error = validate_result.get_err();
    return termite::Result<SizeValue>::err(std::move(error));
  }

  return termite::Result<SizeValue>::ok(SizeValue(std::move(value), nullptr));
}

[[nodiscard]] termite::Result<termite::Empty> SizeValue::set(int value) {
  termite::Result<termite::Empty> validate_result = validate(value);
  if (!validate_result.is_ok()) {
    return validate_result;
  }

  value_ = std::move(value);
  return termite::Result<termite::Empty>::ok(termite::Empty());
}

[[nodiscard]] bool SizeValue::operator==(const SizeValue &x) const {
  return value_ == x.value_;
}
std::ostream &operator<<(std::ostream &os, const SizeValue &x) {
  return os << x.value_;
}

[[nodiscard]] termite::Result<termite::Empty> SizeValue::validate(const int &x) {
  if (!(x > 0)) {
    return termite::Result<termite::Empty>::err(termite::Error("Did not pass constraint: x > 0"));
  }

  return termite::Result<termite::Empty>::ok(termite::Empty());
}

[[nodiscard]] bool Size::operator==(const Size &x) const {
  return this->w == x.w && this->h == x.h && extra_fields == x.extra_fields;
}

std::ostream &operator<<(std::ostream &os, const Size &x) {
  return os << "{ " << "w: " << x.w << ", " << "h: " << x.h << ", " << "extra_fields: " << x.extra_fields << " }";
}

[[nodiscard]] bool Point::operator==(const Point &x) const {
  return this->x == x.x && this->y == x.y && extra_fields == x.extra_fields;
}

std::ostream &operator<<(std::ostream &os, const Point &x) {
  return os << "{ " << "x: " << x.x << ", " << "y: " << x.y << ", " << "extra_fields: " << x.extra_fields << " }";
}

[[nodiscard]] bool State::TypeFilled::operator==(const TypeFilled &) const {
  return true;
}

std::ostream &operator<<(std::ostream &os, const State::TypeFilled &) {
  return os << "{  }";
}

[[nodiscard]] bool State::TypeEdge::operator==(const TypeEdge &x) const {
  return value == x.value;
}

std::ostream &operator<<(std::ostream &os, const State::TypeEdge &x) {
  return os << "{ value: " << x.value << " }";
}

[[nodiscard]] bool State::operator==(const State &x) const {
  return value == x.value;
}

std::ostream &operator<<(std::ostream &os, const State &x) {
  os << "{ value: ";
  switch (static_cast<State::Enum>(x.value.index())) {
  case State::Enum::kFilled:
    os << "Empty";
    break;
  case State::Enum::kEdge:
    os << "Edge(" << std::get<State::TypeEdge>(x.value).value << ")";
    break;
  default:
    os << "Unknown (" << x.value.index() << ")";
    break;
  }
  return os << " }";
}

[[nodiscard]] bool DefaultValues::operator==(const DefaultValues &x) const {
  return this->state == x.state && this->size == x.size && extra_fields == x.extra_fields;
}

[[nodiscard]] State DefaultValues::default_state() {
  return State(State::TypeEdge(SizeValue(1)));
}

std::ostream &operator<<(std::ostream &os, const DefaultValues &x) {
  return os << "{ " << "state: " << x.state << ", " << "size: " << x.size << ", " << "extra_fields: " << x.extra_fields << " }";
}

[[nodiscard]] bool Rectangle::operator==(const Rectangle &x) const {
  return this->center == x.center && this->size == x.size && this->state == x.state && extra_fields == x.extra_fields;
}

[[nodiscard]] std::optional<Size> Rectangle::default_size() {
  return std::nullopt;
}

[[nodiscard]] std::optional<State> Rectangle::default_state() {
  return std::nullopt;
}

std::ostream &operator<<(std::ostream &os, const Rectangle &x) {
  return os << "{ " << "center: " << x.center << ", " << "size: " << x.size << ", " << "state: " << x.state << ", " << "extra_fields: " << x.extra_fields << " }";
}

[[nodiscard]] bool Circle::operator==(const Circle &x) const {
  return this->center == x.center && this->radius == x.radius && this->state == x.state && extra_fields == x.extra_fields;
}

[[nodiscard]] std::optional<State> Circle::default_state() {
  return std::nullopt;
}

std::ostream &operator<<(std::ostream &os, const Circle &x) {
  return os << "{ " << "center: " << x.center << ", " << "radius: " << x.radius << ", " << "state: " << x.state << ", " << "extra_fields: " << x.extra_fields << " }";
}

[[nodiscard]] bool Geometry::operator==(const Geometry &x) const {
  return value == x.value;
}

std::ostream &operator<<(std::ostream &os, const Geometry &x) {
  os << "{ value: ";
  switch (x.value.index()) {
  case 0:
    os << "Rectangle " << std::get<Rectangle>(x.value);
    break;
  case 1:
    os << "Circle " << std::get<Circle>(x.value);
    break;
  default:
    os << "Unknown(" << x.value.index() << ")";
    break;
  }
  return os << " }";
}

bool GeometryList::operator==(const GeometryList &x) const {
  if (values.size() != x.values.size()) {
    return false;
  }

  for (auto lhs = values.cbegin(), rhs = x.values.cbegin(); lhs < values.cend(); ++lhs, ++rhs) {
    if (*lhs != *rhs) {
      return false;
    }
  }

  return true;
}

std::ostream &operator<<(std::ostream &os, const GeometryList &x) {
  os << "{ values: [ ";
  for (auto value = x.values.cbegin(); value < x.values.cend(); ++value) {
    if (value != x.values.cbegin()) {
      os << ", ";
    }
    os << *value;
  }
  return os << " ] }";
}

[[nodiscard]] bool DataModel::operator==(const DataModel &x) const {
  return this->version == x.version && this->defaults == x.defaults && this->geometries == x.geometries && extra_fields == x.extra_fields;
}

[[nodiscard]] VersionString DataModel::default_version() {
  return VersionString("1.0.0");
}

std::ostream &operator<<(std::ostream &os, const DataModel &x) {
  return os << "{ " << "version: " << x.version << ", " << "defaults: " << x.defaults << ", " << "geometries: " << x.geometries << ", " << "extra_fields: " << x.extra_fields << " }";
}

} // namespace test::name::space

namespace termite {

template<>
[[nodiscard]] Result<test::name::space::VersionString> Node::to_value<test::name::space::VersionString>() const {
  Result<String> value = to_value<String>();
  if (!value.is_ok()) {
    return Result<test::name::space::VersionString>::err(Error(value.get_err()));
  }

  return test::name::space::VersionString::from_value(value.get_ok());
}

template<>
[[nodiscard]] Result<test::name::space::SizeValue> Node::to_value<test::name::space::SizeValue>() const {
  Result<int> value = to_value<int>();
  if (!value.is_ok()) {
    return Result<test::name::space::SizeValue>::err(Error(value.get_err()));
  }

  return test::name::space::SizeValue::from_value(value.get_ok());
}

template<>
[[nodiscard]] Result<test::name::space::Size> Node::Map::to_value<test::name::space::Size>() const {
  std::map<std::string, Node> map = map_;

  auto location_w = map.find("w");
  if (location_w == map.end()) {
    return Result<test::name::space::Size>::err(Error("Missing w"));
  }
  Result<test::name::space::SizeValue> raw_value_w = location_w->second.to_value<test::name::space::SizeValue>();
  if (!raw_value_w.is_ok()) {
    Error error = raw_value_w.get_err();
    error.add_field("w");
    return Result<test::name::space::Size>::err(std::move(error));
  }
  test::name::space::SizeValue value_w = raw_value_w.get_ok();
  map.erase(location_w);

  auto location_h = map.find("h");
  if (location_h == map.end()) {
    return Result<test::name::space::Size>::err(Error("Missing h"));
  }
  Result<test::name::space::SizeValue> raw_value_h = location_h->second.to_value<test::name::space::SizeValue>();
  if (!raw_value_h.is_ok()) {
    Error error = raw_value_h.get_err();
    error.add_field("h");
    return Result<test::name::space::Size>::err(std::move(error));
  }
  test::name::space::SizeValue value_h = raw_value_h.get_ok();
  map.erase(location_h);

  return Result<test::name::space::Size>::ok(test::name::space::Size(std::move(value_w), std::move(value_h), Map(std::move(map))));
}

template<>
[[nodiscard]] Result<test::name::space::Point> Node::Map::to_value<test::name::space::Point>() const {
  std::map<std::string, Node> map = map_;

  auto location_x = map.find("x");
  if (location_x == map.end()) {
    return Result<test::name::space::Point>::err(Error("Missing x"));
  }
  Result<int> raw_value_x = location_x->second.to_value<int>();
  if (!raw_value_x.is_ok()) {
    Error error = raw_value_x.get_err();
    error.add_field("x");
    return Result<test::name::space::Point>::err(std::move(error));
  }
  int value_x = raw_value_x.get_ok();
  map.erase(location_x);

  auto location_y = map.find("y");
  if (location_y == map.end()) {
    return Result<test::name::space::Point>::err(Error("Missing y"));
  }
  Result<int> raw_value_y = location_y->second.to_value<int>();
  if (!raw_value_y.is_ok()) {
    Error error = raw_value_y.get_err();
    error.add_field("y");
    return Result<test::name::space::Point>::err(std::move(error));
  }
  int value_y = raw_value_y.get_ok();
  map.erase(location_y);

  return Result<test::name::space::Point>::ok(test::name::space::Point(std::move(value_x), std::move(value_y), Map(std::move(map))));
}

template<>
[[nodiscard]] Result<test::name::space::State> Node::Value::to_value<test::name::space::State>() const {
  if (value_ == "Filled") {
    return Result<test::name::space::State>::ok(test::name::space::State(test::name::space::State::TypeFilled{}));
  }
  if (value_ == "Edge") {
    return Result<test::name::space::State>::err(Error("Enum type Edge must contain a value"));
  }

  std::stringstream ss;
  ss << "Unknown enum type \"" << value_ << "\"";
  return Result<test::name::space::State>::err(Error(ss.str()));
}

template<>
[[nodiscard]] Result<test::name::space::State> Node::Map::to_value<test::name::space::State>() const {
  if (map_.size() != 1) {
    std::stringstream ss;
    ss << "There must be exactly one enum type specified but received " << map_.size();
    return Result<test::name::space::State>::err(Error(ss.str()));
  }

  if (map_.cbegin()->first == "Filled") {
    return Result<test::name::space::State>::err(Error("Enum type Filled must not include values"));
  }
  if (map_.cbegin()->first == "Edge") {
    Result<test::name::space::SizeValue> value = map_.cbegin()->second.to_value<test::name::space::SizeValue>();
    if (value.is_ok()) {
      return Result<test::name::space::State>::ok(test::name::space::State(test::name::space::State::TypeEdge{value.get_ok()}));
    }
    return Result<test::name::space::State>::err(value.get_err().add_field("Edge"));
  }

  std::stringstream ss;
  ss << "Unknown enum type \"" << map_.cbegin()->first << "\"";
  return Result<test::name::space::State>::err(Error(ss.str()));
}

template<>
[[nodiscard]] Result<test::name::space::DefaultValues> Node::Map::to_value<test::name::space::DefaultValues>() const {
  std::map<std::string, Node> map = map_;

  auto location_state = map.find("state");
  test::name::space::State value_state = test::name::space::DefaultValues::default_state();;
  if (location_state != map.end()) {
    Result<test::name::space::State> raw_value_state = location_state->second.to_value<test::name::space::State>();
    if (!raw_value_state.is_ok()) {
      Error error = raw_value_state.get_err();
      error.add_field("state");
      return Result<test::name::space::DefaultValues>::err(std::move(error));
    }
    value_state = raw_value_state.get_ok();
    map.erase(location_state);
  }

  auto location_size = map.find("size");
  if (location_size == map.end()) {
    return Result<test::name::space::DefaultValues>::err(Error("Missing size"));
  }
  Result<test::name::space::Size> raw_value_size = location_size->second.to_value<test::name::space::Size>();
  if (!raw_value_size.is_ok()) {
    Error error = raw_value_size.get_err();
    error.add_field("size");
    return Result<test::name::space::DefaultValues>::err(std::move(error));
  }
  test::name::space::Size value_size = raw_value_size.get_ok();
  map.erase(location_size);

  return Result<test::name::space::DefaultValues>::ok(test::name::space::DefaultValues(std::move(value_state), std::move(value_size), Map(std::move(map))));
}

template<>
[[nodiscard]] Result<test::name::space::Rectangle> Node::Map::to_value<test::name::space::Rectangle>() const {
  std::map<std::string, Node> map = map_;

  auto location_center = map.find("center");
  if (location_center == map.end()) {
    return Result<test::name::space::Rectangle>::err(Error("Missing center"));
  }
  Result<test::name::space::Point> raw_value_center = location_center->second.to_value<test::name::space::Point>();
  if (!raw_value_center.is_ok()) {
    Error error = raw_value_center.get_err();
    error.add_field("center");
    return Result<test::name::space::Rectangle>::err(std::move(error));
  }
  test::name::space::Point value_center = raw_value_center.get_ok();
  map.erase(location_center);

  auto location_size = map.find("size");
  std::optional<test::name::space::Size> value_size = test::name::space::Rectangle::default_size();;
  if (location_size != map.end()) {
    Result<test::name::space::Size> raw_value_size = location_size->second.to_value<test::name::space::Size>();
    if (!raw_value_size.is_ok()) {
      Error error = raw_value_size.get_err();
      error.add_field("size");
      return Result<test::name::space::Rectangle>::err(std::move(error));
    }
    value_size = raw_value_size.get_ok();
    map.erase(location_size);
  }

  auto location_state = map.find("state");
  std::optional<test::name::space::State> value_state = test::name::space::Rectangle::default_state();;
  if (location_state != map.end()) {
    Result<test::name::space::State> raw_value_state = location_state->second.to_value<test::name::space::State>();
    if (!raw_value_state.is_ok()) {
      Error error = raw_value_state.get_err();
      error.add_field("state");
      return Result<test::name::space::Rectangle>::err(std::move(error));
    }
    value_state = raw_value_state.get_ok();
    map.erase(location_state);
  }

  return Result<test::name::space::Rectangle>::ok(test::name::space::Rectangle(std::move(value_center), std::move(value_size), std::move(value_state), Map(std::move(map))));
}

template<>
[[nodiscard]] Result<test::name::space::Circle> Node::Map::to_value<test::name::space::Circle>() const {
  std::map<std::string, Node> map = map_;

  auto location_center = map.find("center");
  if (location_center == map.end()) {
    return Result<test::name::space::Circle>::err(Error("Missing center"));
  }
  Result<test::name::space::Point> raw_value_center = location_center->second.to_value<test::name::space::Point>();
  if (!raw_value_center.is_ok()) {
    Error error = raw_value_center.get_err();
    error.add_field("center");
    return Result<test::name::space::Circle>::err(std::move(error));
  }
  test::name::space::Point value_center = raw_value_center.get_ok();
  map.erase(location_center);

  auto location_radius = map.find("radius");
  if (location_radius == map.end()) {
    return Result<test::name::space::Circle>::err(Error("Missing radius"));
  }
  Result<test::name::space::SizeValue> raw_value_radius = location_radius->second.to_value<test::name::space::SizeValue>();
  if (!raw_value_radius.is_ok()) {
    Error error = raw_value_radius.get_err();
    error.add_field("radius");
    return Result<test::name::space::Circle>::err(std::move(error));
  }
  test::name::space::SizeValue value_radius = raw_value_radius.get_ok();
  map.erase(location_radius);

  auto location_state = map.find("state");
  std::optional<test::name::space::State> value_state = test::name::space::Circle::default_state();;
  if (location_state != map.end()) {
    Result<test::name::space::State> raw_value_state = location_state->second.to_value<test::name::space::State>();
    if (!raw_value_state.is_ok()) {
      Error error = raw_value_state.get_err();
      error.add_field("state");
      return Result<test::name::space::Circle>::err(std::move(error));
    }
    value_state = raw_value_state.get_ok();
    map.erase(location_state);
  }

  return Result<test::name::space::Circle>::ok(test::name::space::Circle(std::move(value_center), std::move(value_radius), std::move(value_state), Map(std::move(map))));
}

template<>
[[nodiscard]] Result<test::name::space::Geometry> Node::to_value<test::name::space::Geometry>() const {
  std::stringstream error;
  error << "Unable to parse any variant: [ ";

  Result<test::name::space::Rectangle> result_rectangle = to_value<test::name::space::Rectangle>();
  if (result_rectangle.is_ok()) {
    return Result<test::name::space::Geometry>::ok(test::name::space::Geometry(result_rectangle.get_ok()));
  }
  error << "test::name::space::Rectangle { " << result_rectangle.get_err() << " }";
  error << ", ";

  Result<test::name::space::Circle> result_circle = to_value<test::name::space::Circle>();
  if (result_circle.is_ok()) {
    return Result<test::name::space::Geometry>::ok(test::name::space::Geometry(result_circle.get_ok()));
  }
  error << "test::name::space::Circle { " << result_circle.get_err() << " }";

  error << " ]";

  return Result<test::name::space::Geometry>::err(Error(error.str()));
}

template<>
[[nodiscard]] Result<test::name::space::GeometryList> Node::List::to_value<test::name::space::GeometryList>() const {
  std::vector<test::name::space::Geometry> values;
  values.reserve(list_.size());
  for (auto node = list_.cbegin(); node < list_.cend(); ++node) {
    Result<test::name::space::Geometry> value = node->to_value<test::name::space::Geometry>();
    if (!value.is_ok()) {
      Error error = value.get_err();
      error.add_list(node - list_.cbegin());
      return Result<test::name::space::GeometryList>::err(std::move(error));
    }
    values.push_back(std::move(value.get_ok()));
  }

  return Result<test::name::space::GeometryList>::ok(test::name::space::GeometryList(std::move(values)));
}

template<>
[[nodiscard]] Result<test::name::space::DataModel> Node::Map::to_value<test::name::space::DataModel>() const {
  std::map<std::string, Node> map = map_;

  auto location_version = map.find("version");
  test::name::space::VersionString value_version = test::name::space::DataModel::default_version();;
  if (location_version != map.end()) {
    Result<test::name::space::VersionString> raw_value_version = location_version->second.to_value<test::name::space::VersionString>();
    if (!raw_value_version.is_ok()) {
      Error error = raw_value_version.get_err();
      error.add_field("version");
      return Result<test::name::space::DataModel>::err(std::move(error));
    }
    value_version = raw_value_version.get_ok();
    map.erase(location_version);
  }

  auto location_defaults = map.find("defaults");
  if (location_defaults == map.end()) {
    return Result<test::name::space::DataModel>::err(Error("Missing defaults"));
  }
  Result<test::name::space::DefaultValues> raw_value_defaults = location_defaults->second.to_value<test::name::space::DefaultValues>();
  if (!raw_value_defaults.is_ok()) {
    Error error = raw_value_defaults.get_err();
    error.add_field("defaults");
    return Result<test::name::space::DataModel>::err(std::move(error));
  }
  test::name::space::DefaultValues value_defaults = raw_value_defaults.get_ok();
  map.erase(location_defaults);

  auto location_geometries = map.find("geometries");
  if (location_geometries == map.end()) {
    return Result<test::name::space::DataModel>::err(Error("Missing geometries"));
  }
  Result<test::name::space::GeometryList> raw_value_geometries = location_geometries->second.to_value<test::name::space::GeometryList>();
  if (!raw_value_geometries.is_ok()) {
    Error error = raw_value_geometries.get_err();
    error.add_field("geometries");
    return Result<test::name::space::DataModel>::err(std::move(error));
  }
  test::name::space::GeometryList value_geometries = raw_value_geometries.get_ok();
  map.erase(location_geometries);

  return Result<test::name::space::DataModel>::ok(test::name::space::DataModel(std::move(value_version), std::move(value_defaults), std::move(value_geometries), Map(std::move(map))));
}

} // namespace termite

// This is the end of my .cpp file
