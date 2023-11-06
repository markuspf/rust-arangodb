#[cxx::bridge(namespace = "arangodb::aql::native")]
mod ffi {
    unsafe extern "C++" {
        include!("rust-arangodb/include/aql.h");

        fn parse_query_string(query: String) -> String;
    }
}

#[test]
fn parse_query_test() {
    ffi::parse_query_string("FOR v IN V RETURN v._doc".to_string());
}
