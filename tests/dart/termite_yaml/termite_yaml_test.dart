import 'dart:io';

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
  if (result is! termite.Ok<termite.Node>) {
    return 'Failed to decode JSON string';
  }

  final node = result.value;
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
  if (encoded is! termite.Ok<String>) {
    return 'Failed to encode JSON';
  }

  final decoded = termite_yaml.fromString(encoded.value);
  if (decoded is! termite.Ok<termite.Node>) {
    return 'Failed to decode encoded JSON';
  }

  final decodedNode = decoded.value;
  if (decodedNode.toObject().toString() != node.toObject().toString()) {
    return 'Roundtrip mismatch';
  }
  return null;
}

String? testFileRoundtrip() {
  final file = File('generated/yaml_test.yaml');
  file.writeAsStringSync('field1: "Test1"\nfield2:\n  - "Test2"\n  - "Test3"');

  final fileData = File('generated/yaml_test.yaml').readAsStringSync();
  final parsed = termite_yaml.fromString(fileData);
  if (parsed is! termite.Ok<termite.Node>) {
    file.deleteSync();
    return 'Failed to parse fixture file';
  }

  final serialized = termite_yaml.toString(parsed.value);
  if (serialized is! termite.Ok<String>) {
    file.deleteSync();
    return 'Failed to serialize parsed fixture';
  }

  file.writeAsStringSync(serialized.value);
  final reparsed = termite_yaml.fromString(file.readAsStringSync());
  if (reparsed is! termite.Ok<termite.Node>) {
    file.deleteSync();
    return 'Failed to reparse runtime file';
  }

  if (reparsed.value.toObject().toString() !=
      parsed.value.toObject().toString()) {
    file.deleteSync();
    return 'File roundtrip mismatch';
  }

  file.deleteSync();
  return null;
}

void main() {
  final code = runTests({
    'testFromString': testFromString,
    'testToStringAndBack': testToStringAndBack,
    'testFileRoundtrip': testFileRoundtrip,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
