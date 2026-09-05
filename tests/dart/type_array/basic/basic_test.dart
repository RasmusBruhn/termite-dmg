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
  if (!loaded.isOk()) {
    return 'Failed to parse array';
  }
  final okValue = loaded.asOk().value;
  if (okValue != DataType([1, 2])) {
    return 'Array values mismatch: $okValue';
  }
  return null;
}

String? testLoadObject() {
  final loaded = DataType.fromObject([1, 2]);
  if (!loaded.isOk()) {
    return 'Failed to parse array';
  }
  final okValue = loaded.asOk().value;
  if (okValue != DataType([1, 2])) {
    return 'Array values mismatch: $okValue';
  }
  return null;
}

String? testInvalid() {
  final invalidElement = DataType.fromNode(
    termite.Node.sequence([termite.Node.value('1'), termite.Node.value('2.5')]),
  );
  if (invalidElement.isOk()) {
    return 'Expected element type error';
  }

  final invalidType = DataType.fromNode(termite.Node.value('1.0'));
  if (invalidType.isOk()) {
    return 'Expected node type error';
  }
  return null;
}

String? testInvalidObject() {
  final invalidElement = DataType.fromObject([1, 2.5]);
  if (invalidElement.isOk()) {
    return 'Expected element type error';
  }

  final invalidType = DataType.fromObject(1.0);
  if (invalidType.isOk()) {
    return 'Expected node type error';
  }
  return null;
}

String? testRoundtrip() {
  final value = DataType([1, 2]);
  final loaded = DataType.fromNode(value.toNode());

  if (!loaded.isOk()) {
    return 'Failed to reload array';
  }
  
  final okValue = loaded.asOk().value;
  if (okValue != value) {
    return 'Reloaded array mismatch: $okValue';
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
