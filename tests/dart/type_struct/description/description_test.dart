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

String? testEmptyStructs() {
  final one = DataType1.fromNode(termite.Node.mapping({}));
  final two = DataType2.fromNode(termite.Node.mapping({}));
  if (one is! termite.Ok<DataType1> || two is! termite.Ok<DataType2>) {
    return 'Failed to load empty structs';
  }
  return null;
}

void main() {
  final code = runTests({'testEmptyStructs': testEmptyStructs});
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
