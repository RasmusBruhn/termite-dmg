/// @file termite-json.dart
/// @brief The Dart Termite Data Model Generator helper functions for using JSON
/// @version 0.8.0
/// @date 2026-08-09
// ignore_for_file: file_names

library;

import 'dart:convert';
import 'dart:io';
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

/// Parses a JSON file at [path] into a [Node] object.
Result<Node> fromFile(String path) {
  try {
    final str = File(path).readAsStringSync();
    return fromString(str);
  } catch (e) {
    return Result.error('Failed to read JSON file: $e', '');
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

/// Converts a [Node] object into a JSON file at [path].
Result<void> toFile(Node node, String path) {
  final str = toString(node);
  if (!str.isOk()) {
    final error = str.asError();
    return Result.error(error.error, error.location);
  }

  try {
    File(path).writeAsStringSync(str.asOk());
    return Result.ok(null);
  } catch (e) {
    return Result.error('Failed to write JSON file: $e', '');
  }
}
