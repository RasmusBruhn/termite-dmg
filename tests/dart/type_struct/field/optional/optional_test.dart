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
  final defaultsOk = defaults.getOk();
  if (defaultsOk != DataType(field1: 1)) {
    return 'Default values are incorrect: $defaultsOk';
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
  final explicitOk = explicit.getOk();
  if (explicitOk != DataType(field1: -2, field2: 3.5)) {
    return 'Explicit values are incorrect: $explicitOk';
  }
  return null;
}

String? testDefaultsAndOptionalObject() {
  final defaults = DataType.fromObject({});
  if (!defaults.isOk()) {
    return 'Failed to load defaults';
  }
  final defaultsOk = defaults.getOk();
  if (defaultsOk != DataType(field1: 1)) {
    return 'Default values are incorrect: $defaultsOk';
  }

  final explicit = DataType.fromObject({'field1': -2, 'field2': 3.5});
  if (!explicit.isOk()) {
    return 'Failed to load explicit values';
  }
  final explicitOk = explicit.getOk();
  if (explicitOk != DataType(field1: -2, field2: 3.5)) {
    return 'Explicit values are incorrect: $explicitOk';
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
    return 'Expected type validation error: ${invalidType.asError().getMessage()}';
  }
  return null;
}

String? testInvalidTypeObject() {
  final invalidType = DataType.fromObject({'field1': 1.0, 'field2': 5.0});
  if (invalidType.isOk()) {
    return 'Expected type validation error: ${invalidType.asError().getMessage()}';
  }
  return null;
}

void main() {
  final code = runTests({
    'testDefaultsAndOptional': testDefaultsAndOptional,
    'testDefaultsAndOptionalObject': testDefaultsAndOptionalObject,
    'testInvalidType': testInvalidType,
    'testInvalidTypeObject': testInvalidTypeObject,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
