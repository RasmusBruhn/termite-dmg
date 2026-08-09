import 'dart:io';

import 'generated/header.dart';
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

String? testHeaderInjected() {
  final generated = File('generated/header.dart').readAsStringSync();
  if (!generated.contains('// Header dart')) {
    return 'Expected header snippet was not injected';
  }
  return null;
}

String? testFromMap() {
  final result = DataType.fromNode(termite.Node.mapping({}));
  if (!result.isOk()) {
    return 'Failed to parse empty mapping as DataType';
  }
  return null;
}

void main() {
  final code = runTests({
    'testHeaderInjected': testHeaderInjected,
    'testFromMap': testFromMap,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
