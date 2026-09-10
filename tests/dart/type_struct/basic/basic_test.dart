import 'generated/basic.dart';
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
  if (!one.isOk()) {
    return 'Failed to load empty struct for DataType1: ${one.asError().getMessage()}';
  }

  final two = DataType2.fromNode(termite.Node.mapping({}));
  if (!two.isOk()) {
    return 'Failed to load empty struct for DataType2: ${two.asError().getMessage()}';
  }

  return null;
}

String? testEmptyStructsObject() {
  final one = DataType1.fromObject({});
  if (!one.isOk()) {
    return 'Failed to load empty struct for DataType1: ${one.asError().getMessage()}';
  }

  final two = DataType2.fromObject({});
  if (!two.isOk()) {
    return 'Failed to load empty struct for DataType2: ${two.asError().getMessage()}';
  }

  return null;
}

String? testWrongStructs() {
  final wrongOne = DataType1.fromNode(termite.Node.value('1.0'));
  if (wrongOne.isOk()) {
    return 'Expected type errors for non-mapping node for DataType1: ${wrongOne.getOk()}';
  }

  final wrongTwo = DataType2.fromNode(termite.Node.value('1.0'));
  if (wrongTwo.isOk()) {
    return 'Expected type errors for non-mapping node for DataType2: ${wrongTwo.getOk()}';
  }

  return null;
}

String? testWrongStructsObject() {
  final wrongOne = DataType1.fromObject('1.0');
  if (wrongOne.isOk()) {
    return 'Expected type errors for non-mapping node for DataType1: ${wrongOne.getOk()}';
  }

  final wrongTwo = DataType2.fromObject('1.0');
  if (wrongTwo.isOk()) {
    return 'Expected type errors for non-mapping node for DataType2: ${wrongTwo.getOk()}';
  }

  return null;
}

void main() {
  final code = runTests({
    'testEmptyStructs': testEmptyStructs,
    'testEmptyStructsObject': testEmptyStructsObject,
    'testWrongStructs': testWrongStructs,
    'testWrongStructsObject': testWrongStructsObject,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
