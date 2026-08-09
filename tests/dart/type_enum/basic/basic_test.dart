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
  final int1 = DataType.fromNode(
    termite.Node.mapping({'Int1': termite.Node.value('1')}),
  );
  if (!int1.isOk() || int1.asOk() is! DataTypeTypeInt1) {
    return 'Failed to parse Int1';
  }

  final int2 = DataType.fromNode(
    termite.Node.mapping({'Int2': termite.Node.value('1')}),
  );
  if (!int2.isOk() || int2.asOk() is! DataTypeTypeInt2) {
    return 'Failed to parse Int2';
  }

  final float = DataType.fromNode(
    termite.Node.mapping({'Float': termite.Node.value('3.5')}),
  );
  if (!float.isOk() || float.asOk() is! DataTypeTypeFloat) {
    return 'Failed to parse Float';
  }

  final empty = DataType.fromNode(termite.Node.value('Empty'));
  if (!empty.isOk() || empty.asOk() is! DataTypeTypeEmpty) {
    return 'Failed to parse Empty';
  }
  return null;
}

String? testInvalid() {
  final wrong1 = DataType.fromNode(termite.Node.value('Int1'));
  final wrong2 = DataType.fromNode(
    termite.Node.mapping({'Empty': termite.Node.value('3.5')}),
  );
  final wrong3 = DataType.fromNode(termite.Node.value('Unknown'));
  if (wrong1.isOk() || wrong2.isOk() || wrong3.isOk()) {
    return 'Expected enum parsing errors';
  }
  return null;
}

String? testRoundtrip() {
  final values = <DataType>[
    DataType.newInt1(1),
    DataType.newInt2(2),
    DataType.newFloat(3.5),
    DataType.newEmpty(),
  ];
  for (final value in values) {
    final loaded = DataType.fromNode(value.toNode());
    if (!loaded.isOk()) {
      return 'Failed to reload enum value: $value';
    }
    if (loaded.asOk() != value) {
      return 'Reloaded enum mismatch: $value vs ${loaded.asOk()}';
    }
  }
  return null;
}

void main() {
  final code = runTests({
    'testLoad': testLoad,
    'testInvalid': testInvalid,
    'testRoundtrip': testRoundtrip,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
