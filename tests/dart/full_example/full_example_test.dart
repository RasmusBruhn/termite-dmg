import 'generated/full_example.dart' as model;
import 'generated/termite.dart' as termite;
import 'generated/termite-json.dart' as termite_json;

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
  final version = (model.VersionString.fromValue('1.0.1') as termite.Ok<model.VersionString>).value;
  final defaultState = model.State.newEdge((model.SizeValue.fromValue(1) as termite.Ok<model.SizeValue>).value);
  final defaults = model.DefaultValues(
    state: defaultState,
    size: model.Size(
      w: (model.SizeValue.fromValue(10) as termite.Ok<model.SizeValue>).value,
      h: (model.SizeValue.fromValue(20) as termite.Ok<model.SizeValue>).value,
    ),
  );

  final rectangle = model.Rectangle(
    center: model.Point(x: 15, y: -30),
    size: null,
    state: model.State.newEdge((model.SizeValue.fromValue(5) as termite.Ok<model.SizeValue>).value),
  );
  final circle = model.Circle(
    center: model.Point(x: 0, y: 0),
    radius: (model.SizeValue.fromValue(7) as termite.Ok<model.SizeValue>).value,
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
  if (json is! termite.Ok<String>) {
    return 'Failed to serialize full example to JSON';
  }

  final parsedNode = termite_json.fromString(json.value);
  if (parsedNode is! termite.Ok<termite.Node>) {
    return 'Failed to parse JSON string back into node';
  }

  final loaded = model.DataModel.fromNode(parsedNode.value);
  if (loaded is! termite.Ok<model.DataModel>) {
    return 'Failed to parse DataModel from node';
  }

  final loadedModel = loaded.value;
  if (loadedModel.version.value != '1.0.1') {
    return 'Version mismatch after reload';
  }
  if (loadedModel.geometries.values.length != 2) {
    return 'Geometry list size mismatch';
  }
  return null;
}

void main() {
  final code = runTests({'testReloadThroughJson': testReloadThroughJson});
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
