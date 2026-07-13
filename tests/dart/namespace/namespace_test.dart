import 'generated/namespace.dart';
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

String? testDefaultAndLoad() {
  final empty = DataType();
  final loaded = DataType.fromNode(termite.Node.mapping({}));
  if (loaded is! termite.Ok<DataType>) {
    return 'Failed to parse empty mapping as DataType';
  }
  if (empty.toString() != loaded.value.toString()) {
    return 'Mismatch between default and loaded DataType';
  }
  return null;
}

void main() {
  final code = runTests({'testDefaultAndLoad': testDefaultAndLoad});
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
