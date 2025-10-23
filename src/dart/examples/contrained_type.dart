// Generated with the Termite Data Model Generator

import 'termite.dart' as termite;
import 'termite-types.dart';

/// A constrained type of [integer] which must fulfill the constraints
///
/// - x > 0
class PositiveInt {
  integer _value;

  PositiveInt._(this._value);

  integer get value => _value;
  set value(integer x) {
    final validation = validate(x);
    if (validation is termite.Error<void>) {
      throw ArgumentError(validation.error);
    }
    _value = x;
  }

  /// Constructs a [PositiveInt] from a [int] if it fulfills the constraints
  ///
  /// - x > 0
  static termite.Result<PositiveInt> fromValue(integer x) {
    final validation = validate(x);
    if (validation is termite.Error<void>) {
      return termite.Result.error(validation.error, validation.location);
    }
    return termite.Result.ok(PositiveInt._(x));
  }

  /// Constructs a [PositiveInt] from a [termite.Node] if it fulfills the constraints
  static termite.Result<PositiveInt> fromNode(termite.Node node) {
    return TermiteNodeParserPositiveInt.fromNode(node);
  }

  /// Converts the [PositiveInt] to a [termite.Node]
  termite.Node toNode() {
    return _value.toNode();
  }

  /// Validates that [x] fullfills the constraints:
  ///
  /// - x > 0
  static termite.Result<void> validate(integer x) {
    if (!(x > 0)) {
      return termite.Result.error('x > 0', '');
    }
    return termite.Result.ok(null);
  }

  @override
  String toString() => '$_value';
}

extension TermiteNodeParserPositiveInt on PositiveInt {
  /// Constructs a [PositiveInt] from a [termite.Node] if it fulfills the constraints
  static termite.Result<PositiveInt> fromNode(termite.Node node) {
    final value = TermiteNodeParserinteger.fromNode(node);
    if (value is termite.Error<integer>) {
      return termite.Result.error(value.error, value.location);
    }
    return PositiveInt.fromValue((value as termite.Ok<integer>).value);
  }
}

/// A constrained type of [PositiveInt] which must fulfill the constraints
///
/// - x.value < 100
class PercentageInt {
  PositiveInt _value;

  PercentageInt._(this._value);

  PositiveInt get value => _value;
  set value(PositiveInt x) {
    final validation = validate(x);
    if (validation is termite.Error<void>) {
      throw ArgumentError(validation.error);
    }
    _value = x;
  }

  /// Constructs a [PercentageInt] from a [PositiveInt] if it fulfills the constraints
  ///
  /// - x.value < 100
  static termite.Result<PercentageInt> fromValue(PositiveInt x) {
    final validation = validate(x);
    if (validation is termite.Error<void>) {
      return termite.Result.error(validation.error, validation.location);
    }
    return termite.Result.ok(PercentageInt._(x));
  }

  /// Constructs a [PercentageInt] from a [termite.Node] if it fulfills the constraints
  static termite.Result<PercentageInt> fromNode(termite.Node node) {
    return TermiteNodeParserPercentageInt.fromNode(node);
  }

  /// Converts the [PercentageInt] to a [termite.Node]
  termite.Node toNode() {
    return _value.toNode();
  }

  /// Validates that [x] fullfills the constraints:
  ///
  /// - x.value < 100
  static termite.Result<void> validate(PositiveInt x) {
    if (!(x.value < 100)) {
      return termite.Result.error('x.value < 100', '');
    }
    return termite.Result.ok(null);
  }

  @override
  String toString() => '$_value';
}

extension TermiteNodeParserPercentageInt on PercentageInt {
  /// Constructs a [PercentageInt] from a [termite.Node] if it fulfills the constraints
  static termite.Result<PercentageInt> fromNode(termite.Node node) {
    final value = PositiveInt.fromNode(node);
    if (value is termite.Error<PositiveInt>) {
      return termite.Result.error(value.error, value.location);
    }
    return PercentageInt.fromValue((value as termite.Ok<PositiveInt>).value);
  }
}
