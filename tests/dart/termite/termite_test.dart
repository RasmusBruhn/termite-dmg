import 'generated/termite.dart' as termite;
import 'generated/termite-types.dart';

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

String? testErrorFormatting() {
  final base = const termite.Result.error('Message', '') as termite.Error;
  final withField = base.addField('field1');
  final withIndex = withField.addIndex('2');
  if (withIndex.location != '[2].field1') {
    return 'Wrong location: ${withIndex.location}';
  }
  if (withIndex.getMessage() != '[2].field1: Message') {
    return 'Wrong message: ${withIndex.getMessage()}';
  }
  return null;
}

String? testResultAccessors() {
  const ok = termite.Result.ok(123);
  if (ok.asOk() != 123) {
    return 'Result.asOk() returned wrong value';
  }

  const err = termite.Result<int>.error('failure', '.x');
  if (err.asError().error != 'failure' || err.asError().location != '.x') {
    return 'Result.asError() returned wrong value';
  }
  return null;
}

String? testNodeParse() {
  final scalar = termite.Node.parse(12);
  if (scalar is! termite.Value || scalar.value != '12') {
    return 'Failed to parse scalar';
  }

  final sequence = termite.Node.parse([1, 2]);
  if (sequence is! termite.Sequence || sequence.values.length != 2) {
    return 'Failed to parse list';
  }

  final mapping = termite.Node.parse({'a': 1, 'b': '2'});
  if (mapping is! termite.Mapping || mapping.map.length != 2) {
    return 'Failed to parse map';
  }
  return null;
}

String? testPrimitiveParsing() {
  final parsedInteger = TermiteNodeParserinteger.fromNode(termite.Node.value('123'));
  if (parsedInteger is! termite.Ok<integer> || parsedInteger.value != 123) {
    return 'Failed to parse integer';
  }

  final invalidInteger = TermiteNodeParserinteger.fromNode(termite.Node.value('12.5'));
  if (invalidInteger is! termite.Error<integer>) {
    return 'Expected invalid integer to fail';
  }

  final wrongType = TermiteNodeParserinteger.fromNode(termite.Node.mapping({}));
  if (wrongType is! termite.Error<integer>) {
    return 'Expected mapping to fail integer parsing';
  }
  return null;
}

void main() {
  final code = runTests({
    'testErrorFormatting': testErrorFormatting,
    'testResultAccessors': testResultAccessors,
    'testNodeParse': testNodeParse,
    'testPrimitiveParsing': testPrimitiveParsing,
  });
  if (code != 0) {
    throw Exception('test failure code: $code');
  }
}
