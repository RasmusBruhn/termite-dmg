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
  final version = model.VersionString('1.0.1');
  final defaultState = model.State.newEdge(model.SizeValue(1));
  final defaults = model.DefaultValues(
    state: defaultState,
    size: model.Size(w: model.SizeValue(10), h: model.SizeValue(20)),
  );

  final rectangle = model.Rectangle(
    center: model.Point(x: 15, y: -30),
    size: null,
    state: model.State.newEdge(model.SizeValue(5)),
  );
  final circle = model.Circle(
    center: model.Point(x: 0, y: 0),
    radius: model.SizeValue(7),
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
    return 'Failed to serialize full example to JSON: ${json.asError().getMessage()}';
  }

  final parsedNode = termite_json.fromString(json.getOk());
  if (!parsedNode.isOk()) {
    return 'Failed to parse JSON string back into node: ${parsedNode.asError().getMessage()}';
  }

  final loaded = model.DataModel.fromNode(parsedNode.getOk());
  if (!loaded.isOk()) {
    return 'Failed to parse DataModel from node: ${loaded.asError().getMessage()}';
  }

  final loadedModel = loaded.getOk();
  if (loadedModel != dataModel) {
    return 'Model mismatch after reload: $loadedModel';
  }
  return null;
}

String? testReloadThroughYaml() {
  final version = model.VersionString('1.0.1');
  final defaultState = model.State.newEdge(model.SizeValue(1));
  final defaults = model.DefaultValues(
    state: defaultState,
    size: model.Size(w: model.SizeValue(10), h: model.SizeValue(20)),
  );

  final rectangle = model.Rectangle(
    center: model.Point(x: 15, y: -30),
    size: null,
    state: model.State.newEdge(model.SizeValue(5)),
  );
  final circle = model.Circle(
    center: model.Point(x: 0, y: 0),
    radius: model.SizeValue(7),
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
    return 'Failed to serialize full example to YAML: ${yaml.asError().getMessage()}';
  }

  final parsedNode = termite_yaml.fromString(yaml.getOk());
  if (!parsedNode.isOk()) {
    return 'Failed to parse YAML string back into node: ${parsedNode.asError().getMessage()}';
  }

  final loaded = model.DataModel.fromNode(parsedNode.getOk());
  if (!loaded.isOk()) {
    return 'Failed to parse DataModel from node: ${loaded.asError().getMessage()}';
  }

  final loadedModel = loaded.getOk();
  if (loadedModel != dataModel) {
    return 'Model mismatch after reload: $loadedModel';
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
