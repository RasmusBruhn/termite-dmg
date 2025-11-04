/// @file termite-types.dart
/// @brief The Dart Termite Data Model Generator helper code which implements JSON Schema types in Dart
/// @version 0.7.0
/// @date 2025-10-30
// ignore_for_file: file_names

library;

// ignore_for_file: camel_case_types

import 'termite.dart' show Node, Result, Value;

typedef boolean = bool;
typedef integer = int;
typedef number = double;
typedef string = String;

extension BooleanExtension on boolean {
  /// Converts the [boolean] to a [Node]
  Node toNode() {
    return Node.value(toString());
  }
}

extension TermiteNodeParserboolean on boolean {
  /// Constructs a [boolean] from a [Node]
  static Result<boolean> fromNode(Node node) {
    if (node is! Value) {
      return Result.error(
        'Unable to parse ${node.runtimeType} as a boolean',
        '',
      );
    }

    if (node.value == '0') {
      return const Result.ok(false);
    }
    if (node.value == '1') {
      return const Result.ok(true);
    }
    final parsed = boolean.tryParse(node.value, caseSensitive: false);
    if (parsed == null) {
      return Result.error('Unable to parse boolean from "${node.value}"', '');
    }
    return Result.ok(parsed);
  }
}

extension IntegerExtension on integer {
  /// Converts the [integer] to a [Node]
  Node toNode() {
    return Node.value(toString());
  }
}

extension TermiteNodeParserinteger on integer {
  /// Constructs a [integer] from a [Node]
  static Result<integer> fromNode(Node node) {
    if (node is! Value) {
      return Result.error(
        'Unable to parse ${node.runtimeType} as an integer',
        '',
      );
    }

    final parsed = integer.tryParse(node.value);
    if (parsed == null) {
      return Result.error('Unable to parse integer from "${node.value}"', '');
    }
    return Result.ok(parsed);
  }
}

extension NumberExtension on number {
  /// Converts the [number] to a [Node]
  Node toNode() {
    return Node.value(toString());
  }
}

extension TermiteNodeParsernumber on number {
  /// Constructs a [number] from a [Node]
  static Result<number> fromNode(Node node) {
    if (node is! Value) {
      return Result.error(
        'Unable to parse ${node.runtimeType} as a number',
        '',
      );
    }

    final parsed = number.tryParse(node.value);
    if (parsed == null) {
      return Result.error('Unable to parse number from "${node.value}"', '');
    }
    return Result.ok(parsed);
  }
}

extension StringExtension on string {
  /// Converts the [string] to a [Node]
  Node toNode() {
    return Node.value(this);
  }
}

extension TermiteNodeParserstring on string {
  /// Constructs a [string] from a [Node]
  static Result<string> fromNode(Node node) {
    if (node is! Value) {
      return Result.error(
        'Unable to parse ${node.runtimeType} as a string',
        '',
      );
    }

    return Result.ok(node.value);
  }
}
