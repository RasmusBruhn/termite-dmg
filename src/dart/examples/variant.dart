// Generated with the Termite Data Model Generator

// ignore_for_file: no_leading_underscores_for_local_identifiers

import 'termite.dart' as termite;
import 'termite-types.dart';

/// Description
sealed class Variant {
  Variant();

  /// Constructs a new [Variant] of type [integer] with a value of [value]
  factory Variant.newinteger(integer value) = VariantTypeinteger._;

  /// Constructs a new [Variant] of type [number] with a value of [value]
  factory Variant.newnumber(number value) = VariantTypenumber._;

  /// Constructs a [Variant] from a [termite.Node]
  static termite.Result<Variant> fromNode(termite.Node node) {
    return TermiteNodeParserVariant.fromNode(node);
  }

  /// Converts the [Variant] to a [termite.Node]
  termite.Node toNode();
}

class VariantTypeinteger extends Variant {
  /// The stored variant value of type [integer]
  integer value;

  VariantTypeinteger._(this.value);

  @override
  termite.Node toNode() {
    return value.toNode();
  }

  @override
  String toString() => '$value';
}

class VariantTypenumber extends Variant {
  /// The stored variant value of type [number]
  number value;

  VariantTypenumber._(this.value);

  @override
  termite.Node toNode() {
    return value.toNode();
  }

  @override
  String toString() => '$value';
}

extension TermiteNodeParserVariant on Variant {
  /// Constructs a [Variant] from a [termite.Node]
  static termite.Result<Variant> fromNode(termite.Node node) {
    termite.Result<integer> __integer = TermiteNodeParserinteger.fromNode(node);
    if (__integer is termite.Ok<integer>) {
      return termite.Result.ok(Variant.newinteger(__integer.value));
    }
    __integer = (__integer as termite.Error<integer>).addField('integer');

    termite.Result<number> __number = TermiteNodeParsernumber.fromNode(node);
    if (__number is termite.Ok<number>) {
      return termite.Result.ok(Variant.newnumber(__number.value));
    }
    __number = (__number as termite.Error<number>).addField('number');

    return termite.Result.error(
      '[{${__integer.getMessage()}}, {${__number.getMessage()}}]',
      '',
    );
  }
}
