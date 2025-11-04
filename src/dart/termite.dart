/// @file termite.dart
/// @brief The Dart Termite Data Model Generator code which implements errors and
/// input output to yaml and json
/// @version 0.7.0
/// @date 2025-10-20
library;

/// A result of an operation that can either be successful ([Ok]) or result in an error ([Error]).
///
/// The [Error] class holds information about what error occured and where it occured.
sealed class Result<T> {
  const Result();

  /// Creates a successful [Result], completed with the specified value [value].
  const factory Result.ok(T value) = Ok._;

  /// Creates an error [Result], completed with the specified error [error] at the location [location].
  const factory Result.error(String error, String location) = Error._;
}

/// A successful [Result] with a returned value [value].
final class Ok<T> extends Result<T> {
  /// The returned value of this result.
  final T value;

  const Ok._(this.value);

  @override
  String toString() => 'Result<$T>.ok($value)';
}

/// An error [Result] with a resulting error [error] at a location [location].
final class Error<T> extends Result<T> {
  /// The resulting error of this result.
  final String error;
  final String location;

  const Error._(this.error, this.location);

  /// Adds a [field] to the error's location.
  Error<T> addField(String field) {
    return Error<T>._(error, '.$field$location');
  }

  /// Adds an [index] to the error's location.
  Error<T> addIndex(String index) {
    return Error<T>._(error, '[$index]$location');
  }

  /// Returns a formatted error message including the location if available.
  String getMessage() {
    return location.isNotEmpty ? '$location: $error' : error;
  }

  @override
  String toString() {
    return 'Result<$T>.error(${getMessage()})';
  }
}

/// A node object which can either be a [Value], a [Sequence] or a [Mapping].
sealed class Node {
  Node();

  /// Creates a [Node] of type [Value] holding a [String] value.
  factory Node.value(String value) = Value._;

  /// Creates a [Node] of type [Sequence] holding a list of other [Node] values.
  factory Node.sequence(List<Node> values) = Sequence._;

  /// Creates a [Node] of type [Mapping] holding a map of [String] keys to other [Node] values.
  factory Node.mapping(Map<String, Node> map) = Mapping._;

  /// Parses a dynamic [value] into a [Node] object.
  static Node parse(dynamic value) {
    // Attempt to parse as a sequence
    final sequenceResult = Sequence.tryParse(value);
    if (sequenceResult is Ok<Node>) {
      return (sequenceResult as Ok<Node>).value;
    }

    // Attempt t0 parse as a mapping
    final mappingResult = Mapping.tryParse(value);
    if (mappingResult is Ok<Node>) {
      return (mappingResult as Ok<Node>).value;
    }

    // Fallback to parsing as a value
    return Value.parse(value);
  }

  /// Converts the [Node] to a dynamic object.
  dynamic toObject();
}

/// A [Node] storing a [String].
class Value extends Node {
  /// The stored [String].
  String value = '';

  /// Creates a [Value] with the given [value].
  Value._(this.value);

  /// Parses a dynamic [value] into a [Value] node.
  static Value parse(dynamic value) {
    return Value._(value.toString());
  }

  @override
  dynamic toObject() {
    return value;
  }

  @override
  String toString() {
    return 'Value($value)';
  }
}

/// A [Node] storing a [List] of other [Node] objects.
class Sequence extends Node {
  /// The stored [List] of values.
  List<Node> values = [];

  /// Creates a [Sequence] with the given [values].
  Sequence._(this.values);

  /// Tries to parse a dynamic [value] into a [Sequence].
  static Result<Sequence> tryParse(dynamic value) {
    if (value is List<dynamic>) {
      return Result.ok(Sequence._(value.map((e) => Node.parse(e)).toList()));
    }
    return const Result.error('Invalid sequence', '');
  }

  @override
  dynamic toObject() {
    return values.map((e) => e.toObject()).toList();
  }

  @override
  String toString() {
    return 'Sequence($values)';
  }
}

/// A [Node] storing a [Map] of [String] keys to other [Node] objects.
class Mapping extends Node {
  /// The stored [Map] of [String] keys to [Node] values.
  Map<String, Node> map = {};

  /// Creates a [Mapping] with the given [map].
  Mapping._(this.map);

  /// Tries to parse a dynamic [value] into a [Mapping] node.
  static Result<Mapping> tryParse(dynamic value) {
    if (value is Map<String, dynamic>) {
      return Result.ok(
        Mapping._(value.map((key, value) => MapEntry(key, Node.parse(value)))),
      );
    }
    return const Result.error('Invalid mapping', '');
  }

  @override
  dynamic toObject() {
    return map.map((key, value) => MapEntry(key, value.toObject()));
  }

  @override
  String toString() {
    return 'Mapping($map)';
  }
}
