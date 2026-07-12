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

String? testLoadAndErrors() {
  final ok = DataType.fromNode(
    termite.Node.mapping({'field1': termite.Node.value('1'), 'field2': termite.Node.value('5.0')}),
  );
  if (ok is! termite.Ok<DataType>) {
    return 'Failed to load valid struct';
  }
  if (ok.value.field1 != 1 || ok.value.field2 != 5.0) {
    return 'Loaded values are incorrect';
  }

  final missing = DataType.fromNode(termite.Node.mapping({'field1': termite.Node.value('1')}));
  if (missing is! termite.Error<DataType>) {
    return 'Expected error when required field is missing';
  }

  final invalidType = DataType.fromNode(
    termite.Node.mapping({'field1': termite.Node.value('1.0'), 'field2': termite.Node.value('5.0')}),
  );
  if (invalidType is! termite.Error<DataType>) {
    return 'Expected error when field type is invalid';
  }

  final wrongNode = DataType.fromNode(termite.Node.value('1.0'));
  if (wrongNode is! termite.Error<DataType>) {
    return 'Expected error when node type is invalid';
  }
  return null;
}

String? testRoundtrip() {
  final value = DataType(field1: 1, field2: 5.0);
  final reloaded = DataType.fromNode(value.toNode());
  if (reloaded is! termite.Ok<DataType>) {
    return 'Failed to reload struct';
  }
  if (reloaded.value.field1 != value.field1 || reloaded.value.field2 != value.field2) {
    return 'Reloaded value mismatch';
  }
  return null;
}

void main() {
  final code = runTests({'testLoadAndErrors': testLoadAndErrors, 'testRoundtrip': testRoundtrip});
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
