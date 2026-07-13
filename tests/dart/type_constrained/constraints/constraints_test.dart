import 'generated/constraints.dart';
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

String? testConstraintChecks() {
  final ok = DataType.fromNode(termite.Node.value('2'));
  if (ok is! termite.Ok<DataType> || ok.value.value != 2) {
    return 'Failed to parse valid constrained value';
  }

  final invalid1 = DataType.fromNode(termite.Node.value('0'));
  final invalid2 = DataType.fromNode(termite.Node.value('1'));
  if (invalid1 is! termite.Error<DataType> || invalid2 is! termite.Error<DataType>) {
    return 'Expected constraint errors';
  }
  return null;
}

String? testRoundtrip() {
  final value = (DataType.fromValue(2) as termite.Ok<DataType>).value;
  final loaded = DataType.fromNode(value.toNode());
  if (loaded is! termite.Ok<DataType> || loaded.value.value != 2) {
    return 'Failed constrained roundtrip';
  }
  return null;
}

void main() {
  final code = runTests({'testConstraintChecks': testConstraintChecks, 'testRoundtrip': testRoundtrip});
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
