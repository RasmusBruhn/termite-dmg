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

String? testLoadAndErrors() {
  final ok = DataType.fromNode(
    termite.Node.mapping({'field1': termite.Node.value('1'), 'field2': termite.Node.value('5.0')}),
  );
  if (ok is! termite.Ok<DataType>) {
    return 'Failed to load valid struct';
  }

  final missing = DataType.fromNode(termite.Node.mapping({'field1': termite.Node.value('1')}));
  if (missing is! termite.Error<DataType>) {
    return 'Expected error when required field is missing';
  }
  return null;
}

void main() {
  final code = runTests({'testLoadAndErrors': testLoadAndErrors});
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
