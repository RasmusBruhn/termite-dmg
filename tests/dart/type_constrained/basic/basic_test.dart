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

String? testLoad() {
  final loaded = DataType.fromNode(termite.Node.value('1'));
  if (loaded is! termite.Ok<DataType>) {
    return 'Failed to parse constrained type';
  }
  if (loaded.value.value != 1) {
    return 'Wrong constrained value';
  }
  return null;
}

String? testInvalid() {
  final invalidValue = DataType.fromNode(termite.Node.value('1.0'));
  if (invalidValue is! termite.Error<DataType>) {
    return 'Expected value parse error';
  }

  final invalidType = DataType.fromNode(termite.Node.sequence([termite.Node.value('1')]));
  if (invalidType is! termite.Error<DataType>) {
    return 'Expected node type parse error';
  }
  return null;
}

String? testRoundtrip() {
  final value = (DataType.fromValue(1) as termite.Ok<DataType>).value;
  final loaded = DataType.fromNode(value.toNode());
  if (loaded is! termite.Ok<DataType> || loaded.value.value != 1) {
    return 'Failed constrained roundtrip';
  }
  return null;
}

void main() {
  final code = runTests({'testLoad': testLoad, 'testInvalid': testInvalid, 'testRoundtrip': testRoundtrip});
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
