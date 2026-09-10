import 'generated/description.dart';
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
  final ok = DataType.fromNode(
    termite.Node.mapping({
      'field1': termite.Node.value('1'),
      'field2': termite.Node.value('5.0'),
    }),
  );
  if (!ok.isOk()) {
    return 'Failed to load valid struct';
  }
  final okOk = ok.getOk();
  if (okOk != DataType(field1: 1, field2: 5.0)) {
    return 'Loaded values are incorrect: $okOk';
  }

  return null;
}

String? testLoadObject() {
  final ok = DataType.fromObject({'field1': 1, 'field2': 5.0});
  if (!ok.isOk()) {
    return 'Failed to load valid struct';
  }
  final okOk = ok.getOk();
  if (okOk != DataType(field1: 1, field2: 5.0)) {
    return 'Loaded values are incorrect: $okOk';
  }

  return null;
}

String? testErrors() {
  final missing = DataType.fromNode(
    termite.Node.mapping({'field1': termite.Node.value('1')}),
  );
  if (missing.isOk()) {
    return 'Expected error when required field is missing: $missing';
  }

  final invalidType = DataType.fromNode(
    termite.Node.mapping({
      'field1': termite.Node.value('1.0'),
      'field2': termite.Node.value('5.0'),
    }),
  );
  if (invalidType.isOk()) {
    return 'Expected error when field type is invalid: $invalidType';
  }

  final wrongNode = DataType.fromNode(termite.Node.value('1.0'));
  if (wrongNode.isOk()) {
    return 'Expected error when node type is invalid: $wrongNode';
  }

  return null;
}

String? testErrorsObject() {
  final missing = DataType.fromObject({'field1': 1});
  if (missing.isOk()) {
    return 'Expected error when required field is missing: $missing';
  }

  final invalidType = DataType.fromObject({'field1': 1.0, 'field2': 5.0});
  if (invalidType.isOk()) {
    return 'Expected error when field type is invalid: $invalidType';
  }

  final wrongNode = DataType.fromObject('1.0');
  if (wrongNode.isOk()) {
    return 'Expected error when node type is invalid: $wrongNode';
  }

  return null;
}

String? testRoundtrip() {
  final value = DataType(field1: 1, field2: 5.0);
  final reloaded = DataType.fromNode(value.toNode());
  if (!reloaded.isOk()) {
    return 'Failed to reload struct';
  }
  final reloadedOk = reloaded.getOk();
  if (reloadedOk != value) {
    return 'Reloaded value mismatch: $reloadedOk';
  }
  return null;
}

void main() {
  final code = runTests({
    'testLoad': testLoad,
    'testLoadObject': testLoadObject,
    'testErrors': testErrors,
    'testErrorsObject': testErrorsObject,
    'testRoundtrip': testRoundtrip,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
