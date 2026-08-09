import 'generated/optional.dart';
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

String? testDefaultsAndOptional() {
  final defaults = DataType.fromNode(termite.Node.mapping({}));
  if (!defaults.isOk()) {
    return 'Failed to load defaults';
  }
  if (defaults.asOk() != DataType(field1: 1)) {
    return 'Default values are incorrect';
  }

  final explicit = DataType.fromNode(
    termite.Node.mapping({
      'field1': termite.Node.value('-2'),
      'field2': termite.Node.value('3.5'),
    }),
  );
  if (!explicit.isOk()) {
    return 'Failed to load explicit values';
  }
  if (explicit.asOk() != DataType(field1: -2, field2: 3.5)) {
    return 'Explicit values are incorrect';
  }
  return null;
}

String? testInvalidType() {
  final invalidType = DataType.fromNode(
    termite.Node.mapping({
      'field1': termite.Node.value('1.0'),
      'field2': termite.Node.value('5.0'),
    }),
  );
  if (invalidType.isOk()) {
    return 'Expected type validation error';
  }
  return null;
}

void main() {
  final code = runTests({
    'testDefaultsAndOptional': testDefaultsAndOptional,
    'testInvalidType': testInvalidType,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
