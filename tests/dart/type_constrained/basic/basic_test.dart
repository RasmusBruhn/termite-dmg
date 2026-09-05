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
  final loaded = DataType.fromNode(termite.Node.value('1'));
  if (!loaded.isOk()) {
    return 'Failed to parse constrained type';
  }

  final okLoaded = loaded.asOk().value;
  if (okLoaded != DataType(1)) {
    return 'Wrong constrained value: $okLoaded';
  }
  return null;
}

String? testLoadObject() {
  final loaded = DataType.fromObject(1);
  if (!loaded.isOk()) {
    return 'Failed to parse constrained type';
  }

  final okLoaded = loaded.asOk().value;
  if (okLoaded != DataType(1)) {
    return 'Wrong constrained value: $okLoaded';
  }
  return null;
}

String? testInvalid() {
  final invalidValue = DataType.fromNode(termite.Node.value('1.0'));
  if (invalidValue.isOk()) {
    return 'Expected value parse error';
  }

  final invalidType = DataType.fromNode(
    termite.Node.sequence([termite.Node.value('1')]),
  );
  if (invalidType.isOk()) {
    return 'Expected node type parse error';
  }
  return null;
}

String? testInvalidObject() {
  final invalidValue = DataType.fromObject(1.0);
  if (invalidValue.isOk()) {
    return 'Expected value parse error';
  }

  final invalidType = DataType.fromObject([1]);
  if (invalidType.isOk()) {
    return 'Expected node type parse error';
  }
  return null;
}

String? testRoundtrip() {
  final value = DataType(1);
  final loaded = DataType.fromNode(value.toNode());

  if (!loaded.isOk()) {
    return 'Failed to reload constrained type';
  }

  final okLoaded = loaded.asOk().value;
  if (okLoaded != value) {
    return 'Reloaded constrained value mismatch: $okLoaded';
  }
  return null;
}

void main() {
  final code = runTests({
    'testLoad': testLoad,
    'testLoadObject': testLoadObject,
    'testInvalid': testInvalid,
    'testInvalidObject': testInvalidObject,
    'testRoundtrip': testRoundtrip,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
