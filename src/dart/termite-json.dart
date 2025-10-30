/// @file termite-json.dart
/// @brief The Dart Termite Data Model Generator helper functions for using JSON
/// @version 0.7.0
/// @date 2025-10-30
// ignore_for_file: file_names

library;

import 'dart:convert';
import 'termite.dart';

/// Parses a JSON [String] into a [Node] object.
Result<Node> fromString(String str) {
  try {
    final dynamic decoded = json.decode(str);
    return Result.ok(Node.parse(decoded));
  } catch (e) {
    return Result.error('Failed to decode JSON: $e', '');
  }
}

/// Converts a [Node] object back into a JSON [String].
Result<String> toString(Node node) {
  final obj = node.toObject();
  try {
    return Result.ok(json.encode(obj));
  } catch (e) {
    return Result.error('Failed to encode JSON: $e', '');
  }
}
