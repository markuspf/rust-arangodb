#include "rust-arangodb/include/aql.h"

#include <iostream>
#include <memory>
#include <string>

namespace arangodb::aql::native {
auto parse_query_string(rust::String query_string) -> rust::String {
  std::cout << "been there, doing that";
  return std::string{R"aql({"typeID": 5, "type": "foo"})aql"};
}
} // namespace arangodb::aql::native
