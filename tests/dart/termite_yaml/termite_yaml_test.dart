import 'generated/termite.dart' as termite;
import 'generated/termite-yaml.dart' as termite_yaml;

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

String? testFromString() {
  final result = termite_yaml.fromString(
    'field1: "Test1"\nfield2:\n  - "Test2"\n  - "Test3"',
  );
  if (!result.isOk()) {
    return 'Failed to decode YAML string';
  }

  final node = result.asOk();
  if (node is! termite.Mapping) {
    return 'Expected mapping';
  }
  if (node.map['field1'] is! termite.Value ||
      (node.map['field1'] as termite.Value).value != 'Test1') {
    return 'Wrong field1 value';
  }
  return null;
}

String? testFromFile() {
  final result = termite_yaml.fromFile('test_yaml.yaml');
  if (!result.isOk()) {
    return 'Failed to decode YAML string';
  }

  final node = result.asOk();
  if (node is! termite.Mapping) {
    return 'Expected mapping';
  }
  if (node.map['field1'] is! termite.Value ||
      (node.map['field1'] as termite.Value).value != 'Test1') {
    return 'Wrong field1 value';
  }
  return null;
}

String? testToStringAndBack() {
  final node = termite.Node.mapping({
    'field1': termite.Node.value('Test1'),
    'field2': termite.Node.sequence([
      termite.Node.value('Test2'),
      termite.Node.value('Test3'),
    ]),
  });

  final encoded = termite_yaml.toString(node);
  if (!encoded.isOk()) {
    return 'Failed to encode YAML';
  }

  final decoded = termite_yaml.fromString(encoded.asOk());
  if (!decoded.isOk()) {
    return 'Failed to decode encoded YAML';
  }

  final decodedNode = decoded.asOk();
  if (decodedNode.toObject().toString() != node.toObject().toString()) {
    return 'Roundtrip mismatch';
  }
  return null;
}

String? testToFileAndBack() {
  final node = termite.Node.mapping({
    'field1': termite.Node.value('Test1'),
    'field2': termite.Node.sequence([
      termite.Node.value('Test2'),
      termite.Node.value('Test3'),
    ]),
  });

  final result = termite_yaml.toFile(node, 'generated/yaml_test.yaml');
  if (!result.isOk()) {
    return 'Failed to encode YAML';
  }

  final decoded = termite_yaml.fromFile('generated/yaml_test.yaml');
  if (!decoded.isOk()) {
    return 'Failed to decode encoded YAML';
  }

  final decodedNode = decoded.asOk();
  if (decodedNode.toObject().toString() != node.toObject().toString()) {
    return 'Roundtrip mismatch';
  }
  return null;
}

void main() {
  final code = runTests({
    'testFromString': testFromString,
    'testFromFile': testFromFile,
    'testToStringAndBack': testToStringAndBack,
    'testToFileAndBack': testToFileAndBack,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
