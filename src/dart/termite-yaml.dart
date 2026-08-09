/// @file termite-yaml.dart
/// @brief The Dart Termite Data Model Generator helper functions for using YAML
/// @version 0.7.0
/// @date 2026-08-09
// ignore_for_file: file_names

library;

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

/// Converts a [Node] object back into a YAML [String].
Result<String> toString(Node node) {
  final obj = node.toObject();
  try {
    return Result.ok(YamlWriter().write(obj));
  } catch (e) {
    return Result.error('Failed to encode YAML: $e', '');
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
