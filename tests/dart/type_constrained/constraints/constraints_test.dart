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
  if (!ok.isOk()) {
    return 'Failed to parse valid constrained value';
  }
  final okValue = ok.asOk().value;
  if (okValue != DataType(2)) {
    return 'Failed to parse valid constrained value: $okValue';
  }

  final invalid1 = DataType.fromNode(termite.Node.value('0'));
  final invalid2 = DataType.fromNode(termite.Node.value('1'));
  if (invalid1.isOk() || invalid2.isOk()) {
    return 'Expected constraint errors';
  }
  return null;
}

String? testConstraintChecksObject() {
  final ok = DataType.fromObject(2);
  if (!ok.isOk()) {
    return 'Failed to parse valid constrained value';
  }
  final okValue = ok.asOk().value;
  if (okValue != DataType(2)) {
    return 'Failed to parse valid constrained value: $okValue';
  }

  final invalid1 = DataType.fromObject(0);
  final invalid2 = DataType.fromObject(1);
  if (invalid1.isOk() || invalid2.isOk()) {
    return 'Expected constraint errors';
  }
  return null;
}

String? testRoundtrip() {
  final value = DataType(2);
  final loaded = DataType.fromNode(value.toNode());

  if (!loaded.isOk()) {
    return 'Failed to reload constrained value';
  }

  final okLoaded = loaded.asOk().value;
  if (okLoaded != value) {
    return 'Reloaded constrained value mismatch: $okLoaded';
  }
  return null;
}

void main() {
  final code = runTests({
    'testConstraintChecks': testConstraintChecks,
    'testConstraintChecksObject': testConstraintChecksObject,
    'testRoundtrip': testRoundtrip,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
