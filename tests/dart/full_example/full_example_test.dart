import 'generated/full_example.dart' as model;
import 'generated/termite-json.dart' as termite_json;
import 'generated/termite-yaml.dart' as termite_yaml;

typedef TestFunction = String? Function();

int runTests(Map<String, TestFunction> tests) {
  print('Running ${tests.length} tests');
  var progress = 1;
  for (final entry in tests.entries) {
    final error = entry.value();
    if (error != null) {
      print('Error occurred at "${entry.key}": $error');
      return progress;
    }
    progress += 1;
  }
  print('No errors');
  return 0;
}

String? testReloadThroughJson() {
  final version = model.VersionString.fromValue('1.0.1').asOk();
  final defaultState = model.State.newEdge(model.SizeValue.fromValue(1).asOk());
  final defaults = model.DefaultValues(
    state: defaultState,
    size: model.Size(
      w: model.SizeValue.fromValue(10).asOk(),
      h: model.SizeValue.fromValue(20).asOk(),
    ),
  );

  final rectangle = model.Rectangle(
    center: model.Point(x: 15, y: -30),
    size: null,
    state: model.State.newEdge(model.SizeValue.fromValue(5).asOk()),
  );
  final circle = model.Circle(
    center: model.Point(x: 0, y: 0),
    radius: model.SizeValue.fromValue(7).asOk(),
    state: null,
  );

  final dataModel = model.DataModel(
    version: version,
    defaults: defaults,
    geometries: model.GeometryList([
      model.Geometry.newRectangle(rectangle),
      model.Geometry.newCircle(circle),
    ]),
  );

  final json = termite_json.toString(dataModel.toNode());
  if (!json.isOk()) {
    return 'Failed to serialize full example to JSON';
  }

  final parsedNode = termite_json.fromString(json.asOk());
  if (!parsedNode.isOk()) {
    return 'Failed to parse JSON string back into node';
  }

  final loaded = model.DataModel.fromNode(parsedNode.asOk());
  if (!loaded.isOk()) {
    return 'Failed to parse DataModel from node';
  }

  final loadedModel = loaded.asOk();
  if (loadedModel != dataModel) {
    return 'Model mismatch after reload: $loadedModel != $dataModel';
  }
  return null;
}

String? testReloadThroughYaml() {
  final version = model.VersionString.fromValue('1.0.1').asOk();
  final defaultState = model.State.newEdge(model.SizeValue.fromValue(1).asOk());
  final defaults = model.DefaultValues(
    state: defaultState,
    size: model.Size(
      w: model.SizeValue.fromValue(10).asOk(),
      h: model.SizeValue.fromValue(20).asOk(),
    ),
  );

  final rectangle = model.Rectangle(
    center: model.Point(x: 15, y: -30),
    size: null,
    state: model.State.newEdge(model.SizeValue.fromValue(5).asOk()),
  );
  final circle = model.Circle(
    center: model.Point(x: 0, y: 0),
    radius: model.SizeValue.fromValue(7).asOk(),
    state: null,
  );

  final dataModel = model.DataModel(
    version: version,
    defaults: defaults,
    geometries: model.GeometryList([
      model.Geometry.newRectangle(rectangle),
      model.Geometry.newCircle(circle),
    ]),
  );

  final yaml = termite_yaml.toString(dataModel.toNode());
  if (!yaml.isOk()) {
    return 'Failed to serialize full example to YAML';
  }

  final parsedNode = termite_yaml.fromString(yaml.asOk());
  if (!parsedNode.isOk()) {
    return 'Failed to parse YAML string back into node';
  }

  final loaded = model.DataModel.fromNode(parsedNode.asOk());
  if (!loaded.isOk()) {
    return 'Failed to parse DataModel from node';
  }

  final loadedModel = loaded.asOk();
  if (loadedModel != dataModel) {
    return 'Model mismatch after reload: $loadedModel != $dataModel';
  }
  return null;
}

void main() {
  final code = runTests({
    'testReloadThroughJson': testReloadThroughJson,
    'testReloadThroughYaml': testReloadThroughYaml,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
