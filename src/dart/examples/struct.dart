// Generated with the Termite Data Model Generator

// ignore_for_file: no_leading_underscores_for_local_identifiers

import 'termite.dart' as termite;
import 'termite-types.dart';

/// Description
class Struct {
  /// Description for [valueRequired]
  integer valueRequired;

  /// Description for [valueDefault]
  boolean valueDefault = false;

  /// Description for [valueOptional]
  number? valueOptional;

  Struct({
    required this.valueRequired,
    boolean? valueDefault,
    this.valueOptional,
  }) {
    this.valueDefault = valueDefault ?? getDefaultValueDefault();
  }

  /// Gets the default value for [valueDefault]
  static boolean getDefaultValueDefault() {
    final node = termite.Node.value('false');
    return (TermiteNodeParserboolean.fromNode(node) as termite.Ok<boolean>)
        .value;
  }

  /// Constructs a [Struct] from a [termite.Node]
  static termite.Result<Struct> fromNode(termite.Node node) {
    return TermiteNodeParserStruct.fromNode(node);
  }

  /// Converts the [Struct] to a [termite.Node]
  termite.Node toNode() {
    final Map<String, termite.Node?> __preMap = {
      "valueRequired": valueRequired.toNode(),
      "valueDefault": valueDefault.toNode(),
      "valueOptional": valueOptional?.toNode(),
    };
    final Map<String, termite.Node> map = Map.fromEntries(
      __preMap.entries
          .where((entry) => entry.value != null)
          .map((entry) => MapEntry(entry.key, entry.value!)),
    );
    return termite.Node.mapping(map);
  }

  @override
  String toString() =>
      '{valueRequired: $valueRequired, valueDefault: $valueDefault, valueOptional: $valueOptional}';
}

extension TermiteNodeParserStruct on Struct {
  /// Constructs a [Struct] from a [termite.Node] if it fulfills the constraints
  static termite.Result<Struct> fromNode(termite.Node node) {
    if (node is! termite.Mapping) {
      return termite.Result.error(
        'Unable to parse ${node.runtimeType} as a Struct',
        "",
      );
    }

    if (!node.map.containsKey('valueRequired')) {
      return termite.Result.error('Missing field "valueRequired"', "");
    }
    final termite.Result<integer> __valueRequired =
        TermiteNodeParserinteger.fromNode(node.map['valueRequired']!);
    if (__valueRequired is termite.Error<integer>) {
      final newError = __valueRequired.addField('valueRequired');
      return termite.Result.error(newError.error, newError.location);
    }
    final integer valueRequired =
        (__valueRequired as termite.Ok<integer>).value;

    boolean valueDefault = Struct.getDefaultValueDefault();
    if (node.map.containsKey('valueDefault')) {
      final termite.Result<boolean> __valueDefault =
          TermiteNodeParserboolean.fromNode(node.map['valueDefault']!);
      if (__valueDefault is termite.Error<boolean>) {
        final newError = __valueDefault.addField('valueDefault');
        return termite.Result.error(newError.error, newError.location);
      }
      valueDefault = (__valueDefault as termite.Ok<boolean>).value;
    }

    number? valueOptional;
    if (node.map.containsKey('valueOptional')) {
      final termite.Result<number> __valueOptional =
          TermiteNodeParsernumber.fromNode(node.map['valueOptional']!);
      if (__valueOptional is termite.Error<number>) {
        final newError = __valueOptional.addField('valueOptional');
        return termite.Result.error(newError.error, newError.location);
      }
      valueOptional = (__valueOptional as termite.Ok<number>).value;
    }

    return termite.Result.ok(
      Struct(
        valueRequired: valueRequired,
        valueDefault: valueDefault,
        valueOptional: valueOptional,
      ),
    );
  }
}
