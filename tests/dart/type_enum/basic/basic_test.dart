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
  if (!int1.isOk()) {
    return 'Failed to parse Int1';
  }
  final okInt1 = int1.asOk().value;
  if (okInt1 != DataType.newInt1(1)) {
    return 'Failed to parse Int1: $okInt1';
  }

  final int2 = DataType.fromNode(
    termite.Node.mapping({'Int2': termite.Node.value('2')}),
  );
  if (!int2.isOk()) {
    return 'Failed to parse Int2';
  }
  final okInt2 = int2.asOk().value;
  if (okInt2 != DataType.newInt2(2)) {
    return 'Failed to parse Int2: $okInt2';
  }

  final float = DataType.fromNode(
    termite.Node.mapping({'Float': termite.Node.value('3.5')}),
  );
  if (!float.isOk()) {
    return 'Failed to parse Float';
  }
  final okFloat = float.asOk().value;
  if (okFloat != DataType.newFloat(3.5)) {
    return 'Failed to parse Float: $okFloat';
  }

  final empty = DataType.fromNode(termite.Node.value('Empty'));
  if (!empty.isOk()) {
    return 'Failed to parse Empty';
  }
  final okEmpty = empty.asOk().value;
  if (okEmpty != DataType.newEmpty()) {
    return 'Failed to parse Empty: $okEmpty';
  }

  return null;
}

String? testLoadObject() {
  final int1 = DataType.fromObject({'Int1': 1});
  if (!int1.isOk()) {
    return 'Failed to parse Int1';
  }
  final okInt1 = int1.asOk().value;
  if (okInt1 != DataType.newInt1(1)) {
    return 'Failed to parse Int1: $okInt1';
  }

  final int2 = DataType.fromObject({'Int2': 2});
  if (!int2.isOk()) {
    return 'Failed to parse Int2';
  }
  final okInt2 = int2.asOk().value;
  if (okInt2 != DataType.newInt2(2)) {
    return 'Failed to parse Int2: $okInt2';
  }

  final float = DataType.fromObject({'Float': 3.5});
  if (!float.isOk()) {
    return 'Failed to parse Float';
  }
  final okFloat = float.asOk().value;
  if (okFloat != DataType.newFloat(3.5)) {
    return 'Failed to parse Float: $okFloat';
  }

  final empty = DataType.fromObject('Empty');
  if (!empty.isOk()) {
    return 'Failed to parse Empty';
  }
  final okEmpty = empty.asOk().value;
  if (okEmpty != DataType.newEmpty()) {
    return 'Failed to parse Empty: $okEmpty';
  }

  return null;
}

String? testInvalid() {
  final wrong1 = DataType.fromNode(termite.Node.value('Int1'));
  if (wrong1.isOk()) {
    return 'Expected enum parsing error for missing value';
  }

  final wrong2 = DataType.fromNode(
    termite.Node.mapping({'Empty': termite.Node.value('3.5')}),
  );
  if (wrong2.isOk()) {
    return 'Expected enum parsing error for unused value in Empty';
  }

  final wrong3 = DataType.fromNode(termite.Node.value('Unknown'));
  if (wrong3.isOk()) {
    return 'Expected enum parsing error for unknown enum value';
  }
  return null;
}

String? testInvalidObject() {
  final wrong1 = DataType.fromObject('Int1');
  if (wrong1.isOk()) {
    return 'Expected enum parsing error for missing value';
  }

  final wrong2 = DataType.fromObject({'Empty': 3.5});
  if (wrong2.isOk()) {
    return 'Expected enum parsing error for unused value in Empty';
  }

  final wrong3 = DataType.fromObject('Unknown');
  if (wrong3.isOk()) {
    return 'Expected enum parsing error for unknown enum value';
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
    final okLoaded = loaded.asOk().value;
    if (okLoaded != value) {
      return 'Reloaded enum mismatch: $value vs $okLoaded';
    }
  }
  return null;
}

void main() {
  final code = runTests({
    'testLoad': testLoad,
    'testLoadObject': testLoadObject,
    'testInvalid': testInvalid,
    'testInvalidObject': testInvalidObject,
    'testRoundtrip': testRoundtrip,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
