import 'generated/macros.dart';
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

String? testMacroDefault() {
  final defaults = DataType.fromNode(termite.Node.mapping({}));
  if (!defaults.isOk()) {
    return 'Failed to load defaults';
  }
  if (defaults.asOk() != DataType(field1: 1)) {
    return 'Macro default values are incorrect';
  }
  return null;
}

String? testRoundtrip() {
  final value = DataType(field1: -2, field2: 3.5);
  final reloaded = DataType.fromNode(value.toNode());
  if (!reloaded.isOk()) {
    return 'Failed to reload struct';
  }
  if (reloaded.asOk() != value) {
    return 'Reloaded values mismatch';
  }
  return null;
}

void main() {
  final code = runTests({
    'testMacroDefault': testMacroDefault,
    'testRoundtrip': testRoundtrip,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
