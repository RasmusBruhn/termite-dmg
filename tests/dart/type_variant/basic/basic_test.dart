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
  final intValue = DataType.fromNode(termite.Node.value('1'));
  if (!intValue.isOk()) {
    return 'Failed to parse integer variant';
  }
  final intValueOk = intValue.getOk();
  if (intValueOk is! DataTypeTypeinteger) {
    return 'Failed to parse integer variant: $intValueOk';
  }

  final floatValue = DataType.fromNode(termite.Node.value('1.5'));
  if (!floatValue.isOk()) {
    return 'Failed to parse number variant';
  }
  final floatValueOk = floatValue.getOk();
  if (floatValueOk is! DataTypeTypenumber) {
    return 'Failed to parse number variant: $floatValueOk';
  }
  return null;
}

String? testError() {
  final invalidValue = DataType.fromNode(termite.Node.value('invalid'));
  if (invalidValue.isOk()) {
    return 'Parsed invalid variant successfully: ${invalidValue.asOk()}';
  }
  return null;
}

String? testRoundtrip() {
  final values = <DataType>[DataType.newinteger(1), DataType.newnumber(1.5)];
  for (final value in values) {
    final loaded = DataType.fromNode(value.toNode());
    if (!loaded.isOk()) {
      return 'Failed to reload variant value: $value with error: ${loaded.asError().getMessage()}';
    }
    if (loaded.getOk() != value) {
      return 'Reloaded variant mismatch for value: $value with loaded: ${loaded.getOk()}';
    }
  }
  return null;
}

void main() {
  final code = runTests({
    'testLoad': testLoad,
    'testError': testError,
    'testRoundtrip': testRoundtrip,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
