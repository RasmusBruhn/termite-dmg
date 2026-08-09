/// @file termite-yaml.dart
/// @brief The Dart Termite Data Model Generator helper functions for using YAML
/// @version 0.8.0
/// @date 2026-08-09
// ignore_for_file: file_names

library;

import 'dart:io';
import 'package:yaml_writer/yaml_writer.dart';
import 'package:yaml/yaml.dart';
import 'termite.dart';

/// Parses a YAML [String] into a [Node] object.
Result<Node> fromString(String str) {
  try {
    final dynamic decoded = loadYaml(str);
    return Result.ok(Node.parse(_convertYamlToObject(decoded)));
  } catch (e) {
    return Result.error('Failed to decode YAML: $e', '');
  }
}

/// Parses a YAML file at [path] into a [Node] object.
Result<Node> fromFile(String path) {
  try {
    final str = File(path).readAsStringSync();
    return fromString(str);
  } catch (e) {
    return Result.error('Failed to read YAML file: $e', '');
  }
}

/// Converts a [Node] object back into a YAML [String].
Result<String> toString(Node node) {
  final obj = node.toObject();
  try {
    return Result.ok(YamlWriter().write(obj));
  } catch (e) {
    return Result.error('Failed to encode YAML: $e', '');
  }
}

/// Converts a [Node] object into a YAML file at [path].
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
    return Result.error('Failed to write YAML file: $e', '');
  }
}

/// Converts an object from yaml.dart into a normal Dart object
dynamic _convertYamlToObject(dynamic yaml) {
  if (yaml is YamlMap) {
    return yaml.map(
      (key, value) => MapEntry(key.toString(), _convertYamlToObject(value)),
    );
  } else if (yaml is YamlList) {
    return yaml.map(_convertYamlToObject).toList();
  } else {
    return yaml;
  }
}
