// Generated with the Termite Data Model Generator

import 'termite.dart' as termite;
import 'termite-types.dart';

/// Description
sealed class Enum {
  Enum();

  /// Constructs a new [Enum] of type TypeEmpty
  factory Enum.newTypeEmpty() = EnumTypeEmpty._;

  /// Constructs a new [Enum] of type TypeInt with a value of [value]
  factory Enum.newTypeInt(integer value) = EnumTypeInt._;

  /// Constructs a [Enum] from a [termite.Node]
  static termite.Result<Enum> fromNode(termite.Node node) {
    return TermiteNodeParserEnum.fromNode(node);
  }

  /// Converts the [Enum] to a [termite.Node]
  termite.Node toNode();
}

class EnumTypeEmpty extends Enum {
  EnumTypeEmpty._();

  @override
  termite.Node toNode() {
    return termite.Node.value('TypeEmpty');
  }

  @override
  String toString() => 'TypeEmpty';
}

class EnumTypeInt extends Enum {
  integer value;

  EnumTypeInt._(this.value);

  @override
  termite.Node toNode() {
    return termite.Node.mapping({'TypeInt': value.toNode()});
  }

  @override
  String toString() => 'TypeInt($value)';
}

extension TermiteNodeParserEnum on Enum {
  /// Constructs a [Enum] from a [termite.Node]
  static termite.Result<Enum> fromNode(termite.Node node) {
    String id;
    if (node is termite.Sequence) {
      return termite.Result.error(
        'Unable to parse ${node.runtimeType} as a Enum',
        '',
      );
    } else if (node is termite.Mapping) {
      if (node.map.length != 1) {
        return termite.Result.error(
          'Unable to parse a Mapping with more or less than 1 entry as an enum',
          '',
        );
      }
      id = node.map.keys.first;
    } else {
      id = (node as termite.Value).value;
    }

    switch (id) {
      case 'TypeEmpty':
        if (node is termite.Value) {
          return termite.Result.ok(Enum.newTypeEmpty());
        }
        return termite.Result.error(
          'Enum type has no data and cannot be constructed from a mapping',
          '.TypeEmpty',
        );
      case 'TypeInt':
        if (node is termite.Mapping) {
          final result = TermiteNodeParserinteger.fromNode(node.map[id]!);
          if (result is termite.Ok<integer>) {
            return termite.Result.ok(Enum.newTypeInt(result.value));
          }
          final newResult = (result as termite.Error).addField('TypeInt');
          return termite.Result.error(newResult.error, newResult.location);
        }
        return termite.Result.error(
          'Enum type has data and cannot be constructed from a value',
          '.TypeInt',
        );
      default:
        return termite.Result.error('Unknown type ($id) for Enum', '');
    }
  }
}
