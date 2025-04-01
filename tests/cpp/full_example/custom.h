#ifndef CUSTOM_H_INCLUDED
#define CUSTOM_H_INCLUDED

#include <algorithm>
#include <string>

namespace custom {

[[nodiscard]] inline bool is_valid_version_string(const std::string &x) {
  // Example validation: check if the string is not empty and contains only
  // digits and dots
  return !x.empty() &&
         std::all_of(x.begin(), x.end(),
                     [](char c) { return std::isdigit(c) || c == '.'; }) &&
         x.find("..") == std::string::npos && x[0] != '.' &&
         x[x.size() - 1] != '.' && std::count(x.cbegin(), x.cend(), '.') <= 2;
}

}  // namespace custom

#endif