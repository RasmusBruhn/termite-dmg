// Generated with the Termite Data Model Generator

import 'termite.dart' as termite;
import 'termite-types.dart';

/// Description
class Array {
  List<integer> values;

  Array(this.values);

  /// Constructs a [Array] from a [termite.Node]
  static termite.Result<Array> fromNode(termite.Node node) {
    return TermiteNodeParserArray.fromNode(node);
  }

  /// Converts the [Struct] to a [termite.Node]
  termite.Node toNode() {
    final list = values.map((element) => element.toNode()).toList();
    return termite.Node.sequence(list);
  }

  @override
  String toString() => '$values';
}

extension TermiteNodeParserArray on Array {
  /// Constructs a [Array] from a [termite.Node] if it fulfills the constraints
  static termite.Result<Array> fromNode(termite.Node node) {
    if (node is! termite.Sequence) {
      return termite.Result.error(
        'Unable to parse ${node.runtimeType} as a Array',
        "",
      );
    }

    termite.Result<List<integer>> values = node.values
        .map((node) => TermiteNodeParserinteger.fromNode(node))
        .indexed
        .fold(termite.Result.ok([]), (acc, result) {
          if (acc is termite.Error) return acc;
          if (result.$2 is termite.Error) {
            final newError = (result.$2 as termite.Error).addIndex(
              '${result.$1}',
            );
            return termite.Result.error(newError.error, newError.location);
          }
          List<integer> list = (acc as termite.Ok<List<integer>>).value;
          list.add((result.$2 as termite.Ok<integer>).value);
          return termite.Result.ok(list);
        });
    if (values is termite.Error<List<integer>>) {
      return termite.Result.error(values.error, values.location);
    }
    return termite.Result.ok(
      Array((values as termite.Ok<List<integer>>).value),
    );
  }
}
