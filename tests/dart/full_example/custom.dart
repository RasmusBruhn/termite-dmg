bool is_valid_version_string(String x) {
  final validChars = RegExp(r'^[0-9.]+$');
  if (x.isEmpty || !validChars.hasMatch(x)) {
    return false;
  }
  if (x.contains('..') || x.startsWith('.') || x.endsWith('.')) {
    return false;
  }
  return '.'.allMatches(x).length <= 2;
}
