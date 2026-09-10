/// @file termite-types.dart
/// @brief The Dart Termite Data Model Generator helper code which implements JSON Schema types in Dart
/// @version 0.8.0
/// @date 2026-08-12
// ignore_for_file: file_names

library;

// ignore_for_file: camel_case_types

import 'termite.dart' show Node, Result, Value;

typedef boolean = bool;
typedef integer = int;
typedef number = double;
typedef string = String;

extension TermiteExtensionboolean on boolean {
  /// Constructs a [boolean] from an [Object]
  static Result<boolean> fromObject(Object obj) {
    if (obj is boolean) {
      return Result.ok(obj);
    }
    return Result.error('Unable to parse ${obj.runtimeType} as a boolean', '');
  }

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

  /// Converts the [boolean] to a [Node]
  Node toNode() {
    return Node.value(toString());
  }
}

extension TermiteExtensioninteger on integer {
  /// Constructs a [integer] from an [Object]
  static Result<integer> fromObject(Object obj) {
    if (obj is integer) {
      return Result.ok(obj);
    }
    return Result.error('Unable to parse ${obj.runtimeType} as an integer', '');
  }

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

  /// Converts the [integer] to a [Node]
  Node toNode() {
    return Node.value(toString());
  }
}

extension TermiteExtensionnumber on number {
  /// Constructs a [number] from an [Object]
  static Result<number> fromObject(Object obj) {
    if (obj is number) {
      return Result.ok(obj);
    }
    if (obj is integer) {
      return Result.ok(obj.toDouble());
    }
    return Result.error('Unable to parse ${obj.runtimeType} as a number', '');
  }

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

  /// Converts the [number] to a [Node]
  Node toNode() {
    return Node.value(toString());
  }
}

extension TermiteExtensionstring on string {
  /// Constructs a [string] from an [Object]
  static Result<string> fromObject(Object obj) {
    return Result.ok(obj.toString());
  }

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

  /// Converts the [string] to a [Node]
  Node toNode() {
    return Node.value(this);
  }
}
