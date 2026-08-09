import 'generated/termite.dart' as termite;
import 'generated/termite-json.dart' as termite_json;

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
  final result = termite_json.fromString(
    '{"field1":"Test1","field2":["Test2","Test3"]}',
  );
  if (!result.isOk()) {
    return 'Failed to decode JSON string';
  }

  final node = result.asOk();
  if (node !=
      termite.Node.mapping({
        'field1': termite.Node.value('Test1'),
        'field2': termite.Node.sequence([
          termite.Node.value('Test2'),
          termite.Node.value('Test3'),
        ]),
      })) {
    return 'Decoded node does not match expected structure';
  }
  return null;
}

String? testFromFile() {
  final result = termite_json.fromFile('test_json.json');
  if (!result.isOk()) {
    return 'Failed to decode JSON string';
  }

  final node = result.asOk();
  if (node !=
      termite.Node.mapping({
        'field1': termite.Node.value('Test1'),
        'field2': termite.Node.sequence([
          termite.Node.value('Test2'),
          termite.Node.value('Test3'),
        ]),
      })) {
    return 'Decoded node does not match expected structure';
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

  final encoded = termite_json.toString(node);
  if (!encoded.isOk()) {
    return 'Failed to encode JSON';
  }

  final decoded = termite_json.fromString(encoded.asOk());
  if (!decoded.isOk()) {
    return 'Failed to decode encoded JSON';
  }

  final decodedNode = decoded.asOk();
  if (decodedNode != node) {
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

  final result = termite_json.toFile(node, 'generated/json_test.json');
  if (!result.isOk()) {
    return 'Failed to encode JSON';
  }

  final decoded = termite_json.fromFile('generated/json_test.json');
  if (!decoded.isOk()) {
    return 'Failed to decode encoded JSON';
  }

  final decodedNode = decoded.asOk();
  if (decodedNode != node) {
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
