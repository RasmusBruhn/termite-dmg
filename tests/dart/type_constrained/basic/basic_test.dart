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
  if (loaded.asOk() != DataType.fromValue(1).asOk()) {
    return 'Wrong constrained value';
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

String? testRoundtrip() {
  final value = DataType.fromValue(1).asOk();
  final loaded = DataType.fromNode(value.toNode());
  if (!loaded.isOk() || loaded.asOk() != value) {
    return 'Failed constrained roundtrip';
  }
  return null;
}

void main() {
  final code = runTests({
    'testLoad': testLoad,
    'testInvalid': testInvalid,
    'testRoundtrip': testRoundtrip,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
