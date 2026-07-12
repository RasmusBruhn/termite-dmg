import 'dart:io';

import 'generated/outline.dart';
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

String? testHeaderAndFooterInjected() {
  final generated = File('generated/outline.dart').readAsStringSync();
  if (!generated.contains('// Header dart')) {
    return 'Expected header snippet was not injected';
  }
  if (!generated.contains('// Footer dart')) {
    return 'Expected footer snippet was not injected';
  }
  return null;
}

String? testLoadAndTypeError() {
  final ok1 = DataType1.fromNode(termite.Node.mapping({}));
  final ok2 = DataType2.fromNode(termite.Node.mapping({}));
  if (ok1 is! termite.Ok<DataType1> || ok2 is! termite.Ok<DataType2>) {
    return 'Failed to parse empty mappings';
  }

  final wrong1 = DataType1.fromNode(termite.Node.value('1.0'));
  final wrong2 = DataType2.fromNode(termite.Node.value('1.0'));
  if (wrong1 is! termite.Error<DataType1> || wrong2 is! termite.Error<DataType2>) {
    return 'Expected type parsing errors';
  }
  return null;
}

void main() {
  final code = runTests({
    'testHeaderAndFooterInjected': testHeaderAndFooterInjected,
    'testLoadAndTypeError': testLoadAndTypeError,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
