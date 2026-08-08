import 'generated/basic.dart';
import 'generated/termite.dart' as termite;

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

String? testLoad() {
  final loaded = DataType.fromNode(
    termite.Node.sequence([termite.Node.value('1'), termite.Node.value('2')]),
  );
  if (loaded is! termite.Ok<DataType>) {
    return 'Failed to parse array';
  }
  if (loaded.value.values.length != 2 || loaded.value.values[0] != 1 || loaded.value.values[1] != 2) {
    return 'Array values mismatch';
  }
  return null;
}

String? testInvalid() {
  final invalidElement = DataType.fromNode(
    termite.Node.sequence([termite.Node.value('1'), termite.Node.value('2.5')]),
  );
  if (invalidElement is! termite.Error<DataType>) {
    return 'Expected element type error';
  }

  final invalidType = DataType.fromNode(termite.Node.value('1.0'));
  if (invalidType is! termite.Error<DataType>) {
    return 'Expected node type error';
  }
  return null;
}

String? testRoundtrip() {
  final value = DataType([1, 2]);
  final loaded = DataType.fromNode(value.toNode());
  if (loaded is! termite.Ok<DataType>) {
    return 'Failed to reload array';
  }
  if (loaded.value.values.toString() != value.values.toString()) {
    return 'Reloaded array mismatch';
  }
  return null;
}

void main() {
  final code = runTests({'testLoad': testLoad, 'testInvalid': testInvalid, 'testRoundtrip': testRoundtrip});
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
