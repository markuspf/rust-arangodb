#pragma once
#include <memory>
#include <string>

#include "rust/cxx.h"

namespace arangodb::aql::native {
auto parse_query_string(rust::String query_string) -> rust::String;
}
