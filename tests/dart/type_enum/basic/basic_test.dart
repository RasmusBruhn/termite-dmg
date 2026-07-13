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
  final int1 = DataType.fromNode(termite.Node.mapping({'Int1': termite.Node.value('1')}));
  if (int1 is! termite.Ok<DataType> || int1.value is! DataTypeTypeInt1) {
    return 'Failed to parse Int1';
  }

  final int2 = DataType.fromNode(termite.Node.mapping({'Int2': termite.Node.value('1')}));
  if (int2 is! termite.Ok<DataType> || int2.value is! DataTypeTypeInt2) {
    return 'Failed to parse Int2';
  }

  final float = DataType.fromNode(termite.Node.mapping({'Float': termite.Node.value('3.5')}));
  if (float is! termite.Ok<DataType> || float.value is! DataTypeTypeFloat) {
    return 'Failed to parse Float';
  }

  final empty = DataType.fromNode(termite.Node.value('Empty'));
  if (empty is! termite.Ok<DataType> || empty.value is! DataTypeTypeEmpty) {
    return 'Failed to parse Empty';
  }
  return null;
}

String? testInvalid() {
  final wrong1 = DataType.fromNode(termite.Node.value('Int1'));
  final wrong2 = DataType.fromNode(termite.Node.mapping({'Empty': termite.Node.value('3.5')}));
  final wrong3 = DataType.fromNode(termite.Node.value('Unknown'));
  if (wrong1 is! termite.Error<DataType> || wrong2 is! termite.Error<DataType> || wrong3 is! termite.Error<DataType>) {
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
    if (loaded is! termite.Ok<DataType>) {
      return 'Failed to reload enum value: $value';
    }
    if (loaded.value.toString() != value.toString()) {
      return 'Reloaded enum mismatch: $value vs ${loaded.value}';
    }
  }
  return null;
}

void main() {
  final code = runTests({'testLoad': testLoad, 'testInvalid': testInvalid, 'testRoundtrip': testRoundtrip});
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
